use std::fs;
use std::collections::HashSet;
use std::io::Write;

#[derive(Clone)]
pub struct Function {
    pub name: String,
    pub address: u64,
    pub code_lines: Vec<String>,
}

pub fn generate_multifunc_rust(
    clean_file: &str,
    functions: Vec<(String, u64)>,
    output_file: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let content = fs::read_to_string(clean_file)?;
    
    // Collect all variables
    let mut all_vars = HashSet::new();
    for line in content.lines() {
        for word in line.split_whitespace() {
            if word.starts_with("var_") {
                let var = word.trim_end_matches(';').trim_end_matches(',').to_string();
                all_vars.insert(var);
            }
        }
    }
    
    // Generate output
    let mut output = String::new();
    
    // Shared variable declarations at top
    output.push_str("// Shared state\n");
    let mut sorted_vars: Vec<_> = all_vars.iter().collect();
    sorted_vars.sort();
    for var in &sorted_vars {
        output.push_str(&format!("thread_local! {{\n    static {}: std::cell::RefCell<i64> = std::cell::RefCell::new(0);\n}}\n", var));
    }
    
    output.push_str("\nfn func_main() {\n");
    output.push_str("    // Entry point\n");
    output.push_str("}\n\n");
    
    output.push_str("fn main() {\n");
    output.push_str("    func_main();\n");
    output.push_str("}\n");
    
    let mut file = fs::File::create(output_file)?;
    file.write_all(output.as_bytes())?;
    
    Ok(())
}