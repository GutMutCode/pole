use inkwell::context::Context;
use inkwell::OptimizationLevel;
use pole_compiler::{parse_ir, CodeGen, CompilerArenas};
use std::fs;

fn main() {
    let ir_source = fs::read_to_string("../examples/01-factorial.pole-ir")
        .expect("Failed to read factorial IR file");

    println!("=== Parsing Pole IR ===");
    let program = parse_ir(&ir_source).expect("Failed to parse IR");
    println!("✓ Parsed successfully");
    println!("Functions: {:?}", program.func_defs.iter().map(|f| &f.name).collect::<Vec<_>>());

    println!("\n=== Compiling to LLVM IR ===");
    let arenas = CompilerArenas::new_default();
    let context = Context::create();
    let mut codegen = CodeGen::new(&context, "factorial", &arenas.codegen_arena);

    codegen
        .compile_program(&program)
        .expect("Failed to compile program");

    println!("✓ Compilation successful\n");
    println!("{}", codegen.print_to_string());

    println!("\n=== Testing via JIT ===");
    let engine = codegen
        .get_module()
        .create_jit_execution_engine(OptimizationLevel::None)
        .expect("Failed to create JIT engine");

    unsafe {
        let factorial = engine
            .get_function::<unsafe extern "C" fn(i64) -> i64>("factorial")
            .expect("Failed to get factorial function");

        let test_cases = [(0, 1), (1, 1), (5, 120), (7, 5040)];

        for (input, expected) in test_cases {
            let result = factorial.call(input);
            let status = if result == expected { "✓" } else { "✗" };
            println!(
                "{} factorial({}) = {} (expected: {})",
                status, input, result, expected
            );
            assert_eq!(result, expected);
        }

        println!("\n✓ All tests passed!");
    }
}
