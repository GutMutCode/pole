use inkwell::context::Context;
use pole_compiler::{parse_ir, CodeGen, CompilerArenas};
use std::fs;
use std::process::Command;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let file = if args.len() > 1 { &args[1] } else { "../examples/42-list-push-test.pole-ir" };
    
    let ir_source = fs::read_to_string(file)
        .expect("Failed to read IR file");

    println!("=== Parsing Pole IR ===");
    let program = parse_ir(&ir_source).expect("Failed to parse IR");
    println!("✓ Parsed successfully");
    println!("  Functions: {}", program.func_defs.len());
    for func in &program.func_defs {
        println!("    - {} : body = {:?}", func.name, func.body);
    }

    println!("\n=== Compiling to LLVM IR ===");
    let arenas = CompilerArenas::new_default();
    let context = Context::create();
    let mut codegen = CodeGen::new(&context, "test_list_push", &arenas.codegen_arena);

    codegen
        .compile_program(&program)
        .expect("Failed to compile program");

    println!("✓ Compilation successful");
    
    // Print memory statistics
    let stats = pole_compiler::MemoryStats::new(
        arenas.parse_allocated(),
        arenas.ir_allocated(),
        arenas.codegen_allocated()
    );
    println!("Memory usage: {}", stats.format_human_readable());

    let ll_path = "/home/gmc/Devs/pole/test_list_push.ll";
    codegen.get_module()
        .print_to_file(ll_path)
        .expect("Failed to write LLVM IR");
    println!("✓ LLVM IR written to {}", ll_path);

    println!("\n=== Compiling with llc and gcc ===");
    
    let s_path = "/home/gmc/Devs/pole/test_list_push.s";
    let output = Command::new("llc")
        .args(&[ll_path, "-o", s_path])
        .output()
        .expect("Failed to run llc");

    if !output.status.success() {
        eprintln!("llc error: {}", String::from_utf8_lossy(&output.stderr));
        std::process::exit(1);
    }
    
    let output = Command::new("gcc")
        .args(&[
            s_path, 
            "../runtime/pole_runtime.o",
            "-o", "/home/gmc/Devs/pole/test_list_push", 
            "-lm", 
            "-lSDL2"
        ])
        .output()
        .expect("Failed to run gcc");

    if !output.status.success() {
        eprintln!("gcc error: {}", String::from_utf8_lossy(&output.stderr));
        std::process::exit(1);
    }
    println!("✓ Compiled to native binary");

    println!("\n=== Running executable ===");
    let output = Command::new("/home/gmc/Devs/pole/test_list_push")
        .output()
        .expect("Failed to run executable");

    println!("{}", String::from_utf8_lossy(&output.stdout));
    if !output.stderr.is_empty() {
        eprintln!("stderr: {}", String::from_utf8_lossy(&output.stderr));
    }

    if output.status.success() {
        println!("✓ Program executed successfully");
    } else {
        eprintln!("✗ Program failed with code {:?}", output.status.code());
    }
}
