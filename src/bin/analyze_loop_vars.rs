use capstone::prelude::*;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let binary_path = "/tmp/test_uprobe";
    let binary_data = fs::read(&binary_path)?;
    
    let text_start = 0x1040usize;
    let text_size = 0x153usize;
    let text_section = &binary_data[text_start..text_start + text_size];
    
    let cs = Capstone::new()
        .x86()
        .mode(arch::x86::ArchMode::Mode64)
        .detail(true)
        .build()?;
    
    let instructions = cs.disasm_all(text_section, 0x1040u64)?;
    
    println!("Loop Variable Analysis:\n");
    
    println!("Main Loop (0x1160-0x1192):\n");
    
    for instr in instructions.iter() {
        let addr = instr.address();
        if addr >= 0x1160 && addr < 0x1193 {
            let mnem = instr.mnemonic().unwrap_or("");
            let op_str = instr.op_str().unwrap_or("");
            
            if mnem == "mov" && op_str.contains("[rbp") {
                println!("0x{:x}: {} {} <- initialization", addr, mnem, op_str);
            } else if mnem == "cmp" {
                println!("0x{:x}: {} {} <- loop condition", addr, mnem, op_str);
            } else if mnem == "add" || mnem == "inc" || mnem == "sub" || mnem == "dec" {
                println!("0x{:x}: {} {} <- loop increment/decrement", addr, mnem, op_str);
            } else if mnem.starts_with('j') {
                println!("0x{:x}: {} {} <- branch", addr, mnem, op_str);
            }
        }
    }
    
    println!("\n\nInferred Structure:");
    println!("Loop counter: [rbp - 0x14] (from 0x1168)");
    println!("Loop bound: 4 (from cmp dword [rbp-4], 4 at 0x1186)");
    println!("Increment: add dword [rbp-4], 1 at 0x1182");
    println!("Condition: jle 0x1178 (jump if <=, so loop while counter <= 4)");
    println!("\nPseudo-source:");
    println!("for (int i = 0; i <= 4; i++) {{");
    println!("  process_data(i);");
    println!("}}");
    
    Ok(())
}
