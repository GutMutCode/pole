use inkwell::context::Context;
use inkwell::OptimizationLevel;

fn main() {
    let context = Context::create();
    let module = context.create_module("hello_llvm");
    let builder = context.create_builder();

    let i64_type = context.i64_type();
    let fn_type = i64_type.fn_type(&[i64_type.into(), i64_type.into()], false);

    let function = module.add_function("add", fn_type, None);
    let basic_block = context.append_basic_block(function, "entry");

    builder.position_at_end(basic_block);

    let x = function.get_nth_param(0).unwrap().into_int_value();
    let y = function.get_nth_param(1).unwrap().into_int_value();

    let sum = builder.build_int_add(x, y, "sum").unwrap();
    builder.build_return(Some(&sum)).unwrap();

    if function.verify(true) {
        println!("✓ Function verified successfully");
    } else {
        eprintln!("✗ Function verification failed");
    }

    println!("\n=== Generated LLVM IR ===\n");
    println!("{}", module.print_to_string().to_string());

    let engine = module
        .create_jit_execution_engine(OptimizationLevel::None)
        .unwrap();

    unsafe {
        let add = engine
            .get_function::<unsafe extern "C" fn(i64, i64) -> i64>("add")
            .unwrap();

        let x = 40;
        let y = 2;
        let result = add.call(x, y);

        println!("\n=== Execution Test ===");
        println!("add({}, {}) = {}", x, y, result);
        assert_eq!(result, 42);
        println!("✓ Test passed!");
    }
}
