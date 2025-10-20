use pole_compiler::parse_ir;

fn main() {
    let test_cases = vec![
        ("Single function with list literal", r#"
func test() -> Int :
  let nums = [1, 2, 3] in
  let x = List_get(nums, 1) in
  x
"#),
        ("Two functions - second regular", r#"
func create_list() -> List<Int>:
  [1, 2, 3]

func test() -> Int :
  let nums = create_list() in
  let x = List_get(nums, 1) in
  x
"#),
        ("Two functions - both multi-line", r#"
func create_list() -> List<Int>:
  let a = 1 in
  let b = 2 in
  [a, b, 3]

func test() -> Int :
  let nums = create_list() in
  let x = List_get(nums, 1) in
  x
"#),
        ("Second function only", r#"
func test() -> Int :
  let nums = create_list() in
  let x = List_get(nums, 1) in
  x
"#),
        ("Function named main", r#"
func main() -> Int :
  let nums = create_list() in
  let x = List_get(nums, 1) in
  x
"#),
        ("With list literal", r#"
func main() -> Int :
  let nums = [1, 2, 3] in
  let x = List_get(nums, 1) in
  x
"#),
        ("Extern + regular function", r#"
@extern("puts")
func c_puts(s: String) -> Int

func test() -> Int :
  let nums = [1, 2, 3] in
  let x = List_get(nums, 1) in
  x
"#),
        ("Extern + two regular functions", r#"
@extern("puts")
func c_puts(s: String) -> Int

func create_list() -> List<Int>:
  [1, 2, 3]

func test() -> Int :
  let nums = create_list() in
  let x = List_get(nums, 1) in
  x
"#),
    ];

    for (name, code) in test_cases {
        println!("\n=== Testing: {} ===", name);
        match parse_ir(code) {
            Ok(prog) => {
                println!("✓ Success - {} functions, {} externs",
                    prog.func_defs.len(), prog.extern_funcs.len());
                for func in &prog.func_defs {
                    match &func.body {
                        pole_compiler::ast::Expr::Variable(v) => 
                            println!("  func {} - Variable({})", func.name, v.name),
                        pole_compiler::ast::Expr::Let(_) => 
                            println!("  func {} - Let(...)", func.name),
                        pole_compiler::ast::Expr::Constructor(_) => 
                            println!("  func {} - Constructor(...)", func.name),
                        _ => 
                            println!("  func {} - {:?}", func.name, func.body),
                    }
                }
            }
            Err(e) => {
                println!("✗ Failed: {}", e);
            }
        }
    }
}
