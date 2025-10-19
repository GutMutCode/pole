use inkwell::context::Context;
use pole_compiler::{parse_ir, CodeGen, CompilerArenas};
use std::fs;
use std::process::Command;

fn main() {
    println!("=== Testing String.length Implementation ===\n");

    let ir_source = r#"
func test_hello() -> Nat :
  let s = "hello" in
  String_length(s)

func test_empty() -> Nat :
  let s = "" in
  String_length(s)

func test_long() -> Nat :
  let s = "This is a longer string with 42 characters" in
  String_length(s)
"#;

    println!("Parsing IR...");
    let program = parse_ir(ir_source).expect("Failed to parse IR");
    println!("✓ Parsed successfully\n");

    println!("Compiling to LLVM IR...");
    let arenas = CompilerArenas::new_default();
    let context = Context::create();
    let mut codegen = CodeGen::new(&context, "string_length_test", &arenas.codegen_arena);

    codegen
        .compile_program(&program)
        .expect("Failed to compile program");
    println!("✓ Compilation successful\n");

    // Test cases
    let test_cases = vec![
        ("test_hello", 5, "hello"),
        ("test_empty", 0, "empty string"),
        ("test_long", 42, "long string"),
    ];

    println!("Running tests...\n");
    let mut passed = 0;
    let mut failed = 0;

    for (func_name, expected, desc) in &test_cases {
        print!("Testing {} ({})... ", func_name, desc);

        // Create main function that calls the test function
        let i32_type = context.i32_type();
        let main_fn_type = i32_type.fn_type(&[], false);
        let main_fn = codegen.get_module().add_function("main", main_fn_type, None);

        let entry_bb = context.append_basic_block(main_fn, "entry");
        let builder = context.create_builder();
        builder.position_at_end(entry_bb);

        let test_fn = codegen
            .get_module()
            .get_function(func_name)
            .expect(&format!("{} function not found", func_name));

        let result = builder
            .build_call(test_fn, &[], "result")
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
        let obj_path = format!("{}.o", func_name);
        codegen
            .write_object_file(std::path::Path::new(&obj_path))
            .expect("Failed to write object file");

        // Link executable
        let exe_path = format!("{}.exe", func_name);
        let link_status = Command::new("cc")
            .args(&["-o", &exe_path, &obj_path])
            .status()
            .expect("Failed to link");

        if !link_status.success() {
            println!("✗ FAIL (linking failed)");
            failed += 1;
            continue;
        }

        // Run executable
        let output = Command::new(format!("./{}", exe_path))
            .output()
            .expect("Failed to execute");

        let exit_code = output.status.code().unwrap_or(-1);

        if exit_code == *expected as i32 {
            println!("✓ PASS (got {})", exit_code);
            passed += 1;
        } else {
            println!("✗ FAIL (expected {}, got {})", expected, exit_code);
            failed += 1;
        }

        // Cleanup
        let _ = fs::remove_file(&obj_path);
        let _ = fs::remove_file(&exe_path);

        // Remove main function for next test
        unsafe {
            main_fn.delete();
        }
    }

    println!("\n=== Test Results ===");
    println!("Passed: {}/{}", passed, test_cases.len());
    println!("Failed: {}", failed);

    if failed == 0 {
        println!("\n✓ All String.length tests passed!");
    } else {
        println!("\n✗ Some tests failed");
        std::process::exit(1);
    }
}
