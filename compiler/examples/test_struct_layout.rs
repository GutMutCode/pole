use pole_compiler::{parse_ir, CodeGen, CompilerArenas};
use inkwell::context::Context;

fn main() {
    let ir_code = r#"
type Point = { x: Int, y: Int }

type Vertex = {
  x: Int,
  y: Int,
  z: Int
}

func test_point(p: Point) -> Int :
  p.x + p.y

func test_vertex(v: Vertex) -> Int :
  v.x + v.y + v.z
"#;

    println!("=== Struct Layout Test ===\n");

    let program = parse_ir(ir_code).expect("Parse failed");
    println!("✓ Parsed types: {}", program.type_defs.len());

    let arenas = CompilerArenas::new_default();
    let context = Context::create();
    let mut codegen = CodeGen::new(&context, "struct_test", &arenas.codegen_arena);

    codegen.compile_program(&program).expect("Compile failed");
    println!("✓ Compiled successfully");

    // Print LLVM IR to see struct layout
    let llvm_ir = codegen.get_module().print_to_string().to_string();
    
    println!("\n=== LLVM Struct Types ===");
    for line in llvm_ir.lines() {
        if line.contains("%Point =") || line.contains("%Vertex =") {
            println!("{}", line.trim());
        }
    }
    
    println!("\n=== Function Signatures ===");
    for line in llvm_ir.lines() {
        if line.contains("define") && (line.contains("test_point") || line.contains("test_vertex")) {
            println!("{}", line.trim());
        }
    }
    
    // Check size and alignment (conceptually)
    println!("\n=== Expected Sizes ===");
    println!("Point:  2 * f64 = 16 bytes");
    println!("Vertex: 6 * f64 = 48 bytes");
    println!("\nNote: LLVM IR shows these as literal struct types");
}
