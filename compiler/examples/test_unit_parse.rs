use pole_compiler::parse_ir;
use std::fs;

fn main() {
    let ir = fs::read_to_string("../examples/17-unit-type.pole-ir")
        .expect("Failed to read file");

    match parse_ir(&ir) {
        Ok(program) => {
            println!("✓ Parsing successful\n");
            for func in &program.func_defs {
                println!("Function: {}", func.name);
                println!("  Return type: {:?}", func.return_type);
                println!("  Body: {:?}\n", func.body);
            }
        }
        Err(e) => {
            eprintln!("✗ Parsing failed: {}", e);
            std::process::exit(1);
        }
    }
}
