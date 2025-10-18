#!/usr/bin/env python3

import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent.parent / "src"))

from pole.runtime.ir_parser import parse_ir
from pole.verifier.type_checker import check_types
from pole.runtime.interpreter import interpret


def test_01_factorial_pipeline():
    """Test complete pipeline: factorial example"""
    examples_dir = Path(__file__).parent.parent / "examples"
    ir_file = examples_dir / "01-factorial.pole-ir"
    ir_code = ir_file.read_text()

    program = parse_ir(ir_code)
    assert program is not None
    assert len(program.func_defs) == 1
    assert program.func_defs[0].name == "factorial"

    type_result = check_types(program)
    assert type_result.success, f"Type check failed: {type_result}"

    assert interpret(program, "factorial", 0) == 1
    assert interpret(program, "factorial", 1) == 1
    assert interpret(program, "factorial", 5) == 120
    assert interpret(program, "factorial", 7) == 5040
    print("✓ test_01_factorial_pipeline")


def test_02_fibonacci_pipeline():
    """Test complete pipeline: fibonacci example"""
    examples_dir = Path(__file__).parent.parent / "examples"
    ir_file = examples_dir / "02-fibonacci.pole-ir"
    ir_code = ir_file.read_text()

    program = parse_ir(ir_code)
    assert program is not None
    assert len(program.func_defs) == 1
    assert program.func_defs[0].name == "fibonacci"

    type_result = check_types(program)
    assert type_result.success, f"Type check failed: {type_result}"

    assert interpret(program, "fibonacci", 1) == 1
    assert interpret(program, "fibonacci", 2) == 1
    assert interpret(program, "fibonacci", 3) == 2
    assert interpret(program, "fibonacci", 8) == 21
    assert interpret(program, "fibonacci", 10) == 55
    print("✓ test_02_fibonacci_pipeline")


def test_04_simple_math_pipeline():
    """Test complete pipeline: simple-math example (abs, max, sum_to_n)"""
    examples_dir = Path(__file__).parent.parent / "examples"
    ir_file = examples_dir / "04-simple-math.pole-ir"
    ir_code = ir_file.read_text()

    program = parse_ir(ir_code)
    assert program is not None
    assert len(program.func_defs) == 3
    func_names = {f.name for f in program.func_defs}
    assert func_names == {"abs", "max", "sum_to_n"}

    type_result = check_types(program)
    assert type_result.success, f"Type check failed: {type_result}"

    assert interpret(program, "abs", 5) == 5
    assert interpret(program, "abs", -3) == 3
    assert interpret(program, "abs", 0) == 0
    assert interpret(program, "abs", -15) == 15

    assert interpret(program, "max", 42, 17) == 42
    assert interpret(program, "max", 10, 20) == 20
    assert interpret(program, "max", -5, -10) == -5

    assert interpret(program, "sum_to_n", 1) == 1
    assert interpret(program, "sum_to_n", 5) == 15
    assert interpret(program, "sum_to_n", 10) == 55
    assert interpret(program, "sum_to_n", 100) == 5050
    print("✓ test_04_simple_math_pipeline")


def test_05_is_even_pipeline():
    """Test complete pipeline: is-even example"""
    examples_dir = Path(__file__).parent.parent / "examples"
    ir_file = examples_dir / "05-is-even.pole-ir"
    ir_code = ir_file.read_text()

    program = parse_ir(ir_code)
    assert program is not None
    assert len(program.func_defs) == 2
    func_names = {f.name for f in program.func_defs}
    assert func_names == {"is_even", "is_even_helper"}

    type_result = check_types(program)
    assert type_result.success, f"Type check failed: {type_result}"

    assert interpret(program, "is_even", 0) == True
    assert interpret(program, "is_even", 2) == True
    assert interpret(program, "is_even", 4) == True
    assert interpret(program, "is_even", 1) == False
    assert interpret(program, "is_even", 3) == False
    assert interpret(program, "is_even", 100) == True
    print("✓ test_05_is_even_pipeline")


def test_07_max_pipeline():
    """Test complete pipeline: max example"""
    examples_dir = Path(__file__).parent.parent / "examples"
    ir_file = examples_dir / "07-max.pole-ir"
    ir_code = ir_file.read_text()

    program = parse_ir(ir_code)
    assert program is not None
    assert len(program.func_defs) == 1
    assert program.func_defs[0].name == "max"

    type_result = check_types(program)
    assert type_result.success, f"Type check failed: {type_result}"

    assert interpret(program, "max", 3, 5) == 5
    assert interpret(program, "max", 10, 7) == 10
    assert interpret(program, "max", -5, -10) == -5
    assert interpret(program, "max", 0, 0) == 0
    print("✓ test_07_max_pipeline")


def test_all_working_examples_parse_successfully():
    """Verify all 5 working examples parse without errors"""
    examples_dir = Path(__file__).parent.parent / "examples"
    working_examples = [
        "01-factorial.pole-ir",
        "02-fibonacci.pole-ir",
        "04-simple-math.pole-ir",
        "05-is-even.pole-ir",
        "07-max.pole-ir",
    ]

    for example in working_examples:
        ir_file = examples_dir / example
        ir_code = ir_file.read_text()
        program = parse_ir(ir_code)
        assert program is not None, f"Failed to parse {example}"
    print("✓ test_all_working_examples_parse_successfully")


def test_all_working_examples_typecheck_successfully():
    """Verify all 5 working examples pass type checking"""
    examples_dir = Path(__file__).parent.parent / "examples"
    working_examples = [
        "01-factorial.pole-ir",
        "02-fibonacci.pole-ir",
        "04-simple-math.pole-ir",
        "05-is-even.pole-ir",
        "07-max.pole-ir",
    ]

    for example in working_examples:
        ir_file = examples_dir / example
        ir_code = ir_file.read_text()
        program = parse_ir(ir_code)
        type_result = check_types(program)
        assert type_result.success, f"Type check failed for {example}: {type_result}"
    print("✓ test_all_working_examples_typecheck_successfully")


def test_interpreter_handles_edge_cases():
    """Verify interpreter handles edge cases correctly"""
    examples_dir = Path(__file__).parent.parent / "examples"

    ir_file = examples_dir / "01-factorial.pole-ir"
    ir_code = ir_file.read_text()
    program = parse_ir(ir_code)
    assert interpret(program, "factorial", 0) == 1

    ir_file = examples_dir / "04-simple-math.pole-ir"
    ir_code = ir_file.read_text()
    program = parse_ir(ir_code)
    assert interpret(program, "abs", 0) == 0
    assert interpret(program, "abs", -0) == 0
    assert interpret(program, "max", 5, 5) == 5
    print("✓ test_interpreter_handles_edge_cases")


if __name__ == "__main__":
    test_01_factorial_pipeline()
    test_02_fibonacci_pipeline()
    test_04_simple_math_pipeline()
    test_05_is_even_pipeline()
    test_07_max_pipeline()
    test_all_working_examples_parse_successfully()
    test_all_working_examples_typecheck_successfully()
    test_interpreter_handles_edge_cases()
    print("\n✅ All end-to-end integration tests passed!")
