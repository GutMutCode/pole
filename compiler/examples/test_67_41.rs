use inkwell::context::Context;
use pole_compiler::{parse_ir, CodeGen, CompilerArenas};
use std::fs;

fn main() {
    let ir = fs::read_to_string("/tmp/test_67_41.pole-ir").unwrap();
    let program = parse_ir(&ir).unwrap();
    
    println!("Functions: {}", program.func_defs.len());
    
    let arenas = CompilerArenas::new_default();
    let context = Context::create();
    let mut codegen = CodeGen::new(&context, "test", &arenas.codegen_arena);
    
    match codegen.compile_program(&program) {
        Ok(_) => println!("✓ Success (first 3 functions)!"),
        Err(e) => println!("✗ Failed: {}", e),
    }
}
