use pole_compiler::parse_ir;
use std::fs;

fn main() {
    let ir = fs::read_to_string("../examples/16-option-match.pole-ir")
        .expect("Failed to read file");

    match parse_ir(&ir) {
        Ok(program) => {
            println!("✓ Parsing successful\n");
            for func in &program.func_defs {
                println!("Function: {}", func.name);
                println!("Body: {:#?}\n", func.body);
            }
        }
        Err(e) => {
            eprintln!("✗ Parsing failed: {}", e);
            std::process::exit(1);
        }
    }
}
