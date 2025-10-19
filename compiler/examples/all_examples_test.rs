use inkwell::context::Context;
use pole_compiler::{parse_ir, codegen::CodeGen, CompilerArenas};
use std::fs;

fn main() {
    println!("=== Testing All Pole IR Examples ===\n");
    
    let examples = vec![
        "01-factorial.pole-ir",
        "02-fibonacci.pole-ir",
        // "03-user-validation.pole-ir",  // Skipped: requires runtime functions
        "04-simple-math.pole-ir",
        "05-is-even.pole-ir",
        "07-max.pole-ir",
        "08-simple-record.pole-ir",
        "09-simple-string.pole-ir",
        "10-string-literal.pole-ir",
        "11-simple-list.pole-ir",
        "12-simple-variant.pole-ir",
        "13-variant-tags.pole-ir",
        "15-simple-option.pole-ir",
        "16-option-match.pole-ir",
        "17-unit-type.pole-ir",
    ];
    
    let mut passed = 0;
    let mut failed = 0;
    
    for example in &examples {
        let path = format!("../examples/{}", example);
        print!("Testing {}: ", example);
        
        match fs::read_to_string(&path) {
            Ok(ir) => {
                match parse_ir(&ir) {
                    Ok(program) => {
                        let arenas = CompilerArenas::new_default();
                        let context = Context::create();
                        let mut codegen = CodeGen::new(&context, "test", &arenas.codegen_arena);
                        
                        match codegen.compile_program(&program) {
                            Ok(()) => {
                                println!("✓ PASS");
                                passed += 1;
                            }
                            Err(e) => {
                                println!("✗ FAIL (codegen: {})", e);
                                failed += 1;
                            }
                        }
                    }
                    Err(e) => {
                        println!("✗ FAIL (parse: {})", e);
                        failed += 1;
                    }
                }
            }
            Err(e) => {
                println!("✗ FAIL (read: {})", e);
                failed += 1;
            }
        }
    }
    
    println!("\n=== Summary ===");
    println!("Total: {}", examples.len());
    println!("Passed: {}", passed);
    println!("Failed: {}", failed);
    println!("Success rate: {:.1}%", (passed as f64 / examples.len() as f64) * 100.0);
    
    if failed > 0 {
        std::process::exit(1);
    }
}
