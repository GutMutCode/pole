#!/usr/bin/env python3

import sys

sys.path.insert(0, "src")

from pole.runtime.ir_parser_rust import parse_ir, RUST_AVAILABLE

print(f"Rust parser available: {RUST_AVAILABLE}\n")

with open("examples/01-factorial.pole-ir", "r") as f:
    ir_content = f.read()

print("Testing Rust parser...")
program_rust = parse_ir(ir_content, force_python=False)
print(f"✓ Rust parser succeeded")
print(f"  Functions: {len(program_rust.func_defs)}")
print(f"  First function: {program_rust.func_defs[0].name}")

print("\nTesting Python parser (fallback)...")
program_python = parse_ir(ir_content, force_python=True)
print(f"✓ Python parser succeeded")
print(f"  Functions: {len(program_python.func_defs)}")
print(f"  First function: {program_python.func_defs[0].name}")

print("\n✓ Both parsers work correctly!")
