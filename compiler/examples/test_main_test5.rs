use inkwell::context::Context;
use pole_compiler::{parse_ir, CodeGen, CompilerArenas};
use std::fs;

fn main() {
    let ir = fs::read_to_string("../games/zomboid/main_test5.pole-ir").unwrap();
    let program = parse_ir(&ir).unwrap();
    let arenas = CompilerArenas::new_default();
    let context = Context::create();
    let mut codegen = CodeGen::new(&context, "test", &arenas.codegen_arena);
    
    match codegen.compile_program(&program) {
        Ok(_) => println!("✓ Success (122 lines with complete update_zombie)!"),
        Err(e) => println!("✗ Failed: {}", e),
    }
}
