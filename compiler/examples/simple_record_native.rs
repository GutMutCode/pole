use inkwell::context::Context;
use pole_compiler::{parse_ir, CodeGen};
use std::fs;
use std::path::Path;
use std::process::Command;

fn main() {
    let ir_source = fs::read_to_string("../examples/08-simple-record.pole-ir")
        .expect("Failed to read simple-record IR file");

    println!("=== Parsing Pole IR ===");
    let program = parse_ir(&ir_source).expect("Failed to parse IR");
    println!("✓ Parsed successfully");
    println!("  Type definitions: {}", program.type_defs.len());
    println!("  Functions: {:?}", program.func_defs.iter().map(|f| &f.name).collect::<Vec<_>>());

    println!("\n=== Compiling to LLVM IR ===");
    let context = Context::create();
    let mut codegen = CodeGen::new(&context, "simple_record");

    match codegen.compile_program(&program) {
        Ok(_) => println!("✓ Compilation successful"),
        Err(e) => {
            eprintln!("✗ Compilation failed: {}", e);
            return;
        }
    }

    let ir_path = Path::new("simple_record.ll");
    codegen
        .write_ir_to_file(ir_path)
        .expect("Failed to write LLVM IR");
    println!("✓ Written LLVM IR to simple_record.ll");

    println!("\n=== Generated LLVM IR (first 30 lines) ===");
    let ir_content = fs::read_to_string("simple_record.ll").unwrap();
    for (i, line) in ir_content.lines().take(30).enumerate() {
        println!("{:3}: {}", i + 1, line);
    }

    let _ = fs::remove_file("simple_record.ll");
}
