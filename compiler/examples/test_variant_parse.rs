use pole_compiler::parse_ir;
use std::fs;

fn main() {
    let ir = fs::read_to_string("../examples/12-simple-variant.pole-ir")
        .expect("Failed to read file");

    match parse_ir(&ir) {
        Ok(program) => {
            println!("✓ Parsing successful");
            println!("\nType definitions: {}", program.type_defs.len());
            for typedef in &program.type_defs {
                println!("  - {}: {:?}", typedef.name, typedef.definition);
            }
            
            println!("\nFunctions: {}", program.func_defs.len());
            for func in &program.func_defs {
                println!("  - {}", func.name);
                println!("    Return type: {:?}", func.return_type);
                println!("    Body: {:?}", func.body);
            }
        }
        Err(e) => {
            eprintln!("✗ Parsing failed: {}", e);
        }
    }
}
