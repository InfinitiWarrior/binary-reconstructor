use nix::unistd::{fork, execv, ForkResult};
use nix::sys::ptrace;
use nix::sys::wait::{waitpid, WaitStatus};
use std::ffi::CString;
use capstone::prelude::*;

fn main() {
    let target = "/bin/echo";
    let args = vec!["hello"];
    
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
            
            let mut instr_count = 0;
            let mut first_stop = true;
            
            loop {
                match waitpid(child, None).expect("waitpid failed") {
                    WaitStatus::Exited(_, code) => {
                        println!("\nChild exited with code: {}", code);
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
                            
                            // Read 15 bytes from RIP for disassembly (x86-64 max instr len)
                            let mut code = [0u8; 15];
                            for i in 0..15usize {
                                match ptrace::read(child, (rip + i as u64) as *mut libc::c_void) {
                                    Ok(val) => code[i] = (val & 0xFF) as u8,
                                    Err(_) => break,
                                }
                            }
                            
                            // Disassemble
                            if let Ok(instrs) = cs.disasm_all(&code, rip) {
                                if let Some(instr) = instrs.first() {
                                    println!("[{}] 0x{:x}: {} {}", 
                                        instr_count, 
                                        instr.address(), 
                                        instr.mnemonic().unwrap_or("?"),
                                        instr.op_str().unwrap_or(""));
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
        }
    }
}