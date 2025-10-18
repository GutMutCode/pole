# Changelog

All notable changes to the Pole project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2025-10-19

### 🎉 First Prototype Release

This is the first working prototype of Pole, an LLM-optimized programming language system.

### ✨ Added

#### Language Design
- **Specification Language** (.pole) - Natural language-friendly syntax for expressing intent
- **Implementation Language** (Pole IR) - Formal, type-safe intermediate representation
- Complete language specifications in `specs/` directory
  - `syntax-v0.md` - Specification language grammar
  - `ir-syntax.md` - IR grammar definition
  - `verification.md` - Verification system design
  - `workflow.md` - LLM transformation workflow (6-step process)

#### Core Implementation
- **Parser** - `.pole` file parsing with AST generation
- **Validator** - Specification completeness checking and ambiguity detection
- **LLM Transformer** - Specification to IR conversion using OpenRouter API
  - Support for Claude (anthropic/claude-3.5-sonnet)
  - Mock LLM client for testing without API
- **IR Interpreter** - Execute IR code with support for:
  - Recursion and pattern matching
  - Type checking and inference
  - Runtime contract verification (requires/ensures)
- **Type Checker** - Static type verification with detailed error messages
- **Contract Verifier** - Runtime precondition/postcondition checking
- **Example Tester** - Automatic test execution from `@test_case` annotations

#### CLI Tools
- `pole check <file>` - Validate specification files
- `pole build <file>` - Generate IR from specifications (LLM-powered)
- `pole run <ir-file> <function> [args...]` - Execute IR functions
- `pole test <ir-file>` - Run all test cases in IR file

#### Quality Features
- **Error System** - Unified error handling with:
  - Source location tracking
  - Code context highlighting
  - Helpful error messages with suggestions
- **Performance** - Excellent performance (< 1ms for most operations)
  - Factorial(20): 0.06ms
  - Deep recursion (200 levels): 0.99ms
- **Testing** - Comprehensive test coverage
  - 9 test modules with all tests passing
  - Performance benchmarks
  - Contract verification tests

#### Examples
- `01-factorial.pole` - Factorial with recursion and pattern matching
- `02-fibonacci.pole` - Fibonacci with simple recursion
- `03-user-validation.pole` - Complex validation constraints
- `04-simple-math.pole` - Math operations (abs, max, sum_to_n)
- All examples include working IR implementations with test cases (15/15 tests passing)

#### Development Environment
- **NixOS/Nix support** - Declarative development environment
  - `shell.nix` for reproducible builds
  - `.envrc` for automatic direnv integration
  - `pole` command wrapper for convenient CLI usage
- **Documentation**
  - `QUICKSTART.md` - User onboarding guide
  - `ARCHITECTURE.md` - System architecture details
  - `DEVELOPMENT.md` - Development setup guide
  - `ROADMAP.md` - Priority-based task management
  - `AGENTS.md` - AI agent development guidelines

### 📊 Statistics
- 28 commits
- 22 Python source files
- 9 test modules (100% passing)
- 4 example programs with 15 test cases
- 5 specification documents

### 🔧 Technical Details
- **Language**: Python 3.11+
- **Dependencies**: Zero required dependencies (OpenRouter API optional for LLM transformation)
- **License**: MIT
- **Architecture**: Two-layer language system (Specification → IR)

### 🎯 Project Status
- ✅ Phase 0: Planning & Documentation (Complete)
- ✅ Phase 1: Language Design (Complete)
- ✅ Phase 2: Prototype Implementation (Complete - All P0/P1 tasks)
- ✅ Phase 3: Quality Improvements (P0 complete, P1 partial)

### 📝 Known Limitations
- No IDE integration (LSP) yet
- No interactive specification improvement tool
- No debugger or profiler
- String operations limited in IR interpreter
- List/Map types not fully implemented in interpreter
- Type system supports basic types (Int, Nat, Bool, String)

### 🚀 Future Work
- IDE integration with LSP support
- Interactive specification refinement tool
- Debugger with execution tracing
- Profiler for performance analysis
- Documentation generator from specifications
- Extended standard library

### 🙏 Acknowledgments
Built with focus on LLM-friendly design and formal verification principles.

---

[0.1.0]: https://github.com/pole-lang/pole/releases/tag/v0.1.0
