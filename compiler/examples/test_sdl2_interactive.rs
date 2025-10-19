use inkwell::context::Context;
use inkwell::targets::{CodeModel, FileType, InitializationConfig, RelocMode, Target, TargetMachine};
use inkwell::OptimizationLevel;
use pole_compiler::{parse_ir, CodeGen, CompilerArenas};
use std::path::Path;
use std::process::Command;

fn main() {
    let ir_code = std::fs::read_to_string("../examples/26-sdl2-interactive.pole-ir")
        .expect("Failed to read IR file");

    println!("=== SDL2 Interactive Demo Compilation ===\n");

    let program = parse_ir(&ir_code).expect("Failed to parse IR");
    println!("✓ Parsed successfully");
    println!("  Functions: {} ({} extern)", 
             program.func_defs.len(), 
             program.extern_funcs.len());

    println!("\n=== Compiling to LLVM IR ===");
    let arenas = CompilerArenas::new_default();
    let context = Context::create();
    let mut codegen = CodeGen::new(&context, "sdl2_interactive", &arenas.codegen_arena);

    codegen.compile_program(&program).expect("Compilation failed");
    println!("✓ LLVM IR generated");

    println!("\n=== Compiling to native code ===");
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

    let object_path = Path::new("/tmp/sdl2_interactive.o");
    target_machine
        .write_to_file(codegen.get_module(), FileType::Object, object_path)
        .expect("Failed to write object file");
    println!("✓ Object file: {}", object_path.display());

    println!("\n=== Linking with SDL2 ===");
    let exe_path = "/tmp/sdl2_interactive";
    
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
        panic!("Linking failed");
    }
    println!("✓ Executable created: {}", exe_path);

    println!("\n=== Running in headless mode ===");
    
    let output = Command::new(exe_path)
        .env("SDL_VIDEODRIVER", "dummy")
        .output()
        .expect("Failed to execute");

    println!("{}", String::from_utf8_lossy(&output.stdout));
    
    if !output.stderr.is_empty() {
        eprintln!("stderr: {}", String::from_utf8_lossy(&output.stderr));
    }

    let exit_code = output.status.code().unwrap_or(-1);
    
    if exit_code == 0 {
        println!("\n✓✓✓ SUCCESS: Interactive demo compiled and ran! ✓✓✓");
        println!("\nTo see the actual window with graphics:");
        println!("  {}", exe_path);
        println!("\nThe window will display:");
        println!("  🟥 Red pattern");
        println!("  🟦 Blue pattern");
        println!("  🟩 Green pattern");
        println!("  🟨 Yellow pattern");
        println!("  🩵 Cyan pattern");
        println!("  🟪 Magenta pattern");
        println!("\nWindow will close automatically after 10 seconds,");
        println!("or you can close it manually.");
    } else {
        println!("\n✗✗✗ FAILED with exit code {} ✗✗✗", exit_code);
    }
}
