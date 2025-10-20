use inkwell::context::Context;
use pole_compiler::{parse_ir, CodeGen, CompilerArenas};
use std::fs;

fn main() {
    let ir = r#"
type Position = { x: Int, y: Int }

type Direction = North | South | East | West

type Player = {
  position: Position,
  health: Int,
  hunger: Int,
  facing: Direction
}

type Tilemap = {
  width: Int,
  height: Int,
  tiles: List<Int>
}

func create_player(x: Int, y: Int) -> Player:
  {
    position: { x: x, y: y },
    health: 100,
    hunger: 100,
    facing: South
  }

func get_tile(tilemap: Tilemap, x: Int, y: Int) -> Int:
  let index = y * tilemap.width + x in
  list_get(tilemap.tiles, index, 0)
"#;
    
    let program = parse_ir(&ir).unwrap();
    let arenas = CompilerArenas::new_default();
    let context = Context::create();
    let mut codegen = CodeGen::new(&context, "test", &arenas.codegen_arena);
    
    match codegen.compile_program(&program) {
        Ok(_) => println!("✓ Success"),
        Err(e) => println!("✗ Failed: {}", e),
    }
}
