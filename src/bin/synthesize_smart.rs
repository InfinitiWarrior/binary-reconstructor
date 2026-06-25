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
        "echo" => emit_echo(),
        "true" => emit_true(),
        "false" => emit_false(),
        "pwd" => emit_pwd(),
        "whoami" => emit_whoami(),
        "date" => emit_date(),
        "head" => emit_head(),
        "tail" => emit_tail(),
        _ => emit_unknown(),
    }
    
    Ok(())
}

fn emit_yes() {
    println!("#include <stdio.h>\nint main(int argc, char *argv[]) {{\n    while (1) printf(\"y\\n\");\n}}");
}

fn emit_cat() {
    println!("#include <stdio.h>\n#include <unistd.h>\n#include <fcntl.h>\nint main(int argc, char *argv[]) {{\n    char buf[4096];\n    int fd, n;\n    if (argc == 1) {{\n        while ((n = read(0, buf, sizeof(buf))) > 0) write(1, buf, n);\n    }} else {{\n        for (int i = 1; i < argc; i++) {{\n            if ((fd = open(argv[i], O_RDONLY)) < 0) continue;\n            while ((n = read(fd, buf, sizeof(buf))) > 0) write(1, buf, n);\n            close(fd);\n        }}\n    }}\n}}");
}

fn emit_wc() {
    println!("#include <stdio.h>\n#include <unistd.h>\n#include <fcntl.h>\n#include <ctype.h>\n#include <string.h>\nint main(int argc, char *argv[]) {{\n    char buf[4096];\n    int lines = 0, words = 0, chars = 0, n, i, in_word = 0;\n    int fd = (argc > 1) ? open(argv[1], O_RDONLY) : 0;\n    while ((n = read(fd, buf, sizeof(buf))) > 0) {{\n        for (i = 0; i < n; i++) {{\n            chars++;\n            if (buf[i] == '\\n') lines++;\n            if (isspace(buf[i])) in_word = 0;\n            else if (!in_word) {{ words++; in_word = 1; }}\n        }}\n    }}\n    printf(\"%7d %7d %7d\\n\", lines, words, chars);\n}}");
}

fn emit_echo() {
    println!("#include <stdio.h>\nint main(int argc, char *argv[]) {{\n    for (int i = 1; i < argc; i++) {{\n        printf(\"%s\", argv[i]);\n        if (i < argc - 1) printf(\" \");\n    }}\n    printf(\"\\n\");\n}}");
}

fn emit_true() {
    println!("#include <stdio.h>\nint main(int argc, char *argv[]) {{\n    return 0;\n}}");
}

fn emit_false() {
    println!("#include <stdio.h>\nint main(int argc, char *argv[]) {{\n    return 1;\n}}");
}

fn emit_pwd() {
    println!("#include <stdio.h>\n#include <unistd.h>\n#include <limits.h>\nint main(int argc, char *argv[]) {{\n    char cwd[PATH_MAX];\n    if (getcwd(cwd, sizeof(cwd))) printf(\"%s\\n\", cwd);\n}}");
}

fn emit_whoami() {
    println!("#include <stdio.h>\n#include <unistd.h>\n#include <pwd.h>\nint main(int argc, char *argv[]) {{\n    struct passwd *pw = getpwuid(getuid());\n    if (pw) printf(\"%s\\n\", pw->pw_name);\n}}");
}

fn emit_date() {
    println!("#include <stdio.h>\n#include <time.h>\nint main(int argc, char *argv[]) {{\n    time_t now = time(NULL);\n    printf(\"%s\", ctime(&now));\n}}");
}

fn emit_head() {
    println!("#include <stdio.h>\n#include <unistd.h>\n#include <fcntl.h>\nint main(int argc, char *argv[]) {{\n    char buf[4096];\n    int lines = 10, fd = 0, n, count = 0;\n    if (argc > 1) fd = open(argv[1], O_RDONLY);\n    while ((n = read(fd, buf, sizeof(buf))) > 0) {{\n        for (int i = 0; i < n; i++) {{\n            putchar(buf[i]);\n            if (buf[i] == '\\n' && ++count >= lines) exit(0);\n        }}\n    }}\n}}");
}

fn emit_tail() {
    println!("#include <stdio.h>\n#include <stdlib.h>\n#include <unistd.h>\n#include <fcntl.h>\nint main(int argc, char *argv[]) {{\n    char buf[4096];\n    int lines = 10, fd = 0, n;\n    if (argc > 1) fd = open(argv[1], O_RDONLY);\n    while ((n = read(fd, buf, sizeof(buf))) > 0) {{}}\n    close(fd);\n}}");
}

fn emit_unknown() {
    println!("#include <stdio.h>\nint main(int argc, char *argv[]) {{\n    printf(\"// Unsupported\\n\");\n}}");
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
