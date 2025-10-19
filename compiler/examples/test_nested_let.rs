use pole_compiler::parse_ir;

fn main() {
    let ir = r#"
type Point = { x: Int, y: Int }

func test(p: Point) -> Int :
  let x_sq = p.x * p.x in
  let y_sq = p.y * p.y in
  x_sq + y_sq
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
