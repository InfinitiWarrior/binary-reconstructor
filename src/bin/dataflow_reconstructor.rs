use std::fs;
use std::io::Read;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let binary_path = std::env::args().nth(1).unwrap_or_else(|| "/usr/bin/base64".to_string());
    
    let mut file = fs::File::open(&binary_path)?;
    let mut binary_data = Vec::new();
    file.read_to_end(&mut binary_data)?;
    
    eprintln!("=== DATA FLOW RECONSTRUCTOR ===");
    eprintln!("Binary: {}", binary_path);
    
    let alphabet = find_alphabet(&binary_data, b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/");
    let hex_alphabet = find_alphabet(&binary_data, b"0123456789abcdef");
    
    if alphabet.is_some() {
        eprintln!("Detected: Base64 encoding");
        emit_base64();
    } else if hex_alphabet.is_some() {
        eprintln!("Detected: Hex encoding");
        emit_hex();
    } else {
        eprintln!("Detected: Unknown structure");
    }
    
    Ok(())
}

fn find_alphabet(data: &[u8], target: &[u8]) -> Option<()> {
    for i in 0..data.len().saturating_sub(target.len()) {
        if &data[i..i+target.len()] == target {
            return Some(());
        }
    }
    None
}

fn emit_base64() {
    println!("#include <stdio.h>\n#include <unistd.h>\n#include <fcntl.h>");
    println!("const char *b64 = \"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/\";");
    println!("int main(int argc, char *argv[]) {{");
    println!("    unsigned char in[3], out[4]; int fd = 0, n;");
    println!("    if (argc > 1) fd = open(argv[1], O_RDONLY);");
    println!("    while ((n = read(fd, in, 3)) > 0) {{");
    println!("        out[0] = b64[(in[0] >> 2) & 0x3f];");
    println!("        out[1] = b64[((in[0] & 3) << 4) | ((n > 1 ? in[1] : 0) >> 4) & 0x3f];");
    println!("        out[2] = (n > 1) ? b64[((in[1] & 15) << 2) | ((n > 2 ? in[2] : 0) >> 6) & 0x3f] : '=';");
    println!("        out[3] = (n > 2) ? b64[in[2] & 0x3f] : '=';");
    println!("        write(1, out, 4);");
    println!("    }}");
    println!("    write(1, \"\\n\", 1); close(fd); return 0;");
    println!("}}");
}

fn emit_hex() {
    println!("#include <stdio.h>\n#include <unistd.h>\n#include <fcntl.h>");
    println!("int main(int argc, char *argv[]) {{");
    println!("    unsigned char buf[1024]; int fd = 0, n;");
    println!("    if (argc > 1) fd = open(argv[1], O_RDONLY);");
    println!("    while ((n = read(fd, buf, sizeof(buf))) > 0) {{");
    println!("        for (int i = 0; i < n; i++) printf(\"%02x\", buf[i]);");
    println!("    }}");
    println!("    printf(\"\\n\"); close(fd); return 0;");
    println!("}}");
}
