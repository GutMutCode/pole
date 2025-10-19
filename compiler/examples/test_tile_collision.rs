use inkwell::context::Context;
use inkwell::execution_engine::{ExecutionEngine, JitFunction};
use inkwell::OptimizationLevel;
use pole_compiler::{parse_ir, CodeGen, CompilerArenas};

type MainFunc = unsafe extern "C" fn() -> i64;

fn main() {
    let ir_code = std::fs::read_to_string("../examples/32-tile-collision.pole-ir")
        .expect("Failed to read IR file");

    println!("=== Tile Collision Detection Test ===\n");
    
    let program = parse_ir(&ir_code).expect("Failed to parse IR");
    println!("✓ Parsed: {} functions, {} types", program.func_defs.len(), program.type_defs.len());
    
    let arenas = CompilerArenas::new_default();
    let context = Context::create();
    let mut codegen = CodeGen::new(&context, "tile_collision", &arenas.codegen_arena);

    codegen.compile_program(&program).expect("Compilation failed");
    println!("✓ LLVM IR generated");
    
    let execution_engine = codegen.get_module()
        .create_jit_execution_engine(OptimizationLevel::None)
        .expect("Failed to create execution engine");
    
    println!("\n=== Running tests ===\n");
    
    unsafe {
        let main: JitFunction<MainFunc> = execution_engine
            .get_function("main")
            .expect("Failed to get main function");
        
        let result = main.call();
        
        println!("Test result: {}", result);
        
        if result == 0 {
            println!("\n✓✓✓ All tests passed! ✓✓✓");
            println!("\nTile collision detection verified:");
            println!("  ✓ Point in tile center detected");
            println!("  ✓ Point outside tile rejected");
            println!("  ✓ find_tile_at_point works correctly");
        } else {
            println!("\n✗ Test {} failed", result);
        }
    }
}
