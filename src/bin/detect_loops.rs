use capstone::prelude::*;
use std::fs;
use std::io::Read;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = read_config("/tmp/analysis.cfg")?;
    let binary_path = config.get("BINARY_PATH").unwrap_or(&"/tmp/test_uprobe".to_string()).clone();
    let text_offset = parse_hex(config.get("TEXT_OFFSET").unwrap_or(&"0x1040".to_string()))?;
    let text_size = parse_hex(config.get("TEXT_SIZE").unwrap_or(&"0x153".to_string()))?;
    
    let mut file = std::fs::File::open(&binary_path)?;
    let mut binary_data = Vec::new();
    file.read_to_end(&mut binary_data)?;
    
    let text_section = &binary_data[text_offset as usize..(text_offset as usize + text_size as usize)];
    
    let cs = Capstone::new()
        .x86()
        .mode(arch::x86::ArchMode::Mode64)
        .detail(true)
        .build()?;
    
    let instructions = cs.disasm_all(text_section, text_offset as u64)?;
    
    let mut backward_edges = Vec::new();
    
    for instr in instructions.iter() {
        let mnem = instr.mnemonic().unwrap_or("");
        if mnem.starts_with('j') || mnem == "jmp" {
            if let Some(op_str) = instr.op_str() {
                if let Some(stripped) = op_str.strip_prefix("0x") {
                    if let Ok(target) = u64::from_str_radix(stripped, 16) {
                        if target < instr.address() {
                            backward_edges.push((instr.address(), target, mnem.to_string()));
                        }
                    }
                }
            }
        }
    }
    
    println!("=== LOOP DETECTION ===\n");
    println!("Binary: {}", binary_path);
    println!("Text section: 0x{:x} to 0x{:x}\n", text_offset, text_offset + text_size);
    println!("Backward edges (loops): {}\n", backward_edges.len());
    
    for (addr, target, mnem) in backward_edges.iter() {
        println!("LOOP DETECTED: Header 0x{:x}, back edge 0x{:x}, mnemonic: {}", target, addr, mnem);
    }
    
    Ok(())
}

fn read_config(path: &str) -> Result<std::collections::HashMap<String, String>, Box<dyn std::error::Error>> {
    let mut config = std::collections::HashMap::new();
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
