# Agent Guidelines for Pole Project

## Project Context
Pole is an LLM-optimized programming language system with two distinct layers:

1. **Specification Language** (.pole files) - Human-written, natural language friendly
2. **Implementation Language** (Pole IR) - LLM-generated, formal, type-safe

**IMPORTANT**: We are designing BOTH languages from scratch. Pole does NOT compile to existing languages like Rust or Python. See ARCHITECTURE.md for the complete pipeline.

## Build/Test Commands
- No build system yet - this is a design/specification phase project
- No tests yet - implementation has not started
- Git operations: standard git commands

## Code Style (When Implementation Begins)
- **Specification Language** (.pole): English keywords for LLM compatibility
- **Documentation**: Korean (한국어) for design docs, English for code/specs
- **Implementation Tool**: Python for transformer/compiler (빠른 프로토타이핑)
- **Format**: All documents should be LLM-friendly and parseable
- **Structure**: Follow the categorical organization in README.md
- **Naming**: English for all code and language constructs
- **Comments**: Avoid unless explicitly requested per project rules

## File Organization
- `specs/` - Language specifications (syntax-v0.md, implementation-lang.md)
- `examples/` - Example programs (*.pole for spec lang, *.pole-ir for IR)
- `src/` - Implementation code (transformer, compiler, verifier)
- Follow the priority system defined in README.md (P0/P1/P2)
- **Critical**: Never confuse "타겟 언어" with "구현 언어" - we design the IR ourselves

## Development Approach
- Focus on "what" and "why" over "how" in specifications
- Allow intentional ambiguity where LLM judgment is delegated
- Support incremental refinement and incomplete specifications
- Document examples, use cases, and edge cases
- Emphasize verification and formal correctness where applicable
