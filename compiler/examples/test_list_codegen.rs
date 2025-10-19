use inkwell::context::Context;
use pole_compiler::{parse_ir, CodeGen};
use std::fs;

fn main() {
    let ir = fs::read_to_string("../examples/11-simple-list.pole-ir")
        .expect("Failed to read file");

    println!("=== Parsing & Compiling List Example ===");
    let program = parse_ir(&ir).expect("Failed to parse");
    
    let context = Context::create();
    let mut codegen = CodeGen::new(&context, "simple_list");

    match codegen.compile_program(&program) {
        Ok(_) => println!("✓ Compilation successful"),
        Err(e) => {
            eprintln!("✗ Compilation failed: {}", e);
            return;
        }
    }

    println!("\n=== Generated LLVM IR ===");
    let ir = codegen.print_to_string();
    for (i, line) in ir.lines().enumerate() {
        println!("{:3}: {}", i + 1, line);
    }
}
