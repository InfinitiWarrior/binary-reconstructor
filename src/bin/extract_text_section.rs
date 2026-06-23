use std::env;
use std::fs::File;
use std::io::Read;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: {} <binary>", args[0]);
        return Ok(());
    }
    
    let binary_path = &args[1];
    let mut file = File::open(binary_path)?;
    let mut data = Vec::new();
    file.read_to_end(&mut data)?;
    
    if data.len() < 0x40 {
        eprintln!("File too small");
        return Ok(());
    }
    
    let e_shoff = u64::from_le_bytes([
        data[0x28], data[0x29], data[0x2a], data[0x2b],
        data[0x2c], data[0x2d], data[0x2e], data[0x2f],
    ]) as usize;
    
    let e_shentsize = u16::from_le_bytes([data[0x3a], data[0x3b]]) as usize;
    let e_shnum = u16::from_le_bytes([data[0x3c], data[0x3d]]) as usize;
    let e_shstrndx = u16::from_le_bytes([data[0x3e], data[0x3f]]) as usize;
    
    println!("ELF Header:");
    println!("  Section header offset: 0x{:x}", e_shoff);
    println!("  Section header size: {} bytes", e_shentsize);
    println!("  Number of sections: {}", e_shnum);
    println!();
    
    let mut text_offset = 0;
    let mut text_size = 0;
    
    for i in 0..e_shnum {
        let sh_offset = e_shoff + i as usize * e_shentsize;
        if sh_offset + 0x28 > data.len() {
            break;
        }
        
        let sh_offset_data = u64::from_le_bytes([
            data[sh_offset + 0x18], data[sh_offset + 0x19], data[sh_offset + 0x1a], data[sh_offset + 0x1b],
            data[sh_offset + 0x1c], data[sh_offset + 0x1d], data[sh_offset + 0x1e], data[sh_offset + 0x1f],
        ]) as usize;
        
        let sh_size = u64::from_le_bytes([
            data[sh_offset + 0x20], data[sh_offset + 0x21], data[sh_offset + 0x22], data[sh_offset + 0x23],
            data[sh_offset + 0x24], data[sh_offset + 0x25], data[sh_offset + 0x26], data[sh_offset + 0x27],
        ]) as usize;
        
        let sh_name = u32::from_le_bytes([
            data[sh_offset], data[sh_offset + 1], 
            data[sh_offset + 2], data[sh_offset + 3],
        ]) as usize;
        
        let strtab_sh_offset = e_shoff + e_shstrndx as usize * e_shentsize;
        let strtab_offset = u64::from_le_bytes([
            data[strtab_sh_offset + 0x18], data[strtab_sh_offset + 0x19],
            data[strtab_sh_offset + 0x1a], data[strtab_sh_offset + 0x1b],
            data[strtab_sh_offset + 0x1c], data[strtab_sh_offset + 0x1d],
            data[strtab_sh_offset + 0x1e], data[strtab_sh_offset + 0x1f],
        ]) as usize;
        
        if strtab_offset + sh_name + 5 < data.len() {
            let name_slice = &data[strtab_offset + sh_name..strtab_offset + sh_name + 6];
            if name_slice == b".text" || name_slice.starts_with(b".text") {
                text_offset = sh_offset_data;
                text_size = sh_size;
                println!(".text section found:");
                println!("  Offset: 0x{:x}", text_offset);
                println!("  Size: 0x{:x} ({} bytes)", text_size, text_size);
            }
        }
    }
    
    Ok(())
}
