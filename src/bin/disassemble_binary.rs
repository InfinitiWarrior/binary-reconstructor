use capstone::prelude::*;
use std::collections::HashSet;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if std::env::args().len() < 2 {
        eprintln!("Usage: {} <binary>", std::env::args().next().unwrap());
        return Ok(());
    }
    
    let binary_path = std::env::args().nth(1).unwrap();
    let binary_data = fs::read(&binary_path)?;
    
    let cs = Capstone::new()
        .x86()
        .mode(arch::x86::ArchMode::Mode64)
        .detail(true)
        .build()?;
    
    let mut block_starts = HashSet::new();
    let mut total_instrs = 0;
    
    for offset in (0..binary_data.len()).step_by(0x1000) {
        if offset + 0x1000 > binary_data.len() {
            break;
        }
        
        let chunk = &binary_data[offset..offset + 0x1000.min(binary_data.len() - offset)];
        let base = offset as u64;
        
        match cs.disasm_all(chunk, base) {
            Ok(instrs) => {
                for instr in instrs.iter() {
                    total_instrs += 1;
                    let mnem = instr.mnemonic().unwrap_or("");
                    
                    if mnem.starts_with('j') || mnem == "call" || mnem == "ret" {
                        block_starts.insert(instr.address() + instr.len() as u64);
                        
                        if mnem.starts_with('j') {
                            if let Some(op_str) = instr.op_str() {
                                if let Ok(target) = op_str.parse::<u64>() {
                                    block_starts.insert(target);
                                }
                            }
                        }
                    }
                }
            }
            Err(_) => continue,
        }
    }
    
    println!("Total instructions disassembled: {}", total_instrs);
    println!("Unique basic block starts: {}", block_starts.len());
    
    let mut sorted: Vec<u64> = block_starts.iter().copied().collect();
    sorted.sort();
    
    println!("\nFirst 20 block boundaries:");
    for addr in sorted.iter().take(20) {
        println!("  0x{:x}", addr);
    }
    
    Ok(())
}
