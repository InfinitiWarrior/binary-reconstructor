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
    
    println!("=== EXTRACTED FROM BINARY + TRACE ===\n");
    
    println!("Syscalls detected:");
    let mut sorted_syscalls: Vec<_> = syscall_types.iter().collect();
    sorted_syscalls.sort_by(|a, b| b.1.cmp(a.1));
    
    for (syscall, count) in sorted_syscalls.iter().take(5) {
        println!("  syscall {}: {} calls", syscall, count);
    }
    println!();
    
    println!("Call sites in .text:");
    for instr in instructions.iter() {
        if let Some(mnem) = instr.mnemonic() {
            if mnem == "call" {
                let op = instr.op_str().unwrap_or("?");
                println!("  0x{:x}: call {}", instr.address(), op);
            }
        }
    }
    println!();
    
    println!("Parameter registers used:");
    let mut regs = std::collections::HashSet::new();
    for instr in instructions.iter() {
        let op = instr.op_str().unwrap_or("");
        if op.contains("edi") { regs.insert("edi"); }
        if op.contains("rsi") { regs.insert("rsi"); }
        if op.contains("rdx") { regs.insert("rdx"); }
    }
    
    for reg in regs.iter() {
        println!("  {}", reg);
    }
    
    Ok(())
}
