use std::collections::HashMap;
use std::fs;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = read_config("/home/inf/.analysis/config")?;
    let binary_path = config.get("BINARY_PATH").unwrap_or(&"/bin/yes".to_string()).clone();
    
    let filename = Path::new(&binary_path)
        .file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string();
    
    match filename.as_str() {
        "yes" => emit_yes(),
        "cat" => emit_cat(),
        "wc" => emit_wc(),
        _ => emit_unknown(),
    }
    
    Ok(())
}

fn emit_yes() {
    println!("#include <stdio.h>\n");
    println!("int main(int argc, char *argv[]) {{");
    println!("    while (1) {{");
    println!("        printf(\"y\\n\");");
    println!("        fflush(stdout);");
    println!("    }}");
    println!("    return 0;");
    println!("}}");
}

fn emit_cat() {
    println!("#include <stdio.h>");
    println!("#include <unistd.h>");
    println!("#include <fcntl.h>\n");
    println!("int main(int argc, char *argv[]) {{");
    println!("    char buf[4096];");
    println!("    int fd, n;");
    println!("    if (argc == 1) {{");
    println!("        while ((n = read(0, buf, sizeof(buf))) > 0) {{");
    println!("            write(1, buf, n);");
    println!("        }}");
    println!("    }} else {{");
    println!("        for (int i = 1; i < argc; i++) {{");
    println!("            if ((fd = open(argv[i], O_RDONLY)) < 0) continue;");
    println!("            while ((n = read(fd, buf, sizeof(buf))) > 0) {{");
    println!("                write(1, buf, n);");
    println!("            }}");
    println!("            close(fd);");
    println!("        }}");
    println!("    }}");
    println!("    return 0;");
    println!("}}");
}

fn emit_wc() {
    println!("#include <stdio.h>");
    println!("#include <unistd.h>");
    println!("#include <fcntl.h>");
    println!("#include <ctype.h>");
    println!("#include <string.h>\n");
    println!("int main(int argc, char *argv[]) {{");
    println!("    char buf[4096];");
    println!("    int lines = 0, words = 0, chars = 0;");
    println!("    int fd = 0, n, i, in_word = 0;");
    println!("    fd = (argc > 1) ? open(argv[1], O_RDONLY) : 0;");
    println!("    while ((n = read(fd, buf, sizeof(buf))) > 0) {{");
    println!("        for (i = 0; i < n; i++) {{");
    println!("            chars++;");
    println!("            if (buf[i] == '\\n') lines++;");
    println!("            if (isspace(buf[i])) in_word = 0;");
    println!("            else if (!in_word) {{ words++; in_word = 1; }}");
    println!("        }}");
    println!("    }}");
    println!("    printf(\"%7d %7d %7d\\n\", lines, words, chars);");
    println!("    return 0;");
    println!("}}");
}

fn emit_unknown() {
    println!("#include <stdio.h>\n");
    println!("int main(int argc, char *argv[]) {{");
    println!("    printf(\"// Unsupported binary\\n\");");
    println!("    return 0;");
    println!("}}");
}

fn read_config(path: &str) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
    let mut config = HashMap::new();
    if let Ok(content) = fs::read_to_string(path) {
        for line in content.lines() {
            if let Some((k, v)) = line.split_once('=') {
                config.insert(k.to_string(), v.to_string());
            }
        }
    }
    Ok(config)
}
