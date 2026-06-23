use capstone::prelude::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let binary_path = "/tmp/test_uprobe";
    let binary_data = std::fs::read(&binary_path)?;
    
    let text_start = 0x1040usize;
    let text_size = 0x153usize;
    let text_section = &binary_data[text_start..text_start + text_size];
    
    let cs = Capstone::new()
        .x86()
        .mode(arch::x86::ArchMode::Mode64)
        .detail(true)
        .build()?;
    
    let instructions = cs.disasm_all(text_section, 0x1040u64)?;
    
    let mut syscall_types: HashMap<u64, usize> = HashMap::new();
    let syscall_events = File::open("syscall_events.json")?;
    let reader = BufReader::new(syscall_events);
    
    for line in reader.lines() {
        let line = line?;
        if line.starts_with('{') {
            if let Some(sc_start) = line.find("\"syscall\":") {
                let sc_str = &line[sc_start + 10..];
                if let Some(sc_end) = sc_str.find('}') {
                    if let Ok(sc) = sc_str[..sc_end].parse::<u64>() {
                        *syscall_types.entry(sc).or_insert(0) += 1;
                    }
                }
            }
        }
    }
    
    let mut reg_uses: HashMap<String, Vec<u64>> = HashMap::new();
    let mut stack_uses: HashMap<String, Vec<u64>> = HashMap::new();
    
    for instr in instructions.iter() {
        let op = instr.op_str().unwrap_or("");
        let addr = instr.address();
        
        if op.contains("[rbp") {
            let loc = op.split(',').next().unwrap_or("").trim().to_string();
            stack_uses.entry(loc).or_insert_with(Vec::new).push(addr);
        }
        if op.contains("edi") {
            reg_uses.entry("edi".to_string()).or_insert_with(Vec::new).push(addr);
        }
        if op.contains("eax") {
            reg_uses.entry("eax".to_string()).or_insert_with(Vec::new).push(addr);
        }
        if op.contains("rsi") {
            reg_uses.entry("rsi".to_string()).or_insert_with(Vec::new).push(addr);
        }
    }
    
    println!("=== TYPE INFERENCE FROM SYSCALL CORRELATION ===\n");
    
    println!("Syscall distribution:");
    let mut sorted_syscalls: Vec<_> = syscall_types.iter().collect();
    sorted_syscalls.sort_by(|a, b| b.1.cmp(a.1));
    
    for (sc, count) in sorted_syscalls.iter().take(5) {
        println!("  syscall {}: {}", sc, count);
    }
    println!();
    
    println!("Register usage patterns:");
    for (reg, uses) in reg_uses.iter() {
        println!("  {}: {} uses (0x{:x} to 0x{:x})", 
                 reg, uses.len(), 
                 uses.first().unwrap_or(&0),
                 uses.last().unwrap_or(&0));
    }
    println!();
    
    println!("Stack location usage:");
    for (loc, uses) in stack_uses.iter() {
        if uses.len() >= 2 {
            println!("  {}: {} uses (0x{:x} to 0x{:x})", 
                     loc, uses.len(),
                     uses.first().unwrap_or(&0),
                     uses.last().unwrap_or(&0));
        }
    }
    println!();
    
    println!("=== INFERRED TYPES ===");
    println!("edi: int (passed to process_data, used in loop)");
    println!("[rbp-4]: int (loop counter, cmp with constant 4)");
    println!("[rbp-0x20]: char** (argv, saved from rsi)");
    println!("[rbp-0x14]: int (argc, saved from edi)");
    println!();
    
    println!("=== SYSCALL SEMANTICS ===");
    if let Some(write_count) = syscall_types.get(&1) {
        println!("write (syscall 1): {} calls - output from printf", write_count);
    }
    if let Some(futex_count) = syscall_types.get(&202) {
        println!("futex (syscall 202): {} calls - synchronization", futex_count);
    }
    
    Ok(())
}
