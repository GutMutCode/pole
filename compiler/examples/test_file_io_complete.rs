use inkwell::context::Context;
use pole_compiler::{parse_ir, CodeGen, CompilerArenas};
use std::fs;
use std::path::Path;
use std::process::Command;

fn main() {
    let ir_source = fs::read_to_string("../examples/52-file-io-complete.pole-ir")
        .expect("Failed to read file-io-complete IR file");

    println!("=== Parsing Pole IR ===");
    let program = parse_ir(&ir_source).expect("Failed to parse IR");
    println!("✓ Parsed {} functions", program.func_defs.len());

    println!("\n=== Compiling to LLVM IR ===");
    let arenas = CompilerArenas::new_default();
    let context = Context::create();
    let mut codegen = CodeGen::new(&context, "file_io_complete", &arenas.codegen_arena);

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
    let obj_path = Path::new("file_io_complete.o");
    codegen
        .write_object_file(obj_path)
        .expect("Failed to write object file");

    // Link to executable
    println!("\n=== Linking executable ===");
    let link_status = Command::new("cc")
        .args(&["-o", "file_io_complete", "file_io_complete.o"])
        .status()
        .expect("Failed to execute linker");

    if !link_status.success() {
        eprintln!("✗ Linking failed");
        return;
    }
    println!("✓ Linked executable");

    // Run the executable
    println!("\n=== Running executable ===");
    let output = Command::new("./file_io_complete")
        .output()
        .expect("Failed to execute");

    let exit_code = output.status.code().unwrap_or(-1);
    
    if !output.stdout.is_empty() {
        print!("{}", String::from_utf8_lossy(&output.stdout));
    }

    println!("\nExit code: {}", exit_code);

    if exit_code == 0 {
        println!("\n✓ File I/O test passed!");
        
        // Check if file was created and show contents
        if Path::new("tilemap_data.txt").exists() {
            println!("\n=== Generated File Contents ===");
            if let Ok(contents) = fs::read_to_string("tilemap_data.txt") {
                print!("{}", contents);
            }
            println!("=== End Contents ===");
            
            // Cleanup
            let _ = fs::remove_file("tilemap_data.txt");
        } else {
            println!("\n✗ Warning: tilemap_data.txt was not created");
        }
    } else {
        println!("\n✗ Test failed");
    }

    // Cleanup
    let _ = fs::remove_file("file_io_complete.o");
    let _ = fs::remove_file("file_io_complete");
}
