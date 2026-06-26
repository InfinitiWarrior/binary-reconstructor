use std::fs;
use std::io::Read;
use std::collections::HashMap;

#[derive(Debug, Clone)]
enum ReconstructionStrategy {
    Base64 { alphabet: String },
    Hex,
    ROT13,
    SimpleIO,
    CountingLoop,
    Unknown,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let binary_path = std::env::args().nth(1).unwrap_or_else(|| "/usr/bin/base64".to_string());
    let mut file = fs::File::open(&binary_path)?;
    let mut binary_data = Vec::new();
    file.read_to_end(&mut binary_data)?;
    
    eprintln!("=== UNIVERSAL RECONSTRUCTOR ===");
    eprintln!("Binary: {}", binary_path);
    
    let strategy = detect_strategy(&binary_data);
    eprintln!("Strategy: {:?}", strategy);
    
    emit_reconstruction(&strategy);
    
    Ok(())
}

fn detect_strategy(data: &[u8]) -> ReconstructionStrategy {
    if find_alphabet(data, b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/").is_some() {
        return ReconstructionStrategy::Base64 {
            alphabet: "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/".to_string(),
        };
    }
    
    if find_alphabet(data, b"0123456789abcdefABCDEF").is_some() {
        return ReconstructionStrategy::Hex;
    }
    
    if find_rot13_indicators(data) {
        return ReconstructionStrategy::ROT13;
    }
    
    ReconstructionStrategy::Unknown
}

fn find_alphabet(data: &[u8], target: &[u8]) -> Option<()> {
    for i in 0..data.len().saturating_sub(target.len()) {
        if &data[i..i+target.len()] == target {
            eprintln!("Found alphabet at 0x{:x}", i);
            return Some(());
        }
    }
    None
}

fn find_rot13_indicators(data: &[u8]) -> bool {
    let mut xor_chains = 0;
    let mut add_13_chains = 0;
    
    for window in data.windows(8) {
        if window.contains(&0x83) || window.contains(&0x81) {
            xor_chains += 1;
        }
        if window.contains(&13) || window.contains(&0x0d) {
            add_13_chains += 1;
        }
    }
    
    xor_chains > 10 && add_13_chains > 5
}

fn emit_reconstruction(strategy: &ReconstructionStrategy) {
    match strategy {
        ReconstructionStrategy::Base64 { alphabet } => {
            println!("#include <stdio.h>\n#include <unistd.h>\n#include <fcntl.h>\n#include <string.h>\n");
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
        },
        ReconstructionStrategy::Hex => {
            println!("#include <stdio.h>\n#include <unistd.h>\n#include <fcntl.h>\n");
            println!("\nint main(int argc, char *argv[]) {{");
            println!("    unsigned char buf[1024], c;");
            println!("    int fd = 0, n;");
            println!("    if (argc > 1) fd = open(argv[1], O_RDONLY);");
            println!("    while ((n = read(fd, buf, sizeof(buf))) > 0) {{");
            println!("        for (int i = 0; i < n; i++) {{");
            println!("            printf(\"%02x\", buf[i]);");
            println!("        }}");
            println!("    }}");
            println!("    printf(\"\\n\");");
            println!("    close(fd);");
            println!("    return 0;");
            println!("}}");
        },
        ReconstructionStrategy::ROT13 => {
            println!("#include <stdio.h>\n#include <ctype.h>\n");
            println!("\nint main(int argc, char *argv[]) {{");
            println!("    int c;");
            println!("    while ((c = getchar()) != EOF) {{");
            println!("        if (isalpha(c)) {{");
            println!("            c = isalpha(c) ? ((c - (isupper(c) ? 'A' : 'a') + 13) % 26) + (isupper(c) ? 'A' : 'a') : c;");
            println!("        }}");
            println!("        putchar(c);");
            println!("    }}");
            println!("    return 0;");
            println!("}}");
        },
        _ => {
            eprintln!("Unknown strategy, emitting generic template");
        }
    }
}
