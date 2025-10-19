use pole_compiler::parse_ir;
use std::fs;

fn main() {
    let ir_source = fs::read_to_string("../examples/08-simple-record.pole-ir")
        .expect("Failed to read IR file");

    match parse_ir(&ir_source) {
        Ok(program) => {
            println!("✓ Parsing successful");
            println!("Type definitions: {}", program.type_defs.len());
            for typedef in &program.type_defs {
                println!("  - {}: {:?}", typedef.name, typedef.definition);
            }
            println!("\nFunctions: {}", program.func_defs.len());
            for func in &program.func_defs {
                println!("  - {}", func.name);
                println!("    Params: {:?}", func.params);
                println!("    Body: {:?}", func.body);
            }
        }
        Err(e) => {
            eprintln!("✗ Parsing failed: {}", e);
        }
    }
}
