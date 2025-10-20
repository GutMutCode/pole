use inkwell::context::Context;
use pole_compiler::{parse_ir, CodeGen, CompilerArenas};
use std::fs;

fn main() {
    println!("=== Testing Example 67: test-player ===\n");
    
    let ir_code = fs::read_to_string("../examples/67-test-player.pole-ir")
        .expect("Failed to read file");
    
    println!("Parsing IR...");
    let program = match parse_ir(&ir_code) {
        Ok(p) => {
            println!("✓ Parse successful!");
            println!("  Type defs: {}", p.type_defs.len());
            println!("  Functions: {}", p.func_defs.len());
            p
        },
        Err(e) => {
            println!("✗ Parse failed: {}", e);
            return;
        }
    };
    
    println!("\nAttempting compilation...");
    let arenas = CompilerArenas::new_default();
    let context = Context::create();
    let mut codegen = CodeGen::new(&context, "test_player", &arenas.codegen_arena);
    
    match codegen.compile_program(&program) {
        Ok(_) => println!("✓ Compilation successful!"),
        Err(e) => println!("✗ Compilation failed: {}", e),
    }
}
