use pole_compiler::parse_ir;

fn main() {
    let ir = r#"
func test_let(x: Int) -> Int :
  let y = x + 1 in y
"#;

    match parse_ir(ir) {
        Ok(program) => {
            println!("✓ Parsing successful");
            for func in &program.func_defs {
                println!("Function: {}", func.name);
                println!("Body: {:?}", func.body);
            }
        }
        Err(e) => {
            eprintln!("✗ Parsing failed: {}", e);
        }
    }
}
