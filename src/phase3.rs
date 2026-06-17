use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

#[derive(Deserialize)]
struct BasicBlock {
    start_address: u64,
    end_address: u64,
    instructions: Vec<Instruction>,
    jumps_to: Vec<u64>,
}

#[derive(Deserialize, Clone)]
struct Instruction {
    address: u64,
    mnemonic: String,
    op_str: String,
}

#[derive(Serialize)]
pub struct PseudoFunction {
    name: String,
    code: String,
}

pub fn generate_pseudocode(cfg_file: &str) -> Result<Vec<PseudoFunction>, Box<dyn std::error::Error>> {
    let data = fs::read_to_string(cfg_file)?;
    let blocks: Vec<BasicBlock> = serde_json::from_str(&data)?;
    
    let mut functions = Vec::new();
    let mut code_lines = Vec::new();
    
    // Create register-to-variable mapping
    let mut reg_map: HashMap<String, String> = HashMap::new();
    let mut var_counter = 0;
    
    for block in &blocks[..blocks.len().min(100)] { // Limit to first 100 blocks for MVP
        for instr in &block.instructions {
            let line = instruction_to_rust(&instr, &mut reg_map, &mut var_counter);
            code_lines.push(line);
        }
    }
    
    let function = PseudoFunction {
        name: "func_main".to_string(),
        code: code_lines.join("\n"),
    };
    
    functions.push(function);
    Ok(functions)
}

fn instruction_to_rust(
    instr: &Instruction,
    reg_map: &mut HashMap<String, String>,
    var_counter: &mut i32,
) -> String {
    let mnemonic = &instr.mnemonic;
    let operands: Vec<&str> = instr.op_str.split(',').map(|s| s.trim()).collect();
    
    match mnemonic.as_str() {
        "mov" if operands.len() == 2 => {
            let src = operand_to_var(operands[1], reg_map, var_counter);
            let dst = operand_to_var(operands[0], reg_map, var_counter);
            format!("    {} = {};", dst, src)
        }
        "add" if operands.len() == 2 => {
            let dst = operand_to_var(operands[0], reg_map, var_counter);
            let src = operand_to_var(operands[1], reg_map, var_counter);
            format!("    {} += {};", dst, src)
        }
        "sub" if operands.len() == 2 => {
            let dst = operand_to_var(operands[0], reg_map, var_counter);
            let src = operand_to_var(operands[1], reg_map, var_counter);
            format!("    {} -= {};", dst, src)
        }
        "xor" if operands.len() == 2 => {
            let dst = operand_to_var(operands[0], reg_map, var_counter);
            let src = operand_to_var(operands[1], reg_map, var_counter);
            format!("    {} ^= {};", dst, src)
        }
        "cmp" if operands.len() == 2 => {
            let left = operand_to_var(operands[0], reg_map, var_counter);
            let right = operand_to_var(operands[1], reg_map, var_counter);
            format!("    // compare {} and {}", left, right)
        }
        "jmp" => format!("    // jmp to {}", operands.get(0).unwrap_or(&"?")),
        "je" | "jne" | "jz" | "jnz" => {
            format!("    // conditional jump to {}", operands.get(0).unwrap_or(&"?"))
        }
        "call" => format!("    // call {}", operands.get(0).unwrap_or(&"?")),
        "ret" => "    return;".to_string(),
        "push" => {
            let val = operand_to_var(operands.get(0).unwrap_or(&"?"), reg_map, var_counter);
            format!("    // push {}", val)
        }
        "pop" => {
            let val = operand_to_var(operands.get(0).unwrap_or(&"?"), reg_map, var_counter);
            format!("    // pop {}", val)
        }
        _ => format!("    // {}", instr.mnemonic),
    }
}

fn operand_to_var(
    operand: &str,
    reg_map: &mut HashMap<String, String>,
    var_counter: &mut i32,
) -> String {
    let clean = operand.trim();
    
    // Check if it's a register
    if is_register(clean) {
        reg_map
            .entry(clean.to_string())
            .or_insert_with(|| {
                *var_counter += 1;
                format!("var_{}", *var_counter - 1)
            })
            .clone()
    } else if clean.starts_with("0x") {
        // Hex immediate
        clean.to_string()
    } else {
        // Memory or other
        clean.to_string()
    }
}

fn is_register(s: &str) -> bool {
    matches!(
        s,
        "rax" | "rbx" | "rcx" | "rdx" | "rsi" | "rdi" | "r8" | "r9" | "r10" | "r11" | "r12"
            | "r13" | "r14" | "r15" | "eax" | "ebx" | "ecx" | "edx" | "esi" | "edi"
    )
}