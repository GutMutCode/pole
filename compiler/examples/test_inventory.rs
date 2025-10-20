use inkwell::context::Context;
use pole_compiler::{parse_ir, CodeGen, CompilerArenas};
use std::fs;
use std::path::Path;
use std::process::Command;

fn main() {
    let ir_source = fs::read_to_string("../examples/61-inventory-system.pole-ir")
        .expect("Failed to read inventory IR file");

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
    let mut codegen = CodeGen::new(&context, "inventory", &arenas.codegen_arena);

    if let Err(e) = codegen.compile_program(&program) {
        eprintln!("✗ Compilation failed: {}", e);
        return;
    }

    println!("✓ Compilation successful");

    // Write LLVM IR
    let ir_path = Path::new("inventory.ll");
    codegen
        .write_ir_to_file(ir_path)
        .expect("Failed to write LLVM IR");
    println!("\n✓ Written LLVM IR to inventory.ll");

    // Write object file
    let obj_path = Path::new("inventory.o");
    codegen
        .write_object_file(obj_path)
        .expect("Failed to write object file");
    println!("✓ Written object file to inventory.o");

    // Link with runtime
    println!("\n=== Linking executable ===");
    let runtime_obj = "../runtime/pole_runtime.o";
    
    let link_status = Command::new("cc")
        .args(&["-o", "inventory", "inventory.o", runtime_obj])
        .status()
        .expect("Failed to execute linker");

    if !link_status.success() {
        eprintln!("✗ Linking failed");
        return;
    }
    println!("✓ Linked executable: inventory");

    // Run the executable
    println!("\n=== Running native executable ===");
    let output = Command::new("./inventory")
        .output()
        .expect("Failed to execute inventory");

    println!("{}", String::from_utf8_lossy(&output.stdout));
    
    if !output.status.success() {
        eprintln!("stderr: {}", String::from_utf8_lossy(&output.stderr));
    }

    // Cleanup
    println!("\n=== Cleanup ===");
    let _ = fs::remove_file("inventory.ll");
    let _ = fs::remove_file("inventory.o");
    let _ = fs::remove_file("inventory");
    println!("✓ Cleaned up generated files");
}
