use pole_compiler::ir_parser::parse_ir;
use pole_compiler::codegen::compile_to_binary;
use std::fs;

fn main() {
    println!("=== Testing Zomboid Main Compilation ===\n");
    
    let ir_code = fs::read_to_string("../games/zomboid/main.pole-ir")
        .expect("Failed to read main.pole-ir");
    
    println!("Parsing IR...");
    let program = parse_ir(&ir_code).expect("Failed to parse IR");
    
    println!("Type defs: {}", program.type_defs.len());
    println!("Functions: {}", program.func_defs.len());
    println!("Externs: {}", program.extern_funcs.len());
    
    println!("\nAttempting compilation...");
    match compile_to_binary(&program, "zomboid_test") {
        Ok(_) => println!("✓ Compilation successful!"),
        Err(e) => println!("✗ Compilation failed: {}", e),
    }
}
