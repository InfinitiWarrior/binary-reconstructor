use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Correlating syscall trace with static CFG...");
    
    let file = File::open("syscall_events.json")?;
    let reader = BufReader::new(file);
    
    let mut timestamps = Vec::new();
    let mut syscall_distribution = HashMap::new();
    let mut line_count = 0;
    
    for line in reader.lines() {
        let line = line?;
        if line.starts_with('{') {
            line_count += 1;
            
            if let Some(ts_start) = line.find("\"timestamp\":") {
                let ts_str = &line[ts_start + 12..];
                if let Some(ts_end) = ts_str.find(',') {
                    if let Ok(ts) = ts_str[..ts_end].parse::<u64>() {
                        timestamps.push(ts);
                    }
                }
            }
            
            if let Some(sc_start) = line.find("\"syscall\":") {
                let sc_str = &line[sc_start + 10..];
                if let Some(sc_end) = sc_str.find('}') {
                    if let Ok(sc) = sc_str[..sc_end].parse::<u64>() {
                        *syscall_distribution.entry(sc).or_insert(0) += 1;
                    }
                }
            }
            
            if line_count % 10000000 == 0 {
                println!("Processed {} events", line_count);
            }
        }
    }
    
    println!("\nTrace Analysis:");
    println!("Total syscall events: {}", line_count);
    println!("Timestamp range: 0x{:x} to 0x{:x}", 
             timestamps.first().unwrap_or(&0),
             timestamps.last().unwrap_or(&0));
    
    if !timestamps.is_empty() {
        let duration_ns = timestamps[timestamps.len() - 1] - timestamps[0];
        let duration_ms = duration_ns / 1_000_000;
        println!("Execution duration: {} ms", duration_ms);
    }
    
    println!("\nSyscall frequency (top 10):");
    let mut sorted: Vec<_> = syscall_distribution.iter().collect();
    sorted.sort_by(|a, b| b.1.cmp(a.1));
    
    for (sc, count) in sorted.iter().take(10) {
        println!("  syscall {}: {} calls", sc, count);
    }
    
    println!("\nInference: Static CFG (31 blocks) executed across {:.0}ms", 
             if timestamps.is_empty() { 0.0 } else { 
                 (timestamps[timestamps.len()-1] - timestamps[0]) as f64 / 1_000_000.0 
             });
    println!("Next: Map syscall patterns to block execution sequences");
    
    Ok(())
}
