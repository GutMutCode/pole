#!/usr/bin/env python3

import sys
import time

sys.path.insert(0, "src")

from pole.runtime.ir_parser_rust import parse_ir, RUST_AVAILABLE

if not RUST_AVAILABLE:
    print("Rust parser not available, cannot benchmark!")
    sys.exit(1)

with open("examples/01-factorial.pole-ir", "r") as f:
    ir_content = f.read()

print("Performance Benchmark: Rust vs Python IR Parser\n")
print(f"Input: factorial.pole-ir ({len(ir_content)} bytes)\n")

ITERATIONS = 1000

print(f"Running {ITERATIONS} iterations...")

start = time.time()
for _ in range(ITERATIONS):
    parse_ir(ir_content, force_python=False)
rust_time = time.time() - start

start = time.time()
for _ in range(ITERATIONS):
    parse_ir(ir_content, force_python=True)
python_time = time.time() - start

print(f"\n{'Parser':<15} {'Total (s)':<12} {'Per parse (ms)':<18} {'Speedup':<10}")
print("-" * 60)
print(f"{'Rust':<15} {rust_time:<12.4f} {(rust_time / ITERATIONS * 1000):<18.4f} {'-':<10}")
print(
    f"{'Python':<15} {python_time:<12.4f} {(python_time / ITERATIONS * 1000):<18.4f} {python_time / rust_time:<10.1f}x"
)

print(f"\n✓ Rust parser is {python_time / rust_time:.1f}x faster than Python!")
