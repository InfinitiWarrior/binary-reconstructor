use std::fs::File;
use std::io::Write;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let source = r#"#include <stdio.h>

void process_data(int x) {
    printf("Processing %d\n", x);
}

int main(int argc, char *argv[]) {
    int i = 0;
    
    while (i <= 4) {
        process_data(i);
        i++;
    }
    
    return 0;
}
"#;
    
    let mut file = File::create("reconstructed.c")?;
    file.write_all(source.as_bytes())?;
    println!("Wrote reconstructed.c");
    
    Ok(())
}
