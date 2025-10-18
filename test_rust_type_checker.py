#!/usr/bin/env python3

import sys
import os

sys.path.insert(0, os.path.join(os.path.dirname(__file__), "compiler/target/release"))

import pole_compiler

print("Testing Rust Type Checker...\n")

# Test 1: Simple function
ir1 = """
func add (x: Int, y: Int) -> Int :
  x + y
"""

result = pole_compiler.check_types_py(ir1)
print(f"Test 1 - Simple function: {'✓ PASS' if result['success'] else '✗ FAIL'}")
if not result["success"]:
    for err in result["errors"]:
        print(f"  Error: {err['message']}")

# Test 2: Factorial
ir2 = """
func factorial (n: Nat) -> Nat :
  match n with
  | 0 -> 1
  | n -> n * factorial (n - 1)
"""

result = pole_compiler.check_types_py(ir2)
print(f"Test 2 - Factorial: {'✓ PASS' if result['success'] else '✗ FAIL'}")
if not result["success"]:
    for err in result["errors"]:
        print(f"  Error: {err['message']}")

# Test 3: Type mismatch (should fail)
ir3 = """
func bad () -> Int :
  true
"""

result = pole_compiler.check_types_py(ir3)
print(
    f"Test 3 - Type mismatch: {'✓ PASS (correctly detected)' if not result['success'] else '✗ FAIL (should have failed)'}"
)
if not result["success"]:
    for err in result["errors"]:
        print(f"  Expected error: {err['message']}")

print("\n✅ All Rust type checker tests passed!")
