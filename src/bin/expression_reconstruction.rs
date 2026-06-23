use capstone::prelude::*;
use std::fs;

#[derive(Clone, Debug)]
struct Expression {
    instructions: Vec<(u64, String, String)>,
    semantic: String,
    inputs: Vec<String>,
    outputs: Vec<String>,
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
    
    println!("=== EXPRESSION RECONSTRUCTION ===\n");
    
    println!("Instruction sequences in process_data (0x1139):");
    let mut current_expr = Vec::new();
    
    for instr in instructions.iter() {
        let addr = instr.address();
        if addr >= 0x1139 && addr < 0x1160 {
            let mnem = instr.mnemonic().unwrap_or("");
            let op_str = instr.op_str().unwrap_or("");
            
            current_expr.push((addr, mnem.to_string(), op_str.to_string()));
            
            if mnem == "call" {
                let semantic = reconstruct_call(&current_expr);
                println!("\nSequence 0x{:x}..0x{:x}:", 
                         current_expr.first().map(|(a, _, _)| a).unwrap_or(&0),
                         addr);
                for (a, m, o) in current_expr.iter() {
                    println!("  0x{:x}: {} {}", a, m, o);
                }
                println!("Reconstructed: {}", semantic);
                current_expr.clear();
            }
        }
    }
    
    println!("\n\nInstruction sequences in main (0x1160):");
    current_expr.clear();
    
    for instr in instructions.iter() {
        let addr = instr.address();
        if addr >= 0x1160 && addr < 0x1193 {
            let mnem = instr.mnemonic().unwrap_or("");
            let op_str = instr.op_str().unwrap_or("");
            
            current_expr.push((addr, mnem.to_string(), op_str.to_string()));
            
            if mnem == "call" || mnem.starts_with('j') {
                let semantic = reconstruct_sequence(&current_expr);
                println!("\nSequence 0x{:x}..0x{:x}:", 
                         current_expr.first().map(|(a, _, _)| a).unwrap_or(&0),
                         addr);
                for (a, m, o) in current_expr.iter() {
                    println!("  0x{:x}: {} {}", a, m, o);
                }
                println!("Reconstructed: {}", semantic);
                current_expr.clear();
            }
        }
    }
    
    Ok(())
}

fn reconstruct_call(instrs: &[(u64, String, String)]) -> String {
    let has_rdi = instrs.iter().any(|(_, m, o)| m == "mov" && o.contains("rdi"));
    let has_esi = instrs.iter().any(|(_, m, o)| m == "mov" && o.contains("esi"));
    let has_lea = instrs.iter().any(|(_, m, _)| m == "lea");
    
    if has_lea && has_esi {
        "printf(format_string, value)".to_string()
    } else if has_rdi {
        "function_call(arg1)".to_string()
    } else {
        "call".to_string()
    }
}

fn reconstruct_sequence(instrs: &[(u64, String, String)]) -> String {
    let mnemonic = instrs.last().map(|(_, m, _)| m.as_str()).unwrap_or("");
    let op_str = instrs.last().map(|(_, _, o)| o.as_str()).unwrap_or("");
    
    if mnemonic.starts_with('j') {
        if mnemonic == "jle" {
            "conditional_branch (<=)".to_string()
        } else if mnemonic == "jne" {
            "conditional_branch (!=)".to_string()
        } else if mnemonic == "je" {
            "conditional_branch (==)".to_string()
        } else {
            format!("jump {}", op_str)
        }
    } else if mnemonic == "call" {
        "function_call".to_string()
    } else {
        "operation".to_string()
    }
}
