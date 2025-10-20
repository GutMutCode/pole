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

### Verification

```bash
# Before committing
make pre-commit              # Format + verify all

# Individual checks
make verify-specs            # .pole syntax
make verify-ir               # .pole-ir with Rust parser
make update-priority         # Update today's priority
```

## 🎯 Current Priority (Week 1, 2025-10-20)

**Active Phase:** Week 1 - Pole Zomboid Demo  
**Goal:** 1-minute playable demo by 2025-10-26

### Today's Task (Day 4 - Thursday) ✅ COMPLETED
1. ✅ Write `games/zomboid/main.pole` specification
2. ✅ Generate main.pole-ir with complete game loop
3. ✅ SDL2 integration: window, renderer, game loop (600 frames)
4. ✅ Game state management: player, zombie, tilemap
5. ✅ Structural integration verified (27 functions, 7 types)

### Next Task (Day 5 - Friday)
**Morning:** Add builtin functions to Rust type checker
**Afternoon:** Pole Engine refactoring

### Pending Task (Day 5 - Friday)
**P1:** Add builtin functions to Rust type checker
- list_get, list_set, list_push
- int_to_float, float_to_int
- See Day 5 section in [docs/WEEK1_PLAN.md](docs/WEEK1_PLAN.md) for details

**Detailed Plan:** See [docs/WEEK1_PLAN.md](docs/WEEK1_PLAN.md)

### Priority Rules

**Hierarchy:** Week Plan > P0 > P1 > P2

- **P0**: Critical - Must complete this week
- **P1**: Important - After P0 completion
- **P2**: Optional - Time permitting

**Before starting work:**
1. ✅ Check this file (CLAUDE.md) for current priority
2. ✅ Check [.claude/PENDING_ISSUES.md](.claude/PENDING_ISSUES.md) for scheduled tasks
3. ✅ Read related guide (WEEK1_PLAN.md)
4. ✅ Confirm with user if unclear

## 🔍 Development Checklist (MUST FOLLOW)

**IMPORTANT:** When starting development work, you MUST create a TODO list with these steps.

### Quick Checklist (11 Steps)

When user says "진행해줘":

1. ✅ **[TodoWrite]** Create 11-step checklist
2. ✅ Check dependencies (previous day's work)
3. ✅ Read syntax examples (.pole and .pole-ir)
4. ✅ Test pole CLI tools
5. ✅ Write/edit specification files
6. ✅ Validate with `pole check`
7. ✅ Generate/verify IR
8. ✅ Run all tests on generated IR
9. ✅ Write integration test file (examples/XX-name.pole-ir) **if required by task**
10. ✅ Run integration test
11. ✅ Commit only if all pass

**Detailed Steps:** See [.claude/DEVELOPMENT_CHECKLIST.md](.claude/DEVELOPMENT_CHECKLIST.md)

### Error Handling (Autonomous)

If errors occur, LLM MUST resolve autonomously:

- **Dependency missing** → Switch to dependency first → Resume
- **Syntax error** → Read examples → Fix → Retry (3x)
- **IR generation failed** → Improve prompt → Mock → Manual
- **Parser error** → Re-read examples → Fix → Retry (3x)
- **Test failure** → Analyze → Fix logic → Retry

**Detailed Protocols:** See [.claude/ERROR_RECOVERY.md](.claude/ERROR_RECOVERY.md)

**Success Rate:** 80-90% autonomous resolution

## Important Conventions

### File Extensions
- `.pole` = Specification language (human-written)
- `.pole-ir` = Implementation language (LLM-generated, formal)
- Never confuse these two! They serve different purposes.

### Syntax Rules
- **`.pole` files:** Use `type Name:` with `fields:` (NOT `type Name = {...}`)
- **`.pole-ir` files:** Use `type Name = {...}` for records
- **Enum types:** Comment in `.pole`, implement as Variant in `.pole-ir`

### LLM Workflow
1. Human writes .pole spec (intentionally incomplete/ambiguous is OK)
2. `pole check` validates spec, detects ambiguities
3. `pole build` sends spec to LLM → generates .pole-ir
4. IR is type-checked, contract-verified, tested
5. If errors, LLM may regenerate or request clarification

### Testing Strategy
- Every .pole-ir file should have @test_case annotations
- `pole test` automatically runs all test cases
- Rust compiler has separate test suite (cargo test)

## Code Structure

### Critical Directories
- **examples/**: Specification (.pole) and IR (.pole-ir) example files
- **specs/**: Language specifications (syntax-v0.md, ir-syntax.md, ffi.md)
- **src/pole/**: Python CLI, parser, validator, LLM integration
- **compiler/src/**: Rust IR parser, type checker, LLVM compiler
- **games/zomboid/specs/**: Game specifications (current work)

### Key Example Files
- `examples/03-user-validation.pole` - .pole syntax reference
- `examples/08-simple-record.pole-ir` - .pole-ir syntax reference
- `specs/syntax-v0.md` - Specification language grammar
- `specs/ir-syntax.md` - Implementation language grammar

## Common Issues

### "pole: command not found"
- NixOS: Run `direnv allow` or `nix-shell`
- Others: Set `export PYTHONPATH=src` and `alias pole="python -m pole.cli.main"`

### Python parser errors (expected)
- Python parser is legacy, use for quick checks only
- Rust parser is authoritative: `cd compiler && cargo run --release --bin polec -- ../file.pole-ir`

### Type errors in generated IR
- LLM sometimes generates incorrect types
- Read examples/08-simple-record.pole-ir for correct syntax
- Fix manually or regenerate with better prompt

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
- AUTOMATION_GUIDE.md: LLM automation (95% automated)
- PRIORITY_ADJUSTMENT.md: Autonomous error recovery

**Claude-specific** (.claude/):
- DEVELOPMENT_CHECKLIST.md: 9-step workflow
- ERROR_RECOVERY.md: Error handling protocols

## Key Files to Read First

1. README.md - Understand the vision
2. ARCHITECTURE.md - System design
3. examples/01-factorial.pole + .pole-ir - See the workflow
4. examples/03-user-validation.pole - Learn .pole syntax
5. examples/08-simple-record.pole-ir - Learn .pole-ir syntax
6. specs/ir-syntax.md - Complete IR grammar
7. .claude/DEVELOPMENT_CHECKLIST.md - Development workflow
8. .claude/ERROR_RECOVERY.md - Error handling

## Quick Commands Reference

```bash
# Priority
make update-priority         # Update today's task

# Development
pole check file.pole         # Validate spec
pole build file.pole         # Generate IR
pole test file.pole-ir       # Run tests

# Verification (before commit)
make verify-specs            # Check all .pole files
make verify-ir               # Check all .pole-ir files  
make pre-commit              # Complete verification

# Automation
make auto-dev FILE=spec.pole # Run complete workflow
```

## License

MIT License - See LICENSE file

---

**Last Updated:** 2025-10-21  
**Current Phase:** Week 1 Day 2 - Zombie specification  
**File Length:** 270 lines (under 500-line limit ✅)
