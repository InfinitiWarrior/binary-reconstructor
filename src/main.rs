use nix::unistd::{fork, execv, ForkResult};
use nix::sys::ptrace;
use nix::sys::wait::{waitpid, WaitStatus};
use nix::sys::signal::Signal;
use std::ffi::CString;
use capstone::prelude::*;
use std::fs::File;
use std::io::{BufWriter, Write};

mod phase2;
mod phase3;
mod phase4;
mod phase5;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <binary> [args...]", args[0]);
        std::process::exit(1);
    }
    
    let target = args[1].clone();
    let bin_args: Vec<String> = args[2..].to_vec();
    
    println!("Target: {}", target);
    println!("Streaming JSON output (constant memory)");
    
    std::fs::create_dir_all("output").expect("Failed to create output directory");
    
    match unsafe { fork() }.expect("fork failed") {
        ForkResult::Child => {
            ptrace::traceme().expect("traceme failed");
            let target_c = CString::new(target.clone()).unwrap();
            let args_c: Vec<CString> = bin_args.iter()
                .map(|s| CString::new(s.as_str()).unwrap())
                .collect();
            let _ = execv(&target_c, &args_c);
            std::process::exit(1);
        }
        ForkResult::Parent { child } => {
            println!("Tracing PID {}", child);
            
            let cs = Capstone::new()
                .x86()
                .mode(arch::x86::ArchMode::Mode64)
                .detail(true)
                .build()
                .expect("Failed to create Capstone");
            
            let file = File::create("output/trace.json").expect("Failed to create trace.json");
            let mut writer = BufWriter::new(file);
            
            writer.write_all(b"[\n").expect("Failed to write opening bracket");
            
            let mut instr_count = 0;
            let mut first_instr = true;
            
            loop {
                match waitpid(child, None).expect("waitpid failed") {
                    WaitStatus::Exited(_, code) => {
                        println!("Child exited with code: {}", code);
                        writer.write_all(b"\n]\n").expect("Failed to write closing bracket");
                        writer.flush().expect("Failed to flush");
                        println!("Traced {} instructions total", instr_count);
                        break;
                    }
                    WaitStatus::Stopped(_, _signal) => {
                        if let Ok(regs) = ptrace::getregs(child) {
                            let rip = regs.rip;
                            instr_count += 1;
                            
                            if instr_count % 100000 == 0 {
                                println!("Progress: {} instructions", instr_count);
                            }
                            
                            let mut code = [0u8; 15];
                            for i in 0..15usize {
                                match ptrace::read(child, (rip + i as u64) as *mut libc::c_void) {
                                    Ok(val) => code[i] = (val & 0xFF) as u8,
                                    Err(_) => break,
                                }
                            }
                            
                            if let Ok(instrs) = cs.disasm_all(&code, rip) {
                                if let Some(instr) = instrs.first() {
                                    let mnemonic = instr.mnemonic().unwrap_or("?").to_string();
                                    let op_str = instr.op_str().unwrap_or("").to_string();
                                    
                                    if !first_instr {
                                        writer.write_all(b",\n").expect("Failed to write comma");
                                    }
                                    first_instr = false;
                                    
                                    let json = format!(
                                        "  {{\n    \"address\": {},\n    \"mnemonic\": \"{}\",\n    \"op_str\": \"{}\",\n    \"rax\": {},\n    \"rbx\": {},\n    \"rcx\": {},\n    \"rdx\": {}\n  }}",
                                        instr.address(),
                                        mnemonic.replace("\"", "\\\""),
                                        op_str.replace("\"", "\\\""),
                                        regs.rax,
                                        regs.rbx,
                                        regs.rcx,
                                        regs.rdx
                                    );
                                    writer.write_all(json.as_bytes()).expect("Failed to write instruction");
                                }
                            }
                        }
                        
                        ptrace::step(child, None).expect("step failed");
                    }
                    _ => {
                        ptrace::step(child, None).expect("step failed");
                    }
                }
            }
            
            println!("Saved trace to output/trace.json");
            
            match phase2::analyze_trace("output/trace.json") {
                Ok(blocks) => {
                    println!("Found {} basic blocks", blocks.len());
                    let cfg_json = serde_json::to_string_pretty(&blocks)
                        .expect("Failed to serialize CFG");
                    let mut file = File::create("output/cfg.json").expect("Failed to create CFG file");
                    file.write_all(cfg_json.as_bytes()).expect("Failed to write CFG");
                    println!("Saved CFG to output/cfg.json");
                }
                Err(e) => println!("Error analyzing trace: {}", e),
            }
            
            match phase3::generate_rust_file("output/cfg.json", "output/reconstructed.rs") {
                Ok(_) => {
                    println!("Generated Rust code to output/reconstructed.rs");
                    if let Ok(content) = std::fs::read_to_string("output/reconstructed.rs") {
                        println!("\n--- First 60 lines ---");
                        for line in content.lines().take(60) {
                            println!("{}", line);
                        }
                    }
                }
                Err(e) => println!("Error generating Rust file: {}", e),
            }

            match phase4::cleanup_rust_code("output/reconstructed.rs", "output/reconstructed_clean.rs") {
                Ok(_) => {
                    println!("Generated clean Rust to output/reconstructed_clean.rs");
                    if let Ok(content) = std::fs::read_to_string("output/reconstructed_clean.rs") {
                        println!("\n--- First 50 lines ---");
                        for line in content.lines().take(50) {
                            println!("{}", line);
                        }
                    }
                }
                Err(e) => println!("Error cleaning Rust: {}", e),
            }

            match phase5::extract_functions("output/cfg.json") {
                Ok(functions) => {
                    println!("\nPhase 5: Function Extraction");
                    println!("Found {} functions:", functions.len());
                    for func in &functions {
                        println!("  {} @ 0x{:x}", func.name, func.address);
                    }
                    
                    if let Ok(clean_content) = std::fs::read_to_string("output/reconstructed_clean.rs") {
                        let mut output = String::from("// Dynamic binary reconstructor - streaming trace\n\n");
                        output.push_str(&clean_content);
                        
                        let mut file = std::fs::File::create("output/reconstructed_multi.rs")
                            .expect("Failed to create multi-function file");
                        file.write_all(output.as_bytes()).expect("Failed to write");
                        println!("Wrote output/reconstructed_multi.rs");
                    }
                }
                Err(e) => println!("Error in phase 5: {}", e),
            }
        }
    }
}