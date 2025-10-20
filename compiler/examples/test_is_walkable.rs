use inkwell::context::Context;
use pole_compiler::{parse_ir, CodeGen, CompilerArenas};
use std::fs;

fn main() {
    let ir = fs::read_to_string("/tmp/test_is_walkable.pole-ir").unwrap();
    
    println!("Parsing...");
    let program = match parse_ir(&ir) {
        Ok(p) => {
            println!("✓ Parse OK");
            p
        }
        Err(e) => {
            println!("✗ Parse failed: {}", e);
            return;
        }
    };
    
    println!("\nCompiling...");
    let arenas = CompilerArenas::new_default();
    let context = Context::create();
    let mut codegen = CodeGen::new(&context, "test", &arenas.codegen_arena);
    
    match codegen.compile_program(&program) {
        Ok(_) => println!("✓ Compilation success!"),
        Err(e) => println!("✗ Compilation failed: {}", e),
    }
}
