use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Clone, Debug)]
struct Block {
    id: usize,
    syscalls: Vec<u64>,
    next: Option<usize>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("syscall_events.json")?;
    let reader = BufReader::new(file);
    
    let mut syscall_sequence = Vec::new();
    let mut blocks = Vec::new();
    let mut current_block_id = 0;
    let mut line_count = 0;
    
    for line in reader.lines() {
        let line = line?;
        if line.starts_with('{') {
            line_count += 1;
            
            if let Some(start) = line.find("\"syscall\":") {
                let rest = &line[start + 10..];
                if let Some(end) = rest.find('}') {
                    if let Ok(syscall) = rest[..end].trim_end_matches(',').parse::<u64>() {
                        syscall_sequence.push(syscall);
                        
                        // Create block every 100 syscalls or on branch prediction
                        if syscall_sequence.len() >= 100 {
                            blocks.push(Block {
                                id: current_block_id,
                                syscalls: syscall_sequence.drain(..).collect(),
                                next: Some(current_block_id + 1),
                            });
                            current_block_id += 1;
                        }
                    }
                }
            }
            
            if line_count % 5000000 == 0 {
                println!("Processed {} events, {} blocks...", line_count, current_block_id);
            }
        }
    }
    
    // Flush remaining syscalls
    if !syscall_sequence.is_empty() {
        blocks.push(Block {
            id: current_block_id,
            syscalls: syscall_sequence,
            next: None,
        });
    }
    
    println!("\nCFG Stats:");
    println!("Total blocks: {}", blocks.len());
    println!("Total events: {}", line_count);
    
    // Count block types
    let mut block_types: HashMap<Vec<u64>, usize> = HashMap::new();
    for block in &blocks {
        if block.syscalls.len() <= 10 {
            *block_types.entry(block.syscalls.clone()).or_insert(0) += 1;
        }
    }
    
    println!("Unique short blocks (<=10 syscalls): {}", block_types.len());
    println!("\nMost common blocks:");
    
    let mut sorted: Vec<_> = block_types.iter().collect();
    sorted.sort_by(|a, b| b.1.cmp(a.1));
    
    for (syscalls, count) in sorted.iter().take(10) {
        println!("  {:?}: {} occurrences", syscalls, count);
    }
    
    Ok(())
}
