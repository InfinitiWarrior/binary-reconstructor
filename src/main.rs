use nix::unistd::{fork, execv, ForkResult};
use nix::sys::ptrace;
use nix::sys::wait::{waitpid, WaitStatus};
use std::ffi::CString;
use capstone::prelude::*;
use std::fs::File;
use std::io::Write;

mod phase2;
mod phase3;
mod phase4;

#[derive(serde::Serialize)]
struct Instruction {
    address: u64,
    mnemonic: String,
    op_str: String,
    rax: u64,
    rbx: u64,
    rcx: u64,
    rdx: u64,
}

fn main() {
    let target = "/bin/echo";
    let args = vec!["hello"];
    
    std::fs::create_dir_all("output").expect("Failed to create output directory");
    
    match unsafe { fork() }.expect("fork failed") {
        ForkResult::Child => {
            ptrace::traceme().expect("traceme failed");
            let target_c = CString::new(target).unwrap();
            let args_c: Vec<CString> = args.iter()
                .map(|s| CString::new(*s).unwrap())
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
            
            let mut instructions = Vec::new();
            let mut instr_count = 0;
            let mut first_stop = true;
            
            loop {
                match waitpid(child, None).expect("waitpid failed") {
                    WaitStatus::Exited(_, code) => {
                        println!("Child exited with code: {}", code);
                        println!("Traced {} instructions", instr_count);
                        break;
                    }
                    WaitStatus::Stopped(_, _signal) => {
                        if first_stop {
                            first_stop = false;
                            ptrace::syscall(child, None).expect("syscall failed");
                            continue;
                        }
                        
                        if let Ok(regs) = ptrace::getregs(child) {
                            let rip = regs.rip;
                            instr_count += 1;
                            
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
                                    
                                    instructions.push(Instruction {
                                        address: instr.address(),
                                        mnemonic,
                                        op_str,
                                        rax: regs.rax,
                                        rbx: regs.rbx,
                                        rcx: regs.rcx,
                                        rdx: regs.rdx,
                                    });
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
            
            let json = serde_json::to_string_pretty(&instructions)
                .expect("Failed to serialize");
            let mut file = File::create("output/trace.json").expect("Failed to create file");
            file.write_all(json.as_bytes()).expect("Failed to write file");
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
                        for (i, line) in content.lines().enumerate().take(60) {
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
        }
    }
}