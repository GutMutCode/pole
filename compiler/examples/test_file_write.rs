use inkwell::context::Context;
use pole_compiler::{parse_ir, CodeGen, CompilerArenas};
use std::fs;
use std::path::Path;
use std::process::Command;

fn main() {
    let ir_source = fs::read_to_string("../examples/50-file-write-simple.pole-ir")
        .expect("Failed to read file-write IR file");

    println!("=== Parsing Pole IR ===");
    let program = parse_ir(&ir_source).expect("Failed to parse IR");
    println!("✓ Parsed {} functions", program.func_defs.len());

    println!("\n=== Compiling to LLVM IR ===");
    let arenas = CompilerArenas::new_default();
    let context = Context::create();
    let mut codegen = CodeGen::new(&context, "file_write", &arenas.codegen_arena);

    codegen
        .compile_program(&program)
        .expect("Failed to compile program");

    println!("✓ Compilation successful");

    // Add main function as entry point
    let i32_type = context.i32_type();
    let main_fn_type = i32_type.fn_type(&[], false);
    let main_fn = codegen.get_module().add_function("main", main_fn_type, None);

    let entry_bb = context.append_basic_block(main_fn, "entry");
    let builder = context.create_builder();
    builder.position_at_end(entry_bb);

    let pole_main = codegen
        .get_module()
        .get_function("main")
        .expect("main function not found");
    
    let result = builder
        .build_call(pole_main, &[], "result")
        .unwrap()
        .try_as_basic_value()
        .left()
        .unwrap()
        .into_int_value();

    let result_i32 = builder
        .build_int_truncate(result, i32_type, "result_i32")
        .unwrap();
    builder.build_return(Some(&result_i32)).unwrap();

    // Write object file
    let obj_path = Path::new("file_write.o");
    codegen
        .write_object_file(obj_path)
        .expect("Failed to write object file");

    // Link to executable
    println!("\n=== Linking executable ===");
    let link_status = Command::new("cc")
        .args(&["-o", "file_write", "file_write.o"])
        .status()
        .expect("Failed to execute linker");

    if !link_status.success() {
        eprintln!("✗ Linking failed");
        return;
    }
    println!("✓ Linked executable");

    // Run the executable
    println!("\n=== Running executable ===");
    let output = Command::new("./file_write")
        .output()
        .expect("Failed to execute");

    let exit_code = output.status.code().unwrap_or(-1);
    
    if !output.stdout.is_empty() {
        print!("{}", String::from_utf8_lossy(&output.stdout));
    }

    println!("\nExit code: {}", exit_code);

    if exit_code == 0 {
        println!("\n✓ File write test passed!");
        
        // Check if file was created
        if Path::new("tilemap.txt").exists() {
            println!("\n=== File Contents ===");
            if let Ok(contents) = fs::read_to_string("tilemap.txt") {
                print!("{}", contents);
            }
            println!("=== End Contents ===");
            
            // Cleanup
            let _ = fs::remove_file("tilemap.txt");
        }
    } else {
        println!("\n✗ Test failed");
    }

    // Cleanup
    let _ = fs::remove_file("file_write.o");
    let _ = fs::remove_file("file_write");
}
