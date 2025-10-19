use inkwell::context::Context;
use pole_compiler::{parse_ir, CodeGen, CompilerArenas};
use std::fs;
use std::path::Path;

fn main() {
    let ir_source = fs::read_to_string("../examples/10-string-literal.pole-ir")
        .expect("Failed to read IR file");

    println!("=== Parsing & Compiling String Literal ===");
    let program = parse_ir(&ir_source).expect("Failed to parse IR");
    
    let arenas = CompilerArenas::new_default();
    let context = Context::create();
    let mut codegen = CodeGen::new(&context, "string_literal", &arenas.codegen_arena);

    codegen.compile_program(&program).expect("Compilation failed");
    println!("✓ Compilation successful");

    let ir_path = Path::new("string_literal.ll");
    codegen.write_ir_to_file(ir_path).expect("Failed to write IR");

    println!("\n=== Generated LLVM IR ===");
    let ir_content = fs::read_to_string("string_literal.ll").unwrap();
    for (i, line) in ir_content.lines().enumerate() {
        println!("{:3}: {}", i + 1, line);
    }

    let _ = fs::remove_file("string_literal.ll");
    
    println!("\n✓ String literal support verified!");
}
