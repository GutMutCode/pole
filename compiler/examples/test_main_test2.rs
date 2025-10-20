use inkwell::context::Context;
use pole_compiler::{parse_ir, CodeGen, CompilerArenas};
use std::fs;

fn main() {
    let ir = fs::read_to_string("../games/zomboid/main_test2.pole-ir").unwrap();
    
    println!("Parsing...");
    let program = match parse_ir(&ir) {
        Ok(p) => {
            println!("✓ Parse OK - {} funcs", p.func_defs.len());
            p
        }
        Err(e) => {
            println!("✗ Parse failed: {}", e);
            return;
        }
    };
    
    let arenas = CompilerArenas::new_default();
    let context = Context::create();
    let mut codegen = CodeGen::new(&context, "test", &arenas.codegen_arena);
    
    println!("Compiling...");
    match codegen.compile_program(&program) {
        Ok(_) => println!("✓ Success!"),
        Err(e) => println!("✗ Failed: {}", e),
    }
}
