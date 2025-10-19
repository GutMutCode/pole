use inkwell::context::Context;
use pole_compiler::{parse_ir, CodeGen, CompilerArenas};
use std::fs;

fn main() {
    println!("=== Testing user-validation Example ===\n");

    // Test individual validation functions first
    let ir_source = r#"
type ValidationError =
  | NameEmpty
  | NameTooLong
  | InvalidEmail
  | InvalidAge

func validate_name(name: String) -> List<ValidationError> :
  let len = String_length(name) in
  if len < 1 then
    [NameEmpty]
  else if len > 50 then
    [NameTooLong]
  else
    []

func validate_email(email: String) -> List<ValidationError> :
  if String_contains(email, "@") then
    []
  else
    [InvalidEmail]

func test_valid_name() -> List<ValidationError> :
  validate_name("John Doe")

func test_empty_name() -> List<ValidationError> :
  validate_name("")

func test_valid_email() -> List<ValidationError> :
  validate_email("john@example.com")

func test_invalid_email() -> List<ValidationError> :
  validate_email("invalid-email")
"#;

    println!("Parsing user-validation IR...");
    match parse_ir(ir_source) {
        Ok(program) => {
            println!("✓ Parsed successfully\n");
            println!("Program structure:");
            println!("  - Type definitions: {}", program.type_defs.len());
            println!("  - Functions: {}", program.func_defs.len());
            
            for func in &program.func_defs {
                println!("    - {}", func.name);
            }

            println!("\nCompiling to LLVM IR...");
            let arenas = CompilerArenas::new_default();
            let context = Context::create();
            let mut codegen = CodeGen::new(&context, "user_validation", &arenas.codegen_arena);

            match codegen.compile_program(&program) {
                Ok(_) => {
                    println!("✓ Compilation successful!\n");
                    println!("user-validation example validation functions compiled successfully.");
                    println!("\nNote: Full integration test requires List.concat implementation.");
                }
                Err(e) => {
                    println!("✗ Compilation failed: {}\n", e);
                    std::process::exit(1);
                }
            }
        }
        Err(e) => {
            println!("✗ Parse failed: {:?}\n", e);
            std::process::exit(1);
        }
    }

    println!("\n✓ user-validation partial test passed!");
}
