use capstone::prelude::*;
use std::collections::HashMap;
use std::fs;
use std::io::Read;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = read_config("/tmp/analysis.cfg")?;
    let binary_path = config.get("BINARY_PATH").unwrap_or(&"/tmp/test_uprobe".to_string()).clone();
    let text_offset = parse_hex(config.get("TEXT_OFFSET").unwrap_or(&"0x1040".to_string()))?;
    let text_size = parse_hex(config.get("TEXT_SIZE").unwrap_or(&"0x153".to_string()))?;
    
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
    
    let mut reg_uses: HashMap<String, (u64, u64, usize)> = HashMap::new();
    
    for instr in instructions.iter() {
        let op_str = instr.op_str().unwrap_or("");
        let regs = ["rax", "rbx", "rcx", "rdx", "rsi", "rdi", "rbp"];
        
        for reg in regs.iter() {
            if op_str.contains(reg) {
                reg_uses.entry(reg.to_string())
                    .and_modify(|(_first, last, count)| {
                        *last = instr.address();
                        *count += 1;
                    })
                    .or_insert((instr.address(), instr.address(), 1));
            }
        }
    }
    
    println!("=== LIVENESS ANALYSIS ===\n");
    println!("Binary: {}", binary_path);
    println!("Text section: 0x{:x}, {} bytes\n", text_offset, text_size);
    
    for (reg, (first, last, count)) in reg_uses.iter() {
        let span = last - first;
        println!("{}: {} uses, lives 0x{:x} to 0x{:x} ({} bytes)", reg, count, first, last, span);
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
