use pole_compiler::parse_ir;
use std::fs;

fn main() {
    let ir = fs::read_to_string("/tmp/test_or.pole-ir").unwrap();
    
    match parse_ir(&ir) {
        Ok(program) => {
            if let Some(func) = program.func_defs.first() {
                println!("Function body:");
                println!("{:#?}", func.body);
            }
        }
        Err(e) => println!("Parse error: {}", e),
    }
}
