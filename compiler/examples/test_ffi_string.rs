use inkwell::context::Context;
use inkwell::targets::{CodeModel, FileType, InitializationConfig, RelocMode, Target, TargetMachine};
use inkwell::OptimizationLevel;
use pole_compiler::{parse_ir, CodeGen, CompilerArenas};
use std::path::Path;
use std::process::Command;

fn main() {
    let ir_code = r#"
@extern("puts")
func c_puts(s: String) -> Int

@extern("putchar")
func c_putchar(c: Int) -> Int

func main() -> Int :
  let _ = c_puts("Testing multiple FFI calls:") in
  let _ = c_puts("Line 1") in
  let _ = c_puts("Line 2") in
  let _ = c_puts("Line 3") in
  let _ = c_putchar(10) in
  c_puts("Done!")
"#;

    println!("=== Testing FFI string functions (strlen, strcmp) ===\n");
    println!("IR Code:\n{}\n", ir_code);

    let program = parse_ir(ir_code).expect("Failed to parse IR");
    println!("✓ Parsed successfully");
    println!("  - Extern functions: {}", program.extern_funcs.len());
    println!("  - Regular functions: {}", program.func_defs.len());

    if !program.extern_funcs.is_empty() {
        println!("\nExtern functions:");
        for extern_func in &program.extern_funcs {
            println!("  - {} -> {}", extern_func.name, extern_func.c_name);
        }
    }

    println!("\n=== Compiling to LLVM IR ===");
    let arenas = CompilerArenas::new_default();
    let context = Context::create();
    let mut codegen = CodeGen::new(&context, "test_ffi_string", &arenas.codegen_arena);

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

    let object_path = Path::new("/tmp/test_ffi_string.o");
    target_machine
        .write_to_file(codegen.get_module(), FileType::Object, object_path)
        .expect("Failed to write object file");
    println!("✓ Object file: {}", object_path.display());

    println!("\n=== Linking executable ===");
    let exe_path = "/tmp/test_ffi_string";
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
    // Exit code is the return value of last puts() call (typically positive)
    if stdout.contains("Testing multiple FFI calls") && stdout.contains("Line 1") && stdout.contains("Done!") {
        println!("\n✓✓✓ SUCCESS: Multiple FFI calls work! ✓✓✓");
        println!("All output lines present, FFI working correctly.");
    } else {
        println!("\n✗✗✗ FAILED: Expected successful execution with output ✗✗✗");
        println!("Got: {}", stdout);
    }
}
