use pole_compiler::parse_ir;
use std::fs;

fn main() {
    let ir = fs::read_to_string("../examples/11-simple-list.pole-ir")
        .expect("Failed to read file");

    match parse_ir(&ir) {
        Ok(program) => {
            println!("✓ Parsing successful");
            for func in &program.func_defs {
                println!("\nFunction: {}", func.name);
                println!("Body: {:#?}", func.body);
            }
        }
        Err(e) => {
            eprintln!("✗ Parsing failed: {}", e);
        }
    }
}
