use capstone::prelude::*;
use std::collections::HashMap;
use std::fs;
use std::io::Read;

#[derive(Clone, Debug)]
struct Instr {
    addr: u64,
    mnem: String,
    op_str: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config_path = "/home/inf/.analysis/config";
    let config = read_config(config_path)?;
    let binary_path = config.get("BINARY_PATH").unwrap_or(&"/bin/ls".to_string()).clone();
    let text_offset = parse_hex(config.get("TEXT_OFFSET").unwrap_or(&"0x3040".to_string()))?;
    let text_size = parse_hex(config.get("TEXT_SIZE").unwrap_or(&"0x19863".to_string()))?;
    
    let mut file = fs::File::open(&binary_path)?;
    let mut binary_data = Vec::new();
    file.read_to_end(&mut binary_data)?;
    
    let text_section = &binary_data[text_offset as usize..(text_offset as usize + text_size as usize)];
    
    let cs = Capstone::new()
        .x86()
        .mode(arch::x86::ArchMode::Mode64)
        .detail(true)
        .build()?;
    
    let instructions = cs.disasm_all(text_section, text_offset as u64)?;
    
    let mut instrs = Vec::new();
    for instr in instructions.iter() {
        instrs.push(Instr {
            addr: instr.address(),
            mnem: instr.mnemonic().unwrap_or("").to_string(),
            op_str: instr.op_str().unwrap_or("").to_string(),
        });
    }
    
    let loop_headers = detect_loop_headers(&instrs);
    
    println!("#include <stdio.h>");
    println!("#include <stdint.h>");
    println!("#include <string.h>\n");
    
    let mut func_addrs = Vec::new();
    for instr in instrs.iter() {
        if instr.mnem == "endbr64" {
            func_addrs.push(instr.addr);
        }
    }
    
    for (i, func_addr) in func_addrs.iter().enumerate() {
        let next_func = if i + 1 < func_addrs.len() {
            func_addrs[i + 1]
        } else {
            text_offset + text_size
        };
        
        let func_instrs: Vec<&Instr> = instrs.iter()
            .filter(|instr| instr.addr >= *func_addr && instr.addr < next_func)
            .collect();
        
        if func_instrs.is_empty() || func_instrs.len() < 2 {
            continue;
        }
        
        let args = infer_arguments(&func_instrs);
        let sig = if args.is_empty() {
            format!("void func_0x{:x}(void)", func_addr)
        } else {
            format!("void func_0x{:x}({})", func_addr, args.join(", "))
        };
        
        println!("{} {{", sig);
        
        let mut in_loop = false;
        let mut loop_depth = 0;
        let mut last_cmp = None;
        
        let mut j = 0;
        while j < func_instrs.len() {
            let instr = func_instrs[j];
            
            if loop_headers.contains(&instr.addr) && !in_loop {
                println!("{}while (condition) {{", "    ".repeat(loop_depth + 1));
                in_loop = true;
                loop_depth += 1;
            }
            
            match instr.mnem.as_str() {
                "call" => {
                    let indent = "    ".repeat(loop_depth + 1);
                    if let Some(target) = instr.op_str.strip_prefix("0x") {
                        if j > 0 && func_instrs[j-1].mnem == "mov" && func_instrs[j-1].op_str.starts_with("rdi") {
                            let arg = func_instrs[j-1].op_str.split(',').nth(1).unwrap_or("").trim();
                            println!("{}func_0x{}({});", indent, target, arg);
                        } else {
                            println!("{}func_0x{}();", indent, target);
                        }
                    }
                }
                "ret" => {
                    if in_loop {
                        println!("{}}}  // end loop", "    ".repeat(loop_depth));
                        in_loop = false;
                        loop_depth -= 1;
                    }
                    println!("{}return;", "    ".repeat(loop_depth + 1));
                }
                "cmp" => {
                    last_cmp = Some(instr.op_str.clone());
                }
                "jle" | "jne" | "je" | "jl" | "jg" | "jge" => {
                    if let Some(ref cmp) = last_cmp {
                        if cmp.contains("[rbp-4]") || cmp.contains("rax") {
                            // Likely a loop condition, but we handle at header detection
                        }
                    }
                }
                "lea" if instr.op_str.contains("rdi") && instr.op_str.contains("[rip") => {
                    let indent = "    ".repeat(loop_depth + 1);
                    println!("{}// load address from RIP", indent);
                }
                "add" if instr.op_str.contains("[rbp-4], 1") => {
                    let indent = "    ".repeat(loop_depth + 1);
                    println!("{}counter++;", indent);
                }
                _ => {}
            }
            
            j += 1;
        }
        
        if in_loop {
            println!("{}}}  // end loop", "    ".repeat(loop_depth));
        }
        
        println!("}}\n");
    }
    
    println!("int main(int argc, char *argv[]) {{");
    println!("    return 0;");
    println!("}}");
    
    Ok(())
}

fn detect_loop_headers(instrs: &[Instr]) -> std::collections::HashSet<u64> {
    let mut headers = std::collections::HashSet::new();
    
    for instr in instrs {
        let mnem = &instr.mnem;
        if mnem.starts_with('j') && mnem != "jmp" {
            if let Some(target) = instr.op_str.strip_prefix("0x") {
                if let Ok(target_addr) = u64::from_str_radix(target, 16) {
                    if target_addr < instr.addr {
                        headers.insert(target_addr);
                    }
                }
            }
        }
    }
    
    headers
}

fn infer_arguments(instrs: &[&Instr]) -> Vec<String> {
    let mut args = Vec::new();
    let arg_regs = vec!["rdi", "rsi", "rdx", "rcx", "r8", "r9"];
    
    for reg in arg_regs {
        for instr in instrs {
            if (instr.mnem == "mov" || instr.mnem == "lea") && instr.op_str.starts_with(reg) {
                args.push(format!("uint64_t arg_{}", reg));
                break;
            }
        }
    }
    
    args
}

fn read_config(path: &str) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
    let mut config = HashMap::new();
    if let Ok(content) = fs::read_to_string(path) {
        for line in content.lines() {
            if let Some((k, v)) = line.split_once('=') {
                config.insert(k.to_string(), v.to_string());
            }
        }
    }
    Ok(config)
}

fn parse_hex(s: &str) -> Result<u64, Box<dyn std::error::Error>> {
    Ok(u64::from_str_radix(s.trim_start_matches("0x"), 16)?)
}
