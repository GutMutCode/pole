#!/usr/bin/env python3

import sys
import time

sys.path.insert(0, "src")
sys.path.insert(0, "compiler/target/release")

import pole_compiler
from pole.runtime.ir_parser import parse_ir as parse_ir_python
from pole.verifier.type_checker import check_types as check_types_python

print("Performance Benchmark: Rust vs Python Type Checker\n")

# Load factorial example
with open("examples/01-factorial.pole-ir", "r") as f:
    ir_content = f.read()

print(f"Input: factorial.pole-ir ({len(ir_content)} bytes)\n")

ITERATIONS = 1000

print(f"Running {ITERATIONS} iterations...\n")

# Benchmark Rust
start = time.time()
for _ in range(ITERATIONS):
    pole_compiler.check_types_py(ir_content)
rust_time = time.time() - start

# Benchmark Python
start = time.time()
for _ in range(ITERATIONS):
    program = parse_ir_python(ir_content)
    check_types_python(program)
python_time = time.time() - start

print(f"{'Type Checker':<15} {'Total (s)':<12} {'Per check (ms)':<18} {'Speedup':<10}")
print("-" * 60)
print(f"{'Rust':<15} {rust_time:<12.4f} {(rust_time / ITERATIONS * 1000):<18.4f} {'-':<10}")
print(
    f"{'Python':<15} {python_time:<12.4f} {(python_time / ITERATIONS * 1000):<18.4f} {python_time / rust_time:<10.1f}x"
)

print(f"\n✓ Rust type checker is {python_time / rust_time:.1f}x faster than Python!")
