use capstone::prelude::*;
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
    
    println!("Loop Detection:\n");
    
    for instr in instructions.iter() {
        let mnem = instr.mnemonic().unwrap_or("");
        
        if mnem.starts_with('j') || mnem == "jmp" {
            let op_str = instr.op_str().unwrap_or("");
            
            let target = if op_str.starts_with("0x") {
                u64::from_str_radix(&op_str[2..], 16).ok()
            } else {
                None
            };
            
            if let Some(target_addr) = target {
                let from = instr.address();
                let is_backward = target_addr < from;
                
                if is_backward {
                    let loop_size = from - target_addr;
                    let semantics = if mnem == "jle" {
                        "for/while (i <= limit)"
                    } else if mnem == "jne" {
                        "while (value != 0)"
                    } else if mnem == "jmp" {
                        "unconditional loop"
                    } else {
                        "conditional loop"
                    };
                    
                    println!("LOOP DETECTED:");
                    println!("  Header: 0x{:x}", target_addr);
                    println!("  Back edge: 0x{:x} -> 0x{:x}", from, target_addr);
                    println!("  Body size: {} bytes", loop_size);
                    println!("  Jump type: {} {}", mnem, op_str);
                    println!("  Likely semantics: {}", semantics);
                    println!();
                }
            }
        }
    }
    
    Ok(())
}
