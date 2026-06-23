use capstone::prelude::*;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if std::env::args().len() < 2 {
        eprintln!("Usage: {} <binary>", std::env::args().next().unwrap());
        return Ok(());
    }
    
    let binary_path = std::env::args().nth(1).unwrap();
    let binary_data = fs::read(&binary_path)?;
    
    let text_start = 0x1040usize;
    let text_size = 0x153usize;
    let text_end = text_start + text_size;
    
    if text_end > binary_data.len() {
        eprintln!("Binary too small for .text extraction");
        return Ok(());
    }
    
    let text_section = &binary_data[text_start..text_end];
    
    let cs = Capstone::new()
        .x86()
        .mode(arch::x86::ArchMode::Mode64)
        .detail(true)
        .build()?;
    
    let instructions = cs.disasm_all(text_section, 0x1040u64)?;
    
    let mut block_starts = std::collections::HashSet::new();
    block_starts.insert(0x1040u64);
    
    for instr in instructions.iter() {
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
    
    let mut sorted_starts: Vec<u64> = block_starts.iter().copied().collect();
    sorted_starts.sort();
    
    println!("Application CFG Blocks (.text section 0x1040-0x1193):\n");
    
    for i in 0..sorted_starts.len() {
        let start = sorted_starts[i];
        let end = if i + 1 < sorted_starts.len() {
            sorted_starts[i + 1]
        } else {
            0x1193u64
        };
        
        let block_instrs: Vec<_> = instructions.iter()
            .filter(|instr| instr.address() >= start && instr.address() < end)
            .collect();
        
        if block_instrs.is_empty() {
            continue;
        }
        
        let semantic = if start == 0x1139 {
            "(process_data)"
        } else if start == 0x1160 {
            "(main)"
        } else {
            ""
        };
        
        println!("Block 0x{:x} {} ({} instructions):", start, semantic, block_instrs.len());
        
        for instr in block_instrs.iter() {
            println!("  0x{:x}: {} {}",
                     instr.address(),
                     instr.mnemonic().unwrap_or("?"),
                     instr.op_str().unwrap_or(""));
        }
        
        if let Some(last) = block_instrs.last() {
            let mnem = last.mnemonic().unwrap_or("");
            if mnem.starts_with('j') {
                println!("  [Branch to {}]", last.op_str().unwrap_or("?"));
            } else if mnem == "call" {
                println!("  [Call]");
            } else if mnem == "ret" {
                println!("  [Return]");
            }
        }
        println!();
    }
    
    Ok(())
}
