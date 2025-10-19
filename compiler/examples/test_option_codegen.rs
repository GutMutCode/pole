use inkwell::context::Context;
use pole_compiler::{parse_ir, codegen::CodeGen};
use std::fs;

fn main() {
    let ir = fs::read_to_string("../examples/15-simple-option.pole-ir")
        .expect("Failed to read file");

    let program = parse_ir(&ir).expect("Failed to parse IR");
    
    let context = Context::create();
    let mut codegen = CodeGen::new(&context, "simple_option");
    
    match codegen.compile_program(&program) {
        Ok(()) => {
            println!("✓ Code generation successful\n");
            println!("Generated LLVM IR:");
            println!("{}", codegen.print_to_string());
        }
        Err(e) => {
            eprintln!("✗ Code generation failed: {}", e);
            std::process::exit(1);
        }
    }
}
