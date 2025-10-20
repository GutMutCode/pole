use pole_compiler::{parse_ir, CodeGen, CompilerArenas};
use inkwell::context::Context;

fn main() {
    let ir = std::fs::read_to_string("../examples/47-test-copy.pole-ir").unwrap();
    
    println!("Parsing...");
    let prog = parse_ir(&ir).expect("Parse failed");
    println!("Parsed {} functions", prog.func_defs.len());
    
    for func in &prog.func_defs {
        println!("  - {} : body = {:?}", func.name, func.body);
    }
    
    println!("\nCompiling...");
    let arenas = CompilerArenas::new_default();
    let context = Context::create();
    let mut codegen = CodeGen::new(&context, "test", &arenas.codegen_arena);
    
    match codegen.compile_program(&prog) {
        Ok(()) => println!("Success!"),
        Err(e) => println!("Error: {}", e),
    }
}
