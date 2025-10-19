use inkwell::context::Context;
use pole_compiler::{parse_ir, CodeGen};
use std::fs;
use std::path::Path;
use std::process::Command;

fn main() {
    let ir_source = fs::read_to_string("../examples/05-is-even.pole-ir")
        .expect("Failed to read is-even IR file");

    println!("=== Parsing Pole IR ===");
    let program = parse_ir(&ir_source).expect("Failed to parse IR");
    println!("✓ Parsed successfully");
    println!("  Functions: is_even_helper, is_even");

    println!("\n=== Compiling to LLVM IR ===");
    let context = Context::create();
    let mut codegen = CodeGen::new(&context, "is_even");

    codegen
        .compile_program(&program)
        .expect("Failed to compile program");

    println!("✓ Compilation successful");

    println!("\n=== Adding main function ===");
    let i1_type = context.bool_type();
    let i32_type = context.i32_type();
    let i64_type = context.i64_type();
    let main_fn_type = i32_type.fn_type(&[], false);
    let main_fn = codegen.get_module().add_function("main", main_fn_type, None);

    let entry_bb = context.append_basic_block(main_fn, "entry");
    let builder = context.create_builder();
    builder.position_at_end(entry_bb);

    let is_even_fn = codegen.get_module().get_function("is_even").expect("is_even not found");

    let arg = i64_type.const_int(7, false);
    let result = builder
        .build_call(is_even_fn, &[arg.into()], "result")
        .unwrap()
        .try_as_basic_value()
        .left()
        .unwrap()
        .into_int_value();

    let result_i32 = builder.build_int_z_extend(result, i32_type, "result_i32").unwrap();
    builder.build_return(Some(&result_i32)).unwrap();

    println!("✓ Main function added: is_even(7)");

    let ir_path = Path::new("is_even.ll");
    codegen
        .write_ir_to_file(ir_path)
        .expect("Failed to write LLVM IR");
    println!("\n✓ Written LLVM IR to is_even.ll");

    let obj_path = Path::new("is_even.o");
    codegen
        .write_object_file(obj_path)
        .expect("Failed to write object file");
    println!("✓ Written object file to is_even.o");

    println!("\n=== Linking executable ===");
    let link_status = Command::new("cc")
        .args(&["-o", "is_even", "is_even.o"])
        .status()
        .expect("Failed to execute linker");

    if !link_status.success() {
        eprintln!("✗ Linking failed");
        return;
    }
    println!("✓ Linked executable: is_even");

    println!("\n=== Running native executable ===");
    let output = Command::new("./is_even")
        .output()
        .expect("Failed to execute is_even");

    let exit_code = output.status.code().unwrap_or(-1);
    println!("is_even(7) = {} (exit code: 0=false, 1=true)", exit_code);
    println!("Expected: 0 (false)");

    if exit_code == 0 {
        println!("\n✓ Native compilation and execution successful!");
    } else {
        println!("\n✗ Unexpected result");
    }

    println!("\n=== Cleanup ===");
    let _ = fs::remove_file("is_even.ll");
    let _ = fs::remove_file("is_even.o");
    let _ = fs::remove_file("is_even");
    println!("✓ Cleaned up generated files");
}
