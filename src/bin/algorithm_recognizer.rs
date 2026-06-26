use capstone::prelude::*;
use std::fs;
use std::io::Read;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let binary_path = "/usr/bin/sort";
    let mut file = fs::File::open(&binary_path)?;
    let mut binary_data = Vec::new();
    file.read_to_end(&mut binary_data)?;
    
    let cs = Capstone::new()
        .x86()
        .mode(arch::x86::ArchMode::Mode64)
        .detail(true)
        .build()?;
    
    let text_offset = 0x3040u64;
    let text_size = 0x14203u64;
    let text_section = &binary_data[text_offset as usize..(text_offset as usize + text_size as usize)];
    
    eprintln!("Disassembling {} bytes from file offset 0x{:x}", text_section.len(), text_offset);
    
    let instructions = cs.disasm_all(text_section, text_offset)?;
    
    eprintln!("Total instructions: {}", instructions.len());
    
    let mut call_count = 0;
    let mut cmp_count = 0;
    let mut xchg_count = 0;
    
    for instr in instructions.iter() {
        let mnem = instr.mnemonic().unwrap_or("");
        if mnem == "call" { call_count += 1; }
        if mnem.starts_with("cmp") { cmp_count += 1; }
        if mnem == "xchg" { xchg_count += 1; }
    }
    
    eprintln!("Calls: {}, Cmps: {}, Xchgs: {}", call_count, cmp_count, xchg_count);
    
    for (i, instr) in instructions.iter().enumerate().take(30) {
        println!("0x{:x}: {} {}", instr.address(), instr.mnemonic().unwrap_or("?"), instr.op_str().unwrap_or(""));
    }
    
    Ok(())
}
