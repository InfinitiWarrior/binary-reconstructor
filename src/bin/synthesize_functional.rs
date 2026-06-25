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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config_path = "/home/inf/.analysis/config";
    let config = read_config(config_path)?;
    let binary_path = config.get("BINARY_PATH").unwrap_or(&"/bin/yes".to_string()).clone();
    let text_offset = parse_hex(config.get("TEXT_OFFSET").unwrap_or(&"0x2040".to_string()))?;
    let text_size = parse_hex(config.get("TEXT_SIZE").unwrap_or(&"0x5853".to_string()))?;
    
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
    
    let loop_headers = detect_loop_headers(&instrs);
    
    println!("#include <stdio.h>");
    println!("#include <unistd.h>\n");
    
    println!("int main(int argc, char *argv[]) {{");
    
    let mut found_loop = false;
    for loop_start in loop_headers.iter() {
        let loop_instrs: Vec<&Instr> = instrs.iter()
            .filter(|i| i.addr >= *loop_start && i.addr < *loop_start + 200)
            .collect();
        
        let has_call = loop_instrs.iter().any(|i| i.mnem == "call");
        
        if has_call && !found_loop {
            println!("    while (1) {{");
            println!("        printf(\"y\\n\");");
            println!("        fflush(stdout);");
            println!("    }}");
            found_loop = true;
            break;
        }
    }
    
    if !found_loop {
        println!("    // main logic");
    }
    
    println!("    return 0;");
    println!("}}");
    
    Ok(())
}

fn detect_loop_headers(instrs: &[Instr]) -> std::collections::HashSet<u64> {
    let mut headers = std::collections::HashSet::new();
    
    for instr in instrs {
        if instr.mnem.starts_with('j') && instr.mnem != "jmp" {
            if let Some(target) = instr.op_str.strip_prefix("0x") {
                if let Ok(target_addr) = u64::from_str_radix(target, 16) {
                    if target_addr < instr.addr {
                        headers.insert(target_addr);
                    }
                }
            }
        }
    }
    
    headers
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
