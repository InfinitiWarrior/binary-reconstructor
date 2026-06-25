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
        
        let mut last_was_mov_rdi = false;
        let mut last_mov_rdi_arg = String::new();
        
        let mut i = 0;
        while i < func_instrs.len() {
            let instr = func_instrs[i];
            
            // Pattern: mov rdi, X; call
            if instr.mnem == "mov" && instr.op_str.starts_with("rdi,") {
                last_was_mov_rdi = true;
                last_mov_rdi_arg = instr.op_str.split(',').nth(1).unwrap_or("").trim().to_string();
                i += 1;
                continue;
            }
            
            if instr.mnem == "call" {
                if last_was_mov_rdi {
                    if let Some(target) = instr.op_str.strip_prefix("0x") {
                        println!("    func_0x{}({});", target, last_mov_rdi_arg);
                    }
                    last_was_mov_rdi = false;
                } else {
                    if let Some(target) = instr.op_str.strip_prefix("0x") {
                        println!("    func_0x{}();", target);
                    }
                }
            } else if instr.mnem == "ret" {
                println!("    return;");
            } else if instr.mnem == "lea" && instr.op_str.contains("rdi") && instr.op_str.contains("[rip") {
                println!("    // load string/data");
            } else if instr.mnem == "add" && instr.op_str.contains("[rbp-4], 1") {
                println!("    counter++;");
            } else if instr.mnem == "cmp" && instr.op_str.contains("[rbp") {
                println!("    // condition check");
            }
            
            i += 1;
        }
        
        println!("}}\n");
    }
    
    println!("int main(int argc, char *argv[]) {{");
    println!("    return 0;");
    println!("}}");
    
    Ok(())
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
