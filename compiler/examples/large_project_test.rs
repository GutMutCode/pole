use pole_compiler::{parse_ir, CodeGen, CompilerArenas};
use inkwell::context::Context;

fn main() {
    println!("=== Large Project Memory Test ===\n");
    
    // Generate a large IR file with many functions
    let mut ir_code = String::new();
    
    // Add 100 functions
    for i in 0..100 {
        ir_code.push_str(&format!(
            "func test_func_{i}(x: Int) -> Int :\n  \
             let a = x + {i} in\n  \
             let b = a * 2 in\n  \
             let c = b - 1 in\n  \
             if c > 10 then c else {i}\n\n",
            i = i
        ));
    }
    
    println!("Generated IR: {} KB", ir_code.len() / 1024);
    println!("Functions: 100\n");
    
    let before = get_memory_usage();
    println!("Before parsing: {} KB", before);
    
    let program = parse_ir(&ir_code).expect("Parse failed");
    let after_parse = get_memory_usage();
    println!("After parsing:  {} KB (delta: {} KB)", after_parse, after_parse - before);
    
    let arenas = CompilerArenas::new_default();
    let context = Context::create();
    let mut codegen = CodeGen::new(&context, "large_test", &arenas.codegen_arena);
    
    codegen.compile_program(&program).expect("Compile failed");
    let after_compile = get_memory_usage();
    println!("After compile:  {} KB (delta: {} KB)", after_compile, after_compile - after_parse);
    
    println!("\nTotal memory: {} KB ({} MB)", 
             after_compile - before,
             (after_compile - before) / 1024);
}

fn get_memory_usage() -> usize {
    if let Ok(status) = std::fs::read_to_string("/proc/self/status") {
        for line in status.lines() {
            if line.starts_with("VmRSS:") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 2 {
                    return parts[1].parse().unwrap_or(0);
                }
            }
        }
    }
    0
}
