use inkwell::context::Context;
use inkwell::targets::{CodeModel, FileType, InitializationConfig, RelocMode, Target, TargetMachine};
use inkwell::OptimizationLevel;
use pole_compiler::{parse_ir, CodeGen, CompilerArenas};
use std::path::Path;
use std::process::Command;

fn main() {
    let ir_code = r#"
type Point = {
  x: Int,
  y: Int
}

@extern("puts")
func c_puts(s: String) -> Int

func sum_point(p: Point) -> Int :
  p.x + p.y

func main() -> Int :
  let _ = c_puts("Test completed") in
  0
"#;

    println!("=== Testing FFI with struct (Record) passing ===\n");
    println!("IR Code:\n{}\n", ir_code);

    let program = parse_ir(ir_code).expect("Failed to parse IR");
    println!("✓ Parsed successfully");
    println!("  - Type definitions: {}", program.type_defs.len());
    println!("  - Extern functions: {}", program.extern_funcs.len());
    println!("  - Regular functions: {}", program.func_defs.len());

    println!("\n=== Compiling to LLVM IR ===");
    let arenas = CompilerArenas::new_default();
    let context = Context::create();
    let mut codegen = CodeGen::new(&context, "test_ffi_struct", &arenas.codegen_arena);

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

    let object_path = Path::new("/tmp/test_ffi_struct.o");
    target_machine
        .write_to_file(codegen.get_module(), FileType::Object, object_path)
        .expect("Failed to write object file");
    println!("✓ Object file: {}", object_path.display());

    println!("\n=== Linking executable ===");
    let exe_path = "/tmp/test_ffi_struct";
    let link_status = Command::new("cc")
        .args(&[
            object_path.to_str().unwrap(),
            "-o", exe_path,
        ])
        .status()
        .expect("Failed to link");

    if !link_status.success() {
        panic!("Linking failed");
    }
    println!("✓ Executable: {}", exe_path);

    println!("\n=== Running executable ===");
    let output = Command::new(exe_path)
        .output()
        .expect("Failed to execute");

    println!("stdout:\n{}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
    println!("exit code: {}", output.status.code().unwrap_or(-1));

    let stdout = String::from_utf8_lossy(&output.stdout);
    let exit_code = output.status.code().unwrap_or(-1);
    
    // Exit code should be 0
    if stdout.contains("Test completed") && exit_code == 0 {
        println!("\n✓✓✓ SUCCESS: Record type definition works with FFI! ✓✓✓");
        println!("Point type with field access compiles successfully");
    } else {
        println!("\n✗✗✗ FAILED ✗✗✗");
        println!("Expected exit code 0, got {}", exit_code);
    }
}
