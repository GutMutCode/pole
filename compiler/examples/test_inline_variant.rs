use pole_compiler::parse_ir;

fn main() {
    let ir1 = r#"
type Color =
  | Red
  | Green

func test() -> Color:
  Red
"#;

    let ir2 = r#"
type State = Idle | Active

func test() -> State:
  Idle
"#;

    println!("Testing multiline variant syntax:");
    match parse_ir(ir1) {
        Ok(p) => {
            println!("✓ Parse OK");
            if let Some(td) = p.type_defs.first() {
                println!("  Type: {} = {:?}", td.name, td.definition);
            }
        },
        Err(e) => println!("✗ Parse failed: {}", e),
    }

    println!("\nTesting inline variant syntax:");
    match parse_ir(ir2) {
        Ok(p) => {
            println!("✓ Parse OK");
            if let Some(td) = p.type_defs.first() {
                println!("  Type: {} = {:?}", td.name, td.definition);
            }
        },
        Err(e) => println!("✗ Parse failed: {}", e),
    }
}
