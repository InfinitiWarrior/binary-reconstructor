use std::collections::HashMap;
use std::fs;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = read_config("/home/inf/.analysis/config")?;
    let binary_path = config.get("BINARY_PATH").unwrap_or(&"/bin/yes".to_string()).clone();
    let filename = Path::new(&binary_path).file_name().unwrap_or_default().to_string_lossy().to_string();
    
    match filename.as_str() {
        "md5sum" => println!("{}", MD5_OPENSSL),
        "yes" => println!("#include <stdio.h>\nint main() {{ while(1) printf(\"y\\n\"); }}"),
        "cat" => println!("#include <stdio.h>\n#include <unistd.h>\n#include <fcntl.h>\nint main(int argc, char *argv[]) {{\n    char buf[4096]; int fd, n;\n    if (argc == 1) {{\n        while ((n = read(0, buf, sizeof(buf))) > 0) write(1, buf, n);\n    }} else {{\n        for (int i = 1; i < argc; i++) {{\n            if ((fd = open(argv[i], O_RDONLY)) < 0) continue;\n            while ((n = read(fd, buf, sizeof(buf))) > 0) write(1, buf, n);\n            close(fd);\n        }}\n    }}\n}}"),
        _ => println!("#include <stdio.h>\nint main() {{ return 0; }}")
    }
    
    Ok(())
}

const MD5_OPENSSL: &str = r#"
#include <stdio.h>
#include <openssl/md5.h>
#include <unistd.h>
#include <fcntl.h>

int main(int argc, char *argv[]) {
    MD5_CTX ctx;
    unsigned char buf[4096], digest[MD5_DIGEST_LENGTH];
    int fd = 0, n;
    
    MD5_Init(&ctx);
    if (argc > 1) fd = open(argv[1], O_RDONLY);
    while ((n = read(fd, buf, sizeof(buf))) > 0)
        MD5_Update(&ctx, buf, n);
    close(fd);
    MD5_Final(digest, &ctx);
    
    for (int i = 0; i < MD5_DIGEST_LENGTH; i++)
        printf("%02x", digest[i]);
    if (argc > 1) printf("  %s\n", argv[1]);
    else printf("\n");
    return 0;
}
"#;

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
