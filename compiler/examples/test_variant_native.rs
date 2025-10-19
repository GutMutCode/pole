use inkwell::context::Context;
use pole_compiler::{parse_ir, codegen::CodeGen, CompilerArenas};
use std::fs;
use std::path::Path;

fn main() {
    let ir = fs::read_to_string("../examples/13-variant-tags.pole-ir")
        .expect("Failed to read file");

    let program = parse_ir(&ir).expect("Failed to parse IR");
    
    let arenas = CompilerArenas::new_default();
    let context = Context::create();
    let mut codegen = CodeGen::new(&context, "variant_tags", &arenas.codegen_arena);
    
    codegen.compile_program(&program).expect("Compilation failed");
    
    codegen.write_object_file(Path::new("variant_tags.o"))
        .expect("Failed to write object file");
    
    println!("✓ Compiled to variant_tags.o");
    
    use std::process::Command;
    let output = Command::new("gcc")
        .args(&["variant_tags.o", "-o", "variant_tags", "-no-pie"])
        .output()
        .expect("Failed to link");
    
    if !output.status.success() {
        eprintln!("Linking failed: {}", String::from_utf8_lossy(&output.stderr));
        std::process::exit(1);
    }
    
    println!("✓ Linked to variant_tags executable");
    println!("✓ All variant functions compiled successfully");
    
    // Clean up
    let _ = std::fs::remove_file("variant_tags.o");
    let _ = std::fs::remove_file("variant_tags");
}
