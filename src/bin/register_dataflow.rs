use capstone::prelude::*;
use std::collections::HashMap;
use std::fs;

#[derive(Clone, Debug)]
struct RegisterLife {
    register: String,
    writes: Vec<u64>,
    reads: Vec<u64>,
    first_write: u64,
    last_read: u64,
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
    
    let mut reg_lives: HashMap<String, RegisterLife> = HashMap::new();
    
    for instr in instructions.iter() {
        let mnem = instr.mnemonic().unwrap_or("");
        let op_str = instr.op_str().unwrap_or("");
        let addr = instr.address();
        
        let parts: Vec<&str> = op_str.split(',').map(|s| s.trim()).collect();
        
        if mnem == "mov" || mnem == "lea" || mnem == "add" || mnem == "sub" {
            if let Some(dest) = parts.first() {
                let regs = ["rax", "rbx", "rcx", "rdx", "rsi", "rdi", "r8", "r9", "eax", "edi", "esi"];
                for reg in regs.iter() {
                    if dest.contains(reg) {
                        reg_lives.entry(reg.to_string())
                            .and_modify(|life| {
                                life.writes.push(addr);
                                life.last_read = addr;
                            })
                            .or_insert_with(|| RegisterLife {
                                register: reg.to_string(),
                                writes: vec![addr],
                                reads: vec![],
                                first_write: addr,
                                last_read: addr,
                            });
                    }
                }
            }
        }
        
        if mnem == "cmp" || mnem == "test" || mnem == "call" {
            let regs = ["rax", "rbx", "rcx", "rdx", "rsi", "rdi", "r8", "r9", "eax", "edi", "esi"];
            for reg in regs.iter() {
                if op_str.contains(reg) {
                    reg_lives.entry(reg.to_string())
                        .and_modify(|life| {
                            life.reads.push(addr);
                            life.last_read = addr;
                        })
                        .or_insert_with(|| RegisterLife {
                            register: reg.to_string(),
                            writes: vec![],
                            reads: vec![addr],
                            first_write: addr,
                            last_read: addr,
                        });
                }
            }
        }
    }
    
    println!("=== REGISTER DATA FLOW ANALYSIS ===\n");
    
    println!("Register lifetimes:");
    let mut sorted: Vec<_> = reg_lives.iter().collect();
    sorted.sort_by_key(|(_, life)| life.first_write);
    
    for (reg, life) in sorted.iter() {
        if life.writes.len() > 0 || life.reads.len() > 0 {
            println!("{}:", reg);
            println!("  Writes: {}", life.writes.len());
            println!("  Reads: {}", life.reads.len());
            println!("  Span: {} bytes", life.last_read - life.first_write);
        }
    }
    println!();
    
    println!("=== COMPUTED DATA DEPENDENCIES ===");
    for (reg, life) in sorted.iter() {
        if life.writes.len() > 0 && life.reads.len() > 0 {
            println!("{}: {} writes, {} reads - live across {} bytes",
                     reg, life.writes.len(), life.reads.len(), life.last_read - life.first_write);
        } else if life.writes.len() > 0 {
            println!("{}: {} writes, no reads - potentially dead store",
                     reg, life.writes.len());
        } else if life.reads.len() > 0 {
            println!("{}: {} reads, no writes - parameter or pre-initialized",
                     reg, life.reads.len());
        }
    }
    
    Ok(())
}
