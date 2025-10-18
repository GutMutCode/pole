# Agent Guidelines for Pole Project

## Project Context
This is a conceptual repository defining design principles for an LLM-optimized programming language. Currently contains only specification documents in Korean.

## Build/Test Commands
- No build system yet - this is a design/specification phase project
- No tests yet - implementation has not started
- Git operations: standard git commands

## Code Style (When Implementation Begins)
- **Language**: To be determined based on project evolution
- **Documentation**: Write all documents in Korean (한국어) to match README.md
- **Format**: Design documents should be LLM-friendly and parseable
- **Structure**: Follow the categorical organization in README.md (명세 언어, 구현 언어, 검증 시스템, etc.)
- **Naming**: Use descriptive Korean or English terms appropriate to the domain
- **Comments**: Avoid unless explicitly requested per project rules

## File Organization
- Design specifications and principles in root-level markdown files
- Follow the priority system defined in README.md (P0/P1/P2)
- Maintain clear separation between specification language (명세 언어) and implementation language (구현 언어) concepts

## Development Approach
- Focus on "what" and "why" over "how" in specifications
- Allow intentional ambiguity where LLM judgment is delegated
- Support incremental refinement and incomplete specifications
- Document examples, use cases, and edge cases
- Emphasize verification and formal correctness where applicable
