use inkwell::context::Context;
use pole_compiler::{parse_ir, CodeGen, CompilerArenas};
use std::fs;

fn main() {
    let ir = fs::read_to_string("/tmp/test_67_90.pole-ir").unwrap();
    
    println!("Parsing...");
    let program = match parse_ir(&ir) {
        Ok(p) => {
            println!("✓ Parse OK - {} functions", p.func_defs.len());
            for (i, func) in p.func_defs.iter().enumerate() {
                println!("  {}: {}", i, func.name);
            }
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
        Ok(_) => println!("✓ Success (90 lines)!"),
        Err(e) => println!("✗ Failed (90 lines): {}", e),
    }
}
