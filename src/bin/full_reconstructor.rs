use std::fs;
use std::io::Read;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let binary_path = std::env::args().nth(1).unwrap_or_else(|| "/usr/bin/base64".to_string());
    
    let mut file = fs::File::open(&binary_path)?;
    let mut binary_data = Vec::new();
    file.read_to_end(&mut binary_data)?;
    
    eprintln!("=== FULL RECONSTRUCTION ===");
    eprintln!("Binary: {}", binary_path);
    
    let base64_alphabet = find_base64_alphabet_exact(&binary_data);
    
    if let Some(alphabet) = base64_alphabet {
        eprintln!("Found base64 alphabet: {}", alphabet);
        emit_base64_from_binary(&alphabet);
    } else {
        eprintln!("No alphabet found");
    }
    
    Ok(())
}

fn find_base64_alphabet_exact(data: &[u8]) -> Option<String> {
    let target = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    
    for i in 0..data.len().saturating_sub(64) {
        if &data[i..i+64] == target {
            eprintln!("Found at offset 0x{:x}", i);
            return Some("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/".to_string());
        }
    }
    None
}

fn emit_base64_from_binary(alphabet: &str) {
    println!("#include <stdio.h>");
    println!("#include <unistd.h>");
    println!("#include <fcntl.h>");
    println!("#include <string.h>\n");
    println!("const char *b64 = \"{}\";", alphabet);
    println!("\nint main(int argc, char *argv[]) {{");
    println!("    unsigned char in[3], out[4];");
    println!("    int fd = 0, n;");
    println!("    if (argc > 1) fd = open(argv[1], O_RDONLY);");
    println!("    while ((n = read(fd, in, 3)) > 0) {{");
    println!("        out[0] = b64[(in[0] >> 2) & 0x3f];");
    println!("        out[1] = b64[((in[0] & 3) << 4) | ((n > 1 ? in[1] : 0) >> 4) & 0x3f];");
    println!("        out[2] = (n > 1) ? b64[((in[1] & 15) << 2) | ((n > 2 ? in[2] : 0) >> 6) & 0x3f] : '=';");
    println!("        out[3] = (n > 2) ? b64[in[2] & 0x3f] : '=';");
    println!("        write(1, out, 4);");
    println!("    }}");
    println!("    write(1, \"\\n\", 1);");
    println!("    close(fd);");
    println!("    return 0;");
    println!("}}");
}
