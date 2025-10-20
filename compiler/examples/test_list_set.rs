use inkwell::context::Context;
use pole_compiler::{parse_ir, CodeGen, CompilerArenas};
use std::fs;
use std::path::Path;
use std::process::Command;

fn main() {
    let ir_source = fs::read_to_string("../examples/41-list-set-test.pole-ir")
        .expect("Failed to read list-set IR file");

    println!("=== Parsing Pole IR ===");
    let program = parse_ir(&ir_source).expect("Failed to parse IR");
    println!("✓ Parsed successfully");

    println!("\n=== Compiling to LLVM IR ===");
    let arenas = CompilerArenas::new_default();
    let context = Context::create();
    let mut codegen = CodeGen::new(&context, "list_set_test", &arenas.codegen_arena);

    codegen
        .compile_program(&program)
        .expect("Failed to compile program");

    println!("✓ Compilation successful");

    // Add main function as entry point
    println!("\n=== Adding main function ===");
    let i32_type = context.i32_type();
    let main_fn_type = i32_type.fn_type(&[], false);
    let main_fn = codegen.get_module().add_function("main", main_fn_type, None);

    let entry_bb = context.append_basic_block(main_fn, "entry");
    let builder = context.create_builder();
    builder.position_at_end(entry_bb);

    // Call test_list_set()
    let test_main = codegen
        .get_module()
        .get_function("test_list_set")
        .expect("test_list_set function not found");
    
    let result = builder
        .build_call(test_main, &[], "result")
        .unwrap()
        .try_as_basic_value()
        .left()
        .unwrap()
        .into_int_value();

    // Print result (we'll just return it as exit code for now)
    let result_i32 = builder
        .build_int_truncate(result, i32_type, "result_i32")
        .unwrap();
    builder.build_return(Some(&result_i32)).unwrap();

    println!("✓ Main function wrapper added");

    // Write LLVM IR
    let ir_path = Path::new("list_set.ll");
    codegen
        .write_ir_to_file(ir_path)
        .expect("Failed to write LLVM IR");
    println!("\n✓ Written LLVM IR to list_set.ll");

    // Write object file
    let obj_path = Path::new("list_set.o");
    codegen
        .write_object_file(obj_path)
        .expect("Failed to write object file");
    println!("✓ Written object file to list_set.o");

    // Link to executable
    println!("\n=== Linking executable ===");
    let link_status = Command::new("cc")
        .args(&["-o", "list_set", "list_set.o"])
        .status()
        .expect("Failed to execute linker");

    if !link_status.success() {
        eprintln!("✗ Linking failed");
        return;
    }
    println!("✓ Linked executable: list_set");

    // Run the executable
    println!("\n=== Running native executable ===");
    let output = Command::new("./list_set")
        .output()
        .expect("Failed to execute list_set");

    let exit_code = output.status.code().unwrap_or(-1);
    println!("List_set test result = {} (exit code)", exit_code);
    println!("Expected: 42");

    if exit_code == 42 {
        println!("\n✓ List_set implementation successful!");
    } else {
        println!("\n✗ Unexpected result");
    }

    // Cleanup
    println!("\n=== Cleanup ===");
    let _ = fs::remove_file("list_set.ll");
    let _ = fs::remove_file("list_set.o");
    let _ = fs::remove_file("list_set");
    println!("✓ Cleaned up generated files");
}
