use capstone::prelude::*;
use std::collections::{HashMap, VecDeque};
use std::fs;
use std::io::Read;

#[derive(Clone, Debug)]
struct Instr {
    addr: u64,
    mnem: String,
    op_str: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config_path = "/home/inf/.analysis/config";
    let config = read_config(config_path)?;
    let binary_path = config.get("BINARY_PATH").unwrap_or(&"/bin/ls".to_string()).clone();
    let text_offset = parse_hex(config.get("TEXT_OFFSET").unwrap_or(&"0x3040".to_string()))?;
    let text_size = parse_hex(config.get("TEXT_SIZE").unwrap_or(&"0x19863".to_string()))?;
    
    let mut file = fs::File::open(&binary_path)?;
    let mut binary_data = Vec::new();
    file.read_to_end(&mut binary_data)?;
    
    let text_section = &binary_data[text_offset as usize..(text_offset as usize + text_size as usize)];
    
    let cs = Capstone::new()
        .x86()
        .mode(arch::x86::ArchMode::Mode64)
        .detail(true)
        .build()?;
    
    let instructions = cs.disasm_all(text_section, text_offset as u64)?;
    
    let mut instrs = Vec::new();
    for instr in instructions.iter() {
        instrs.push(Instr {
            addr: instr.address(),
            mnem: instr.mnemonic().unwrap_or("").to_string(),
            op_str: instr.op_str().unwrap_or("").to_string(),
        });
    }
    
    println!("#include <stdio.h>");
    println!("#include <stdint.h>");
    println!("#include <string.h>\n");
    
    let mut func_addrs = Vec::new();
    for instr in instrs.iter() {
        if instr.mnem == "endbr64" {
            func_addrs.push(instr.addr);
        }
    }
    
    for (i, func_addr) in func_addrs.iter().enumerate() {
        let next_func = if i + 1 < func_addrs.len() {
            func_addrs[i + 1]
        } else {
            text_offset + text_size
        };
        
        let func_instrs: Vec<&Instr> = instrs.iter()
            .filter(|instr| instr.addr >= *func_addr && instr.addr < next_func)
            .collect();
        
        if func_instrs.is_empty() {
            continue;
        }
        
        println!("void func_0x{:x}(void) {{", func_addr);
        
        let mut var_map = HashMap::new();
        var_map.insert("[rbp-4]".to_string(), "counter".to_string());
        var_map.insert("[rbp-8]".to_string(), "temp".to_string());
        var_map.insert("[rbp-16]".to_string(), "buffer".to_string());
        
        let mut i = 0;
        while i < func_instrs.len() {
            let instr = func_instrs[i];
            
            // Pattern: mov [rbp-X], 0; add [rbp-X], 1 (loop init + increment)
            if instr.mnem == "mov" && instr.op_str.contains("[rbp-") && instr.op_str.contains("0x0") {
                if let Some(var) = extract_stack_var(&instr.op_str) {
                    if i + 2 < func_instrs.len() {
                        let next = func_instrs[i + 1];
                        if next.mnem == "add" && next.op_str.contains(&var) && next.op_str.contains("1") {
                            println!("    int {} = 0;", var);
                            var_map.insert(var.clone(), format!("i ({})", var));
                            i += 2;
                            continue;
                        }
                    }
                }
            }
            
            // Pattern: lea rdi, [rip+XXX]; call (string reference)
            if instr.mnem == "lea" && instr.op_str.contains("rdi") && instr.op_str.contains("[rip") {
                if i + 1 < func_instrs.len() && func_instrs[i + 1].mnem == "call" {
                    println!("    // string reference");
                    i += 1;
                    continue;
                }
            }
            
            // Pattern: mov rdi, ...; call (function argument)
            if instr.mnem == "mov" && instr.op_str.starts_with("rdi") {
                if i + 1 < func_instrs.len() && func_instrs[i + 1].mnem == "call" {
                    let arg = instr.op_str.split(',').nth(1).unwrap_or("").trim();
                    println!("    func_call({});", arg);
                    i += 1;
                    continue;
                }
            }
            
            match instr.mnem.as_str() {
                "call" => {
                    if let Some(target) = instr.op_str.strip_prefix("0x") {
                        println!("    func_0x{}();", target);
                    }
                }
                "ret" => {
                    println!("    return;");
                }
                "cmp" => {
                    if instr.op_str.contains("[rbp-4]") {
                        println!("    // loop condition");
                    }
                }
                "add" => {
                    if instr.op_str.contains("[rbp-4], 1") {
                        println!("    counter++;");
                    } else {
                        println!("    // arithmetic");
                    }
                }
                "mov" if instr.op_str.contains("[rbp-") => {
                    if let Some(var) = extract_stack_var(&instr.op_str) {
                        if let Some(mapped) = var_map.get(&var) {
                            println!("    {} = ...;", mapped);
                        } else {
                            println!("    var_{} = ...;", var.replace("[rbp-", "").replace("]", ""));
                        }
                    }
                }
                _ => {}
            }
            
            i += 1;
        }
        
        println!("}}\n");
    }
    
    println!("int main(int argc, char *argv[]) {{");
    println!("    return 0;");
    println!("}}");
    
    Ok(())
}

fn extract_stack_var(op_str: &str) -> Option<String> {
    if let Some(start) = op_str.find("[rbp-") {
        if let Some(end) = op_str[start..].find(']') {
            return Some(op_str[start..start+end+1].to_string());
        }
    }
    None
}

fn read_config(path: &str) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
    let mut config = HashMap::new();
    if let Ok(content) = fs::read_to_string(path) {
        for line in content.lines() {
            if let Some((k, v)) = line.split_once('=') {
                config.insert(k.to_string(), v.to_string());
            }
        }
    }
    Ok(config)
}

fn parse_hex(s: &str) -> Result<u64, Box<dyn std::error::Error>> {
    Ok(u64::from_str_radix(s.trim_start_matches("0x"), 16)?)
}
