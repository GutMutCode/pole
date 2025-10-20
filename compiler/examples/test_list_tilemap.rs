use inkwell::context::Context;
use pole_compiler::{parse_ir, CodeGen, CompilerArenas};
use std::fs;
use std::path::Path;
use std::process::Command;

fn main() {
    let ir_source = fs::read_to_string("../examples/42-list-tilemap.pole-ir")
        .expect("Failed to read list-tilemap IR file");

    println!("=== Parsing Pole IR ===");
    let program = parse_ir(&ir_source).expect("Failed to parse IR");
    println!("✓ Parsed successfully");
    println!("  Functions: {}", program.func_defs.len());
    println!("  Extern functions: {}", program.extern_funcs.len());

    println!("\n=== Compiling to LLVM IR ===");
    let arenas = CompilerArenas::new_default();
    let context = Context::create();
    let mut codegen = CodeGen::new(&context, "list_tilemap", &arenas.codegen_arena);

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

    // Call main()
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

    println!("✓ Main function wrapper added");

    // Write LLVM IR
    let ir_path = Path::new("list_tilemap.ll");
    codegen
        .write_ir_to_file(ir_path)
        .expect("Failed to write LLVM IR");
    println!("\n✓ Written LLVM IR to list_tilemap.ll");

    // Write object file
    let obj_path = Path::new("list_tilemap.o");
    codegen
        .write_object_file(obj_path)
        .expect("Failed to write object file");
    println!("✓ Written object file to list_tilemap.o");

    // Link to executable with SDL2
    println!("\n=== Linking executable ===");
    let link_status = Command::new("cc")
        .args(&[
            "-o", "list_tilemap",
            "list_tilemap.o",
            "-lSDL2",
            "-L/usr/lib/x86_64-linux-gnu",
        ])
        .status()
        .expect("Failed to execute linker");

    if !link_status.success() {
        eprintln!("✗ Linking failed");
        eprintln!("Note: SDL2 must be installed (apt install libsdl2-dev)");
        return;
    }
    println!("✓ Linked executable: list_tilemap");

    // Run the executable
    println!("\n=== Running native executable ===");
    println!("(Will display SDL2 window for 5 seconds)");
    let output = Command::new("./list_tilemap")
        .output()
        .expect("Failed to execute list_tilemap");

    let exit_code = output.status.code().unwrap_or(-1);
    
    // Print stdout
    if !output.stdout.is_empty() {
        println!("\n--- Program Output ---");
        print!("{}", String::from_utf8_lossy(&output.stdout));
        println!("--- End Output ---");
    }
    
    // Print stderr if any
    if !output.stderr.is_empty() {
        eprintln!("\n--- Errors ---");
        eprint!("{}", String::from_utf8_lossy(&output.stderr));
        eprintln!("--- End Errors ---");
    }

    println!("\nExit code: {}", exit_code);
    println!("Expected: 0");

    if exit_code == 0 {
        println!("\n✓ List-based tilemap successfully rendered!");
    } else {
        println!("\n✗ Unexpected result");
    }

    // Cleanup
    println!("\n=== Cleanup ===");
    let _ = fs::remove_file("list_tilemap.ll");
    let _ = fs::remove_file("list_tilemap.o");
    let _ = fs::remove_file("list_tilemap");
    println!("✓ Cleaned up generated files");
}
