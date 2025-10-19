use inkwell::context::Context;
use inkwell::targets::{CodeModel, FileType, InitializationConfig, RelocMode, Target, TargetMachine};
use inkwell::OptimizationLevel;
use pole_compiler::{parse_ir, CodeGen, CompilerArenas};
use std::path::Path;
use std::process::Command;

fn main() {
    let ir_code = std::fs::read_to_string("../examples/25-sdl2-rendering.pole-ir")
        .expect("Failed to read IR file");

    println!("=== SDL2 Rendering Demo ===\n");

    let program = parse_ir(&ir_code).expect("Failed to parse IR");
    println!("✓ Parsed successfully");
    println!("  Type defs: {}", program.type_defs.len());
    println!("  Extern funcs: {}", program.extern_funcs.len());
    println!("  Functions: {}", program.func_defs.len());

    println!("\n=== Compiling to LLVM IR ===");
    let arenas = CompilerArenas::new_default();
    let context = Context::create();
    let mut codegen = CodeGen::new(&context, "sdl2_rendering", &arenas.codegen_arena);

    codegen.compile_program(&program).expect("Compilation failed");
    println!("✓ LLVM IR generated");

    println!("\n=== Compiling to object file ===");
    Target::initialize_native(&InitializationConfig::default())
        .expect("Failed to initialize native target");

    let target_triple = TargetMachine::get_default_triple();
    let target = Target::from_triple(&target_triple)
        .expect("Failed to create target from triple");

    let target_machine = target
        .create_target_machine(
            &target_triple,
            "generic",
            "",
            OptimizationLevel::Default,
            RelocMode::PIC,
            CodeModel::Default,
        )
        .expect("Failed to create target machine");

    let object_path = Path::new("/tmp/sdl2_rendering.o");
    target_machine
        .write_to_file(codegen.get_module(), FileType::Object, object_path)
        .expect("Failed to write object file");
    println!("✓ Object file: {}", object_path.display());

    println!("\n=== Linking executable with SDL2 ===");
    let exe_path = "/tmp/sdl2_rendering";
    
    let sdl2_libdir = std::env::var("SDL2_LIBDIR")
        .unwrap_or_else(|_| "/nix/store/bdnbmvvqsl7jw8kgnsgnf7scrxi42mis-sdl2-compat-2.32.56/lib".to_string());
    
    let link_status = Command::new("cc")
        .args(&[
            object_path.to_str().unwrap(),
            &format!("-L{}", sdl2_libdir),
            "-lSDL2",
            "-o", exe_path,
        ])
        .status()
        .expect("Failed to link");

    if !link_status.success() {
        panic!("Linking failed - is SDL2 installed?");
    }
    println!("✓ Executable: {}", exe_path);

    println!("\n=== Running executable ===");
    println!("NOTE: Using SDL dummy video driver (headless mode)");
    println!("      In real environment, this would display a window with colored rectangles!\n");
    
    let output = Command::new(exe_path)
        .env("SDL_VIDEODRIVER", "dummy")
        .output()
        .expect("Failed to execute");

    println!("stdout:\n{}", String::from_utf8_lossy(&output.stdout));
    
    if !output.stderr.is_empty() {
        println!("stderr:\n{}", String::from_utf8_lossy(&output.stderr));
    }
    
    println!("exit code: {}", output.status.code().unwrap_or(-1));

    let stdout = String::from_utf8_lossy(&output.stdout);
    let exit_code = output.status.code().unwrap_or(-1);
    
    if stdout.contains("Pixels rendered!") 
        && stdout.contains("Done!") 
        && exit_code == 0 {
        println!("\n✓✓✓ SUCCESS: SDL2 Rendering Demo works! ✓✓✓");
        println!("\nWhat was rendered:");
        println!("  🟥 Red pattern   at (100, 100) - 5 pixels");
        println!("  🟦 Blue pattern  at (200, 200) - 5 pixels");
        println!("  🟩 Green pattern at (300, 300) - 5 pixels");
        println!("\nTo see actual window, run without SDL_VIDEODRIVER=dummy:");
        println!("  {}", exe_path);
    } else {
        println!("\n✗✗✗ FAILED ✗✗✗");
        if !stdout.contains("Pixels rendered!") {
            println!("Rendering may have failed");
        }
    }
}
