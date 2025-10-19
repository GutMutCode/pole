use pole_compiler::parse_ir;
use std::fs;

fn main() {
    let ir = fs::read_to_string("../examples/14-option-type.pole-ir")
        .expect("Failed to read file");

    match parse_ir(&ir) {
        Ok(program) => {
            println!("✓ Parsing successful");
            println!("\nFunctions: {}", program.func_defs.len());
            for func in &program.func_defs {
                println!("  - {}", func.name);
                println!("    Return type: {:?}", func.return_type);
            }
        }
        Err(e) => {
            eprintln!("✗ Parsing failed: {}", e);
            std::process::exit(1);
        }
    }
}
