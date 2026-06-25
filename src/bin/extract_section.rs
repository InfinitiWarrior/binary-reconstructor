use std::fs;
use std::io::Read;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let binary_path = std::env::args().nth(1).unwrap_or("/bin/ls".to_string());
    let mut file = fs::File::open(&binary_path)?;
    let mut data = Vec::new();
    file.read_to_end(&mut data)?;
    
    let e_shoff = u32::from_le_bytes([data[32], data[33], data[34], data[35]]) as usize;
    let e_shentsize = u16::from_le_bytes([data[46], data[47]]) as usize;
    let e_shnum = u16::from_le_bytes([data[48], data[49]]) as usize;
    
    for i in 0..e_shnum {
        let offset = e_shoff + i * e_shentsize;
        if offset + 64 > data.len() { break; }
        
        let sh_offset = u64::from_le_bytes([data[offset+16], data[offset+17], data[offset+18], data[offset+19], data[offset+20], data[offset+21], data[offset+22], data[offset+23]]) as usize;
        let sh_size = u64::from_le_bytes([data[offset+32], data[offset+33], data[offset+34], data[offset+35], data[offset+36], data[offset+37], data[offset+38], data[offset+39]]) as usize;
        
        if sh_size == 0 || sh_size > 0x1000000 { continue; }
        if sh_offset + sh_size > data.len() { continue; }
        
        println!("Section {}: offset=0x{:x}, size=0x{:x}", i, sh_offset, sh_size);
    }
    
    Ok(())
}
