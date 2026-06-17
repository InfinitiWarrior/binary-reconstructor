use std::fs;
use std::collections::HashSet;
use std::io::Write;

pub fn cleanup_rust_code(input_file: &str, output_file: &str) -> Result<(), Box<dyn std::error::Error>> {
    let content = fs::read_to_string(input_file)?;
    let lines: Vec<&str> = content.lines().collect();
    
    let mut output = Vec::new();
    let mut in_function = false;
    let mut all_vars = HashSet::new();
    
    // First pass: find all variables used in code
    for line in &lines {
        let trimmed = line.trim();
        
        if trimmed.starts_with("fn func_main()") {
            in_function = true;
            continue;
        }
        
        if in_function && !trimmed.starts_with("fn ") && trimmed != "}" {
            // Extract var_X from code
            for word in trimmed.split_whitespace() {
                if word.starts_with("var_") {
                    let var = word.trim_end_matches(';').trim_end_matches(',').to_string();
                    all_vars.insert(var);
                }
            }
        }
    }
    
    // Second pass: generate cleaned code
    in_function = false;
    let mut first_code_line = true;
    
    for (_idx, line) in lines.iter().enumerate() {
        let trimmed = line.trim();
        
        if trimmed.starts_with("fn func_main()") {
            in_function = true;
            output.push("fn func_main() {".to_string());
            continue;
        }
        
        if !in_function {
            output.push(line.to_string());
            continue;
        }
        
        // Skip ALL declarations from original
        if trimmed.starts_with("let mut") || trimmed.contains("::") {
            continue;
        }
        
        // Skip main function
        if trimmed.starts_with("fn main") || trimmed == "fn main() {" {
            continue;
        }
        
        // Skip memory operations
        if trimmed.contains("dword ptr") || trimmed.contains("qword ptr") {
            continue;
        }
        
        // Skip comments
        if trimmed.starts_with("//") {
            continue;
        }
        
        // Add clean declarations before first real code line
        if first_code_line && !trimmed.is_empty() && trimmed != "}" && !trimmed.starts_with("//") {
            let mut sorted_vars: Vec<_> = all_vars.iter().collect();
            sorted_vars.sort();
            for var in sorted_vars {
                output.push(format!("    let mut {}: i64 = 0;", var));
            }
            output.push("".to_string());
            first_code_line = false;
        }
        
        // Clean and add code lines
        if !trimmed.is_empty() && !trimmed.starts_with("fn ") && trimmed != "}" {
            let cleaned = clean_line(trimmed);
            if !cleaned.is_empty() {
                output.push(format!("    {}", cleaned));
            }
        }
        
        if trimmed == "}" && in_function {
            output.push("}".to_string());
            in_function = false;
        }
    }
    
    let final_code = output.join("\n");
    let mut file = std::fs::File::create(output_file)?;
    file.write_all(final_code.as_bytes())?;
    
    Ok(())
}

fn extract_var_name(line: &str) -> Option<String> {
    if line.contains("let mut ") {
        let start = line.find("let mut ")? + 8;
        let rest = &line[start..];
        if let Some(colon_pos) = rest.find(':') {
            let var = rest[..colon_pos].trim().to_string();
            if var.starts_with("var_") {
                return Some(var);
            }
        }
    }
    None
}

fn clean_line(line: &str) -> String {
    let mut result = line.to_string();
    
    // Remove memory syntax
    result = result.replace("dword ptr [", "[");
    result = result.replace("qword ptr [", "[");
    
    // Convert ALL register names to variables (both LHS and RHS)
    let registers = vec![
        "rbp", "rsp", "r12d", "r9d", "r8d", "r10d", "r11d", "r13d", "r14d", "r15d",
        "rdi", "rsi", "rax", "rbx", "rcx", "rdx",
    ];
    
    for reg in registers {
        // RHS (after spaces, commas, operators)
        result = result.replace(&format!(" {}", reg), &format!(" var_{}", reg));
        result = result.replace(&format!("={}", reg), &format!("=var_{}", reg));
        result = result.replace(&format!("({}", reg), &format!("(var_{}", reg));
        // LHS
        result = result.replace(&format!("{} =", reg), &format!("var_{} =", reg));
        result = result.replace(&format!("{} +=", reg), &format!("var_{} +=", reg));
        result = result.replace(&format!("{} -=", reg), &format!("var_{} -=", reg));
        result = result.replace(&format!("{} ^=", reg), &format!("var_{} ^=", reg));
    }
    
    // Keep valid Rust
    if result.contains('=') || result.contains("+=") || result.contains("-=") || 
       result.contains("^=") || result.contains("if ") {
        return result;
    }
    
    String::new()
}