fn main() {
    let code = r#"func create_list() -> List<Int>:
  [1, 2, 3]

func test() -> Int :
  let nums = create_list() in
  let x = List_get(nums, 1) in
  x
"#;

    println!("Input:");
    println!("{:?}", code);
    println!("\nInput bytes:");
    for (i, line) in code.lines().enumerate() {
        println!("Line {}: {:?}", i, line);
        for (j, byte) in line.bytes().enumerate() {
            print!("{:02x} ", byte);
            if (j + 1) % 16 == 0 {
                println!();
            }
        }
        println!();
    }
    
    match pole_compiler::parse_ir(code) {
        Ok(prog) => {
            println!("\n✓ Parsed {} functions", prog.func_defs.len());
            for (i, func) in prog.func_defs.iter().enumerate() {
                println!("\nFunction {}: {}", i, func.name);
                println!("Body: {:#?}", func.body);
            }
        }
        Err(e) => {
            println!("\n✗ Parse error: {}", e);
        }
    }
}
