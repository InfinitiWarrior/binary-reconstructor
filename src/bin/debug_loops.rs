use capstone::prelude::*;
use std::collections::HashMap;
use std::fs;
use std::io::Read;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = read_config("/home/inf/.analysis/config")?;
    let binary_path = config.get("BINARY_PATH").unwrap_or(&"/bin/yes".to_string()).clone();
    let text_offset = parse_hex(config.get("TEXT_OFFSET").unwrap_or(&"0x2040".to_string()))?;
    let text_size = parse_hex(config.get("TEXT_SIZE").unwrap_or(&"0x5853".to_string()))?;
    
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
    
    println!("=== BACKWARD JUMPS ===");
    let mut count = 0;
    for instr in instructions.iter() {
        if instr.mnemonic().unwrap_or("").starts_with('j') && instr.mnemonic().unwrap_or("") != "jmp" {
            if let Some(op_str) = instr.op_str() {
                if let Some(target) = op_str.strip_prefix("0x") {
                    if let Ok(target_addr) = u64::from_str_radix(target, 16) {
                        if target_addr < instr.address() {
                            println!("Loop header: 0x{:x}", target_addr);
                            count += 1;
                        }
                    }
                }
            }
        }
    }
    println!("Total loops found: {}", count);
    
    println!("\n=== CALLS IN FIRST 200 INSTRUCTIONS ===");
    for (i, instr) in instructions.iter().take(200).enumerate() {
        if instr.mnemonic().unwrap_or("") == "call" {
            println!("{}: 0x{:x} call {}", i, instr.address(), instr.op_str().unwrap_or("?"));
        }
    }
    
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
