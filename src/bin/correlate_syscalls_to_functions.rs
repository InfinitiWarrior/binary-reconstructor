use serde_json::{json, Value};
use std::collections::HashMap;
use std::fs;
use std::io::Read;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config_path = "/home/inf/.analysis/config";
    let config = read_config(config_path)?;
    let binary_path = config.get("BINARY_PATH").unwrap_or(&"/bin/md5sum".to_string()).clone();
    
    let syscall_file = format!("/home/inf/.analysis/{}_syscalls.json", 
        binary_path.split('/').last().unwrap_or("binary"));
    
    if !std::path::Path::new(&syscall_file).exists() {
        eprintln!("Syscall file not found: {}", syscall_file);
        return Ok(());
    }
    
    let mut file = fs::File::open(&syscall_file)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    
    let syscalls: Vec<Value> = serde_json::from_str(&content).unwrap_or_default();
    
    let mut func_syscalls: HashMap<String, HashMap<String, usize>> = HashMap::new();
    
    for syscall in syscalls.iter() {
        if let (Some(addr), Some(name)) = (syscall["addr"].as_str(), syscall["name"].as_str()) {
            func_syscalls
                .entry(addr.to_string())
                .or_insert_with(HashMap::new)
                .entry(name.to_string())
                .and_modify(|c| *c += 1)
                .or_insert(1);
        }
    }
    
    let mut annotations = Vec::new();
    for (addr, syscall_map) in func_syscalls.iter() {
        let mut purpose = "utility".to_string();
        let total: usize = syscall_map.values().sum();
        
        if let Some(&write_count) = syscall_map.get("write") {
            if write_count > total / 2 {
                purpose = "output/formatting".to_string();
            }
        }
        
        if let Some(&read_count) = syscall_map.get("read") {
            if read_count > total / 2 {
                purpose = "input/file reading".to_string();
            }
        }
        
        if let Some(&futex_count) = syscall_map.get("futex") {
            if futex_count > total / 3 {
                purpose = "synchronization/locking".to_string();
            }
        }
        
        if syscall_map.contains_key("open") || syscall_map.contains_key("openat") {
            purpose = format!("file I/O ({})", total);
        }
        
        annotations.push(json!({
            "addr": addr,
            "purpose": purpose,
            "syscalls": syscall_map
        }));
    }
    
    fs::write("/home/inf/.analysis/function_purposes.json", serde_json::to_string_pretty(&annotations)?)?;
    
    println!("Analyzed {} functions from syscall trace", annotations.len());
    for ann in annotations.iter().take(10) {
        println!("  0x{}: {}", ann["addr"], ann["purpose"]);
    }
    
    Ok(())
}

fn read_config(path: &str) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
    let mut config = HashMap::new();
    if let Ok(content) = fs::read_to_string(path) {
        for line in content.lines() {
            if let Some((k, v)) = line.split_once('=') {
                config.insert(k.to_string(), v.to_string());
            }
        }
    }
    Ok(config)
}
