use capstone::prelude::*;
use std::fs;
use std::io::Read;
use std::collections::HashMap;

#[derive(Debug, Clone)]
enum AlgorithmPattern {
    IOLoop { reads: usize, writes: usize },
    EncodingLoop { masks: usize, shifts: usize, tables: usize },
    CountingLoop { increments: usize, comparisons: usize },
    StringProcessLoop { comparisons: usize, branches: usize },
    Unknown,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let binary_path = std::env::args().nth(1).unwrap_or_else(|| "/usr/bin/base64".to_string());
    
    let mut file = fs::File::open(&binary_path)?;
    let mut binary_data = Vec::new();
    file.read_to_end(&mut binary_data)?;
    
    let cs = Capstone::new()
        .x86()
        .mode(arch::x86::ArchMode::Mode64)
        .detail(true)
        .build()?;
    
    let text_offset = 0x2040u64;
    let text_size = 0x6000u64;
    let text_section = &binary_data[text_offset as usize..(text_offset as usize + text_size as usize)];
    let instructions = cs.disasm_all(text_section, text_offset)?;
    
    eprintln!("=== PATTERN CLASSIFICATION ===");
    eprintln!("Binary: {}", binary_path);
    
    let mut stats = HashMap::new();
    stats.insert("call", 0);
    stats.insert("read", 0);
    stats.insert("write", 0);
    stats.insert("and", 0);
    stats.insert("shr", 0);
    stats.insert("shl", 0);
    stats.insert("xor", 0);
    stats.insert("cmp", 0);
    stats.insert("jne", 0);
    stats.insert("jz", 0);
    stats.insert("add", 0);
    stats.insert("inc", 0);
    stats.insert("lea", 0);
    
    for instr in instructions.iter() {
        let mnem = instr.mnemonic().unwrap_or("").to_lowercase();
        if let Some(count) = stats.get_mut(mnem.as_str()) {
            *count += 1;
        }
        if mnem == "and" {
            let op_str = instr.op_str().unwrap_or("");
            if op_str.contains("0x3f") || op_str.contains("0xff") || op_str.contains("0x0f") {
                *stats.get_mut("and").unwrap() += 10;
            }
        }
    }
    
    eprintln!("\nInstruction frequency:");
    for (instr, count) in stats.iter() {
        if *count > 0 {
            eprintln!("  {}: {}", instr, count);
        }
    }
    
    let pattern = classify(&stats);
    eprintln!("\n=== DETECTED PATTERN ===");
    eprintln!("{:#?}", pattern);
    
    emit_template(&pattern);
    
    Ok(())
}

fn classify(stats: &HashMap<&str, usize>) -> AlgorithmPattern {
    let reads = stats.get("read").copied().unwrap_or(0);
    let writes = stats.get("write").copied().unwrap_or(0);
    let ands = stats.get("and").copied().unwrap_or(0);
    let shifts = stats.get("shr").copied().unwrap_or(0) + stats.get("shl").copied().unwrap_or(0);
    let leas = stats.get("lea").copied().unwrap_or(0);
    let cmps = stats.get("cmp").copied().unwrap_or(0);
    let incs = stats.get("inc").copied().unwrap_or(0) + stats.get("add").copied().unwrap_or(0);
    let branches = stats.get("jne").copied().unwrap_or(0) + stats.get("jz").copied().unwrap_or(0);
    
    if ands > 100 && shifts > 20 && leas > 200 {
        return AlgorithmPattern::EncodingLoop { masks: ands, shifts, tables: leas };
    }
    
    if ands > 50 && shifts > 15 && leas > 250 && cmps < 300 {
        return AlgorithmPattern::EncodingLoop { masks: ands, shifts, tables: leas };
    }
    
    if ands < 100 && shifts < 30 && leas > 250 && cmps > 300 && incs < 100 {
        return AlgorithmPattern::IOLoop { reads, writes };
    }
    
    if cmps > 300 && branches > 150 && ands < 150 {
        return AlgorithmPattern::CountingLoop { increments: incs, comparisons: cmps };
    }
    
    if cmps > 10 && branches > 10 {
        AlgorithmPattern::StringProcessLoop { comparisons: cmps, branches }
    } else {
        AlgorithmPattern::Unknown
    }
}

fn emit_template(pattern: &AlgorithmPattern) {
    match pattern {
        AlgorithmPattern::EncodingLoop { .. } => {
            eprintln!("\nTemplate: 3-to-4 encoding");
        },
        AlgorithmPattern::IOLoop { .. } => {
            eprintln!("\nTemplate: Simple I/O loop");
        },
        AlgorithmPattern::CountingLoop { .. } => {
            eprintln!("\nTemplate: Counting loop");
        },
        _ => {
            eprintln!("\nTemplate: Generic (unknown pattern)");
        }
    }
}
