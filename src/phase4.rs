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
    
    for line in lines.iter() {
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
        
        // Skip original declarations
        if trimmed.starts_with("let mut") || trimmed.contains("::") {
            continue;
        }
        
        // Skip main function
        if trimmed.starts_with("fn main") || trimmed == "fn main() {" {
            continue;
        }
        
        // Skip pure comments
        if trimmed.starts_with("//") {
            continue;
        }
        
        // Skip closing brace of original function
        if trimmed == "}" && in_function {
            continue;
        }
        
        // Add declarations before first code line
        if first_code_line && !trimmed.is_empty() && !trimmed.starts_with("fn ") {
            let mut sorted_vars: Vec<_> = all_vars.iter().collect();
            sorted_vars.sort();
            for var in sorted_vars {
                output.push(format!("    let mut {}: i64 = 0;", var));
            }
            output.push("".to_string());
            first_code_line = false;
        }
        
        // Clean and add code lines
        if !trimmed.is_empty() && !trimmed.starts_with("fn ") {
            let cleaned = clean_line(trimmed);
            if !cleaned.is_empty() {
                output.push(format!("    {}", cleaned));
            }
        }
    }
    
    output.push("}".to_string());
    output.push("".to_string());
    output.push("fn main() {".to_string());
    output.push("    func_main();".to_string());
    output.push("}".to_string());
    
    let final_code = output.join("\n");
    let mut file = std::fs::File::create(output_file)?;
    file.write_all(final_code.as_bytes())?;
    
    Ok(())
}

fn clean_line(line: &str) -> String {
    let mut result = line.to_string();
    
    // Strip all memory syntax prefixes
    result = result.replace("qword ptr ", "");
    result = result.replace("dword ptr ", "");
    result = result.replace("word ptr ", "");
    result = result.replace("byte ptr ", "");
    
    // Convert memory address patterns to comments
    if result.contains('[') && result.contains(']') {
        return format!("// {}", result);
    }
    
    // Register mappings: all 8-bit, 16-bit, 32-bit, 64-bit variants
    let registers = vec![
        ("sil", "var_si"), ("dil", "var_di"), ("al", "var_al"), ("bl", "var_bl"),
        ("cl", "var_cl"), ("dl", "var_dl"), ("r8b", "var_r8"), ("r9b", "var_r9"),
        ("r10b", "var_r10"), ("r11b", "var_r11"), ("r12b", "var_r12"), ("r13b", "var_r13"),
        ("r14b", "var_r14"), ("r15b", "var_r15"),
        ("si", "var_si"), ("di", "var_di"), ("ax", "var_ax"), ("bx", "var_bx"),
        ("cx", "var_cx"), ("dx", "var_dx"), ("r8w", "var_r8"), ("r9w", "var_r9"),
        ("r10w", "var_r10"), ("r11w", "var_r11"), ("r12w", "var_r12"), ("r13w", "var_r13"),
        ("r14w", "var_r14"), ("r15w", "var_r15"),
        ("esi", "var_esi"), ("edi", "var_edi"), ("eax", "var_eax"), ("ebx", "var_ebx"),
        ("ecx", "var_ecx"), ("edx", "var_edx"), ("r8d", "var_r8"), ("r9d", "var_r9"),
        ("r10d", "var_r10"), ("r11d", "var_r11"), ("r12d", "var_r12"), ("r13d", "var_r13"),
        ("r14d", "var_r14"), ("r15d", "var_r15"),
        ("rsi", "var_rsi"), ("rdi", "var_rdi"), ("rax", "var_rax"), ("rbx", "var_rbx"),
        ("rcx", "var_rcx"), ("rdx", "var_rdx"), ("rbp", "var_rbp"), ("rsp", "var_rsp"),
        ("r8", "var_r8"), ("r9", "var_r9"), ("r10", "var_r10"), ("r11", "var_r11"),
        ("r12", "var_r12"), ("r13", "var_r13"), ("r14", "var_r14"), ("r15", "var_r15"),
    ];
    
    for (reg, replacement) in registers {
        // Match register as whole word (boundary checks)
        result = result.replace(&format!(" {}", reg), &format!(" {}", replacement));
        result = result.replace(&format!("={}", reg), &format!("={}", replacement));
        result = result.replace(&format!("({}", reg), &format!("({}", replacement));
        result = result.replace(&format!(" {}{}", reg, ","), &format!(" {}{}", replacement, ","));
        result = result.replace(&format!("{}(", reg), &format!("{}(", replacement));
        result = result.replace(&format!("{} =", reg), &format!("{} =", replacement));
        result = result.replace(&format!("{} +=", reg), &format!("{} +=", replacement));
        result = result.replace(&format!("{} -=", reg), &format!("{} -=", replacement));
        result = result.replace(&format!("{} ^=", reg), &format!("{} ^=", replacement));
    }
    
    // Only keep valid Rust statements
    if result.contains('=') || result.contains("+=") || result.contains("-=") || 
       result.contains("^=") || result.contains("&=") || result.contains("|=") ||
       result.contains("if ") || result.contains("while ") {
        return result;
    }
    
    String::new()
}