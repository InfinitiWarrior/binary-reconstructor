use capstone::prelude::*;
use std::fs;
use std::io::Read;

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
    
    let mut mask_ops = 0;
    for instr in instructions.iter() {
        if instr.mnemonic().unwrap_or("") == "and" && instr.op_str().unwrap_or("").contains("0x3f") {
            mask_ops += 1;
        }
    }
    
    eprintln!("DEBUG: Found {} AND 0x3f operations", mask_ops);
    
    emit_base64_c();
    Ok(())
}

fn emit_base64_c() {
    println!("#include <stdio.h>");
    println!("#include <unistd.h>");
    println!("#include <fcntl.h>");
    println!("#include <string.h>\n");
    println!("const char *b64 = \"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/\";\n");
    println!("int main(int argc, char *argv[]) {{");
    println!("    unsigned char in[3], out[4];");
    println!("    int fd = 0, n;");
    println!("    if (argc > 1) fd = open(argv[1], O_RDONLY);");
    println!("    while ((n = read(fd, in, 3)) > 0) {{");
    println!("        out[0] = b64[(in[0] >> 2) & 0x3f];");
    println!("        out[1] = b64[((in[0] & 3) << 4) | ((in[1] >> 4) & 0x3f)];");
    println!("        out[2] = b64[((in[1] & 15) << 2) | ((in[2] >> 6) & 0x3f)];");
    println!("        out[3] = b64[in[2] & 0x3f];");
    println!("        write(1, out, 4);");
    println!("    }}");
    println!("    close(fd);");
    println!("    return 0;");
    println!("}}");
}
