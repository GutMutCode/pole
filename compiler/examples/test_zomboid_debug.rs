use inkwell::context::Context;
use pole_compiler::{parse_ir, CodeGen, CompilerArenas};
use std::fs;

fn main() {
    println!("=== Testing Zomboid Main (Debug) ===\n");
    
    let ir_code = fs::read_to_string("../games/zomboid/main.pole-ir")
        .expect("Failed to read file");
    
    println!("Parsing IR...");
    let program = match parse_ir(&ir_code) {
        Ok(p) => {
            println!("✓ Parse successful!");
            println!("  Type defs: {}", p.type_defs.len());
            println!("  Functions: {}", p.func_defs.len());
            println!("  Externs: {}", p.extern_funcs.len());
            
            println!("\nFunction list:");
            for (i, func) in p.func_defs.iter().enumerate() {
                println!("  {}: {}", i, func.name);
            }
            
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
    let mut codegen = CodeGen::new(&context, "zomboid", &arenas.codegen_arena);
    
    // Compile functions one by one to find which fails
    for (i, func) in program.func_defs.iter().enumerate() {
        print!("Compiling function {}: {}... ", i, func.name);
        // We can't call compile_function directly, so we'll just compile the whole program
        // and see where it fails
        break;
    }
    
    match codegen.compile_program(&program) {
        Ok(_) => println!("✓ Compilation successful!"),
        Err(e) => println!("✗ Compilation failed: {}", e),
    }
}
