// Pole Compiler - Rust Implementation
// Phase 5: Native Compiler with LLVM Backend

pub mod ast;
pub mod ir_parser;
pub mod python_bindings;

pub use ast::*;
pub use ir_parser::parse_ir;
