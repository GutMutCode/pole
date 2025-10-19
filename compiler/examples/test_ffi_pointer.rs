use inkwell::context::Context;
use inkwell::targets::{CodeModel, FileType, InitializationConfig, RelocMode, Target, TargetMachine};
use inkwell::OptimizationLevel;
use pole_compiler::{parse_ir, CodeGen, CompilerArenas};
use std::path::Path;
use std::process::Command;

fn main() {
    let ir_code = r#"
@extern("malloc")
func c_malloc(size: Int) -> Ptr<Unit>

@extern("free")
func c_free(ptr: Ptr<Unit>) -> Unit

@extern("puts")
func c_puts(s: String) -> Int

func main() -> Int :
  let size = 1024 in
  let buffer = c_malloc(size) in
  let _ = c_puts("Allocated 1024 bytes") in
  let _ = c_free(buffer) in
  let _ = c_puts("Freed buffer") in
  0
"#;

    println!("=== Testing Ptr<T> pointer type with malloc/free ===\n");
    println!("IR Code:\n{}\n", ir_code);

    let program = parse_ir(ir_code).expect("Failed to parse IR");
    println!("✓ Parsed successfully");
    println!("  - Extern functions: {}", program.extern_funcs.len());

    for func in &program.extern_funcs {
        println!("  - {} :: {:?} -> {:?}", func.name, func.params, func.return_type);
    }

    println!("\n=== Compiling to LLVM IR ===");
    let arenas = CompilerArenas::new_default();
    let context = Context::create();
    let mut codegen = CodeGen::new(&context, "test_ffi_pointer", &arenas.codegen_arena);

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

    let object_path = Path::new("/tmp/test_ffi_pointer.o");
    target_machine
        .write_to_file(codegen.get_module(), FileType::Object, object_path)
        .expect("Failed to write object file");
    println!("✓ Object file: {}", object_path.display());

    println!("\n=== Linking executable ===");
    let exe_path = "/tmp/test_ffi_pointer";
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
    
    if stdout.contains("Allocated 1024 bytes") && stdout.contains("Freed buffer") && exit_code == 0 {
        println!("\n✓✓✓ SUCCESS: Ptr<T> type works! malloc/free successful! ✓✓✓");
    } else {
        println!("\n✗✗✗ FAILED ✗✗✗");
    }
}
