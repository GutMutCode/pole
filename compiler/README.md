# Pole Compiler (Rust)

> Phase 5: Native Compiler with LLVM Backend

## Overview

Rust implementation of the Pole language compiler using LLVM for native code generation.

## Requirements

- Rust 1.75+ (2024 Edition)
- LLVM 17.0+
- Python 3.11+ (for bindings)

## Build

```bash
# Build Rust library
cargo build --release

# Build Python bindings
pip install maturin
maturin develop
```

## Development

See [../DEVELOPMENT.md](../DEVELOPMENT.md) for complete setup instructions.

## Architecture

```
compiler/
├── src/
│   ├── lib.rs          # Library root & Python bindings
│   ├── ir_to_llvm.rs   # Pole IR → LLVM IR transformation
│   ├── codegen.rs      # Code generation
│   ├── optimization.rs # Optimization passes
│   └── memory/         # Memory management
│       ├── mod.rs
│       ├── gc.rs       # Garbage collection (RC)
│       └── allocator.rs # Custom allocators
├── tests/              # Integration tests
└── benches/            # Benchmarks
```

## Phase 5.1 Milestones

- **M0**: Rust learning & LLVM setup (3 months)
- **M1**: Basic function compilation (3 months)
- **M2**: Control flow (2 months)
- **M3**: Recursive functions (2 months)
- **M4**: Full example compilation (2 months)

See [../ROADMAP.md](../ROADMAP.md) for details.
