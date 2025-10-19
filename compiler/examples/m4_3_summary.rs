use inkwell::context::Context;
use pole_compiler::{parse_ir, codegen::CodeGen};
use std::fs;

fn main() {
    println!("=== M4.3: Variant Type Summary ===\n");
    
    // Test 1: Simple variant parsing
    println!("Test 1: Parsing variants");
    let ir = fs::read_to_string("../examples/12-simple-variant.pole-ir")
        .expect("Failed to read file");
    let program = parse_ir(&ir).expect("Parse failed");
    println!("✓ Parsed Color and Shape variant types");
    
    // Test 2: Variant codegen
    println!("\nTest 2: Variant codegen");
    let context = Context::create();
    let mut codegen = CodeGen::new(&context, "simple_variant");
    codegen.compile_program(&program).expect("Codegen failed");
    let llvm_ir = codegen.print_to_string();
    
    assert!(llvm_ir.contains("define i32 @get_red()"));
    assert!(llvm_ir.contains("ret i32 0"));
    assert!(llvm_ir.contains("define i32 @get_circle()"));
    println!("✓ Generated LLVM functions for variant constructors");
    
    // Test 3: Tag values
    println!("\nTest 3: Variant tag values");
    let ir2 = fs::read_to_string("../examples/13-variant-tags.pole-ir")
        .expect("Failed to read file");
    let program2 = parse_ir(&ir2).expect("Parse failed");
    
    let context2 = Context::create();
    let mut codegen2 = CodeGen::new(&context2, "variant_tags");
    codegen2.compile_program(&program2).expect("Codegen failed");
    let llvm_ir2 = codegen2.print_to_string();
    
    assert!(llvm_ir2.contains("ret i32 0"), "Red should be tag 0");
    assert!(llvm_ir2.contains("ret i32 1"), "Green should be tag 1");
    assert!(llvm_ir2.contains("ret i32 2"), "Blue should be tag 2");
    assert!(llvm_ir2.contains("icmp eq i32 %c, 0"), "Should compare with Red tag");
    println!("✓ Correct tag values: Red=0, Green=1, Blue=2");
    println!("✓ Comparison with variant constructors works");
    
    println!("\n=== M4.3 Complete ===");
    println!("\nImplemented:");
    println!("  - Variant type parsing (TypeDefKind::Variant)");
    println!("  - Variant constructor handling in compile_variable");
    println!("  - Variant type mapping: simple enum -> i32");
    println!("  - Tag value assignment: index in constructor list");
    println!("  - Comparison operations with variant constructors");
    
    println!("\nExamples:");
    println!("  - 12-simple-variant.pole-ir: Basic variant types");
    println!("  - 13-variant-tags.pole-ir: Tag values and comparison");
    
    println!("\nLimitations:");
    println!("  - Only simple enums (no constructor arguments yet)");
    println!("  - No pattern matching on variants yet");
    println!("  - No support for Option/Result with payload");
    
    println!("\nNext: M4.4 - Result and Option types");
}
