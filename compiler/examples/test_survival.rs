use inkwell::context::Context;
use pole_compiler::{parse_ir, CodeGen, CompilerArenas};
use std::fs;
use std::path::Path;
use std::process::Command;

fn main() {
    let ir_source = fs::read_to_string("../examples/63-survival-loop.pole-ir")
        .expect("Failed to read survival IR file");

    println!("=== Parsing Pole IR ===");
    let program = match parse_ir(&ir_source) {
        Ok(p) => {
            println!("✓ Parsed successfully");
            p
        }
        Err(e) => {
            eprintln!("✗ Parse failed: {}", e);
            return;
        }
    };

    println!("\n=== Compiling to LLVM IR ===");
    let arenas = CompilerArenas::new_default();
    let context = Context::create();
    let mut codegen = CodeGen::new(&context, "survival", &arenas.codegen_arena);

    if let Err(e) = codegen.compile_program(&program) {
        eprintln!("✗ Compilation failed: {}", e);
        return;
    }

    println!("✓ Compilation successful");

    let ir_path = Path::new("survival.ll");
    codegen
        .write_ir_to_file(ir_path)
        .expect("Failed to write LLVM IR");
    println!("\n✓ Written LLVM IR to survival.ll");

    let obj_path = Path::new("survival.o");
    codegen
        .write_object_file(obj_path)
        .expect("Failed to write object file");
    println!("✓ Written object file to survival.o");

    println!("\n=== Linking executable ===");
    let runtime_obj = "../runtime/pole_runtime.o";
    
    let link_status = Command::new("cc")
        .args(&["-o", "survival", "survival.o", runtime_obj])
        .status()
        .expect("Failed to execute linker");

    if !link_status.success() {
        eprintln!("✗ Linking failed");
        return;
    }
    println!("✓ Linked executable: survival");

    println!("\n=== Running native executable ===");
    let output = Command::new("./survival")
        .output()
        .expect("Failed to execute survival");

    println!("{}", String::from_utf8_lossy(&output.stdout));
    
    if !output.status.success() {
        eprintln!("stderr: {}", String::from_utf8_lossy(&output.stderr));
    }

    println!("\n=== Cleanup ===");
    let _ = fs::remove_file("survival.ll");
    let _ = fs::remove_file("survival.o");
    let _ = fs::remove_file("survival");
    println!("✓ Cleaned up generated files");
}
