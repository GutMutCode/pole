use pole_compiler::parse_ir;
use std::fs;

fn main() {
    let ir_code = fs::read_to_string("../games/zomboid/main.pole-ir")
        .expect("Failed to read file");
    
    match parse_ir(&ir_code) {
        Ok(program) => {
            println!("✓ Parse OK");
            for (i, func) in program.func_defs.iter().enumerate() {
                println!("Function {}: {} - params: {}, body expr: {:?}", 
                    i, 
                    func.name,
                    func.params.len(),
                    match &func.body {
                        pole_compiler::ast::Expr::Record(_) => "Record",
                        pole_compiler::ast::Expr::Let(_) => "Let",
                        pole_compiler::ast::Expr::If(_) => "If",
                        pole_compiler::ast::Expr::BinaryOp(_) => "BinaryOp",
                        pole_compiler::ast::Expr::Application(_) => "Application",
                        _ => "Other",
                    }
                );
            }
        }
        Err(e) => println!("✗ Parse failed: {}", e),
    }
}
