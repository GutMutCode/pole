use pole_compiler::parse_ir;
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

func create_player(x: Int, y: Int) -> Player:
  {
    position: { x: x, y: y },
    health: 100,
    hunger: 100,
    facing: South
  }
"#;
    
    match parse_ir(ir) {
        Ok(program) => {
            println!("Parse OK!");
            if let Some(func) = program.func_defs.first() {
                println!("Function: {}", func.name);
                println!("Body: {:#?}", func.body);
            }
        }
        Err(e) => println!("Parse failed: {}", e),
    }
}
