use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("syscall_events.json")?;
    let reader = BufReader::new(file);
    
    let mut syscalls: HashMap<u64, u64> = HashMap::new();
    let mut line_count = 0;
    
    for line in reader.lines() {
        let line = line?;
        if line.starts_with('{') {
            line_count += 1;
            
            if let Some(start) = line.find("\"syscall\":") {
                let rest = &line[start + 10..];
                if let Some(end) = rest.find('}') {
                    if let Ok(syscall) = rest[..end].trim_end_matches(',').parse::<u64>() {
                        *syscalls.entry(syscall).or_insert(0) += 1;
                    }
                }
            }
            
            if line_count % 1000000 == 0 {
                println!("Processed {} events...", line_count);
            }
        }
    }
    
    println!("\nTotal events: {}", line_count);
    println!("\nTop 20 syscalls:");
    
    let mut sorted: Vec<_> = syscalls.iter().collect();
    sorted.sort_by(|a, b| b.1.cmp(a.1));
    
    for (syscall, count) in sorted.iter().take(20) {
        println!("  syscall {}: {} calls", syscall, count);
    }
    
    Ok(())
}
