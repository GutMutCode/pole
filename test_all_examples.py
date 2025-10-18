#!/usr/bin/env python3

import sys
import glob

sys.path.insert(0, "src")

from pole.runtime.ir_parser_rust import parse_ir, RUST_AVAILABLE

if not RUST_AVAILABLE:
    print("Rust parser not available!")
    sys.exit(1)

examples = sorted(glob.glob("examples/*.pole-ir"))

print(f"Testing Rust parser against {len(examples)} example files\n")

passed = 0
failed = 0

for example in examples:
    filename = example.split("/")[-1]
    try:
        with open(example, "r") as f:
            ir_content = f.read()

        program_rust = parse_ir(ir_content, force_python=False)
        program_python = parse_ir(ir_content, force_python=True)

        if len(program_rust.func_defs) != len(program_python.func_defs):
            print(
                f"✗ {filename}: Function count mismatch (Rust: {len(program_rust.func_defs)}, Python: {len(program_python.func_defs)})"
            )
            failed += 1
            continue

        for i, (rust_func, python_func) in enumerate(
            zip(program_rust.func_defs, program_python.func_defs)
        ):
            if rust_func.name != python_func.name:
                print(
                    f"✗ {filename}: Function name mismatch (Rust: {rust_func.name}, Python: {python_func.name})"
                )
                failed += 1
                break
        else:
            print(f"✓ {filename}: {len(program_rust.func_defs)} function(s) parsed correctly")
            passed += 1

    except Exception as e:
        print(f"✗ {filename}: {e}")
        failed += 1

print(f"\n{'=' * 60}")
print(f"Results: {passed} passed, {failed} failed")

if failed == 0:
    print("✓ All examples parsed successfully!")
else:
    print(f"✗ {failed} example(s) failed")
    sys.exit(1)
