use std::fs;
use std::process::Command;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== COMPREHENSIVE BINARY ANALYSIS ===\n");
    
    let tools = vec![
        ("Static CFG", "show_cfg_details"),
        ("Loop Detection", "detect_loops"),
        ("Type Inference", "type_inference"),
        ("Register Data Flow", "register_dataflow"),
        ("Call Graph", "call_graph"),
        ("Final Report", "final_report"),
    ];
    
    for (name, binary) in tools.iter() {
        println!("--- {} ---", name);
        let _ = Command::new(format!("./target/release/{}", binary))
            .output();
        println!();
    }
    
    Ok(())
}
