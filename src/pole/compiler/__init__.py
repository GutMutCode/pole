"""
Pole Compiler Module

Phase 5: Native Compiler & High-Performance Runtime

The actual compiler is implemented in Rust (see `compiler/` directory).
This Python module provides bindings via PyO3.

Components:
- LLVM Backend (Pole IR → LLVM IR → Native Code) [Rust]
- Memory Management System [Rust]
- Performance Optimization [Rust]
- Python Bindings [This module]

For development, see compiler/README.md
"""

from pole.compiler.bindings import compile_to_native

__all__ = ["compile_to_native"]
