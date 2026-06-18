use std::collections::{HashMap, HashSet};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Clone)]
pub struct BasicBlock {
    pub start_address: u64,
    pub end_address: u64,
    pub instructions: Vec<Instruction>,
    pub jumps_to: Vec<u64>,
}

#[derive(Deserialize, Clone)]
pub struct Instruction {
    pub address: u64,
    pub mnemonic: String,
    pub op_str: String,
}

#[derive(Serialize, Clone)]
pub struct RustFunction {
    pub name: String,
    pub address: u64,
    pub code: String,
}

pub fn extract_functions(cfg_file: &str) -> Result<Vec<RustFunction>, Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string(cfg_file)?;
    let blocks: Vec<BasicBlock> = serde_json::from_str(&content)?;
    
    let mut functions = Vec::new();
    let mut entry_points = HashSet::new();
    let mut block_map: HashMap<u64, &BasicBlock> = HashMap::new();
    
    // Build address map
    for block in &blocks {
        block_map.insert(block.start_address, block);
    }
    
    // Find entry points
    entry_points.insert(blocks[0].start_address);
    
    for block in &blocks {
        for instr in &block.instructions {
            if instr.mnemonic == "call" {
                // Entry point after call
                if let Some(next_block) = blocks.iter()
                    .find(|b| b.instructions.iter().any(|i| i.address > instr.address)) {
                    entry_points.insert(next_block.start_address);
                }
            }
        }
    }
    
    let mut sorted_entries: Vec<_> = entry_points.iter().copied().collect();
    sorted_entries.sort();
    
    // Extract functions
    for (i, entry) in sorted_entries.iter().enumerate() {
        let next_entry = sorted_entries.get(i + 1).copied();
        
        let func_name = if *entry == blocks[0].start_address {
            "func_main".to_string()
        } else {
            format!("func_{:x}", entry)
        };
        
        let mut func_blocks = Vec::new();
        let mut code_lines = Vec::new();
        
        for block in &blocks {
            if block.start_address >= *entry {
                if let Some(next) = next_entry {
                    if block.start_address >= next {
                        break;
                    }
                }
                func_blocks.push(block);
                
                // Add block instructions
                for instr in &block.instructions {
                    let line = instruction_to_line(&instr);
                    if !line.is_empty() {
                        code_lines.push(line);
                    }
                }
            }
        }
        
        let code = if code_lines.is_empty() {
            format!("fn {}() {{\n    // {} blocks\n}}\n", func_name, func_blocks.len())
        } else {
            format!("fn {}() {{\n{}\n}}\n", func_name, code_lines.join("\n"))
        };
        
        functions.push(RustFunction {
            name: func_name,
            address: *entry,
            code,
        });
    }
    
    Ok(functions)
}

fn instruction_to_line(instr: &Instruction) -> String {
    match instr.mnemonic.as_str() {
        "ret" => "    return;".to_string(),
        "call" => format!("    // call 0x{:x}", instr.address),
        mnem if mnem.starts_with("j") => format!("    // jump"),
        _ => String::new(),
    }
}