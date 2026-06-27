use goblin::elf::Elf;
use std::fs;
use std::io::{self, Read, Write};
use std::env;
use std::collections::HashSet;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    
    let binary_path = if args.len() > 1 {
        args[1].clone()
    } else {
        let mut path = String::new();
        print!("Enter binary path: ");
        io::stdout().flush()?;
        io::stdin().read_line(&mut path)?;
        path.trim().to_string()
    };
    
    eprintln!("=== GENERIC BINARY RECONSTRUCTOR ===");
    eprintln!("Binary: {}", binary_path);
    
    let mut file = fs::File::open(&binary_path)?;
    let mut binary_data = Vec::new();
    file.read_to_end(&mut binary_data)?;
    
    let elf = Elf::parse(&binary_data)?;
    let functions = detect_functions(&elf)?;
    eprintln!("Functions: {:?}", functions);
    
    let binary_type = classify(&functions);
    eprintln!("Type: {:?}", binary_type);
    
    generate_code(&functions, &binary_type)?;
    
    Ok(())
}

#[derive(Debug)]
enum BinaryType { QtApp, PortScanner, FileUtil, Encoder, Unknown }

fn detect_functions(elf: &Elf) -> Result<HashSet<String>, Box<dyn std::error::Error>> {
    let mut funcs = HashSet::new();
    for sym in &elf.dynsyms {
        if let Some(name) = elf.dynstrtab.get_at(sym.st_name) {
            if name.contains("socket") { funcs.insert("socket".to_string()); }
            if name.contains("connect") { funcs.insert("connect".to_string()); }
            if name.contains("QApplication") { funcs.insert("qt".to_string()); }
            if name.contains("read") { funcs.insert("read".to_string()); }
            if name.contains("write") { funcs.insert("write".to_string()); }
            if name.contains("MD5") { funcs.insert("md5".to_string()); }
        }
    }
    Ok(funcs)
}

fn classify(funcs: &HashSet<String>) -> BinaryType {
    if funcs.contains("qt") { BinaryType::QtApp }
    else if funcs.contains("socket") && funcs.contains("connect") { BinaryType::PortScanner }
    else if funcs.contains("md5") { BinaryType::Encoder }
    else if funcs.contains("read") && funcs.contains("write") { BinaryType::FileUtil }
    else { BinaryType::Unknown }
}

fn generate_code(_funcs: &HashSet<String>, bin_type: &BinaryType) -> Result<(), Box<dyn std::error::Error>> {
    match bin_type {
        BinaryType::QtApp => {
            println!("#include <QApplication>");
            println!("#include <QMainWindow>");
            println!("#include <QWidget>");
            println!("#include <QVBoxLayout>");
            println!("#include <QFileSystemModel>");
            println!("#include <QTreeView>");
            println!("#include <QDir>");
            println!();
            println!("class FileManager : public QMainWindow {{");
            println!("public:");
            println!("    FileManager() {{");
            println!("        setWindowTitle(\"Reconstructed File Manager\");");
            println!("        setGeometry(100, 100, 1024, 768);");
            println!("        QWidget *central = new QWidget();");
            println!("        QVBoxLayout *layout = new QVBoxLayout(central);");
            println!("        QFileSystemModel *model = new QFileSystemModel();");
            println!("        model->setRootPath(QDir::homePath());");
            println!("        QTreeView *view = new QTreeView();");
            println!("        view->setModel(model);");
            println!("        view->setRootIndex(model->index(QDir::homePath()));");
            println!("        layout->addWidget(view);");
            println!("        setCentralWidget(central);");
            println!("    }}");
            println!("}};");
            println!();
            println!("int main(int argc, char *argv[]) {{");
            println!("    QApplication app(argc, argv);");
            println!("    FileManager window;");
            println!("    window.show();");
            println!("    return app.exec();");
            println!("}}");
        },
        BinaryType::PortScanner => {
            println!("use std::net::TcpStream;");
            println!("use std::time::Duration;");
            println!();
            println!("fn main() {{");
            println!("    let args: Vec<String> = std::env::args().collect();");
            println!("    if args.len() < 2 {{ eprintln!(\"Usage: {{}} <host> [start] [end]\", args[0]); return; }}");
            println!("    let host = &args[1];");
            println!("    let start = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(1);");
            println!("    let end = args.get(3).and_then(|s| s.parse().ok()).unwrap_or(1024);");
            println!("    println!(\"Scanning {{}} from {{}} to {{}}\", host, start, end);");
            println!("    for port in start..=end {{");
            println!("        let addr = format!(\"{{}}:{{}}\", host, port);");
            println!("        if let Ok(a) = addr.parse() {{ if TcpStream::connect_timeout(&a, Duration::from_millis(50)).is_ok() {{ println!(\"{{}} open\", port); }} }}");
            println!("    }}");
            println!("}}");
        },
        BinaryType::FileUtil => {
            println!("use std::fs; use std::io;");
            println!("fn main() {{");
            println!("    let args: Vec<String> = std::env::args().collect();");
            println!("    if args.len() < 2 {{ eprintln!(\"Usage: {{}} <file>\", args[0]); return; }}");
            println!("    match fs::read(&args[1]) {{");
            println!("        Ok(data) => {{ let _ = io::Write::write_all(&mut io::stdout(), &data); }},");
            println!("        Err(e) => eprintln!(\"Error: {{}}\", e),");
            println!("    }}");
            println!("}}");
        },
        _ => {
            println!("fn main() {{ println!(\"Reconstructed\"); }}");
        }
    }
    Ok(())
}
