use inkwell::context::Context;
use pole_compiler::{parse_ir, CodeGen, CompilerArenas};
use std::fs;
use std::path::Path;
use std::process::Command;

fn main() {
    let ir_source = fs::read_to_string("../examples/35-keyboard-camera.pole-ir")
        .expect("Failed to read IR file");

    println!("=== Keyboard Camera Control Test ===\n");
    
    println!("Step 1: Parsing IR...");
    let program = parse_ir(&ir_source).expect("Failed to parse IR");
    println!("✓ IR parsed successfully");
    println!("  Functions: {}", program.func_defs.len());

    println!("\nStep 2: Compiling to LLVM IR...");
    let arenas = CompilerArenas::new_default();
    let context = Context::create();
    let mut codegen = CodeGen::new(&context, "keyboard_camera", &arenas.codegen_arena);

    codegen.compile_program(&program).expect("Compilation failed");
    println!("✓ LLVM IR generated");

    println!("\nStep 3: Writing object file...");
    codegen.write_object_file(Path::new("keyboard_camera.o")).expect("Failed to write object file");
    println!("✓ Object file written");

    println!("\nStep 4: Linking with SDL2 and runtime...");
    let link_status = Command::new("cc")
        .args(&[
            "-o", "keyboard_camera",
            "keyboard_camera.o",
            "../runtime/pole_runtime.c",
            "-lSDL2",
        ])
        .status()
        .expect("Failed to link");

    if !link_status.success() {
        eprintln!("✗ Linking failed");
        return;
    }
    println!("✓ Executable created: ./keyboard_camera");

    println!("\nStep 5: Running keyboard camera demo...");
    println!("(Press WASD to move camera, ESC to exit)");
    println!();
    
    let output = Command::new("./keyboard_camera")
        .output()
        .expect("Failed to execute");

    println!("{}", String::from_utf8_lossy(&output.stdout));
    
    if output.status.success() {
        println!("\n✓ Demo completed successfully!");
    } else {
        println!("\n✗ Demo exited with error");
    }

    // Cleanup
    let _ = fs::remove_file("keyboard_camera.o");
    let _ = fs::remove_file("keyboard_camera");
}
