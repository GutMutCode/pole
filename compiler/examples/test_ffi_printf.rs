use inkwell::context::Context;
use inkwell::targets::{CodeModel, FileType, InitializationConfig, RelocMode, Target, TargetMachine};
use inkwell::OptimizationLevel;
use pole_compiler::{parse_ir, CodeGen, CompilerArenas};
use std::path::Path;
use std::process::Command;

fn main() {
    let ir_code = r#"
@extern("printf")
@variadic
func c_printf(format: String) -> Int

func main() -> Int :
  c_printf("Hello from C!")
"#;

    println!("=== Testing FFI printf example ===\n");
    println!("IR Code:\n{}\n", ir_code);

    let program = parse_ir(ir_code).expect("Failed to parse IR");
    println!("✓ Parsed successfully");
    println!("  - Extern functions: {}", program.extern_funcs.len());
    println!("  - Regular functions: {}", program.func_defs.len());

    if !program.extern_funcs.is_empty() {
        for extern_func in &program.extern_funcs {
            println!("\nExtern function:");
            println!("  Pole name: {}", extern_func.name);
            println!("  C name: {}", extern_func.c_name);
            println!("  Annotations: {:?}", extern_func.annotations);
            println!("  Params: {:?}", extern_func.params);
            println!("  Return: {:?}", extern_func.return_type);
        }
    }

    println!("\n=== Compiling to LLVM IR ===");
    let arenas = CompilerArenas::new_default();
    let context = Context::create();
    let mut codegen = CodeGen::new(&context, "test_ffi_printf", &arenas.codegen_arena);

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

    let object_path = Path::new("/tmp/test_ffi_printf.o");
    target_machine
        .write_to_file(codegen.get_module(), FileType::Object, object_path)
        .expect("Failed to write object file");
    println!("✓ Object file: {}", object_path.display());

    println!("\n=== Linking executable ===");
    let exe_path = "/tmp/test_ffi_printf";
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

    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
    println!("exit code: {}", output.status.code().unwrap_or(-1));

    if output.stdout == b"Hello from C!" {
        println!("\n✓✓✓ SUCCESS: FFI printf works! ✓✓✓");
    } else {
        println!("\n✗✗✗ FAILED: Expected 'Hello from C!' but got {:?} ✗✗✗", output.stdout);
    }
}
