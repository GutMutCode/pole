// Pole Compiler - Rust Implementation
// Phase 5: Native Compiler with LLVM Backend

pub mod ast;
pub mod ir_parser;
pub mod type_checker;
pub mod python_bindings;
pub mod codegen;
pub mod arena;
pub mod memory;

pub use ast::*;
pub use ir_parser::parse_ir;
pub use type_checker::{check_types, TypeCheckResult, TypeError};
pub use codegen::CodeGen;
pub use arena::CompilerArenas;
pub use memory::{CompileError, MemoryStats};
