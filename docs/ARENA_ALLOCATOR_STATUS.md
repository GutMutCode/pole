# Arena Allocator Implementation Status

**Date**: 2025-10-20  
**Phase**: 5.1.5 - Compiler Memory Management Optimization  
**Status**: Partial Implementation

## Overview

Arena allocator infrastructure is in place but not fully utilized. Current implementation provides memory tracking without the full benefits of arena allocation.

## Implemented ✅

### 1. Arena Infrastructure
- **File**: `compiler/src/arena.rs`
- **Status**: Complete
- **Features**:
  - `CompilerArenas` struct with 3 arenas (parse, ir, codegen)
  - Default capacity: 100MB total (33.3MB each)
  - Memory tracking methods: `allocated_bytes()`, `total_allocated()`
  - Reset capability for batch compilation

### 2. Memory Statistics
- **File**: `compiler/src/memory.rs`
- **Status**: Complete
- **Features**:
  - `MemoryStats` struct for tracking usage
  - Human-readable formatting (KB/MB)
  - `CompileError::OutOfMemory` error type

### 3. Integration
- **CodeGen**: Arena passed as parameter, `alloc_temp()` method exists
- **test_list_push**: Memory statistics printed after compilation

## Current Memory Usage

**Measured Examples**:
- `01-factorial.pole-ir`: ~100 MB total
- Small examples: ~100 MB baseline
- Large examples (100 zombies): TBD

## Not Implemented ❌

### 1. Actual Arena Usage
- AST nodes still use `Box<T>` and `Vec<T>` (42 instances in ast.rs)
- CodeGen still uses `HashMap` and heap allocation
- `alloc_temp()` method is dead code (unused)

### 2. Reference-Based AST
- Current AST uses owned types (`Box<Expr>`)
- Arena-based would need borrowed types (`&'arena Expr`)
- **Impact**: Major refactor required for all parser/type checker

### 3. OOM Recovery
- No actual memory limits enforced
- `CompileError::OutOfMemory` defined but never thrown
- No graceful degradation

## Why Not Fully Implemented?

### Complexity vs Benefit
1. **Large Refactor**: Converting AST to references requires rewriting parser, type checker, codegen
2. **Current Performance**: Compiler works fine for all examples (up to 100 zombies)
3. **Priority**: Game features (Q2) more valuable than compiler optimization
4. **Timeline**: Full implementation = 1 month of work

### Technical Challenges
```rust
// Current (owned):
pub struct Expr {
    kind: Box<ExprKind>,
}

// Arena-based (borrowed):
pub struct Expr<'arena> {
    kind: &'arena ExprKind<'arena>,
}
```
**Impact**: Lifetime annotations propagate through entire codebase.

## Partial Completion Rationale

**Phase 5.1.5 Goal**: "SQLite 스타일 메모리 관리로 컴파일러 안정성/성능 대폭 개선"

**What We Achieved**:
- ✅ Arena infrastructure (foundation for future work)
- ✅ Memory tracking and visibility
- ✅ OOM error types (ready for enforcement)

**What Remains**:
- ❌ 75% memory reduction (requires AST refactor)
- ❌ Actual arena allocation usage
- ❌ OOM recovery mechanism

## Recommendation

**Option 1**: Mark as "Foundation Complete" and proceed to Q2 game features  
**Option 2**: Invest 1 month for full AST refactor (delays game development)

**Chosen**: Option 1 - Game development takes priority

## Next Steps (If Revisited)

1. **Milestone M1** (1 week): Convert `Expr` to use arena
2. **Milestone M2** (1 week): Convert `Type` and patterns
3. **Milestone M3** (1 week): Update parser to use arena
4. **Milestone M4** (1 week): Benchmark and optimize

**Total Estimate**: 1 month for full implementation

## References

- **ROADMAP.md**: Phase 5.1.5
- **Code**: `compiler/src/arena.rs`, `compiler/src/memory.rs`
- **Design**: SQLite-style arena allocation pattern
