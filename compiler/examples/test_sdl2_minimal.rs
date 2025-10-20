use inkwell::context::Context;
use pole_compiler::{parse_ir, CodeGen, CompilerArenas};
use std::fs;
use std::path::Path;

fn main() {
    println!("=== Testing SDL2 Minimal Example ===\n");
    
    let ir_code = fs::read_to_string("../examples/70-sdl2-minimal.pole-ir")
        .expect("Failed to read file");
    
    println!("Parsing IR...");
    let program = match parse_ir(&ir_code) {
        Ok(p) => {
            println!("✓ Parse successful!");
            println!("  Functions: {}", p.func_defs.len());
            println!("  Externs: {}", p.extern_funcs.len());
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
    let mut codegen = CodeGen::new(&context, "sdl2_minimal", &arenas.codegen_arena);
    
    match codegen.compile_program(&program) {
        Ok(_) => {
            println!("✓ Compilation successful!");
            println!("\nGenerating object file...");
            
            match codegen.write_object_file(Path::new("/tmp/sdl2_minimal.o")) {
                Ok(_) => println!("✓ Object file written"),
                Err(e) => println!("✗ Failed: {}", e),
            }
        },
        Err(e) => println!("✗ Compilation failed: {}", e),
    }
}
