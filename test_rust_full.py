#!/usr/bin/env python3

import sys
import os

sys.path.insert(0, os.path.join(os.path.dirname(__file__), "compiler/target/release"))

import pole_compiler
import json

with open("examples/01-factorial.pole-ir", "r") as f:
    ir_content = f.read()

print("IR Content (first 200 chars):")
print(ir_content[:200])
print("\nParsing...")

try:
    result = pole_compiler.parse_ir(ir_content)
    print("\n✓ Parsing succeeded!")
    print(f"\nResult type: {type(result)}")
    print(f"\nParsed program:")
    print(json.dumps(result, indent=2))

except Exception as e:
    print(f"\n✗ Parsing failed: {e}")
    import traceback

    traceback.print_exc()
    sys.exit(1)
