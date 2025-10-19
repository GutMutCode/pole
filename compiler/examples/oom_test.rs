use inkwell::context::Context;
use pole_compiler::{parse_ir, CodeGen, CompilerArenas};
use std::fs;

fn main() {
    println!("=== Out-of-Memory (OOM) Test ===\n");

    let ir_file = "../examples/01-factorial.pole-ir";
    let ir_source = fs::read_to_string(ir_file)
        .expect("Failed to read factorial IR file");

    let program = parse_ir(&ir_source).expect("Failed to parse IR");

    println!("Test 1: Normal compilation with sufficient memory (100 MB)");
    {
        let arenas = CompilerArenas::new(100 * 1024 * 1024);
        let context = Context::create();
        let mut codegen = CodeGen::new(&context, "factorial", &arenas.codegen_arena);
        
        match codegen.compile_program(&program) {
            Ok(_) => {
                println!("✓ Compilation successful");
                println!("  Memory used: {:.2} MB", arenas.total_allocated() as f64 / (1024.0 * 1024.0));
            }
            Err(e) => {
                println!("✗ Compilation failed: {}", e);
            }
        }
    }

    println!("\nTest 2: Compilation with limited memory (1 KB)");
    {
        let arenas = CompilerArenas::new(1024);
        let context = Context::create();
        let mut codegen = CodeGen::new(&context, "factorial", &arenas.codegen_arena);
        
        match codegen.compile_program(&program) {
            Ok(_) => {
                println!("✓ Compilation successful (unexpected!)");
                println!("  Memory used: {} bytes", arenas.total_allocated());
            }
            Err(e) => {
                println!("✗ Compilation failed (expected): {}", e);
            }
        }
    }

    println!("\nTest 3: Very small arena (100 bytes)");
    {
        let arenas = CompilerArenas::new(100);
        let context = Context::create();
        let mut codegen = CodeGen::new(&context, "factorial", &arenas.codegen_arena);
        
        match codegen.compile_program(&program) {
            Ok(_) => {
                println!("✓ Compilation successful (unexpected!)");
                println!("  Memory used: {} bytes", arenas.total_allocated());
            }
            Err(e) => {
                println!("✗ Compilation failed (expected): {}", e);
            }
        }
    }

    println!("\n=== OOM Test Notes ===");
    println!("Note: bumpalo doesn't enforce hard memory limits by default.");
    println!("It will allocate more chunks as needed.");
    println!("For true OOM detection, we need custom allocation tracking.");
    println!("\n✓ OOM test completed!");
}
