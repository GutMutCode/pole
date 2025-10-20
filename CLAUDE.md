# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

**Pole** is an LLM-native programming language system where:
- Humans write **specification language** (.pole files) expressing "what" they want
- LLM transforms specs into **implementation language** (Pole IR, .pole-ir files) defining "how"
- **Rust compiler** compiles IR to native code via LLVM backend

This is NOT a transpiler to existing languages - Pole IR is a new language we're designing, compiled to native machine code.

## Architecture

```
[Human] .pole (spec language - natural, intent-focused)
   ↓ LLM transformation
[LLM] .pole-ir (implementation language - formal, type-safe)
   ↓ Rust compiler (LLVM backend)
[Native] machine code (x86_64, ARM64, etc.)
```

**Hybrid Implementation:**
- **Python layer**: CLI, spec parser/validator, LLM integration (maintained)
- **Rust layer**: IR parser, type checker, LLVM compiler (performance-critical)
- **Integration**: PyO3 bindings connect Python and Rust

## Essential Commands

### Development Workflow

```bash
# 1. Check specification file (.pole)
pole check examples/01-factorial.pole

# 2. Generate IR from spec (requires OPENROUTER_API_KEY)
pole build examples/01-factorial.pole --output output.pole-ir
# OR use mock LLM (no API needed)
pole build examples/01-factorial.pole --mock

# 3. Run IR function
pole run examples/01-factorial.pole-ir factorial 5

# 4. Test all @test_case annotations in IR
pole test examples/01-factorial.pole-ir

# 5. Compile IR to native code (LLVM backend)
cd compiler && cargo run --release --bin polec -- ../examples/01-factorial.pole-ir -o factorial
./factorial  # Run native binary
```

### Development Environment

**NixOS users:**
```bash
direnv allow  # Auto-activate environment
# OR
nix-shell     # Manual activation
```

**Other systems:**
```bash
export PYTHONPATH=src
alias pole="python -m pole.cli.main"

# Rust compiler
cd compiler && cargo build --release
```

### Testing

```bash
# Python tests
pytest tests/ -v

# Rust tests
cd compiler && cargo test

# Integration tests (all examples)
./test_all_examples.py

# Benchmarks (Rust)
cd compiler && cargo bench
```

### Verification

```bash
# Before committing, verify tools are available
pole --version
rustc --version
sdl2-config --version

# Python code quality (optional)
make lint     # ruff check
make format   # black + ruff fix
make typecheck # mypy
```

## Code Structure

### Critical Python Components (src/pole/)
- **cli/main.py**: Command-line interface (check, build, run, test)
- **parser/spec_parser.py**: Parse .pole specification files
- **validator/spec_validator.py**: Validate specs for completeness/ambiguity
- **transformer/llm_transformer.py**: LLM integration for spec→IR transformation
- **transformer/llm_client.py**: OpenRouter API client + Mock client
- **runtime/interpreter.py**: IR interpreter (fallback execution)
- **verifier/example_tester.py**: Test runner for @test_case annotations

### Critical Rust Components (compiler/src/)
- **ir_parser.rs**: Parse .pole-ir files (23.4x faster than Python)
- **type_checker.rs**: Type checking and inference (25.6x faster)
- **codegen.rs**: LLVM IR generation and native compilation
- **ast.rs**: IR abstract syntax tree definitions
- **python_bindings.rs**: PyO3 bindings for Python integration
- **memory.rs**: Memory management (arena allocator)

### Key Directories
- **examples/**: Specification (.pole) and IR (.pole-ir) example files
- **specs/**: Language specifications (syntax-v0.md, ir-syntax.md, ffi.md, etc.)
- **tests/**: Python integration tests
- **compiler/tests/**: Rust unit tests
- **pole_engine/**: Reusable 2D game engine modules (render, input, physics, AI, network)
- **games/zomboid/**: Project Zomboid clone game (Pole language showcase)

## Language Design Principles

### Specification Language (.pole) - Human-Written
- Natural language friendly, intent-focused expressions
- Ambiguity allowed (LLM resolves)
- Minimal syntax enforcement
- Example-driven (input→output examples)
- Domain-specific terminology encouraged

### Implementation Language (.pole-ir) - LLM-Generated
- Formal semantics, zero ambiguity
- Static type system (Nat, Int, Bool, String, List<T>, Record, Option<T>, Ptr<T>)
- Verifiable contracts (@requires, @ensures)
- Pattern matching (match/with syntax)
- FFI support (@extern, @variadic for C interop)

## Current Status (Phase 6.1 Complete)

**✅ Working Features:**
- Spec parsing, validation, LLM transformation
- Rust IR parser/type checker (20x+ faster than Python)
- LLVM native compilation (100x+ faster than interpreter)
- FFI system (SDL2 integration, C function calls)
- Multi-argument functions, advanced types (Record, List, Option, Ptr<T>)
- 24+ example programs working

**🚀 Current Phase: Phase 7 - Game Development**
- Building Pole Zomboid (Project Zomboid clone)
- Extracting reusable patterns into Pole Engine
- Iterative language improvements based on real-world usage

## 🎯 Current Priority (Week 1, 2025-10-20)

**Active Phase:** Week 1 - Pole Zomboid Demo  
**Goal:** 1-minute playable demo by 2025-10-26

### Today's Task (Day 2 - Tuesday)
1. ⭐ **P0** Write `games/zomboid/specs/zombie.pole` specification
2. **P0** Generate IR with LLM (`pole build zombie.pole`)
3. **P0** Test player + zombie integration

**Detailed Plan:** See [docs/WEEK1_PLAN.md](docs/WEEK1_PLAN.md)

### Priority Rules

**Hierarchy:** Week Plan > P0 > P1 > P2

- **P0**: Critical - Must complete this week
- **P1**: Important - After P0 completion
- **P2**: Optional - Time permitting

**Before starting work:**
1. ✅ Check this file (CLAUDE.md) for current priority
2. ✅ Read related guide (WEEK1_PLAN.md)
3. ✅ Confirm with user if unclear

## Important Conventions

### File Extensions
- `.pole` = Specification language (human-written)
- `.pole-ir` = Implementation language (LLM-generated, formal)
- Never confuse these two! They serve different purposes.

### LLM Workflow
1. Human writes .pole spec (intentionally incomplete/ambiguous is OK)
2. `pole check` validates spec, detects ambiguities
3. `pole build` sends spec to LLM → generates .pole-ir
4. IR is type-checked, contract-verified, tested
5. If errors, LLM may regenerate or request clarification

### Python vs Rust Decision
- **Keep in Python**: User-facing tools (CLI, LLM API, spec parsing)
- **Move to Rust**: Performance-critical components (IR parsing, type checking, compilation)
- Use PyO3 bindings for seamless integration

### Testing Strategy
- Every .pole-ir file should have @test_case annotations
- `pole test` automatically runs all test cases
- Rust compiler has separate test suite (cargo test)

## Development Tips

### Modifying the Compiler
1. Rust changes go in `compiler/src/`
2. Update corresponding Python bindings if needed (`compiler/src/python_bindings.rs`)
3. Rebuild: `cd compiler && cargo build --release`
4. Test: `cargo test && pytest tests/`

### Adding Language Features
1. Update language spec (specs/ir-syntax.md)
2. Implement in Rust compiler (ast.rs, ir_parser.rs, codegen.rs)
3. Add Python wrapper if CLI needs access
4. Create example in examples/
5. Document in CHANGELOG.md

### Working with LLM Integration
- Set `OPENROUTER_API_KEY` environment variable
- Use `--mock` flag during development to avoid API costs
- LLM prompts are in `src/pole/transformer/`
- Improve prompts incrementally based on output quality

### Performance Considerations
- Rust compiler is 20-100x faster than Python for critical paths
- Use Rust for hot loops, data structures, algorithms
- Python is fine for I/O, API calls, high-level orchestration

## Common Issues

### "pole: command not found"
- NixOS: Run `direnv allow` or `nix-shell`
- Others: Set `export PYTHONPATH=src` and `alias pole="python -m pole.cli.main"`

### "LLVM not found" (Rust compiler)
- Install LLVM 17.0+: `apt install llvm-17-dev` or equivalent
- Verify: `llvm-config --version`

### "SDL2 not found" (FFI examples)
- Install SDL2 development libraries: `apt install libsdl2-dev`
- Verify: `sdl2-config --version`

### Type errors in generated IR
- LLM sometimes generates incorrect types
- Manually fix .pole-ir file or regenerate with better prompt
- Report patterns to improve transformer prompts

## Documentation Strategy

**Core docs** (repository root):
- README.md: Project intro, design principles
- ARCHITECTURE.md: System design, component structure
- DEVELOPMENT.md: Developer index, guides
- ROADMAP.md: Development timeline, current phase

**Detailed guides** (docs/guides/):
- LANGUAGE_DEV.md: Language development workflow
- ENGINE_DEV.md: Game engine development
- GAME_DEV.md: Game development with Pole
- llm/: LLM usage guides (4 files)

**Language specs** (specs/):
- syntax-v0.md: Specification language grammar
- ir-syntax.md: Implementation language grammar
- ffi.md: Foreign Function Interface
- verification.md: Contract and type checking

**Progress reports** (docs/reports/):
- Milestone completion reports
- Bug fix documentation
- Performance benchmarks

## Related Projects

- **Pole Engine**: 2D game engine modules (pole_engine/)
- **Pole Zomboid**: Project Zomboid clone game (games/zomboid/)
- These are developed alongside the language to drive real-world requirements

## Key Files to Read First

1. README.md - Understand the vision
2. ARCHITECTURE.md - System design
3. examples/01-factorial.pole + .pole-ir - See the workflow
4. specs/ir-syntax.md - Learn IR grammar
5. compiler/src/codegen.rs - LLVM backend implementation

## License

MIT License - See LICENSE file
