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

## 🔍 Development Checklist (MUST FOLLOW)

**IMPORTANT:** When starting development work, you MUST create a TODO list with these steps.
Use the TodoWrite tool to track progress through each step.

### Mandatory TODO Template

When user says "진행해줘" or "개발 진행해", you MUST:

1. **Create TODO list** with TodoWrite tool:
   ```
   - Check dependencies (previous day's work)
   - Read syntax examples (03-user-validation.pole, 08-simple-record.pole-ir)
   - Test pole CLI tools
   - Write .pole specification
   - Validate with pole check
   - Generate IR with pole build (or write manually if LLM fails)
   - Verify with Rust parser
   - Run test cases
   - Commit only if all tests pass
   ```

2. **Mark each TODO as in_progress** before starting
3. **Mark as completed** immediately after finishing
4. **Never skip steps** - complete in order

### Before Writing Code

**1. Check Dependencies:**
```bash
# If Day N task depends on Day N-1:
ls games/zomboid/specs/player.pole  # Does previous work exist?
```

**2. Verify Syntax:**
```bash
# Read example files first
cat examples/03-user-validation.pole  # For .pole syntax
cat examples/08-simple-record.pole-ir  # For .pole-ir syntax
cat specs/syntax-v0.md  # For grammar rules
```

**3. Test Tools:**
```bash
pole --version  # Verify pole CLI works
pole check examples/01-factorial.pole  # Test basic functionality
```

### While Writing Code

**4. Follow Examples:**
- `.pole` files: Use `type Name:` with `fields:` (NOT `type Name = {...}`)
- `.pole-ir` files: Use `type Name = {...}` for records
- Enum types: Comment in `.pole`, implement in `.pole-ir`

**5. Incremental Validation:**
```bash
pole check file.pole  # After writing .pole
pole build file.pole  # Generate .pole-ir
# If LLM fails: Check examples, improve prompt, or write manually
```

### After Writing Code

**6. Multi-level Testing:**
```bash
# Level 1: Python parser (quick check)
pole check file.pole-ir

# Level 2: Rust parser (authoritative)
cd compiler && cargo run --release --bin polec -- ../file.pole-ir

# Level 3: Test cases
pole test file.pole-ir

# Level 4: Integration
./test_all_examples.py  # If adding to examples/
```

**7. Commit Only If:**
- ✅ Rust parser validates successfully
- ✅ All test cases pass
- ✅ No TODO/FIXME comments without issue tracking

### Example Workflow

```
User: "진행해줘"

LLM: 
1. Creates TODO list with TodoWrite
2. Marks "Check dependencies" as in_progress
3. Runs: ls games/zomboid/specs/player.pole
4. Marks completed, moves to next TODO
5. Marks "Read syntax examples" as in_progress
6. Runs: cat examples/03-user-validation.pole
7. ... continues through all steps
8. Only commits when ALL TODOs are completed
```

## 🔧 Autonomous Error Recovery (CRITICAL)

When errors occur during workflow, LLM MUST autonomously resolve them without user intervention.

### Error Type 1: Dependency Missing

**Detection:**
```bash
ls games/zomboid/specs/player.pole
# Error: file not found
```

**Autonomous Resolution:**
1. **Analyze:** Day N depends on Day N-1
2. **Decide:** Switch to Day N-1 task first
3. **Update TODO:** Add Day N-1 tasks to beginning of list
4. **Execute:** Complete Day N-1
5. **Resume:** Return to Day N tasks

**Example:**
```
[TodoWrite] Add urgent tasks:
- Write player.pole (Day 1 - P0)
- Generate player.pole-ir (Day 1 - P0)
- Continue with zombie.pole (Day 2 - P0)
```

### Error Type 2: Syntax Error

**Detection:**
```bash
pole check file.pole
# Error: Invalid type definition at line 10
```

**Autonomous Resolution:**
1. **Read error message** - Identify exact problem
2. **Re-read examples** - Study correct syntax
3. **Compare** - Find difference between example and your code
4. **Fix** - Edit file with correct syntax
5. **Retry** - Run pole check again (max 3 attempts)
6. **Escalate** - If 3 failures, ask user for guidance

**Example:**
```
Error: type Position = {...}  (wrong)
Example: type Position: fields: ... (correct)

[Edit] Fix syntax error
[Bash] pole check file.pole
✅ Success - continue
```

### Error Type 3: IR Generation Failed

**Detection:**
```bash
pole build file.pole
# Error: LLM generated invalid IR / Type check failed
```

**Autonomous Resolution:**
1. **Attempt 1:** Improve LLM prompt with more examples
2. **Attempt 2:** Use --mock flag for template
3. **Attempt 3:** Write IR manually following examples/08-simple-record.pole-ir
4. **Verify:** Run pole check on generated IR
5. **Success:** Continue to next step

**Example:**
```
Attempt 1: pole build file.pole
❌ Failed

Attempt 2: [Read] examples/08-simple-record.pole-ir
[Write] file.pole-ir (manual)
[Bash] pole check file.pole-ir
✅ Success - continue
```

### Error Type 4: Rust Parser Error

**Detection:**
```bash
cargo run --release --bin polec -- ../file.pole-ir
# Error: Invalid match expression syntax
```

**Autonomous Resolution:**
1. **Read error line number**
2. **Re-read IR examples** - Study correct match syntax
3. **Fix syntax** - Edit file.pole-ir
4. **Retry verification** (max 3 attempts)

### Error Type 5: Test Failures

**Detection:**
```bash
pole test file.pole-ir
# ✗ test_create_player failed
```

**Autonomous Resolution:**
1. **Analyze test failure** - Read expected vs actual
2. **Fix logic** - Edit function implementation
3. **Retry test**
4. **If persistent:** Review spec for correctness

### Priority Re-evaluation Rules

**When to change priority:**

1. **Blocking Dependency Missing (P0 → URGENT)**
   ```
   Current: Day 2 zombie.pole (P0)
   Blocker: Day 1 player.pole not found
   Action: Switch to Day 1 player.pole (URGENT)
   ```

2. **Critical Bug in Previous Work (P1 → P0)**
   ```
   Current: Day 3 rendering (P1)
   Issue: Day 2 zombie.pole has type error
   Action: Fix Day 2 first, then resume Day 3
   ```

3. **Tool/Compiler Broken (Any → URGENT)**
   ```
   Current: Any task
   Issue: pole CLI not working
   Action: Fix tool first, then resume
   ```

### Autonomous Decision Framework

```
FOR each error:
  1. Classify error type (1-5 above)
  2. Select resolution strategy
  3. Execute fix attempts (max 3)
  4. IF successful: Continue workflow
  5. ELSE IF max attempts: 
     - Update TODO with "blocked by X"
     - Switch to alternative task OR ask user
  6. Log resolution in TODO notes
```

### Example: Complete Autonomous Recovery

```
Step 5: pole check zombie.pole
❌ Error: Invalid type definition at line 15

LLM Autonomous Action:
1. [Read] Error message - "type ZombieState ="
2. [Read] examples/03-user-validation.pole - Study syntax
3. [Compare] My code vs example
4. [Edit] Change "type ZombieState =" to comment
5. [Bash] pole check zombie.pole
6. ✅ Success - continue to Step 6

Total time: 30 seconds
User intervention: 0
```

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
