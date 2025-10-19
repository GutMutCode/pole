use inkwell::context::Context;
use pole_compiler::{parse_ir, CodeGen, CompilerArenas};
use std::fs;
use std::path::Path;
use std::process::Command;

fn main() {
    let ir_source = fs::read_to_string("../examples/38-mouse-hover.pole-ir")
        .expect("Failed to read IR file");

    println!("=== Mouse Hover Test ===\n");
    
    println!("Step 1: Parsing IR...");
    let program = parse_ir(&ir_source).expect("Failed to parse IR");
    println!("✓ Parsed {} functions", program.func_defs.len());

    println!("\nStep 2: Compiling to LLVM IR...");
    let arenas = CompilerArenas::new_default();
    let context = Context::create();
    let mut codegen = CodeGen::new(&context, "mouse_hover", &arenas.codegen_arena);

    codegen.compile_program(&program).expect("Compilation failed");
    println!("✓ LLVM IR generated");

    println!("\nStep 3: Writing object file...");
    codegen.write_object_file(Path::new("mouse_hover.o")).expect("Failed to write");
    println!("✓ Object file written");

    println!("\nStep 4: Linking...");
    let link_status = Command::new("cc")
        .args(&[
            "-o", "mouse_hover",
            "mouse_hover.o",
            "../runtime/pole_runtime.c",
            "-lSDL2",
        ])
        .status()
        .expect("Failed to link");

    if !link_status.success() {
        eprintln!("✗ Linking failed");
        return;
    }
    println!("✓ Executable created: ./mouse_hover");

    println!("\nStep 5: Running mouse hover demo...");
    println!("(Move mouse over window, ESC to exit)");
    println!();
    
    let output = Command::new("./mouse_hover")
        .output()
        .expect("Failed to execute");

    println!("{}", String::from_utf8_lossy(&output.stdout));
    
    if output.status.success() {
        println!("\n✓ Demo completed successfully!");
    } else {
        println!("\n✗ Demo exited with error");
    }

    // Cleanup
    let _ = fs::remove_file("mouse_hover.o");
    let _ = fs::remove_file("mouse_hover");
}
