#!/usr/bin/env python3

import sys
import os

sys.path.insert(0, os.path.join(os.path.dirname(__file__), "compiler/target/release"))

try:
    import pole_compiler

    print("✓ Rust module loaded successfully")
    print(f"  Module: {pole_compiler}")
    print(f"  Functions: {dir(pole_compiler)}")

    test_ir = """
function factorial(n: int) -> int:
  requires: n >= 0
  ensures: result >= 1
  body:
    match n with
    | 0 -> 1
    | _ -> n * factorial (n - 1)
"""

    print("\n Testing parse_ir()...")
    result = pole_compiler.parse_ir(test_ir)
    print("✓ parse_ir() succeeded")
    print(f"  Result type: {type(result)}")
    print(f"  Result: {result}")

except ImportError as e:
    print(f"✗ Failed to import Rust module: {e}")
    sys.exit(1)
except Exception as e:
    print(f"✗ Error during test: {e}")
    import traceback

    traceback.print_exc()
    sys.exit(1)
