use inkwell::context::Context;
use pole_compiler::{parse_ir, CodeGen, CompilerArenas};
use std::fs;
use std::path::Path;
use std::process::Command;
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

    // Add main function that calls factorial many times
    let i32_type = context.i32_type();
    let i64_type = context.i64_type();
    let main_fn_type = i32_type.fn_type(&[], false);
    let main_fn = codegen.get_module().add_function("main", main_fn_type, None);

    let entry_bb = context.append_basic_block(main_fn, "entry");
    let builder = context.create_builder();
    builder.position_at_end(entry_bb);

    let factorial = codegen
        .get_module()
        .get_function("factorial")
        .expect("factorial function not found");

    // Call factorial(20) in a loop 100,000 times
    let iterations = i64_type.const_int(100_000, false);
    let test_value = i64_type.const_int(20, false);
    let counter_alloca = builder.build_alloca(i64_type, "counter").unwrap();
    builder
        .build_store(counter_alloca, i64_type.const_zero())
        .unwrap();

    let loop_bb = context.append_basic_block(main_fn, "loop");
    let body_bb = context.append_basic_block(main_fn, "body");
    let exit_bb = context.append_basic_block(main_fn, "exit");

    builder.build_unconditional_branch(loop_bb).unwrap();

    builder.position_at_end(loop_bb);
    let counter = builder
        .build_load(i64_type, counter_alloca, "counter")
        .unwrap()
        .into_int_value();
    let cond = builder
        .build_int_compare(inkwell::IntPredicate::SLT, counter, iterations, "cond")
        .unwrap();
    builder
        .build_conditional_branch(cond, body_bb, exit_bb)
        .unwrap();

    builder.position_at_end(body_bb);
    let _ = builder
        .build_call(factorial, &[test_value.into()], "result")
        .unwrap();
    let next_counter = builder.build_int_add(counter, i64_type.const_int(1, false), "next").unwrap();
    builder.build_store(counter_alloca, next_counter).unwrap();
    builder.build_unconditional_branch(loop_bb).unwrap();

    builder.position_at_end(exit_bb);
    builder
        .build_return(Some(&i32_type.const_zero()))
        .unwrap();

    // Write object file
    let obj_path = Path::new("factorial_bench.o");
    codegen
        .write_object_file(obj_path)
        .expect("Failed to write object file");

    // Link to executable
    let link_status = Command::new("cc")
        .args(&["-o", "factorial_bench", "factorial_bench.o"])
        .status()
        .expect("Failed to execute linker");

    if !link_status.success() {
        eprintln!("✗ Linking failed");
        return;
    }

    println!("Native (LLVM Compiled) Benchmark:");
    println!("  Test: factorial(20)");
    println!("  Iterations: 100,000");
    
    // Run native executable and measure time
    let start = Instant::now();
    let status = Command::new("./factorial_bench")
        .status()
        .expect("Failed to execute factorial_bench");
    let native_duration = start.elapsed();

    if !status.success() {
        eprintln!("✗ Native execution failed");
        return;
    }

    let native_per_call_ns = native_duration.as_nanos() / 100_000;
    println!("  Total time: {:?}", native_duration);
    println!("  Per call: {} ns", native_per_call_ns);

    // Benchmark Python interpreter (single call due to overhead)
    println!("\n--- Python Interpreter Benchmark ---");
    println!("Running: pole run ../examples/01-factorial.pole-ir factorial 20\n");

    let start = Instant::now();
    let output = Command::new("pole")
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
        let result = String::from_utf8_lossy(&output.stdout).trim().to_string();
        println!("Interpreter result: {}", result);
        println!("Interpreter time (single call with overhead): {:?}", interpreter_duration);
    }

    // Calculate speedup
    println!("\n=== Performance Summary ===");
    println!("Native (per call): {} ns", native_per_call_ns);
    println!("Interpreter (single call + overhead): {:?}", interpreter_duration);
    
    let interpreter_ns = interpreter_duration.as_nanos();
    let speedup = interpreter_ns as f64 / native_per_call_ns as f64;
    println!("\n🚀 Native speedup: {:.1}x faster", speedup);
    println!("\nNote: Interpreter includes Python startup + parsing overhead.");
    println!("Native is pure computation (optimized machine code).");

    // Cleanup
    let _ = fs::remove_file("factorial_bench.o");
    let _ = fs::remove_file("factorial_bench");
}
