use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fs;

#[derive(Deserialize, Serialize, Clone)]
struct Instruction {
    address: u64,
    mnemonic: String,
    op_str: String,
}

#[derive(Serialize)]
pub struct BasicBlock {
    start_address: u64,
    end_address: u64,
    instructions: Vec<Instruction>,
    jumps_to: Vec<u64>,
}

pub fn analyze_trace(trace_file: &str) -> Result<Vec<BasicBlock>, Box<dyn std::error::Error>> {
    let data = fs::read_to_string(trace_file)?;
    let instructions: Vec<Instruction> = serde_json::from_str(&data)?;
    
    let mut blocks = Vec::new();
    let mut current_block: Vec<Instruction> = Vec::new();
    let mut block_starts = HashSet::new();
        
    // First pass: identify block starts
    block_starts.insert(instructions[0].address);
    
    for instr in &instructions {
        if is_control_flow(&instr.mnemonic) {
            block_starts.insert(instr.address);
            if let Some(next) = instructions.iter().find(|i| i.address > instr.address) {
                block_starts.insert(next.address);
            }
        }
    }
    
    // Second pass: build blocks
    for instr in &instructions {
        if !current_block.is_empty() && block_starts.contains(&instr.address) && instr.address != current_block[0].address {
            // End current block and start new one
            let block = build_block(current_block.clone());
            blocks.push(block);
            current_block.clear();
        }
        
        current_block.push(instr.clone());
        
        if is_control_flow(&instr.mnemonic) {
            let block = build_block(current_block.clone());
            blocks.push(block);
            current_block.clear();
        }
    }
    
    Ok(blocks)
}

fn is_control_flow(mnemonic: &str) -> bool {
    matches!(mnemonic, 
        "jmp" | "je" | "jne" | "jz" | "jnz" | "ja" | "jb" | "jae" | "jbe" |
        "jg" | "jl" | "jge" | "jle" | "jo" | "jno" | "jp" | "jnp" |
        "call" | "ret" | "jmpq"
    )
}

fn build_block(instrs: Vec<Instruction>) -> BasicBlock {
    let start = instrs[0].address;
    let end = instrs[instrs.len() - 1].address;
    let mut jumps_to = Vec::new();
    
    // Extract jump targets from the last instruction if it's a jump
    if let Some(last) = instrs.last() {
        if is_control_flow(&last.mnemonic) && last.mnemonic != "call" && last.mnemonic != "ret" {
            if let Some(target_str) = last.op_str.split_whitespace().next() {
                if let Ok(target) = u64::from_str_radix(target_str.trim_start_matches("0x"), 16) {
                    jumps_to.push(target);
                }
            }
        }
    }
    
    BasicBlock {
        start_address: start,
        end_address: end,
        instructions: instrs,
        jumps_to,
    }
}