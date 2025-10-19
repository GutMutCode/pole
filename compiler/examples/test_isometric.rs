use inkwell::context::Context;
use inkwell::targets::{CodeModel, FileType, InitializationConfig, RelocMode, Target, TargetMachine};
use inkwell::OptimizationLevel;
use pole_compiler::{parse_ir, CodeGen, CompilerArenas};
use std::path::Path;
use std::process::Command;

fn main() {
    println!("=== Project Zomboid - Isometric Rendering Demo ===\n");
    
    // Read the isometric demo IR
    let ir_code = std::fs::read_to_string("../examples/27-isometric-simple.pole-ir")
        .expect("Failed to read 27-isometric-simple.pole-ir");
    
    println!("Step 1: Parsing IR...");
    let program = parse_ir(&ir_code).expect("Failed to parse IR");
    println!("✓ IR parsed successfully");
    println!("  Type defs: {}", program.type_defs.len());
    println!("  Extern funcs: {}", program.extern_funcs.len());
    println!("  Functions: {}", program.func_defs.len());
    
    println!("\nStep 2: Compiling to LLVM IR...");
    let arenas = CompilerArenas::new_default();
    let context = Context::create();
    let mut codegen = CodeGen::new(&context, "isometric_demo", &arenas.codegen_arena);
    
    codegen.compile_program(&program).expect("Compilation failed");
    println!("✓ LLVM IR generated");
    
    println!("\nStep 3: Compiling to object file...");
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
    
    let object_path = Path::new("/tmp/isometric_demo.o");
    target_machine
        .write_to_file(codegen.get_module(), FileType::Object, object_path)
        .expect("Failed to write object file");
    println!("✓ Object file written to: {}", object_path.display());
    
    println!("\nStep 4: Linking with SDL2...");
    let executable_path = "/tmp/isometric_demo";
    let link_result = Command::new("gcc")
        .arg(object_path)
        .arg("-o")
        .arg(executable_path)
        .arg("-lSDL2")
        .arg("-lm")
        .output()
        .expect("Failed to execute linker");
    
    if !link_result.status.success() {
        eprintln!("Linking failed!");
        eprintln!("stderr: {}", String::from_utf8_lossy(&link_result.stderr));
        panic!("Failed to link executable");
    }
    println!("✓ Executable created: {}", executable_path);
    
    println!("\nStep 5: Running isometric demo...");
    println!("(SDL2 window will display for 10 seconds)\n");
    
    let run_result = Command::new(executable_path)
        .output()
        .expect("Failed to execute demo");
    
    if !run_result.status.success() {
        eprintln!("Execution failed!");
        eprintln!("stdout: {}", String::from_utf8_lossy(&run_result.stdout));
        eprintln!("stderr: {}", String::from_utf8_lossy(&run_result.stderr));
    } else {
        println!("✓ Demo completed successfully!");
    }
    
    println!("\n=== Demo Features ===");
    println!("- 10x10 isometric grid");
    println!("- Coordinate transformation (tile → screen)");
    println!("- Y-sorting (depth ordering)");
    println!("- Different tile types:");
    println!("  * Green: Grass");
    println!("  * Gray: Road");
    println!("  * Brown: Building floor");
    println!("\nNext Steps:");
    println!("- Day 3-4: Add camera controls (WASD)");
    println!("- Day 5-6: Add zoom and mouse interaction");
    println!("- Day 7: Record demo video for YouTube!");
}
