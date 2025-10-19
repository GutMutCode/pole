use inkwell::context::Context;
use pole_compiler::{parse_ir, CodeGen, CompilerArenas};
use std::fs;
use std::path::Path;
use std::process::Command;

fn main() {
    let ir_source = fs::read_to_string("../examples/08-simple-record.pole-ir")
        .expect("Failed to read simple-record IR file");

    println!("=== Parsing Pole IR ===");
    let program = parse_ir(&ir_source).expect("Failed to parse IR");
    println!("✓ Parsed successfully");
    println!("  Type definitions: {}", program.type_defs.len());
    println!("  Functions: {:?}", program.func_defs.iter().map(|f| &f.name).collect::<Vec<_>>());

    println!("\n=== Compiling to LLVM IR ===");
    let arenas = CompilerArenas::new_default();
    let context = Context::create();
    let mut codegen = CodeGen::new(&context, "simple_record", &arenas.codegen_arena);

    match codegen.compile_program(&program) {
        Ok(_) => println!("✓ Compilation successful"),
        Err(e) => {
            eprintln!("✗ Compilation failed: {}", e);
            return;
        }
    }

    println!("\n=== Adding main function ===");
    let i32_type = context.i32_type();
    let i64_type = context.i64_type();
    let point_type = context.struct_type(&[i64_type.into(), i64_type.into()], false);
    
    let main_fn_type = i32_type.fn_type(&[], false);
    let main_fn = codegen.get_module().add_function("main", main_fn_type, None);

    let entry_bb = context.append_basic_block(main_fn, "entry");
    let builder = context.create_builder();
    builder.position_at_end(entry_bb);

    // Create Point { x: 3, y: 4 }
    let x_val = i64_type.const_int(3, false);
    let y_val = i64_type.const_int(4, false);
    let point = point_type.const_named_struct(&[x_val.into(), y_val.into()]);

    // Call distance_from_origin(point)
    let distance_fn = codegen
        .get_module()
        .get_function("distance_from_origin")
        .expect("distance_from_origin not found");
    
    let result = builder
        .build_call(distance_fn, &[point.into()], "result")
        .unwrap()
        .try_as_basic_value()
        .left()
        .unwrap()
        .into_int_value();

    // Return result as exit code
    let result_i32 = builder
        .build_int_truncate(result, i32_type, "result_i32")
        .unwrap();
    builder.build_return(Some(&result_i32)).unwrap();

    println!("✓ Main function added: distance_from_origin({{x: 3, y: 4}})");

    let ir_path = Path::new("simple_record.ll");
    codegen
        .write_ir_to_file(ir_path)
        .expect("Failed to write LLVM IR");
    println!("\n✓ Written LLVM IR to simple_record.ll");

    let obj_path = Path::new("simple_record.o");
    codegen
        .write_object_file(obj_path)
        .expect("Failed to write object file");
    println!("✓ Written object file to simple_record.o");

    println!("\n=== Linking executable ===");
    let link_status = Command::new("cc")
        .args(&["-o", "simple_record", "simple_record.o"])
        .status()
        .expect("Failed to execute linker");

    if !link_status.success() {
        eprintln!("✗ Linking failed");
        return;
    }
    println!("✓ Linked executable: simple_record");

    println!("\n=== Running native executable ===");
    let output = Command::new("./simple_record")
        .output()
        .expect("Failed to execute simple_record");

    let exit_code = output.status.code().unwrap_or(-1);
    println!("distance_from_origin({{x: 3, y: 4}}) = {} (exit code)", exit_code);
    println!("Expected: 3*3 + 4*4 = 25");

    if exit_code == 25 {
        println!("\n✓ Native compilation and execution successful!");
    } else {
        println!("\n✗ Unexpected result");
    }

    println!("\n=== Cleanup ===");
    let _ = fs::remove_file("simple_record.ll");
    let _ = fs::remove_file("simple_record.o");
    let _ = fs::remove_file("simple_record");
    println!("✓ Cleaned up generated files");
}
