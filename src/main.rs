use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

// Use algo:: (matches the package name in Cargo.toml)
use algo::ast::Program;
use algo::codegen::generate_c_code;
use algo::flowchart::generate_mermaid;
use algo::lexer::tokenize;
use algo::parser::parse;

fn print_usage() {
    println!("Usage:");
    println!("  algo run <file.algo>                     - Runs GUI viewer for the flowchart");
    println!("  algo compile <file.algo> -o <out.flw>    - Compiles .algo to .flw GUI format");
    println!("  algo build <file.algo> -o <out.exe>      - Compiles .algo to native .exe binary");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        print_usage();
        return;
    }

    let command = &args[1];
    let input_path = &args[2];

    // Read source file
    let source_code = match fs::read_to_string(input_path) {
        Ok(code) => code,
        Err(err) => {
            eprintln!("Error reading file '{}': {}", input_path, err);
            return;
        }
    };

    // Run Lexer and Parser
    let tokens = tokenize(&source_code);
    let program = match parse(&tokens) {
        Ok(p) => p,
        Err(err) => {
            eprintln!("\n[COMPILER ERROR]\n{}", err);
            return;
        }
    };

    // Determine output path if provided via -o
    let mut output_path = String::new();
    if let Some(o_index) = args.iter().position(|r| r == "-o") {
        if o_index + 1 < args.len() {
            output_path = args[o_index + 1].clone();
        }
    }

    match command.as_str() {
        "compile" | "run" => {
            if output_path.is_empty() {
                output_path = Path::new(input_path)
                    .with_extension("flw")
                    .to_string_lossy()
                    .into_owned();
            }

            compile_to_flw(&program, &output_path);
        }
        "build" => {
            if output_path.is_empty() {
                output_path = Path::new(input_path)
                    .with_extension("exe")
                    .to_string_lossy()
                    .into_owned();
            }

            compile_to_exe(&program, &output_path);
        }
        _ => print_usage(),
    }
}

// Generates the GUI .flw document
fn compile_to_flw(program: &Program, output_path: &str) {
    let mermaid_code = generate_mermaid(program);

    let flw_content = format!(
        r#"<!-- ALGO NATIVE FLOWCHART GUI -->
<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <title>Algo Flowchart Viewer</title>
    <style>
        body {{ background-color: #0d1117; color: #c9d1d9; font-family: sans-serif; display: flex; justify-content: center; padding: 20px; }}
        .mermaid {{ background: #161b22; padding: 20px; border-radius: 10px; box-shadow: 0 4px 12px rgba(0,0,0,0.5); }}
    </style>
</head>
<body>
    <pre class="mermaid">
{}
    </pre>
    <script type="module">
        import mermaid from 'https://cdn.jsdelivr.net/npm/mermaid@10/dist/mermaid.esm.min.mjs';
        mermaid.initialize({{ startOnLoad: true, theme: 'dark' }});
    </script>
</body>
</html>"#,
        mermaid_code
    );

    match fs::write(output_path, flw_content) {
        Ok(_) => println!("[SUCCESS] Flowchart generated: {}", output_path),
        Err(e) => eprintln!("Error writing output file: {}", e),
    }
}

// Generates native .exe using GCC/Clang transpile backend
fn compile_to_exe(program: &Program, output_path: &str) {
    let c_code = generate_c_code(program);
    let temp_c = "temp_output.c";

    if let Err(e) = fs::write(temp_c, c_code) {
        eprintln!("Failed to create temporary C source: {}", e);
        return;
    }

    let status = Command::new("gcc")
        .arg(temp_c)
        .arg("-o")
        .arg(output_path)
        .status();

    let _ = fs::remove_file(temp_c);

    match status {
        Ok(s) if s.success() => println!("[SUCCESS] Executable built: {}", output_path),
        _ => eprintln!("[ERROR] C compilation failed. Ensure GCC is installed and in PATH."),
    }
}