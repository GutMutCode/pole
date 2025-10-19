use inkwell::context::Context;
use pole_compiler::{parse_ir, CodeGen};
use std::fs;
use std::process::Command;

struct TestCase {
    name: &'static str,
    ir_file: &'static str,
    function: &'static str,
    args: Vec<i64>,
    expected: i64,
}

fn main() {
    let test_cases = vec![
        TestCase {
            name: "factorial",
            ir_file: "../examples/01-factorial.pole-ir",
            function: "factorial",
            args: vec![5],
            expected: 120,
        },
        TestCase {
            name: "fibonacci",
            ir_file: "../examples/02-fibonacci.pole-ir",
            function: "fibonacci",
            args: vec![10],
            expected: 55,
        },
        TestCase {
            name: "max",
            ir_file: "../examples/07-max.pole-ir",
            function: "max",
            args: vec![42, 17],
            expected: 42,
        },
    ];

    println!("=== Running All Native Compilation Tests ===\n");

    let mut passed = 0;
    let mut failed = 0;

    for test in &test_cases {
        print!("Testing {} ... ", test.name);

        match run_test(test) {
            Ok(result) if result == test.expected => {
                println!("✓ PASS (got {})", result);
                passed += 1;
            }
            Ok(result) => {
                println!("✗ FAIL (expected {}, got {})", test.expected, result);
                failed += 1;
            }
            Err(e) => {
                println!("✗ ERROR: {}", e);
                failed += 1;
            }
        }
    }

    println!("\n=== Test Results ===");
    println!("Passed: {}/{}", passed, test_cases.len());
    println!("Failed: {}", failed);

    if failed == 0 {
        println!("\n✓ All tests passed!");
    } else {
        println!("\n✗ Some tests failed");
        std::process::exit(1);
    }
}

fn run_test(test: &TestCase) -> Result<i64, String> {
    let ir_source = fs::read_to_string(test.ir_file)
        .map_err(|e| format!("Failed to read IR file: {}", e))?;

    let program = parse_ir(&ir_source)
        .map_err(|e| format!("Failed to parse IR: {}", e))?;

    let context = Context::create();
    let mut codegen = CodeGen::new(&context, test.name);

    codegen
        .compile_program(&program)
        .map_err(|e| format!("Failed to compile: {}", e))?;

    let i32_type = context.i32_type();
    let i64_type = context.i64_type();
    let main_fn_type = i32_type.fn_type(&[], false);
    let main_fn = codegen.get_module().add_function("main", main_fn_type, None);

    let entry_bb = context.append_basic_block(main_fn, "entry");
    let builder = context.create_builder();
    builder.position_at_end(entry_bb);

    let func = codegen
        .get_module()
        .get_function(test.function)
        .ok_or_else(|| format!("Function {} not found", test.function))?;

    let args: Vec<_> = test
        .args
        .iter()
        .map(|&arg| i64_type.const_int(arg as u64, false).into())
        .collect();

    let result = builder
        .build_call(func, &args, "result")
        .unwrap()
        .try_as_basic_value()
        .left()
        .unwrap()
        .into_int_value();

    let result_i32 = builder
        .build_int_truncate(result, i32_type, "result_i32")
        .unwrap();
    builder.build_return(Some(&result_i32)).unwrap();

    let obj_file = format!("{}.o", test.name);
    let exe_file = format!("{}_test", test.name);

    codegen
        .write_object_file(std::path::Path::new(&obj_file))
        .map_err(|e| format!("Failed to write object file: {}", e))?;

    let link_status = Command::new("cc")
        .args(&["-o", &exe_file, &obj_file])
        .status()
        .map_err(|e| format!("Failed to link: {}", e))?;

    if !link_status.success() {
        return Err("Linking failed".to_string());
    }

    let output = Command::new(format!("./{}", exe_file))
        .output()
        .map_err(|e| format!("Failed to execute: {}", e))?;

    let _ = fs::remove_file(&obj_file);
    let _ = fs::remove_file(&exe_file);

    Ok(output.status.code().unwrap_or(-1) as i64)
}
