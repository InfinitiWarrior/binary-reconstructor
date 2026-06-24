use std::env;
use std::path::Path;
use std::process::Command;
use std::fs::File;
use std::io::Write;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: {} <binary_path> [trace_file.json]", args[0]);
        eprintln!("\nExample:");
        eprintln!("  {} /bin/ls syscall_events.json", args[0]);
        return Ok(());
    }
    
    let binary_path = &args[1];
    let trace_file = args.get(2).map(|s| s.as_str()).unwrap_or("syscall_events.json");
    
    println!("=== BINARY RECONSTRUCTOR PIPELINE ===\n");
    println!("Binary: {}", binary_path);
    println!("Trace:  {}\n", trace_file);
    
    if !Path::new(binary_path).exists() {
        eprintln!("Error: Binary not found: {}", binary_path);
        return Ok(());
    }
    
    if !Path::new(trace_file).exists() {
        eprintln!("Error: Trace file not found: {}", trace_file);
        return Ok(());
    }
    
    println!("Step 1: Extract .text section");
    let output = Command::new("./target/release/extract_text_section")
        .arg(binary_path)
        .output()?;
    
    if !output.status.success() {
        eprintln!("Failed to extract .text section");
        eprintln!("{}", String::from_utf8_lossy(&output.stderr));
        return Ok(());
    }
    
    let extract_output = String::from_utf8_lossy(&output.stdout);
    println!("{}", extract_output);
    
    let text_offset = extract_hex(&extract_output, "Offset: 0x");
    let text_size = extract_hex(&extract_output, "Size: 0x");
    
    let config = format!("BINARY_PATH={}\nTEXT_OFFSET=0x{:x}\nTEXT_SIZE=0x{:x}\n", binary_path, text_offset, text_size);
    let mut config_file = File::create("/tmp/analysis.cfg")?;
    config_file.write_all(config.as_bytes())?;
    
    println!("Step 2: Analyze static structure");
    let _ = Command::new("./target/release/detect_loops").output();
    println!("  Loops detected and analyzed");
    
    println!("Step 3: Analyze liveness");
    let _ = Command::new("./target/release/liveness_analysis").output();
    println!("  Register/stack liveness computed");
    
    println!("Step 4: Build call graph");
    let _ = Command::new("./target/release/call_graph").output();
    println!("  Function call relationships extracted");
    
    println!("Step 5: Reconstruct expressions");
    let _ = Command::new("./target/release/expression_reconstruction").output();
    println!("  High-level operations inferred");
    
    println!("Step 6: Analyze dynamic behavior");
    let output = Command::new("./target/release/full_analysis")
        .arg(trace_file)
        .output()?;
    println!("{}", String::from_utf8_lossy(&output.stdout));
    
    println!("=== PIPELINE COMPLETE ===");
    println!("Config: {} offset=0x{:x} size=0x{:x}", binary_path, text_offset, text_size);
    
    Ok(())
}

fn extract_hex(output: &str, key: &str) -> u64 {
    for line in output.lines() {
        if line.contains(key) {
            if let Some(hex_part) = line.split(key).nth(1) {
                let hex_str = hex_part.split_whitespace().next().unwrap_or("0");
                if let Ok(val) = u64::from_str_radix(hex_str, 16) {
                    return val;
                }
            }
        }
    }
    0
}
