use pole_compiler::parse_ir;
use std::fs;

fn main() {
    let ir = fs::read_to_string("../examples/40-list-get-test.pole-ir").unwrap();
    match parse_ir(&ir) {
        Ok(prog) => {
            println!("Parsed {} functions:", prog.func_defs.len());
            for f in &prog.func_defs {
                println!("  - {}", f.name);
            }
        }
        Err(e) => {
            eprintln!("Parse error: {}", e);
        }
    }
}
