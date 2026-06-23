use capstone::prelude::*;
use std::collections::{HashMap, HashSet};
use std::fs;

#[derive(Clone, Debug)]
struct LivenessInfo {
    location: String,
    first_use: u64,
    last_use: u64,
    uses: Vec<u64>,
}

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
    
    let mut liveness: HashMap<String, LivenessInfo> = HashMap::new();
    
    for instr in instructions.iter() {
        let addr = instr.address();
        let mnem = instr.mnemonic().unwrap_or("");
        let op_str = instr.op_str().unwrap_or("");
        
        let operands: Vec<&str> = op_str.split(',').map(|s| s.trim()).collect();
        
        for operand in operands.iter() {
            if operand.contains("[rbp") {
                let loc = operand.to_string();
                liveness.entry(loc.clone())
                    .and_modify(|info| {
                        info.last_use = addr;
                        info.uses.push(addr);
                    })
                    .or_insert_with(|| LivenessInfo {
                        location: loc,
                        first_use: addr,
                        last_use: addr,
                        uses: vec![addr],
                    });
            } else if operand.contains("edi") || operand.contains("eax") || operand.contains("rsi") {
                let loc = operand.to_string();
                liveness.entry(loc.clone())
                    .and_modify(|info| {
                        info.last_use = addr;
                        info.uses.push(addr);
                    })
                    .or_insert_with(|| LivenessInfo {
                        location: loc,
                        first_use: addr,
                        last_use: addr,
                        uses: vec![addr],
                    });
            }
        }
    }
    
    println!("=== VARIABLE LIVENESS ANALYSIS ===\n");
    
    println!("Live ranges (stack locations):");
    let mut stack_vars: Vec<_> = liveness.iter()
        .filter(|(k, _)| k.contains("[rbp"))
        .collect();
    stack_vars.sort_by_key(|a| a.1.first_use);
    
    for (loc, info) in stack_vars.iter() {
        let span = info.last_use - info.first_use;
        println!("  {}: 0x{:x} to 0x{:x} ({} bytes, {} uses)", 
                 loc, info.first_use, info.last_use, span, info.uses.len());
    }
    println!();
    
    println!("Live ranges (registers):");
    let mut regs: Vec<_> = liveness.iter()
        .filter(|(k, _)| !k.contains("[rbp"))
        .collect();
    regs.sort_by_key(|a| a.1.first_use);
    
    for (reg, info) in regs.iter().take(10) {
        let span = info.last_use - info.first_use;
        println!("  {}: 0x{:x} to 0x{:x} ({} bytes, {} uses)", 
                 reg, info.first_use, info.last_use, span, info.uses.len());
    }
    println!();
    
    println!("=== INFERRED VARIABLE LIFETIMES ===");
    println!();
    
    for (loc, info) in stack_vars.iter() {
        if info.uses.len() >= 2 {
            println!("{}: lives from 0x{:x} to 0x{:x}", loc, info.first_use, info.last_use);
        }
    }
    
    Ok(())
}
