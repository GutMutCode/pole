use inkwell::context::Context;
use pole_compiler::{parse_ir, CodeGen, CompilerArenas};
use std::fs;

fn main() {
    let ir = fs::read_to_string("/tmp/test_zomboid_types_only.pole-ir").unwrap();
    
    let program = match parse_ir(&ir) {
        Ok(p) => {
            println!("✓ Parse OK - {} types, {} funcs, {} externs", 
                p.type_defs.len(), p.func_defs.len(), p.extern_funcs.len());
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
    
    match codegen.compile_program(&program) {
        Ok(_) => println!("✓ Compilation success!"),
        Err(e) => println!("✗ Compilation failed: {}", e),
    }
}
