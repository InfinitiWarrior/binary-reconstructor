use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== /bin/ls SYSCALL TRACE ANALYSIS ===\n");
    
    let file = File::open("ls_syscalls.json")?;
    let reader = BufReader::new(file);
    
    let mut syscall_types: HashMap<u64, usize> = HashMap::new();
    let mut total = 0;
    let mut timestamps = Vec::new();
    
    for line in reader.lines() {
        let line = line?;
        if line.starts_with('{') {
            total += 1;
            
            if let Some(sc_start) = line.find("\"syscall\":") {
                let sc_str = &line[sc_start + 10..];
                if let Some(sc_end) = sc_str.find('}') {
                    if let Ok(sc) = sc_str[..sc_end].parse::<u64>() {
                        *syscall_types.entry(sc).or_insert(0) += 1;
                    }
                }
            }
            
            if let Some(ts_start) = line.find("\"timestamp\":") {
                let ts_str = &line[ts_start + 12..];
                if let Some(ts_end) = ts_str.find(',') {
                    if let Ok(ts) = ts_str[..ts_end].parse::<u64>() {
                        timestamps.push(ts);
                    }
                }
            }
            
            if total % 5000000 == 0 {
                println!("Processed {} events", total);
            }
        }
    }
    
    println!("\nTrace Statistics:");
    println!("Total syscall events: {}", total);
    
    if !timestamps.is_empty() {
        let duration_ns = timestamps[timestamps.len() - 1] - timestamps[0];
        let duration_ms = duration_ns / 1_000_000;
        println!("Execution duration: {} ms", duration_ms);
    }
    
    println!("\nTop syscalls:");
    let mut sorted: Vec<_> = syscall_types.iter().collect();
    sorted.sort_by(|a, b| b.1.cmp(a.1));
    
    for (sc, count) in sorted.iter().take(10) {
        let pct = (**count as f64 / total as f64) * 100.0;
        println!("  syscall {}: {} ({:.1}%)", sc, count, pct);
    }
    
    Ok(())
}
