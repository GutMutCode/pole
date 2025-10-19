# M5: Runtime Functions Implementation

**Start Date:** 2025-10-19  
**Status:** In Progress

## Overview

M5 focuses on implementing runtime utility functions needed by Pole programs:
- String operations (length, contains, concat)
- List operations (length, concat, map, filter)
- I/O operations (print, read)

## Goals

1. Implement core String functions
2. Implement core List functions
3. Enable user-validation example (03-user-validation.pole-ir)
4. Add FFI basics for calling C functions

## Task Breakdown

### 5.1 String Functions

**Required for user-validation:**
- `String.length : String -> Nat` - Get string length
- `String.contains : String -> String -> Bool` - Check substring

**Additional utilities:**
- `String.concat : String -> String -> String`
- `String.substring : String -> Nat -> Nat -> String`
- `String.to_upper : String -> String`
- `String.to_lower : String -> String`

**Implementation approach:**
- External C functions (using libc)
- OR: Inline LLVM IR implementations
- OR: Rust implementations with FFI

### 5.2 List Functions

**Required for user-validation:**
- `List.concat : List<List<T>> -> List<T>` - Flatten list of lists

**Additional utilities:**
- `List.length : List<T> -> Nat`
- `List.map : (T -> U) -> List<T> -> List<U>`
- `List.filter : (T -> Bool) -> List<T> -> List<T>`
- `List.fold : (Acc -> T -> Acc) -> Acc -> List<T> -> Acc`

### 5.3 I/O Functions

**Basic I/O:**
- `print : String -> Unit` - Print to stdout
- `println : String -> Unit` - Print with newline
- `read_line : Unit -> String` - Read from stdin

**Implementation:**
- FFI to C printf/puts
- OR: LLVM IR directly
- Proper error handling

## Current Status

- [ ] String.length
- [ ] String.contains  
- [ ] List.concat
- [ ] print/println
- [ ] Enable 03-user-validation.pole-ir

## Next Steps

1. Choose implementation approach (FFI vs inline)
2. Implement String.length as proof of concept
3. Test with simple example
4. Implement remaining functions
5. Verify user-validation compiles and runs

## Success Criteria

- All String/List functions work correctly
- 03-user-validation.pole-ir compiles successfully
- Runtime performance is acceptable
- 15/15 examples pass (14 current + user-validation)
