use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: {} <syscall_trace.json>", args[0]);
        return Ok(());
    }
    
    let trace_file = &args[1];
    let file = File::open(trace_file)?;
    let reader = BufReader::new(file);
    
    let mut syscall_types: HashMap<u64, usize> = HashMap::new();
    let mut total = 0;
    let mut timestamps = Vec::new();
    let mut pid_set = std::collections::HashSet::new();
    
    for line in reader.lines() {
        let line = line?;
        if !line.starts_with('{') {
            continue;
        }
        
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
        
        if let Some(pid_start) = line.find("\"pid\":") {
            let pid_str = &line[pid_start + 6..];
            if let Some(pid_end) = pid_str.find(',') {
                if let Ok(pid) = pid_str[..pid_end].parse::<u64>() {
                    pid_set.insert(pid);
                }
            }
        }
        
        if total % 10000000 == 0 {
            println!("  Processed {} events...", total);
        }
    }
    
    println!("=== FULL BINARY ANALYSIS REPORT ===\n");
    println!("Trace: {}", trace_file);
    println!();
    
    println!("EXECUTION SUMMARY");
    println!("=================");
    println!("Total syscall events: {}", total);
    println!("Unique processes: {}", pid_set.len());
    
    if !timestamps.is_empty() {
        let duration_ns = timestamps[timestamps.len() - 1] - timestamps[0];
        let duration_ms = duration_ns / 1_000_000;
        let duration_s = duration_ms / 1000;
        println!("Execution time: {}.{:03}s", duration_s, duration_ms % 1000);
    }
    println!();
    
    println!("SYSCALL DISTRIBUTION");
    println!("====================");
    let mut sorted: Vec<_> = syscall_types.iter().collect();
    sorted.sort_by(|a, b| b.1.cmp(a.1));
    
    for (sc, count) in sorted.iter().take(10) {
        let pct = (**count as f64 / total as f64) * 100.0;
        println!("syscall {}: {} calls ({:.1}%)", sc, count, pct);
    }
    println!();
    
    println!("INFERRED BEHAVIOR");
    println!("=================");
    
    let write_count = syscall_types.get(&1).unwrap_or(&0);
    let read_count = syscall_types.get(&0).unwrap_or(&0);
    let open_count = syscall_types.get(&2).unwrap_or(&0);
    let mmap_count = syscall_types.get(&9).unwrap_or(&0);
    
    if *write_count > 0 {
        println!("Output operations: {} write() calls", write_count);
    }
    if *read_count > 0 {
        println!("Input operations: {} read() calls", read_count);
    }
    if *open_count > 0 {
        println!("File operations: {} open() calls", open_count);
    }
    if *mmap_count > 0 {
        println!("Memory operations: {} mmap() calls", mmap_count);
    }
    println!();
    
    println!("NEXT STEPS");
    println!("==========");
    println!("1. Run: ./target/release/show_cfg_details <binary>");
    println!("2. Run: ./target/release/detect_loops");
    println!("3. Run: ./target/release/type_inference");
    println!("4. Run: ./target/release/final_report");
    
    Ok(())
}
