use inkwell::context::Context;
use pole_compiler::{parse_ir, CodeGen, CompilerArenas};
use std::fs;

fn main() {
    let ir_code = fs::read_to_string("/tmp/test_zomboid_first_func.pole-ir")
        .expect("Failed to read file");
    
    let program = parse_ir(&ir_code).expect("Parse failed");
    
    let arenas = CompilerArenas::new_default();
    let context = Context::create();
    let mut codegen = CodeGen::new(&context, "test", &arenas.codegen_arena);
    
    match codegen.compile_program(&program) {
        Ok(_) => println!("✓ Success"),
        Err(e) => println!("✗ Failed: {}", e),
    }
}
