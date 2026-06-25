use capstone::prelude::*;
use std::collections::HashMap;
use std::fs;
use std::io::Read;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config_path = "/home/inf/.analysis/config";
    let config = read_config(config_path)?;
    let binary_path = config.get("BINARY_PATH").unwrap_or(&"/bin/md5sum".to_string()).clone();
    let text_offset = parse_hex(config.get("TEXT_OFFSET").unwrap_or(&"0x2040".to_string()))?;
    let text_size = parse_hex(config.get("TEXT_SIZE").unwrap_or(&"0x6693".to_string()))?;
    
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
    
    println!("#include <stdio.h>\n");
    
    let mut func_addrs = Vec::new();
    for instr in instructions.iter() {
        if instr.mnemonic().unwrap_or("") == "endbr64" {
            func_addrs.push(instr.address());
        }
    }
    
    for func_addr in func_addrs.iter() {
        println!("void func_0x{:x}() {{", func_addr);
        
        let indent = "    ";
        
        for instr in instructions.iter() {
            let addr = instr.address();
            if addr < *func_addr || addr >= *func_addr + 500 {
                continue;
            }
            
            let mnem = instr.mnemonic().unwrap_or("");
            let op_str = instr.op_str().unwrap_or("");
            
            match mnem {
                "call" => {
                    if let Some(target) = op_str.strip_prefix("0x") {
                        println!("{}func_0x{}();", indent, target);
                    } else {
                        println!("{}call_helper();", indent);
                    }
                }
                "ret" => {
                    println!("{}return;", indent);
                }
                "add" => {
                    if op_str.contains("[rbp-4], 1") {
                        println!("{}i++;", indent);
                    }
                }
                "mov" if op_str.contains("[rbp-4], 0") => {
                    println!("{}int i = 0;", indent);
                }
                _ => {}
            }
        }
        
        println!("}}\n");
    }
    
    println!("int main() {{");
    println!("    return 0;");
    println!("}}");
    
    Ok(())
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
