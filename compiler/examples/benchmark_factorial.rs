use inkwell::context::Context;
use inkwell::OptimizationLevel;
use pole_compiler::{parse_ir, CodeGen, CompilerArenas};
use std::fs;
use std::time::Instant;

fn main() {
    let ir_source = fs::read_to_string("../examples/01-factorial.pole-ir")
        .expect("Failed to read factorial IR file");

    println!("=== Factorial Performance Benchmark ===\n");

    // Compile to native code
    let program = parse_ir(&ir_source).expect("Failed to parse IR");
    let arenas = CompilerArenas::new_default();
    let context = Context::create();
    let mut codegen = CodeGen::new(&context, "factorial", &arenas.codegen_arena);
    codegen
        .compile_program(&program)
        .expect("Failed to compile program");

    // Create JIT engine
    let engine = codegen
        .get_module()
        .create_jit_execution_engine(OptimizationLevel::Aggressive)
        .expect("Failed to create JIT engine");

    let (native_per_call, native_result) = unsafe {
        let factorial = engine
            .get_function::<unsafe extern "C" fn(i64) -> i64>("factorial")
            .expect("Failed to get factorial function");

        // Warm up
        for _ in 0..100 {
            factorial.call(20);
        }

        // Benchmark native (JIT) execution
        let iterations = 100_000;
        let test_value = 20i64;

        let start = Instant::now();
        for _ in 0..iterations {
            let _ = factorial.call(test_value);
        }
        let native_duration = start.elapsed();
        let native_per_call = native_duration.as_nanos() / iterations as u128;

        println!("Native (LLVM JIT) Compilation:");
        println!("  Test: factorial({})", test_value);
        println!("  Iterations: {}", iterations);
        println!("  Total time: {:?}", native_duration);
        println!("  Per call: {} ns", native_per_call);
        
        let result = factorial.call(test_value);
        println!("  Result: {}", result);
        
        (native_per_call, result)
    };

    // Benchmark Python interpreter
    println!("\n--- Comparing with Python Interpreter ---");
    println!("Running: pole run ../examples/01-factorial.pole-ir factorial 20\n");

    let start = Instant::now();
    let output = std::process::Command::new("pole")
        .args(&[
            "run",
            "../examples/01-factorial.pole-ir",
            "factorial",
            "20",
        ])
        .output()
        .expect("Failed to run pole interpreter");
    let interpreter_duration = start.elapsed();

    if output.status.success() {
        let result = String::from_utf8_lossy(&output.stdout);
        println!("Interpreter result:\n{}", result);
        println!("Interpreter time (single call): {:?}", interpreter_duration);
    } else {
        println!("Interpreter failed: {}", String::from_utf8_lossy(&output.stderr));
    }

    // Calculate speedup
    println!("\n=== Performance Summary ===");
    println!("Native (per call): {} ns", native_per_call);
    println!("Interpreter (single call): {:?}", interpreter_duration);
    
    let interpreter_ns = interpreter_duration.as_nanos();
    if interpreter_ns > 0 {
        let speedup = interpreter_ns as f64 / native_per_call as f64;
        println!("\n🚀 Native speedup: {:.1}x faster than interpreter", speedup);
    }

    println!("\nNote: Interpreter timing includes process startup overhead.");
    println!("Native timing is pure computation (JIT-compiled LLVM code).");
}
