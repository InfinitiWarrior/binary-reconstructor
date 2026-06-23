use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

#[derive(Clone, Debug)]
struct Epoch {
    start_time: u64,
    end_time: u64,
    syscalls: Vec<u64>,
    primary_syscall: u64,
    count: usize,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Building execution epochs from syscall patterns...");
    
    let file = File::open("syscall_events.json")?;
    let reader = BufReader::new(file);
    
    let mut epochs = Vec::new();
    let mut current_epoch: Option<Epoch> = None;
    let mut line_count = 0;
    let mut syscall_changes = 0;
    
    for line in reader.lines() {
        let line = line?;
        if !line.starts_with('{') {
            continue;
        }
        
        line_count += 1;
        
        let ts = if let Some(ts_start) = line.find("\"timestamp\":") {
            let ts_str = &line[ts_start + 12..];
            if let Some(ts_end) = ts_str.find(',') {
                ts_str[..ts_end].parse::<u64>().unwrap_or(0)
            } else {
                0
            }
        } else {
            0
        };
        
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
        
        match &mut current_epoch {
            None => {
                current_epoch = Some(Epoch {
                    start_time: ts,
                    end_time: ts,
                    syscalls: vec![sc],
                    primary_syscall: sc,
                    count: 1,
                });
            }
            Some(epoch) => {
                if epoch.primary_syscall == sc && ts - epoch.end_time < 1_000_000 {
                    epoch.syscalls.push(sc);
                    epoch.end_time = ts;
                    epoch.count += 1;
                } else {
                    if epoch.count >= 100 {
                        epochs.push(epoch.clone());
                    }
                    syscall_changes += 1;
                    current_epoch = Some(Epoch {
                        start_time: ts,
                        end_time: ts,
                        syscalls: vec![sc],
                        primary_syscall: sc,
                        count: 1,
                    });
                }
            }
        }
        
        if line_count % 5000000 == 0 {
            println!("Processed {} events, {} epochs so far", line_count, epochs.len());
        }
    }
    
    if let Some(epoch) = current_epoch {
        if epoch.count >= 100 {
            epochs.push(epoch);
        }
    }
    
    println!("\nExecution Flow Analysis:");
    println!("Total events: {}", line_count);
    println!("Syscall transitions: {}", syscall_changes);
    println!("Significant epochs (100+ syscalls): {}", epochs.len());
    
    println!("\nFirst 15 epochs:");
    for (i, epoch) in epochs.iter().take(15).enumerate() {
        let duration_us = (epoch.end_time - epoch.start_time) / 1000;
        println!("Epoch {}: syscall {} ({} calls, {}us)", 
                 i, epoch.primary_syscall, epoch.count, duration_us);
    }
    
    Ok(())
}
