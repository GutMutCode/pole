use inkwell::context::Context;
use pole_compiler::{parse_ir, CodeGen, CompilerArenas};
use std::fs;
use std::path::Path;
use std::process::Command;

fn main() {
    let ir_source = fs::read_to_string("../examples/39-texture-demo.pole-ir")
        .expect("Failed to read IR file");

    println!("=== Texture Demo Test ===\n");
    
    println!("Step 1: Parsing IR...");
    let program = parse_ir(&ir_source).expect("Failed to parse IR");
    println!("✓ Parsed {} functions", program.func_defs.len());

    println!("\nStep 2: Compiling to LLVM IR...");
    let arenas = CompilerArenas::new_default();
    let context = Context::create();
    let mut codegen = CodeGen::new(&context, "texture_demo", &arenas.codegen_arena);

    codegen.compile_program(&program).expect("Compilation failed");
    println!("✓ LLVM IR generated");

    println!("\nStep 3: Writing object file...");
    codegen.write_object_file(Path::new("texture_demo.o")).expect("Failed to write");
    println!("✓ Object file written");

    println!("\nStep 4: Linking...");
    let link_status = Command::new("cc")
        .args(&[
            "-o", "texture_demo",
            "texture_demo.o",
            "../runtime/pole_runtime.c",
            "-lSDL2",
        ])
        .status()
        .expect("Failed to link");

    if !link_status.success() {
        eprintln!("✗ Linking failed");
        return;
    }
    println!("✓ Executable created: ./texture_demo");

    println!("\nStep 5: Running texture demo...");
    println!("(5x5 grid with filled rectangles, 5 seconds)");
    println!();
    
    let output = Command::new("./texture_demo")
        .output()
        .expect("Failed to execute");

    println!("{}", String::from_utf8_lossy(&output.stdout));
    
    if output.status.success() {
        println!("\n✓ Demo completed successfully!");
    } else {
        println!("\n✗ Demo exited with error");
    }

    // Cleanup
    let _ = fs::remove_file("texture_demo.o");
    let _ = fs::remove_file("texture_demo");
}
