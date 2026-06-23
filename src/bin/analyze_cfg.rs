use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("syscall_events.json")?;
    let reader = BufReader::new(file);
    
    let mut syscall_seq = Vec::new();
    let mut block_sizes: HashMap<usize, usize> = HashMap::new();
    let mut line_count = 0;
    
    for line in reader.lines() {
        let line = line?;
        if line.starts_with('{') {
            line_count += 1;
            
            if let Some(start) = line.find("\"syscall\":") {
                let rest = &line[start + 10..];
                if let Ok(syscall) = rest.split('}').next().unwrap_or("").trim_end_matches(',').parse::<u64>() {
                    syscall_seq.push(syscall);
                    
                    if syscall_seq.len() >= 100 {
                        *block_sizes.entry(syscall_seq.len()).or_insert(0) += 1;
                        syscall_seq.clear();
                    }
                }
            }
            
            if line_count % 10000000 == 0 {
                println!("Analyzed {} events...", line_count);
            }
        }
    }
    
    println!("\nBlock Size Distribution:");
    let mut sizes: Vec<_> = block_sizes.iter().collect();
    sizes.sort_by_key(|a| a.0);
    
    for (size, count) in sizes {
        println!("  {} syscalls: {} blocks", size, count);
    }
    
    Ok(())
}
