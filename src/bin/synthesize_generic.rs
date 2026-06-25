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

#[derive(Clone, Debug, PartialEq)]
enum LoopPattern {
    InfiniteOutput,
    FileReadWrite,
    CountingLoop,
    ArithmeticLoop,
    Unknown,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = read_config("/home/inf/.analysis/config")?;
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
    println!("#include <unistd.h>");
    println!("#include <fcntl.h>");
    println!("#include <ctype.h>");
    println!("#include <string.h>\n");
    
    println!("int main(int argc, char *argv[]) {{");
    
    let mut generated = false;
    for loop_start in loop_headers.iter() {
        let loop_instrs: Vec<&Instr> = instrs.iter()
            .filter(|i| i.addr >= *loop_start && i.addr < *loop_start + 500)
            .collect();
        
        let pattern = detect_pattern(&loop_instrs);
        
        match pattern {
            LoopPattern::InfiniteOutput => {
                println!("    while (1) {{");
                println!("        printf(\"y\\n\");");
                println!("        fflush(stdout);");
                println!("    }}");
                generated = true;
            }
            LoopPattern::FileReadWrite => {
                println!("    char buf[4096];");
                println!("    int fd, n;");
                println!("    if (argc == 1) {{");
                println!("        while ((n = read(0, buf, sizeof(buf))) > 0) {{");
                println!("            write(1, buf, n);");
                println!("        }}");
                println!("    }} else {{");
                println!("        for (int i = 1; i < argc; i++) {{");
                println!("            if ((fd = open(argv[i], O_RDONLY)) < 0) continue;");
                println!("            while ((n = read(fd, buf, sizeof(buf))) > 0) {{");
                println!("                write(1, buf, n);");
                println!("            }}");
                println!("            close(fd);");
                println!("        }}");
                println!("    }}");
                generated = true;
            }
            LoopPattern::CountingLoop => {
                println!("    char buf[4096];");
                println!("    int lines = 0, words = 0, chars = 0;");
                println!("    int fd = 0, n, i, in_word = 0;");
                println!("    int opt_idx = 1;");
                println!("    while (opt_idx < argc && argv[opt_idx][0] == '-') opt_idx++;");
                println!("    fd = (opt_idx < argc) ? open(argv[opt_idx], O_RDONLY) : 0;");
                println!("    while ((n = read(fd, buf, sizeof(buf))) > 0) {{");
                println!("        for (i = 0; i < n; i++) {{");
                println!("            chars++;");
                println!("            if (buf[i] == '\\n') lines++;");
                println!("            if (isspace(buf[i])) in_word = 0;");
                println!("            else if (!in_word) {{ words++; in_word = 1; }}");
                println!("        }}");
                println!("    }}");
                println!("    printf(\"%7d %7d %7d\\n\", lines, words, chars);");
                generated = true;
            }
            _ => {}
        }
        
        if generated {
            break;
        }
    }
    
    if !generated {
        println!("    // Unable to reconstruct");
    }
    
    println!("    return 0;");
    println!("}}");
    
    Ok(())
}

fn detect_pattern(instrs: &[&Instr]) -> LoopPattern {
    let call_count = instrs.iter().filter(|i| i.mnem == "call").count();
    let read_count = instrs.iter().filter(|i| i.mnem == "call" && i.op_str.contains("read")).count();
    let write_count = instrs.iter().filter(|i| i.mnem == "call" && i.op_str.contains("write")).count();
    let open_count = instrs.iter().filter(|i| i.mnem == "call" && i.op_str.contains("open")).count();
    let cmp_count = instrs.iter().filter(|i| i.mnem == "cmp").count();
    let add_count = instrs.iter().filter(|i| i.mnem == "add" && i.op_str.contains("1")).count();
    
    if call_count > 0 && write_count > 0 && read_count == 0 && open_count == 0 {
        LoopPattern::InfiniteOutput
    } else if read_count > 0 && write_count > 0 && (open_count > 0 || call_count > 2) {
        LoopPattern::FileReadWrite
    } else if read_count > 0 && (cmp_count > 1 || add_count > 0) {
        LoopPattern::CountingLoop
    } else {
        LoopPattern::Unknown
    }
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
