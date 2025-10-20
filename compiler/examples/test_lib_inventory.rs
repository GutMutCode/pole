use inkwell::context::Context;
use pole_compiler::{parse_ir, CodeGen, CompilerArenas};
use std::fs;
use std::path::Path;
use std::process::Command;

fn main() {
    // Concatenate core + inventory + test
    let core = fs::read_to_string("../games/zomboid/lib/core.pole-ir")
        .expect("Failed to read core.pole-ir");
    let inventory = fs::read_to_string("../games/zomboid/lib/inventory.pole-ir")
        .expect("Failed to read inventory.pole-ir");
    
    let test_code = r#"
@extern("puts")
func c_puts(s: String) -> Int

func test_inventory_lib(dummy: Unit) -> Int:
let _ = c_puts("=== Testing Inventory Library ===") in
let inv = inventory_create(10) in
let _ = c_puts("Created 10-slot inventory") in
let _ = inventory_add_item(inv, 10, 1, 5) in
let _ = c_puts("Added item 1 (qty 5)") in
let _ = inventory_add_item(inv, 10, 1, 3) in
let _ = c_puts("Added item 1 (qty 3) - should stack") in
let item_id = inventory_get_item_id(inv, 0) in
let qty = inventory_get_quantity(inv, 0) in
let _ = c_puts("Item in slot 0 found") in
let _ = inventory_consume(inv, 0, 2) in
let _ = c_puts("Consumed 2 units") in
let _ = inventory_free(inv) in
let _ = c_puts("Test complete!") in
0

func main() -> Int:
test_inventory_lib(())
"#;

    let ir_source = format!("{}\n{}\n{}", core, inventory, test_code);

    println!("=== Parsing Pole IR ===");
    let program = match parse_ir(&ir_source) {
        Ok(p) => {
            println!("✓ Parsed successfully");
            p
        }
        Err(e) => {
            eprintln!("✗ Parse failed: {}", e);
            return;
        }
    };

    println!("\n=== Compiling to LLVM IR ===");
    let arenas = CompilerArenas::new_default();
    let context = Context::create();
    let mut codegen = CodeGen::new(&context, "test_lib_inventory", &arenas.codegen_arena);

    if let Err(e) = codegen.compile_program(&program) {
        eprintln!("✗ Compilation failed: {}", e);
        return;
    }

    println!("✓ Compilation successful");

    let ir_path = Path::new("test_lib_inventory.ll");
    codegen
        .write_ir_to_file(ir_path)
        .expect("Failed to write LLVM IR");
    println!("\n✓ Written LLVM IR");

    let obj_path = Path::new("test_lib_inventory.o");
    codegen
        .write_object_file(obj_path)
        .expect("Failed to write object file");
    println!("✓ Written object file");

    println!("\n=== Linking executable ===");
    let runtime_obj = "../runtime/pole_runtime.o";
    
    let link_status = Command::new("cc")
        .args(&["-o", "test_lib_inventory", "test_lib_inventory.o", runtime_obj])
        .status()
        .expect("Failed to execute linker");

    if !link_status.success() {
        eprintln!("✗ Linking failed");
        return;
    }
    println!("✓ Linked executable");

    println!("\n=== Running native executable ===");
    let output = Command::new("./test_lib_inventory")
        .output()
        .expect("Failed to execute");

    println!("{}", String::from_utf8_lossy(&output.stdout));
    
    if !output.status.success() {
        eprintln!("stderr: {}", String::from_utf8_lossy(&output.stderr));
    }

    println!("\n=== Cleanup ===");
    let _ = fs::remove_file("test_lib_inventory.ll");
    let _ = fs::remove_file("test_lib_inventory.o");
    let _ = fs::remove_file("test_lib_inventory");
    println!("✓ Cleaned up");
}
