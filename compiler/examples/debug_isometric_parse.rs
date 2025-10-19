use pole_compiler::parse_ir;

fn main() {
    let ir_code = std::fs::read_to_string("../examples/27-isometric-simple.pole-ir")
        .expect("Failed to read IR");
    
    let program = parse_ir(&ir_code).expect("Parse failed");
    
    println!("=== Parsed Program ===");
    println!("Type defs: {}", program.type_defs.len());
    println!("Extern funcs: {}", program.extern_funcs.len());
    println!("Functions: {}", program.func_defs.len());
    
    for func in &program.func_defs {
        println!("\n=== Function: {} ===", func.name);
        println!("Params: {:?}", func.params);
        println!("Return type: {:?}", func.return_type);
        println!("Body: {:#?}", func.body);
    }
}
