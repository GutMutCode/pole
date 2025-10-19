use inkwell::context::Context;
use pole_compiler::{parse_ir, CodeGen, CompilerArenas};
use std::fs;
use std::process::Command;

fn main() {
    println!("=== Testing print/println Implementation ===\n");

    let ir_source = r#"
func test_print() -> Unit :
  print("Hello, ")

func test_println() -> Unit :
  println("World!")

func test_both() -> Unit :
  println("Hello, World!")
"#;

    println!("Parsing IR...");
    let program = parse_ir(ir_source).expect("Failed to parse IR");
    println!("✓ Parsed successfully\n");

    println!("Compiling to LLVM IR...");
    let arenas = CompilerArenas::new_default();
    let context = Context::create();
    let mut codegen = CodeGen::new(&context, "print_test", &arenas.codegen_arena);

    codegen
        .compile_program(&program)
        .expect("Failed to compile program");
    println!("✓ Compilation successful\n");

    // Create main function that calls test_main
    let i32_type = context.i32_type();
    let main_fn_type = i32_type.fn_type(&[], false);
    let main_fn = codegen.get_module().add_function("main", main_fn_type, None);

    let entry_bb = context.append_basic_block(main_fn, "entry");
    let builder = context.create_builder();
    builder.position_at_end(entry_bb);

    let test_both = codegen
        .get_module()
        .get_function("test_both")
        .expect("test_both function not found");

    builder
        .build_call(test_both, &[], "result")
        .unwrap();

    // Return 0
    builder
        .build_return(Some(&i32_type.const_zero()))
        .unwrap();

    // Write object file
    let obj_path = "print_test.o";
    codegen
        .write_object_file(std::path::Path::new(obj_path))
        .expect("Failed to write object file");

    // Link executable
    let exe_path = "print_test.exe";
    let link_status = Command::new("cc")
        .args(&["-o", exe_path, obj_path])
        .status()
        .expect("Failed to link");

    if !link_status.success() {
        eprintln!("✗ Linking failed");
        std::process::exit(1);
    }

    println!("✓ Linked successfully\n");

    // Run executable
    println!("Running print_test...");
    println!("Expected output: Hello, World!");
    println!("Actual output:");
    
    let output = Command::new(format!("./{}", exe_path))
        .output()
        .expect("Failed to execute");

    let stdout = String::from_utf8_lossy(&output.stdout);
    print!("{}", stdout);

    if stdout.trim() == "Hello, World!" {
        println!("\n✓ PASS: Output matches expected");
    } else {
        println!("\n✗ FAIL: Output doesn't match");
        println!("Expected: 'Hello, World!'");
        println!("Got: '{}'", stdout.trim());
        std::process::exit(1);
    }

    // Cleanup
    let _ = fs::remove_file(obj_path);
    let _ = fs::remove_file(exe_path);

    println!("\n✓ All print/println tests passed!");
}
