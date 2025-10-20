use inkwell::context::Context;
use pole_compiler::{parse_ir, CodeGen, CompilerArenas};
use std::fs;
use std::path::Path;
use std::process::Command;

fn main() {
    println!("=== Testing Simple Game Demo ===\n");
    
    let ir_code = fs::read_to_string("../examples/72-simple-game.pole-ir")
        .expect("Failed to read file");
    
    println!("Parsing IR...");
    let program = match parse_ir(&ir_code) {
        Ok(p) => {
            println!("✓ Parse successful!");
            println!("  Functions: {}", p.func_defs.len());
            println!("  Externs: {}", p.extern_funcs.len());
            p
        },
        Err(e) => {
            println!("✗ Parse failed: {}", e);
            return;
        }
    };
    
    println!("\nCompiling to LLVM...");
    let arenas = CompilerArenas::new_default();
    let context = Context::create();
    let mut codegen = CodeGen::new(&context, "simple_game", &arenas.codegen_arena);
    
    match codegen.compile_program(&program) {
        Ok(_) => {
            println!("✓ Compilation successful!");
            
            println!("\nGenerating object file...");
            let obj_path = Path::new("/tmp/simple_game.o");
            match codegen.write_object_file(obj_path) {
                Ok(_) => {
                    println!("✓ Object file written");
                    
                    println!("\nLinking with SDL2...");
                    let output = Command::new("gcc")
                        .arg(obj_path)
                        .arg("-o")
                        .arg("/tmp/simple_game")
                        .arg("-lSDL2")
                        .output();
                    
                    match output {
                        Ok(result) => {
                            if result.status.success() {
                                println!("✓ Linked successfully: /tmp/simple_game");
                                
                                println!("\nRunning game demo...");
                                let run_result = Command::new("/tmp/simple_game")
                                    .env("SDL_VIDEODRIVER", "dummy")
                                    .output();
                                
                                match run_result {
                                    Ok(run_output) => {
                                        let exit_code = run_output.status.code().unwrap_or(-1);
                                        if exit_code == 0 {
                                            println!("✓ Game ran successfully! Exit code: {}", exit_code);
                                            println!("\n🎮 GAME DEMO WORKS!");
                                        } else {
                                            println!("✗ Game failed with exit code: {}", exit_code);
                                        }
                                    }
                                    Err(e) => println!("✗ Failed to run game: {}", e),
                                }
                            } else {
                                println!("✗ Linking failed:");
                                println!("{}", String::from_utf8_lossy(&result.stderr));
                            }
                        }
                        Err(e) => println!("✗ Failed to run gcc: {}", e),
                    }
                }
                Err(e) => println!("✗ Failed to write object file: {}", e),
            }
        }
        Err(e) => println!("✗ Compilation failed: {}", e),
    }
}
