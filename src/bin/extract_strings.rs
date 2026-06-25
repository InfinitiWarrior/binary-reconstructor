use std::fs;
use std::io::Read;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let binary_path = std::env::args().nth(1).unwrap_or("/bin/yes".to_string());
    let mut file = fs::File::open(&binary_path)?;
    let mut data = Vec::new();
    file.read_to_end(&mut data)?;
    
    println!("=== RODATA STRINGS ===");
    
    for window in data.windows(8) {
        if window[0] >= 32 && window[0] < 127 {
            let mut s = String::new();
            for &byte in window {
                if byte >= 32 && byte < 127 {
                    s.push(byte as char);
                } else {
                    break;
                }
            }
            if s.len() > 1 && s.len() < 100 {
                println!("\"{}\"", s);
            }
        }
    }
    
    Ok(())
}
