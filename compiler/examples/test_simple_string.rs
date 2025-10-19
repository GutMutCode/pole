use inkwell::context::Context;
use pole_compiler::{parse_ir, CodeGen, CompilerArenas};
use std::fs;

fn main() {
    let ir_source = fs::read_to_string("../examples/09-simple-string.pole-ir")
        .expect("Failed to read IR file");

    println!("=== Parsing Pole IR ===");
    let program = parse_ir(&ir_source).expect("Failed to parse IR");
    println!("✓ Parsed {} functions", program.func_defs.len());

    println!("\n=== Compiling to LLVM IR ===");
    let arenas = CompilerArenas::new_default();
    let context = Context::create();
    let mut codegen = CodeGen::new(&context, "simple_string", &arenas.codegen_arena);

    match codegen.compile_program(&program) {
        Ok(_) => println!("✓ Compilation successful"),
        Err(e) => {
            eprintln!("✗ Compilation failed: {}", e);
            return;
        }
    }

    println!("\n=== Generated LLVM IR ===");
    let ir = codegen.print_to_string();
    for (i, line) in ir.lines().take(30).enumerate() {
        println!("{:3}: {}", i + 1, line);
    }
}
