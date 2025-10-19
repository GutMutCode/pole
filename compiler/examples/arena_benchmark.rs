use inkwell::context::Context;
use pole_compiler::{parse_ir, CodeGen, CompilerArenas, MemoryStats};
use std::fs;
use std::time::Instant;

fn main() {
    println!("=== Arena Allocator Benchmark ===\n");

    let ir_file = "../examples/01-factorial.pole-ir";
    let ir_source = fs::read_to_string(ir_file)
        .expect("Failed to read factorial IR file");

    let program = parse_ir(&ir_source).expect("Failed to parse IR");

    println!("Benchmark: Compiling factorial 100 times\n");

    let iterations = 100;

    println!("--- With Arena Allocator ---");
    let start = Instant::now();
    let arenas = CompilerArenas::new_default();
    
    for i in 0..iterations {
        let context = Context::create();
        let mut codegen = CodeGen::new(&context, "factorial", &arenas.codegen_arena);
        codegen.compile_program(&program).expect("Compilation failed");
        
        if i == 0 {
            let stats = MemoryStats::new(
                arenas.parse_allocated(),
                arenas.ir_allocated(),
                arenas.codegen_allocated(),
            );
            println!("Memory usage (first iteration): {}", stats.format_human_readable());
        }
    }
    
    let arena_duration = start.elapsed();
    let arena_memory = arenas.total_allocated();
    
    println!("Time: {:?}", arena_duration);
    println!("Total memory allocated: {} bytes ({:.2} KB)", 
             arena_memory, arena_memory as f64 / 1024.0);
    println!("Average time per compilation: {:?}", arena_duration / iterations);

    println!("\n=== Summary ===");
    println!("Arena allocator:");
    println!("  - Total time: {:?}", arena_duration);
    println!("  - Memory allocated: {:.2} KB", arena_memory as f64 / 1024.0);
    println!("  - Avg per iteration: {:?}", arena_duration / iterations);
    
    println!("\n✓ Arena allocator benchmark completed!");
}
