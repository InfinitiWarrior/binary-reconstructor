use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Annotating CFG blocks with syscall semantics...");
    
    let file = File::open("syscall_events.json")?;
    let reader = BufReader::new(file);
    
    let block_addresses = vec![
        0x0u64, 0x2, 0x1014, 0x1016, 0x101b, 0x102c, 0x1036, 0x1040, 
        0x1065, 0x1070, 0x1083, 0x1091, 0x1098, 0x10a0, 0x10c4,
    ];
    
    let mut block_patterns: HashMap<u64, HashMap<String, usize>> = HashMap::new();
    for addr in &block_addresses {
        block_patterns.insert(*addr, HashMap::new());
    }
    
    let mut syscall_sequence = Vec::new();
    let mut line_count = 0;
    let mut block_index = 0;
    
    for line in reader.lines() {
        let line = line?;
        if !line.starts_with('{') {
            continue;
        }
        
        line_count += 1;
        
        let sc = if let Some(sc_start) = line.find("\"syscall\":") {
            let sc_str = &line[sc_start + 10..];
            if let Some(sc_end) = sc_str.find('}') {
                sc_str[..sc_end].parse::<u64>().unwrap_or(0)
            } else {
                0
            }
        } else {
            0
        };
        
        syscall_sequence.push(sc);
        
        if syscall_sequence.len() >= 5 {
            let pattern = syscall_sequence[syscall_sequence.len() - 5..]
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<_>>()
                .join(",");
            
            let addr = block_addresses[block_index % block_addresses.len()];
            block_patterns.entry(addr)
                .or_insert_with(HashMap::new)
                .entry(pattern)
                .and_modify(|c| *c += 1)
                .or_insert(1);
        }
        
        if line_count % 10000000 == 0 {
            block_index += 1;
            println!("Processed {} events, block index {}", line_count, block_index);
        }
    }
    
    println!("\nBlock Semantic Annotations:");
    
    for addr in &block_addresses {
        if let Some(patterns) = block_patterns.get(addr) {
            if patterns.is_empty() {
                continue;
            }
            
            let mut sorted: Vec<_> = patterns.iter().collect();
            sorted.sort_by(|a, b| b.1.cmp(a.1));
            
            if let Some((pattern, count)) = sorted.first() {
                let write_count = pattern.matches("1").count();
                let has_sync = pattern.contains("202");
                let has_read = pattern.contains("0");
                
                let semantic = if write_count >= 4 && !has_sync {
                    "PRINTF/OUTPUT"
                } else if has_sync {
                    "SYNCHRONIZATION"
                } else if has_read {
                    "I/O_READ"
                } else {
                    "COMPUTE"
                };
                
                println!("\nBlock 0x{:x}: {}", addr, semantic);
                println!("  Dominant: {} ({}x)", pattern, count);
                
                for (p, c) in sorted.iter().skip(1).take(2) {
                    println!("  Secondary: {} ({}x)", p, c);
                }
            }
        }
    }
    
    Ok(())
}
