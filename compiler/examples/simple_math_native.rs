use inkwell::context::Context;
use pole_compiler::{parse_ir, CodeGen};
use std::fs;
use std::path::Path;
use std::process::Command;

fn main() {
    let ir_source = fs::read_to_string("../examples/04-simple-math.pole-ir")
        .expect("Failed to read simple-math IR file");

    println!("=== Parsing Pole IR ===");
    let program = parse_ir(&ir_source).expect("Failed to parse IR");
    println!("✓ Parsed successfully");
    println!("  Functions: abs, max, sum_to_n");

    println!("\n=== Compiling to LLVM IR ===");
    let context = Context::create();
    let mut codegen = CodeGen::new(&context, "simple_math");

    codegen
        .compile_program(&program)
        .expect("Failed to compile program");

    println!("✓ Compilation successful");

    println!("\n=== Adding main function ===");
    let i32_type = context.i32_type();
    let i64_type = context.i64_type();
    let main_fn_type = i32_type.fn_type(&[], false);
    let main_fn = codegen.get_module().add_function("main", main_fn_type, None);

    let entry_bb = context.append_basic_block(main_fn, "entry");
    let builder = context.create_builder();
    builder.position_at_end(entry_bb);

    let abs_fn = codegen.get_module().get_function("abs").expect("abs not found");
    let sum_to_n_fn = codegen.get_module().get_function("sum_to_n").expect("sum_to_n not found");

    let arg1 = i64_type.const_int(-10i64 as u64, true);
    let abs_result = builder
        .build_call(abs_fn, &[arg1.into()], "abs_result")
        .unwrap()
        .try_as_basic_value()
        .left()
        .unwrap()
        .into_int_value();

    let arg2 = i64_type.const_int(5, false);
    let sum_result = builder
        .build_call(sum_to_n_fn, &[arg2.into()], "sum_result")
        .unwrap()
        .try_as_basic_value()
        .left()
        .unwrap()
        .into_int_value();

    let combined = builder.build_int_add(abs_result, sum_result, "combined").unwrap();

    let result_i32 = builder
        .build_int_truncate(combined, i32_type, "result_i32")
        .unwrap();
    builder.build_return(Some(&result_i32)).unwrap();

    println!("✓ Main function added: abs(-10) + sum_to_n(5)");

    let ir_path = Path::new("simple_math.ll");
    codegen
        .write_ir_to_file(ir_path)
        .expect("Failed to write LLVM IR");
    println!("\n✓ Written LLVM IR to simple_math.ll");

    let obj_path = Path::new("simple_math.o");
    codegen
        .write_object_file(obj_path)
        .expect("Failed to write object file");
    println!("✓ Written object file to simple_math.o");

    println!("\n=== Linking executable ===");
    let link_status = Command::new("cc")
        .args(&["-o", "simple_math", "simple_math.o"])
        .status()
        .expect("Failed to execute linker");

    if !link_status.success() {
        eprintln!("✗ Linking failed");
        return;
    }
    println!("✓ Linked executable: simple_math");

    println!("\n=== Running native executable ===");
    let output = Command::new("./simple_math")
        .output()
        .expect("Failed to execute simple_math");

    let exit_code = output.status.code().unwrap_or(-1);
    println!("abs(-10) + sum_to_n(5) = {} (exit code)", exit_code);
    println!("Expected: 10 + 15 = 25");

    if exit_code == 25 {
        println!("\n✓ Native compilation and execution successful!");
    } else {
        println!("\n✗ Unexpected result");
    }

    println!("\n=== Cleanup ===");
    let _ = fs::remove_file("simple_math.ll");
    let _ = fs::remove_file("simple_math.o");
    let _ = fs::remove_file("simple_math");
    println!("✓ Cleaned up generated files");
}
