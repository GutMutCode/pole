use inkwell::context::Context;
use inkwell::targets::{CodeModel, FileType, InitializationConfig, RelocMode, Target, TargetMachine};
use inkwell::OptimizationLevel;
use pole_compiler::{parse_ir, CodeGen, CompilerArenas};
use std::path::Path;
use std::process::Command;

fn main() {
    let ir_code = std::fs::read_to_string("../examples/30-camera-simple.pole-ir")
        .expect("Failed to read IR file");

    println!("=== Simple Camera Demo ===\n");
    
    let program = parse_ir(&ir_code).expect("Failed to parse IR");
    println!("✓ Parsed: {} functions, {} types", program.func_defs.len(), program.type_defs.len());
    
    let arenas = CompilerArenas::new_default();
    let context = Context::create();
    let mut codegen = CodeGen::new(&context, "camera_simple", &arenas.codegen_arena);

    codegen.compile_program(&program).expect("Compilation failed");
    println!("✓ LLVM IR generated");
    
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

    let object_path = Path::new("/tmp/camera_simple.o");
    target_machine
        .write_to_file(codegen.get_module(), FileType::Object, object_path)
        .expect("Failed to write object file");

    let exe_path = "/tmp/camera_simple";
    
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
    println!("✓ Executable: {}", exe_path);
    
    println!("\n=== Running demo ===\n");
    
    let output = Command::new(exe_path)
        .output()
        .expect("Failed to execute");

    println!("{}", String::from_utf8_lossy(&output.stdout));
    
    if output.status.success() {
        println!("\n✓✓✓ SUCCESS! Camera system works! ✓✓✓");
    }
}
