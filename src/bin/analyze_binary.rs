use std::env;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: {} <binary_path>", args[0]);
        return Ok(());
    }
    
    let binary_path = &args[1];
    let binary_data = fs::read(binary_path)?;
    
    println!("=== BINARY ANALYSIS FRAMEWORK ===\n");
    println!("Target: {}", binary_path);
    println!("Size: {} bytes", binary_data.len());
    println!();
    
    println!("Analysis modules available:");
    println!("  1. Static CFG reconstruction (show_cfg_details)");
    println!("  2. Loop detection (detect_loops)");
    println!("  3. Type inference (type_inference)");
    println!("  4. Register dataflow (register_dataflow)");
    println!("  5. Call graph (call_graph)");
    println!("  6. Expression reconstruction (expression_reconstruction)");
    println!("  7. Liveness analysis (liveness_analysis)");
    println!("  8. Final report (final_report)");
    println!();
    
    println!("Workflow:");
    println!("  1. sudo ./ebpf/uprobe_reader ./ebpf/syscall_tracer.o {} 30", binary_path);
    println!("  2. ./target/release/show_cfg_details {}", binary_path);
    println!("  3. ./target/release/final_report");
    
    Ok(())
}
