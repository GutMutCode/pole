use inkwell::context::Context;
use pole_compiler::{parse_ir, CodeGen, CompilerArenas};
use std::fs;
use std::path::Path;
use std::process::Command;

fn main() {
    let ir_source = fs::read_to_string("../examples/09-add-points.pole-ir")
        .expect("Failed to read IR file");

    println!("=== Parsing Pole IR ===");
    let program = parse_ir(&ir_source).expect("Failed to parse IR");
    println!("✓ Parsed {} functions", program.func_defs.len());

    println!("\n=== Compiling to LLVM IR ===");
    let arenas = CompilerArenas::new_default();
    let context = Context::create();
    let mut codegen = CodeGen::new(&context, "test_add_points", &arenas.codegen_arena);

    codegen.compile_program(&program).expect("Compilation failed");
    println!("✓ Compilation successful");

    println!("\n=== Adding main function ===");
    let i32_type = context.i32_type();
    let i64_type = context.i64_type();
    let point_type = context.struct_type(&[i64_type.into(), i64_type.into()], false);
    
    let main_fn_type = i32_type.fn_type(&[], false);
    let main_fn = codegen.get_module().add_function("main", main_fn_type, None);

    let entry_bb = context.append_basic_block(main_fn, "entry");
    let builder = context.create_builder();
    builder.position_at_end(entry_bb);

    // Create Point p1 = { x: 1, y: 2 }
    let p1 = point_type.const_named_struct(&[
        i64_type.const_int(1, false).into(),
        i64_type.const_int(2, false).into(),
    ]);

    // Create Point p2 = { x: 4, y: 6 }
    let p2 = point_type.const_named_struct(&[
        i64_type.const_int(4, false).into(),
        i64_type.const_int(6, false).into(),
    ]);

    // Call add_points(p1, p2)
    let add_points_fn = codegen
        .get_module()
        .get_function("add_points")
        .expect("add_points not found");
    
    let result = builder
        .build_call(add_points_fn, &[p1.into(), p2.into()], "result")
        .unwrap()
        .try_as_basic_value()
        .left()
        .unwrap()
        .into_struct_value();

    // Extract x field from result
    let result_x = builder
        .build_extract_value(result, 0, "result_x")
        .unwrap()
        .into_int_value();

    // Return x as exit code
    let result_i32 = builder
        .build_int_truncate(result_x, i32_type, "result_i32")
        .unwrap();
    builder.build_return(Some(&result_i32)).unwrap();

    println!("✓ Main function added");

    let obj_path = Path::new("test_add_points.o");
    codegen.write_object_file(obj_path).expect("Failed to write object file");

    let link_status = Command::new("cc")
        .args(&["-o", "test_add_points", "test_add_points.o"])
        .status()
        .expect("Failed to link");

    if !link_status.success() {
        eprintln!("✗ Linking failed");
        return;
    }

    println!("\n=== Running native executable ===");
    let output = Command::new("./test_add_points")
        .output()
        .expect("Failed to execute");

    let exit_code = output.status.code().unwrap_or(-1);
    println!("add_points({{1,2}}, {{4,6}}).x = {} (exit code)", exit_code);
    println!("Expected: 1 + 4 = 5");

    if exit_code == 5 {
        println!("\n✓ Record construction successful!");
    } else {
        println!("\n✗ Unexpected result");
    }

    let _ = fs::remove_file("test_add_points.o");
    let _ = fs::remove_file("test_add_points");
}
