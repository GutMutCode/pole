use inkwell::context::Context;
use pole_compiler::{parse_ir, codegen::CodeGen, CompilerArenas};
use std::fs;

fn main() {
    println!("=== M4.4: Option & Result Type Summary ===\n");
    
    // Test 1: Simple Option constructors
    println!("Test 1: Option constructors");
    let ir1 = fs::read_to_string("../examples/15-simple-option.pole-ir")
        .expect("Failed to read file");
    let program1 = parse_ir(&ir1).expect("Parse failed");
    
    let arenas1 = CompilerArenas::new_default();
    let context1 = Context::create();
    let mut codegen1 = CodeGen::new(&context1, "simple_option", &arenas1.codegen_arena);
    codegen1.compile_program(&program1).expect("Codegen failed");
    let llvm_ir1 = codegen1.print_to_string();
    
    assert!(llvm_ir1.contains("{ i32, i64 }"), "Option should be struct with tag and value");
    assert!(llvm_ir1.contains("ret { i32, i64 } { i32 0, i64 undef }"), "None should be tag 0");
    assert!(llvm_ir1.contains("ret { i32, i64 } { i32 1, i64 42 }"), "Some(42) should be tag 1");
    println!("✓ None -> {{ i32 0, i64 undef }}");
    println!("✓ Some(42) -> {{ i32 1, i64 42 }}");
    
    // Test 2: Pattern matching on Option
    println!("\nTest 2: Pattern matching on Option");
    let ir2 = fs::read_to_string("../examples/16-option-match.pole-ir")
        .expect("Failed to read file");
    let program2 = parse_ir(&ir2).expect("Parse failed");
    
    let arenas2 = CompilerArenas::new_default();
    let context2 = Context::create();
    let mut codegen2 = CodeGen::new(&context2, "option_match", &arenas2.codegen_arena);
    codegen2.compile_program(&program2).expect("Codegen failed");
    let llvm_ir2 = codegen2.print_to_string();
    
    assert!(llvm_ir2.contains("extractvalue { i32, i64 } %opt, 0"), "Extract tag");
    assert!(llvm_ir2.contains("icmp eq i32 %tag, 1"), "Check if Some");
    assert!(llvm_ir2.contains("extractvalue { i32, i64 } %opt, 1"), "Extract value");
    assert!(llvm_ir2.contains("phi i64"), "Merge results with phi");
    println!("✓ Extract tag from Option struct");
    println!("✓ Branch on tag value (Some vs None)");
    println!("✓ Extract and bind value in Some case");
    println!("✓ Merge branches with PHI node");
    
    println!("\n=== M4.4 Complete ===");
    println!("\nImplemented:");
    println!("  - Option<T> type: {{ i32 tag, T value }}");
    println!("  - Result<T,E> type: {{ i32 tag, max(T,E) value }}");
    println!("  - None constructor (tag=0)");
    println!("  - Some(x) constructor (tag=1)");
    println!("  - Ok(x) constructor (tag=1)");
    println!("  - Err(e) constructor (tag=0)");
    println!("  - Pattern matching on Option/Result");
    println!("  - Variable binding in patterns (Some(x) -> x)");
    
    println!("\nExamples:");
    println!("  - 15-simple-option.pole-ir: None and Some constructors");
    println!("  - 16-option-match.pole-ir: Pattern matching with unwrap_or");
    
    println!("\nNext: M4.5 - Unit type and runtime functions");
}
