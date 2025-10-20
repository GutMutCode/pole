use inkwell::context::Context;
use pole_compiler::{parse_ir, CodeGen, CompilerArenas};
use std::fs;

fn main() {
    println!("=== Testing Zomboid Main ===\n");
    
    let ir_code = fs::read_to_string("../games/zomboid/main.pole-ir")
        .expect("Failed to read file");
    
    println!("Parsing IR ({} bytes)...", ir_code.len());
    let program = match parse_ir(&ir_code) {
        Ok(p) => {
            println!("✓ Parse successful!");
            println!("  Type defs: {}", p.type_defs.len());
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
    let mut codegen = CodeGen::new(&context, "zomboid_main", &arenas.codegen_arena);
    
    match codegen.compile_program(&program) {
        Ok(_) => {
            println!("✓ Compilation successful!");
            
            // Write LLVM IR
            codegen.write_ir_to_file(std::path::Path::new("/tmp/zomboid_main.ll"))
                .expect("Failed to write LLVM IR");
            println!("  LLVM IR: /tmp/zomboid_main.ll");
            
            // Write object file
            codegen.write_object_file(std::path::Path::new("/tmp/zomboid_main.o"))
                .expect("Failed to write object file");
            println!("  Object file: /tmp/zomboid_main.o");
        },
        Err(e) => {
            println!("✗ Compilation failed: {}", e);
        }
    }
}
