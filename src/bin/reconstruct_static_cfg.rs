use capstone::prelude::*;
use std::collections::{HashMap, BTreeMap};
use std::fs;

#[derive(Clone, Debug)]
struct Block {
    start: u64,
    end: u64,
    instructions: Vec<(u64, String, String)>,
    successors: Vec<u64>,
}

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
    
    let mut all_instructions = Vec::new();
    let mut block_starts = std::collections::HashSet::new();
    block_starts.insert(0u64);
    
    for offset in (0..binary_data.len()).step_by(0x1000) {
        if offset + 0x1000 > binary_data.len() {
            break;
        }
        
        let chunk = &binary_data[offset..offset + 0x1000.min(binary_data.len() - offset)];
        let base = offset as u64;
        
        if let Ok(instrs) = cs.disasm_all(chunk, base) {
            for instr in instrs.iter() {
                let mnem = instr.mnemonic().unwrap_or("");
                let op_str = instr.op_str().unwrap_or("");
                
                all_instructions.push((instr.address(), mnem.to_string(), op_str.to_string(), instr.len()));
                
                if mnem.starts_with('j') || mnem == "call" || mnem == "ret" {
                    block_starts.insert(instr.address() + instr.len() as u64);
                    
                    if mnem.starts_with('j') {
                        if let Ok(target) = op_str.parse::<u64>() {
                            block_starts.insert(target);
                        }
                    }
                }
            }
        }
    }
    
    let mut sorted_starts: Vec<u64> = block_starts.iter().copied().collect();
    sorted_starts.sort();
    
    let mut blocks: BTreeMap<u64, Block> = BTreeMap::new();
    
    for i in 0..sorted_starts.len() {
        let start = sorted_starts[i];
        let end = if i + 1 < sorted_starts.len() {
            sorted_starts[i + 1]
        } else {
            u64::MAX
        };
        
        let instrs: Vec<(u64, String, String)> = all_instructions
            .iter()
            .filter(|(addr, _, _, _)| *addr >= start && *addr < end)
            .map(|(addr, mnem, op_str, _)| (*addr, mnem.clone(), op_str.clone()))
            .collect();
        
        if !instrs.is_empty() {
            blocks.insert(start, Block {
                start,
                end,
                instructions: instrs.clone(),
                successors: vec![],
            });
        }
    }
    
    println!("Reconstructed {} basic blocks from static analysis", blocks.len());
    println!("\nFirst 10 blocks:");
    
    for (i, (addr, block)) in blocks.iter().enumerate().take(10) {
        println!("\nBlock 0x{:x} ({}..{}): {} instrs",
                 addr, block.start, block.end, block.instructions.len());
        for (instr_addr, mnem, op_str) in block.instructions.iter().take(5) {
            println!("  0x{:x}: {} {}", instr_addr, mnem, op_str);
        }
    }
    
    Ok(())
}
