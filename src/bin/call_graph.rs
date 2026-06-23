use capstone::prelude::*;
use std::collections::HashMap;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let binary_path = "/tmp/test_uprobe";
    let binary_data = fs::read(&binary_path)?;
    
    let text_start = 0x1040usize;
    let text_size = 0x153usize;
    let text_section = &binary_data[text_start..text_start + text_size];
    
    let cs = Capstone::new()
        .x86()
        .mode(arch::x86::ArchMode::Mode64)
        .detail(true)
        .build()?;
    
    let instructions = cs.disasm_all(text_section, 0x1040u64)?;
    
    let mut functions: HashMap<u64, String> = HashMap::new();
    functions.insert(0x1040, "libc_start".to_string());
    functions.insert(0x1139, "process_data".to_string());
    functions.insert(0x1160, "main".to_string());
    
    let mut call_edges: Vec<(u64, String)> = Vec::new();
    
    for instr in instructions.iter() {
        if let Some(mnem) = instr.mnemonic() {
            if mnem == "call" {
                let op_str = instr.op_str().unwrap_or("");
                let from_func = functions.iter()
                    .filter(|(addr, _)| **addr <= instr.address())
                    .max_by_key(|(addr, _)| *addr)
                    .map(|(_, name)| name.clone())
                    .unwrap_or_else(|| format!("0x{:x}", instr.address()));
                
                let to_addr = if op_str.starts_with("0x") {
                    if let Ok(addr) = u64::from_str_radix(&op_str[2..], 16) {
                        addr
                    } else {
                        0
                    }
                } else {
                    0
                };
                
                let to_func = if to_addr > 0 {
                    functions.get(&to_addr)
                        .cloned()
                        .unwrap_or_else(|| format!("0x{:x}", to_addr))
                } else if op_str.contains("plt") {
                    "libc (plt)".to_string()
                } else {
                    op_str.to_string()
                };
                
                call_edges.push((instr.address(), format!("{} -> {}", from_func, to_func)));
            }
        }
    }
    
    println!("=== CALL GRAPH ===\n");
    
    println!("Detected functions:");
    for (addr, name) in functions.iter() {
        println!("  0x{:x}: {}", addr, name);
    }
    println!();
    
    println!("Call edges:");
    for (from_addr, label) in call_edges.iter() {
        println!("  0x{:x}: {}", from_addr, label);
    }
    println!();
    
    let mut call_counts: HashMap<String, usize> = HashMap::new();
    for (_, label) in call_edges.iter() {
        *call_counts.entry(label.clone()).or_insert(0) += 1;
    }
    
    println!("Call frequency:");
    let mut sorted: Vec<_> = call_counts.iter().collect();
    sorted.sort_by(|a, b| b.1.cmp(a.1));
    
    for (label, count) in sorted.iter() {
        println!("  {}: {} calls", label, count);
    }
    
    Ok(())
}
