# Pole IR → LLVM IR Mapping Strategy

> Design document for M2: LLVM Backend - Basic Function Compilation

## Overview

This document defines how Pole IR constructs map to LLVM IR during compilation.

## Type Mapping

### Basic Types

| Pole IR Type | LLVM Type | Notes |
|--------------|-----------|-------|
| `Int` | `i64` | 64-bit signed integer |
| `Nat` | `i64` | 64-bit unsigned (treated as signed, runtime check) |
| `Bool` | `i1` | 1-bit boolean |
| `Float64` | `double` | IEEE 754 double precision |
| `String` | `i8*` | Pointer to UTF-8 byte array (heap allocated) |
| `Unit` | `void` | Empty type |

**Rationale:**
- `Nat` uses `i64` for simplicity (M2 scope). Future: separate unsigned type with overflow checks.
- `String` initially simplified as C-string. Future: length-prefixed or fat pointer.

### Compound Types (M3+)

| Pole IR Type | LLVM Type | Notes |
|--------------|-----------|-------|
| `{ field: Type }` | `{ Type }` | LLVM struct |
| `Option<T>` | `{ i1, T }` | Tagged union: `{ is_some: i1, value: T }` |
| `List<T>` | `{ i64, T* }` | Fat pointer: `{ len: i64, data: T* }` |
| `(T1, T2)` | `{ T1, T2 }` | Tuple as anonymous struct |

## Expression Mapping

### Literals

```pole-ir
42              → i64 42
true            → i1 1
3.14            → double 3.14
```

**LLVM Code:**
```llvm
%1 = i64 42
%2 = i1 1
%3 = double 3.14
```

### Binary Operations

```pole-ir
a + b           → %result = add i64 %a, %b
a - b           → %result = sub i64 %a, %b
a * b           → %result = mul i64 %a, %b
a / b           → %result = sdiv i64 %a, %b    ; signed division
a % b           → %result = srem i64 %a, %b    ; signed remainder
```

### Comparison Operations

```pole-ir
a == b          → %result = icmp eq i64 %a, %b
a != b          → %result = icmp ne i64 %a, %b
a < b           → %result = icmp slt i64 %a, %b   ; signed less than
a <= b          → %result = icmp sle i64 %a, %b
a > b           → %result = icmp sgt i64 %a, %b
a >= b          → %result = icmp sge i64 %a, %b
```

### Logical Operations

```pole-ir
a && b          → %and = and i1 %a, %b
a || b          → %or = or i1 %a, %b
!a              → %not = xor i1 %a, true
```

### If-Then-Else

```pole-ir
if cond then expr1 else expr2
```

**LLVM Code:**
```llvm
entry:
  %cond = ...                              ; evaluate condition
  br i1 %cond, label %then, label %else

then:
  %then_val = ...                          ; evaluate expr1
  br label %merge

else:
  %else_val = ...                          ; evaluate expr2
  br label %merge

merge:
  %result = phi i64 [ %then_val, %then ], [ %else_val, %else ]
  ret i64 %result
```

**Phi Node:** Merges values from different control flow paths (SSA form requirement).

### Match Expressions (Pattern Matching)

#### Simple Integer Match

```pole-ir
match n with
| 0 -> 1
| n -> n * factorial(n - 1)
```

**LLVM Strategy (M2):**

Compile to if-then-else chain:

```llvm
entry:
  %cond_0 = icmp eq i64 %n, 0
  br i1 %cond_0, label %case_0, label %case_default

case_0:
  br label %merge

case_default:
  %n_minus_1 = sub i64 %n, 1
  %rec_call = call i64 @factorial(i64 %n_minus_1)
  %result = mul i64 %n, %rec_call
  br label %merge

merge:
  %final = phi i64 [ 1, %case_0 ], [ %result, %case_default ]
  ret i64 %final
```

**M3 Optimization:** Use switch instruction for integer patterns.

### Function Calls

#### Direct Call (Non-Recursive)

```pole-ir
max(a, b)
```

**LLVM Code:**
```llvm
%result = call i64 @max(i64 %a, i64 %b)
```

#### Recursive Call

```pole-ir
factorial(n - 1)
```

**LLVM Code:**
```llvm
%n_minus_1 = sub i64 %n, 1
%rec_result = call i64 @factorial(i64 %n_minus_1)
```

**Note:** LLVM handles recursion via stack frames. No tail call optimization in M2.

### Let Bindings

```pole-ir
let x = expr1 in expr2
```

**LLVM Strategy:**

Evaluate `expr1`, bind to SSA register, use in `expr2`:

```llvm
%x = ...           ; evaluate expr1
%result = ...      ; evaluate expr2 using %x
```

**Note:** SSA form eliminates need for explicit let binding - registers are immutable.

## Function Compilation

### Function Signature

```pole-ir
func factorial (n: Nat) -> Nat:
  ...
```

**LLVM Code:**
```llvm
define i64 @factorial(i64 %n) {
entry:
  ...
}
```

**Components:**
- `define`: Function definition
- `i64`: Return type
- `@factorial`: Function name (global symbol)
- `i64 %n`: Parameter (type + SSA register)
- `entry`: First basic block

### Multi-Parameter Functions

```pole-ir
func add (a: Int, b: Int) -> Int:
  a + b
```

**LLVM Code:**
```llvm
define i64 @add(i64 %a, i64 %b) {
entry:
  %sum = add i64 %a, %b
  ret i64 %sum
}
```

## Contract Compilation (M3+)

```pole-ir
func factorial (n: Nat) -> Nat
  requires n >= 0
  ensures result >= 1
:
  ...
```

**M2 Strategy:** Ignore contracts (focus on basic compilation).

**M3 Strategy:** Compile to runtime assertions:

```llvm
define i64 @factorial(i64 %n) {
entry:
  ; requires n >= 0
  %precond = icmp sge i64 %n, 0
  br i1 %precond, label %body, label %precond_fail

precond_fail:
  call void @contract_violation(i8* getelementptr inbounds ([...], [...]*, i64 0, i64 0))
  unreachable

body:
  ; ... function body ...
  
  ; ensures result >= 1
  %postcond = icmp sge i64 %result, 1
  br i1 %postcond, label %return, label %postcond_fail

postcond_fail:
  call void @contract_violation(...)
  unreachable

return:
  ret i64 %result
}
```

## Memory Management (M2 Scope)

**M2:** All stack-allocated (no heap, no GC).

- Parameters: Stack (LLVM calling convention)
- Local variables: SSA registers (no explicit allocation)
- Return values: Register or stack (depends on size)

**M3+:** Heap allocation for strings, lists, records.

## Module Structure

### Single Module (M2)

```llvm
; ModuleID = 'factorial'
source_filename = "factorial.pole-ir"

define i64 @factorial(i64 %n) {
  ; function body
}

define i64 @main() {
  %result = call i64 @factorial(i64 5)
  ret i64 %result
}
```

### Multi-Module (M4+)

Link multiple LLVM modules using LLVM linker.

## Implementation Plan (M2)

### Phase 1: Basic Types & Literals
- [x] Int, Bool literals → LLVM constants
- [x] Arithmetic operations (+, -, *, /)
- [x] Comparison operations (==, <, <=, etc.)

### Phase 2: Control Flow
- [x] If-then-else → br + phi
- [ ] Match expressions → if-then-else chain (M2) / switch (M3)

### Phase 3: Functions
- [ ] Function definition → LLVM define
- [ ] Function calls (direct, non-recursive)
- [ ] Recursive calls

### Phase 4: Factorial Example
- [ ] Compile factorial.pole-ir to LLVM IR
- [ ] Verify correctness (factorial(5) == 120)
- [ ] Execute via JIT

## Testing Strategy

### Unit Tests (Per Feature)

```rust
#[test]
fn test_compile_int_literal() {
    let ir = parse_ir("42");
    let llvm_value = compile_expr(&context, &ir);
    assert_eq!(llvm_value.print_to_string(), "i64 42");
}
```

### Integration Tests (Full Functions)

```rust
#[test]
fn test_compile_factorial() {
    let ir_source = read_file("examples/01-factorial.pole-ir");
    let module = compile_module(&context, &ir_source);
    
    let engine = module.create_jit_execution_engine(OptimizationLevel::None).unwrap();
    let factorial = engine.get_function::<extern "C" fn(i64) -> i64>("factorial").unwrap();
    
    assert_eq!(factorial.call(0), 1);
    assert_eq!(factorial.call(5), 120);
}
```

### End-to-End Tests (CLI)

```bash
$ pole compile examples/01-factorial.pole-ir -o factorial
$ ./factorial 5
120
```

## Error Handling

### Compile-Time Errors

- Type mismatch: `Type error: expected Int, got Bool`
- Undefined function: `Function 'foo' not found`
- Invalid pattern: `Pattern match not exhaustive`

### Runtime Errors (M3+)

- Division by zero: Trap (LLVM generates `udiv` with trap on zero)
- Contract violation: Call `@contract_violation` + unreachable

## LLVM IR Output Format

### Human-Readable (Development)

```bash
$ pole compile factorial.pole-ir --emit-llvm -o factorial.ll
```

Output:
```llvm
; factorial.ll
define i64 @factorial(i64 %n) {
  ; readable LLVM IR
}
```

### Bitcode (Production)

```bash
$ pole compile factorial.pole-ir -o factorial.bc
```

Binary format for fast loading + linking.

### Object File (Final)

```bash
$ pole compile factorial.pole-ir -o factorial.o
```

Native object file (linkable with C/Rust).

## Optimization Passes (M3+)

M2: No optimization (focus on correctness).

M3: Enable LLVM optimization passes:

```rust
let pass_manager = PassManager::create(&module);
pass_manager.add_instruction_combining_pass();
pass_manager.add_reassociate_pass();
pass_manager.add_gvn_pass();
pass_manager.add_cfg_simplification_pass();
pass_manager.run_on(&module);
```

Expected improvements:
- Constant folding
- Dead code elimination
- Tail call optimization (for recursive functions)

## References

- [LLVM Language Reference Manual](https://llvm.org/docs/LangRef.html)
- [inkwell Documentation](https://thedan64.github.io/inkwell/)
- [Kaleidoscope Tutorial](https://llvm.org/docs/tutorial/)

## Next Steps

1. Implement IR → LLVM type mapper
2. Implement expression compiler (literals, binops)
3. Implement if-then-else compiler
4. Implement function compiler
5. Test with factorial example
