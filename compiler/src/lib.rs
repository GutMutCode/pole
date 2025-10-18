// Pole Compiler - Rust Implementation
// Phase 5: Native Compiler with LLVM Backend

pub mod ast;
pub mod ir_parser;

#[cfg(feature = "python")]
pub mod python_bindings;

// Re-exports
pub use ast::*;
pub use ir_parser::parse_ir;
