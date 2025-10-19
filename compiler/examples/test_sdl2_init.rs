use inkwell::context::Context;
use inkwell::targets::{CodeModel, FileType, InitializationConfig, RelocMode, Target, TargetMachine};
use inkwell::OptimizationLevel;
use pole_compiler::{parse_ir, CodeGen, CompilerArenas};
use std::path::Path;
use std::process::Command;

fn main() {
    let ir_code = r#"
@extern("SDL_Init")
func SDL_Init(flags: Int) -> Int

@extern("SDL_Quit")
func SDL_Quit(dummy: Unit) -> Unit

@extern("puts")
func c_puts(s: String) -> Int

func main() -> Int :
  let _ = c_puts("Initializing SDL2...") in
  let SDL_INIT_VIDEO = 32 in
  let result = SDL_Init(SDL_INIT_VIDEO) in
  if result == 0 then
    let _ = c_puts("SDL2 initialized successfully!") in
    let _ = SDL_Quit(()) in
    let _ = c_puts("SDL2 quit") in
    0
  else
    let _ = c_puts("SDL2 initialization failed") in
    1
"#;

    println!("=== Testing SDL2 initialization ===\n");

    let program = parse_ir(ir_code).expect("Failed to parse IR");
    println!("✓ Parsed successfully");

    println!("\n=== Compiling to LLVM IR ===");
    let arenas = CompilerArenas::new_default();
    let context = Context::create();
    let mut codegen = CodeGen::new(&context, "test_sdl2_init", &arenas.codegen_arena);

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

    let object_path = Path::new("/tmp/test_sdl2_init.o");
    target_machine
        .write_to_file(codegen.get_module(), FileType::Object, object_path)
        .expect("Failed to write object file");
    println!("✓ Object file: {}", object_path.display());

    println!("\n=== Linking executable with SDL2 ===");
    let exe_path = "/tmp/test_sdl2_init";
    
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
    let output = Command::new(exe_path)
        .output()
        .expect("Failed to execute");

    println!("stdout:\n{}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
    println!("exit code: {}", output.status.code().unwrap_or(-1));

    let stdout = String::from_utf8_lossy(&output.stdout);
    let exit_code = output.status.code().unwrap_or(-1);
    
    if stdout.contains("SDL2 initialized successfully") && stdout.contains("SDL2 quit") && exit_code == 0 {
        println!("\n✓✓✓ SUCCESS: SDL2 works with Pole FFI! ✓✓✓");
    } else {
        println!("\n✗✗✗ FAILED ✗✗✗");
        if !stdout.contains("SDL2 initialized successfully") {
            println!("SDL2 initialization may have failed");
        }
    }
}
