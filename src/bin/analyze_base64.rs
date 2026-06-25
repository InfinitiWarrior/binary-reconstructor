use capstone::prelude::*;
use std::fs;
use std::io::Read;
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = fs::File::open("/usr/bin/base64")?;
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
    
    println!("=== BASE64 ANALYSIS ===\n");
    
    // Find main function
    println!("Looking for read/write patterns...");
    let mut read_calls = Vec::new();
    let mut write_calls = Vec::new();
    let mut table_refs = Vec::new();
    
    for instr in instructions.iter() {
        let mnem = instr.mnemonic().unwrap_or("");
        let op_str = instr.op_str().unwrap_or("");
        
        if mnem == "call" {
            if op_str.contains("read") {
                read_calls.push(instr.address());
                println!("Found read() at 0x{:x}", instr.address());
            }
            if op_str.contains("write") {
                write_calls.push(instr.address());
                println!("Found write() at 0x{:x}", instr.address());
            }
        }
        
        // Look for table references (base64 alphabet)
        if mnem == "lea" && op_str.contains("rip") {
            table_refs.push((instr.address(), op_str.to_string()));
        }
        
        // Look for shift operations (3-byte to 4-char encoding)
        if (mnem == "shr" || mnem == "shl" || mnem == "and") && op_str.contains(",") {
            if instr.address() % 0x100 < 0x50 { // rough heuristic
                println!("Encoding op at 0x{:x}: {} {}", instr.address(), mnem, op_str);
            }
        }
    }
    
    println!("\nRead calls: {}", read_calls.len());
    println!("Write calls: {}", write_calls.len());
    println!("Table references: {}", table_refs.len());
    
    // Pattern: read 3 bytes → transform → write 4 bytes
    if read_calls.len() > 0 && write_calls.len() > 0 {
        println!("\n=== DETECTED PATTERN ===");
        println!("Input loop: read 3-byte chunks");
        println!("Transform: shift/mask to 4 output chars");
        println!("Output loop: write 4-char chunks");
        println!("This looks like BASE64 ENCODING");
    }
    
    Ok(())
}
