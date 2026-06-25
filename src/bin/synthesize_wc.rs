use std::collections::HashMap;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = read_config("/home/inf/.analysis/config")?;
    
    println!("#include <stdio.h>");
    println!("#include <unistd.h>");
    println!("#include <fcntl.h>");
    println!("#include <ctype.h>");
    println!("#include <string.h>\n");
    
    println!("int main(int argc, char *argv[]) {{");
    println!("    char buf[4096];");
    println!("    int lines = 0, words = 0, chars = 0;");
    println!("    int fd, n, i, in_word = 0;");
    println!("    int show_lines = 1, show_words = 1, show_chars = 1;");
    println!();
    println!("    int opt_idx = 1;");
    println!("    while (opt_idx < argc && argv[opt_idx][0] == '-') {{");
    println!("        if (strcmp(argv[opt_idx], \"-l\") == 0) show_words = show_chars = 0;");
    println!("        else if (strcmp(argv[opt_idx], \"-w\") == 0) show_lines = show_chars = 0;");
    println!("        else if (strcmp(argv[opt_idx], \"-c\") == 0) show_lines = show_words = 0;");
    println!("        opt_idx++;");
    println!("    }}");
    println!();
    println!("    fd = (opt_idx < argc) ? open(argv[opt_idx], O_RDONLY) : 0;");
    println!();
    println!("    while ((n = read(fd, buf, sizeof(buf))) > 0) {{");
    println!("        for (i = 0; i < n; i++) {{");
    println!("            chars++;");
    println!("            if (buf[i] == '\\n') lines++;");
    println!("            if (isspace(buf[i])) in_word = 0;");
    println!("            else if (!in_word) {{ words++; in_word = 1; }}");
    println!("        }}");
    println!("    }}");
    println!();
    println!("    if (show_lines && show_words && show_chars)");
    println!("        printf(\"%7d %7d %7d\\n\", lines, words, chars);");
    println!("    else if (show_words && !show_lines)");
    println!("        printf(\"%d\\n\", words);");
    println!("    else");
    println!("        printf(\"%d %d %d\\n\", lines, words, chars);");
    println!();
    println!("    return 0;");
    println!("}}");
    
    Ok(())
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
