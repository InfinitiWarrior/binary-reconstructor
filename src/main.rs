mod insn;
mod ir;
mod lifter;
mod cfg;
mod dataflow;
mod emitter;

use goblin::elf::Elf;
use goblin::elf::section_header::SHF_EXECINSTR;
use capstone::prelude::*;
use std::fs;
use std::io::{self, Read, Write};
use std::env;
use std::collections::HashMap;

use crate::cfg::build_cfg;
use crate::insn::InsnInfo;
use crate::dataflow::{resolve_calls, find_struct_patterns};
use crate::emitter::Emitter;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    let binary_path = if args.len() > 1 {
        args[1].clone()
    } else {
        let mut path = String::new();
        print!("Enter binary path: ");
        io::stdout().flush()?;
        io::stdin().read_line(&mut path)?;
        path.trim().to_string()
    };

    let mut file = fs::File::open(&binary_path)?;
    let mut binary_data = Vec::new();
    file.read_to_end(&mut binary_data)?;

    let elf = Elf::parse(&binary_data)?;

    // ----------------------------------------------------------------
    // Step 1: Build import table (GOT slot VA -> symbol name)
    //
    // Modern hardened binaries use -fno-plt: calls go to [rip + got_offset].
    // .rela.dyn GLOB_DAT entries map GOT slot VAs to symbol names.
    // For binaries with a PLT, we also map PLT stub addresses.
    // ----------------------------------------------------------------
    let mut imports: HashMap<u64, String> = HashMap::new();

    // .rela.dyn entries (GLOB_DAT, no-plt style).
    for reloc in elf.dynrelas.iter() {
        let sym_idx = reloc.r_sym;
        if sym_idx > 0 {
            if let Some(sym) = elf.dynsyms.get(sym_idx) {
                if let Some(name) = elf.dynstrtab.get_at(sym.st_name) {
                    if !name.is_empty() {
                        imports.insert(reloc.r_offset, name.to_string());
                    }
                }
            }
        }
    }
    eprintln!("[*] GLOB_DAT entries (dynrelas): {}", imports.len());

    // .rela.plt entries (PLT-using binaries).
    let mut plt_relocs: Vec<(u64, String)> = Vec::new();
    for reloc in elf.pltrelocs.iter() {
        let sym_idx = reloc.r_sym;
        if sym_idx > 0 {
            if let Some(sym) = elf.dynsyms.get(sym_idx) {
                if let Some(name) = elf.dynstrtab.get_at(sym.st_name) {
                    plt_relocs.push((reloc.r_offset, name.to_string()));
                }
            }
        }
    }
    eprintln!("[*] PLT relocations: {}", plt_relocs.len());

    // Map PLT stub addresses if .plt section exists.
    let plt_base_and_size: Option<(u64, u64)> = elf.section_headers.iter()
        .find(|sh| elf.shdr_strtab.get_at(sh.sh_name).map_or(false, |n| n == ".plt"))
        .map(|sh| (sh.sh_addr, sh.sh_size));

    if let Some((plt_addr, _)) = plt_base_and_size {
        let stub_size: u64 = 16;
        let plt_base = plt_addr + stub_size; // skip resolver stub
        for (i, (_, name)) in plt_relocs.iter().enumerate() {
            let stub_addr = plt_base + i as u64 * stub_size;
            imports.insert(stub_addr, name.clone());
        }
    } else {
        // No .plt: treat pltrelocs offsets as GOT slots directly.
        for (addr, name) in &plt_relocs {
            imports.insert(*addr, name.clone());
        }
    }

    // Dynamic symbol addresses (shared lib exports).
    for sym in elf.dynsyms.iter() {
        if sym.is_function() && sym.st_value != 0 {
            if let Some(name) = elf.dynstrtab.get_at(sym.st_name) {
                if !name.is_empty() {
                    imports.insert(sym.st_value, name.to_string());
                }
            }
        }
    }

    // Static symbol table (unstripped binaries).
    let mut func_names: HashMap<u64, String> = HashMap::new();
    for sym in elf.syms.iter() {
        if sym.is_function() && sym.st_value != 0 {
            if let Some(name) = elf.strtab.get_at(sym.st_name) {
                if !name.is_empty() {
                    func_names.insert(sym.st_value, name.to_string());
                    imports.insert(sym.st_value, name.to_string());
                }
            }
        }
    }

    eprintln!("[*] Imports total: {}", imports.len());
    eprintln!("[*] Named functions (symtab): {}", func_names.len());

    // ----------------------------------------------------------------
    // Step 2: Extract .rodata strings
    // ----------------------------------------------------------------
    let mut rodata: HashMap<u64, String> = HashMap::new();
    for sh in &elf.section_headers {
        if let Some(name) = elf.shdr_strtab.get_at(sh.sh_name) {
            if name == ".rodata" {
                let start = sh.sh_offset as usize;
                let end = start + sh.sh_size as usize;
                if end <= binary_data.len() {
                    extract_strings(&binary_data[start..end], sh.sh_addr, &mut rodata);
                }
            }
        }
    }
    eprintln!("[*] Strings extracted from .rodata: {}", rodata.len());

    // ----------------------------------------------------------------
    // Step 3: Disassemble
    // ----------------------------------------------------------------
    let cs = Capstone::new()
        .x86()
        .mode(arch::x86::ArchMode::Mode64)
        .detail(true)
        .build()?;

    // Collect executable section info as owned values to avoid borrow issues.
    let exec_sections: Vec<(u64, u64, u64)> = elf.section_headers.iter()
        .filter(|sh| sh.sh_flags & (SHF_EXECINSTR as u64) != 0)
        .map(|sh| (sh.sh_addr, sh.sh_size, sh.sh_offset))
        .collect();

    if exec_sections.is_empty() {
        eprintln!("[!] No executable sections found.");
        return Ok(());
    }

    // Discover function boundaries from the binary when symtab is absent.
    // Strategy: disassemble .text, collect all direct call targets within the
    // section and all endbr64 addresses (CET landing pads mark function entries).
    if func_names.is_empty() {
        eprintln!("[*] No symtab; discovering functions from call targets and endbr64...");
        // Scan ALL executable sections, not just the first.
        // Some binaries spread code across .init, .text, .fini, .plt.sec etc.
        for &(sec_addr, sec_size, sec_offset) in &exec_sections {
            eprintln!("[*] exec section: addr={:#x} size={:#x} file_offset={:#x}", sec_addr, sec_size, sec_offset);
            let file_start = sec_offset as usize;
            let file_end = file_start + sec_size as usize;
            if file_end > binary_data.len() {
                eprintln!("[!] Section extends past file, skipping");
                continue;
            }
            if sec_size < 4 { continue; } // skip trivially small sections
            let sec_bytes = &binary_data[file_start..file_end];
            match cs.disasm_all(sec_bytes, sec_addr) {
                Err(e) => eprintln!("[!] Disasm failed: {:?}", e),
                Ok(all_insns) => {
                    let sec_end = sec_addr + sec_size;
                    eprintln!("[*]   {} instructions", all_insns.len());
                    func_names.entry(sec_addr).or_insert_with(|| format!("fn_{:#x}", sec_addr));
                    for insn in all_insns.iter() {
                        let mnemonic = insn.mnemonic().unwrap_or("");
                        if mnemonic == "endbr64" {
                            let addr = insn.address();
                            func_names.entry(addr).or_insert_with(|| format!("fn_{:#x}", addr));
                        }
                        if mnemonic == "call" {
                            if let Ok(detail) = cs.insn_detail(insn) {
                                let arch = detail.arch_detail();
                                if let Some(x86) = arch.x86() {
                                    use capstone::arch::x86::X86OperandType as OpTy;
                                    for op in x86.operands() {
                                        if let OpTy::Imm(target) = op.op_type {
                                            let t = target as u64;
                                            if t >= sec_addr && t < sec_end {
                                                func_names.entry(t).or_insert_with(|| format!("fn_{:#x}", t));
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        eprintln!("[*] Discovered {} functions", func_names.len());
    }

    let functions_to_lift: Vec<(u64, u64, String)> = if func_names.is_empty() {
        let (addr, size, _) = exec_sections[0];
        vec![(addr, size, "text".to_string())]
    } else {
        let mut addrs: Vec<(u64, String)> = func_names.iter()
            .map(|(a, n)| (*a, n.clone()))
            .collect();
        addrs.sort_by_key(|(a, _)| *a);

        let mut result: Vec<(u64, u64, String)> = addrs.windows(2).map(|w| {
            let (addr, name) = &w[0];
            let (next_addr, _) = &w[1];
            (*addr, next_addr - addr, name.clone())
        }).collect();

        if let Some((addr, name)) = addrs.last() {
            let (sec_addr, sec_size, _) = exec_sections[0];
            let end = sec_addr + sec_size;
            let size = end.saturating_sub(*addr);
            if size > 0 && size < 1_000_000 {
                result.push((*addr, size, name.clone()));
            }
        }
        result
    };

    eprintln!("[*] Functions to lift: {}", functions_to_lift.len());

    // ----------------------------------------------------------------
    // Step 4: Lift, analyze, emit
    // ----------------------------------------------------------------
    let mut out = Emitter::new();
    out.emit_header_comment(&binary_path, functions_to_lift.len(), imports.len());

    let mut total_calls = 0usize;

    for (func_addr, func_size, func_name) in &functions_to_lift {
        let section = exec_sections.iter().find(|&&(addr, size, _)| {
            addr <= *func_addr && *func_addr < addr + size
        });
        let (sec_addr, _, sec_offset) = match section {
            Some(&s) => s,
            None => continue,
        };

        let offset_in_section = (func_addr - sec_addr) as usize;
        let file_offset = sec_offset as usize + offset_in_section;
        let size = (*func_size as usize).min(binary_data.len().saturating_sub(file_offset));
        if size == 0 { continue; }

        let func_bytes = &binary_data[file_offset..file_offset + size];
        let insns = cs.disasm_all(func_bytes, *func_addr)?;
        if insns.is_empty() { continue; }

        let mut insn_infos: Vec<InsnInfo> = Vec::new();
        for insn in insns.iter() {
            if let Ok(detail) = cs.insn_detail(insn) {
                insn_infos.push(InsnInfo::extract(insn, &detail));
            }
        }

        if insn_infos.is_empty() { continue; }

        let func_ir = build_cfg(&insn_infos, func_name, &imports, &rodata);
        let resolved = resolve_calls(&func_ir, &rodata);
        let structs = find_struct_patterns(&func_ir);

        total_calls += resolved.len();

        let has_content = !resolved.is_empty()
            || func_ir.blocks.iter().any(|bb| bb.stmts.len() > 1);

        if has_content {
            out.emit_struct_types(&structs);
            out.emit_function(&func_ir, &resolved, &structs);
        }
    }

    eprintln!("[*] Total resolved call sites: {}", total_calls);
    print!("{}", out.finish());
    Ok(())
}

fn extract_strings(data: &[u8], base_addr: u64, out: &mut HashMap<u64, String>) {
    let min_len = 4;
    let mut start = 0;
    for (i, &b) in data.iter().enumerate() {
        if b == 0 {
            let s = &data[start..i];
            if s.len() >= min_len && s.iter().all(|&c| c.is_ascii() && (c >= 0x20 || c == b'\n' || c == b'\t')) {
                let addr = base_addr + start as u64;
                let text = String::from_utf8_lossy(s).to_string();
                out.insert(addr, text);
            }
            start = i + 1;
        }
    }
}