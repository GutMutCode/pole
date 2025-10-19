use pole_compiler::parse_ir;

fn main() {
    let ir = r#"
type Point = { x: Int, y: Int }

func make_point(x: Int, y: Int) -> Point :
  { x = x, y = y }
"#;

    match parse_ir(ir) {
        Ok(program) => {
            println!("✓ Parsing successful");
            for func in &program.func_defs {
                println!("Function: {}", func.name);
                println!("Body: {:#?}", func.body);
            }
        }
        Err(e) => {
            eprintln!("✗ Parsing failed: {}", e);
        }
    }
}
