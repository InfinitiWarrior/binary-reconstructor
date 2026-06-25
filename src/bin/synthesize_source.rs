use capstone::prelude::*;
use std::collections::HashMap;
use std::fs;
use std::io::Read;

#[derive(Clone, Debug)]
struct Instr {
    addr: u64,
    mnem: String,
    op_str: String,
}

#[derive(Clone, Debug)]
struct FuncSig {
    args: Vec<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config_path = "/home/inf/.analysis/config";
    let config = read_config(config_path)?;
    let binary_path = config.get("BINARY_PATH").unwrap_or(&"/bin/md5sum".to_string()).clone();
    let text_offset = parse_hex(config.get("TEXT_OFFSET").unwrap_or(&"0x2040".to_string()))?;
    let text_size = parse_hex(config.get("TEXT_SIZE").unwrap_or(&"0x6693".to_string()))?;
    
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
    
    let (loop_headers, loop_conditions) = detect_loops(&instrs);
    
    let mut func_addrs = Vec::new();
    for instr in instrs.iter() {
        if instr.mnem == "endbr64" {
            func_addrs.push(instr.addr);
        }
    }
    
    let mut func_sigs = HashMap::new();
    for (i, func_addr) in func_addrs.iter().enumerate() {
        let next_func = if i + 1 < func_addrs.len() {
            func_addrs[i + 1]
        } else {
            text_offset + text_size
        };
        
        let func_instrs: Vec<&Instr> = instrs.iter()
            .filter(|instr| instr.addr >= *func_addr && instr.addr < next_func)
            .collect();
        
        let args = infer_arguments(&func_instrs);
        func_sigs.insert(*func_addr, FuncSig { args });
    }
    
    println!("#include <stdio.h>");
    println!("#include <stdint.h>");
    println!("#include <string.h>\n");
    
    println!("// Forward declarations");
    for addr in func_addrs.iter() {
        if let Some(sig) = func_sigs.get(addr) {
            if sig.args.is_empty() {
                println!("void func_0x{:x}(void);", addr);
            } else {
                println!("void func_0x{:x}({});", addr, sig.args.join(", "));
            }
        }
    }
    println!();
    
    for (i, func_addr) in func_addrs.iter().enumerate() {
        let next_func = if i + 1 < func_addrs.len() {
            func_addrs[i + 1]
        } else {
            text_offset + text_size
        };
        
        let func_instrs: Vec<&Instr> = instrs.iter()
            .filter(|instr| instr.addr >= *func_addr && instr.addr < next_func)
            .collect();
        
        if func_instrs.is_empty() || func_instrs.len() < 2 {
            continue;
        }
        
        let args = &func_sigs.get(func_addr).unwrap().args;
        let sig = if args.is_empty() {
            format!("void func_0x{:x}(void)", func_addr)
        } else {
            format!("void func_0x{:x}({})", func_addr, args.join(", "))
        };
        
        println!("{} {{", sig);
        println!("    int i = 0, result = 0, max = 0, min = 0;");
        
        let mut in_loop = false;
        let mut loop_depth = 0;
        
        let mut j = 0;
        while j < func_instrs.len() {
            let instr = func_instrs[j];
            
            if loop_headers.contains(&instr.addr) && !in_loop {
                let cond = loop_conditions.get(&instr.addr).map(|s| s.as_str()).unwrap_or("1");
                println!("{}while ({}) {{", "    ".repeat(loop_depth + 1), cond);
                in_loop = true;
                loop_depth += 1;
            }
            
            match instr.mnem.as_str() {
                "call" => {
                    let indent = "    ".repeat(loop_depth + 1);
                    if let Some(target) = instr.op_str.strip_prefix("0x") {
                        if let Ok(target_addr) = u64::from_str_radix(target, 16) {
                            if let Some(target_sig) = func_sigs.get(&target_addr) {
                                if target_sig.args.is_empty() {
                                    println!("{}func_0x{}();", indent, target);
                                } else {
                                    let arg_list = vec!["0"; target_sig.args.len()].join(", ");
                                    println!("{}func_0x{}({});", indent, target, arg_list);
                                }
                            }
                        }
                    }
                }
                "ret" => {
                    if in_loop {
                        println!("{}}}  // end loop", "    ".repeat(loop_depth));
                        in_loop = false;
                        loop_depth = loop_depth.saturating_sub(1);
                    }
                    println!("{}return;", "    ".repeat(loop_depth + 1));
                }
                "lea" if instr.op_str.contains("rdi") && instr.op_str.contains("[rip") => {
                    let indent = "    ".repeat(loop_depth + 1);
                    println!("{}// data reference", indent);
                }
                _ => {}
            }
            
            j += 1;
        }
        
        if in_loop {
            println!("{}}}  // end loop", "    ".repeat(loop_depth));
        }
        
        println!("}}\n");
    }
    
    println!("int main(int argc, char *argv[]) {{");
    println!("    return 0;");
    println!("}}");
    
    Ok(())
}

fn detect_loops(instrs: &[Instr]) -> (std::collections::HashSet<u64>, HashMap<u64, String>) {
    let mut headers = std::collections::HashSet::new();
    let mut conditions = HashMap::new();
    
    for instr in instrs {
        if instr.mnem.starts_with('j') && instr.mnem != "jmp" {
            if let Some(target) = instr.op_str.strip_prefix("0x") {
                if let Ok(target_addr) = u64::from_str_radix(target, 16) {
                    if target_addr < instr.addr {
                        headers.insert(target_addr);
                        
                        let cond = match instr.mnem.as_str() {
                            "jle" | "jbe" => "i <= max",
                            "jl" | "jb" => "i < max",
                            "jge" | "jae" => "i >= min",
                            "jg" | "ja" => "i > min",
                            "je" | "jz" => "result == 0",
                            "jne" | "jnz" => "result != 0",
                            _ => "condition",
                        };
                        
                        conditions.insert(target_addr, cond.to_string());
                    }
                }
            }
        }
    }
    
    (headers, conditions)
}

fn infer_arguments(instrs: &[&Instr]) -> Vec<String> {
    let mut args = Vec::new();
    let arg_regs = vec!["rdi", "rsi", "rdx", "rcx", "r8", "r9"];
    
    for reg in arg_regs {
        for instr in instrs {
            if (instr.mnem == "mov" || instr.mnem == "lea") && instr.op_str.starts_with(reg) {
                args.push(format!("uint64_t arg_{}", reg));
                break;
            }
        }
    }
    
    args
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
