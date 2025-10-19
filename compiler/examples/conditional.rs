use inkwell::context::Context;
use inkwell::IntPredicate;
use inkwell::OptimizationLevel;

fn main() {
    let context = Context::create();
    let module = context.create_module("conditional");
    let builder = context.create_builder();

    let i64_type = context.i64_type();
    let fn_type = i64_type.fn_type(&[i64_type.into()], false);

    let function = module.add_function("abs", fn_type, None);

    let entry_bb = context.append_basic_block(function, "entry");
    let then_bb = context.append_basic_block(function, "then");
    let else_bb = context.append_basic_block(function, "else");
    let merge_bb = context.append_basic_block(function, "merge");

    builder.position_at_end(entry_bb);
    let n = function.get_nth_param(0).unwrap().into_int_value();
    let zero = i64_type.const_int(0, true);
    let cond = builder
        .build_int_compare(IntPredicate::SLT, n, zero, "is_negative")
        .unwrap();
    builder
        .build_conditional_branch(cond, then_bb, else_bb)
        .unwrap();

    builder.position_at_end(then_bb);
    let negated = builder.build_int_neg(n, "negated").unwrap();
    builder.build_unconditional_branch(merge_bb).unwrap();

    builder.position_at_end(else_bb);
    builder.build_unconditional_branch(merge_bb).unwrap();

    builder.position_at_end(merge_bb);
    let phi = builder.build_phi(i64_type, "result").unwrap();
    phi.add_incoming(&[(&negated, then_bb), (&n, else_bb)]);
    builder.build_return(Some(&phi.as_basic_value())).unwrap();

    if function.verify(true) {
        println!("✓ Function verified successfully");
    } else {
        eprintln!("✗ Function verification failed");
        return;
    }

    println!("\n=== Generated LLVM IR (abs function with if/else) ===\n");
    println!("{}", module.print_to_string().to_string());

    let engine = module
        .create_jit_execution_engine(OptimizationLevel::None)
        .unwrap();

    unsafe {
        let abs_fn = engine
            .get_function::<unsafe extern "C" fn(i64) -> i64>("abs")
            .unwrap();

        println!("\n=== Execution Tests ===");
        let test_cases = [(-42, 42), (17, 17), (0, 0), (-1, 1)];
        for (input, expected) in test_cases {
            let result = abs_fn.call(input);
            println!("abs({}) = {} (expected: {})", input, result, expected);
            assert_eq!(result, expected);
        }
        println!("✓ All tests passed!");
    }
}
