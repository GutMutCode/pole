import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent.parent / "src"))

from pole.runtime.interpreter import interpret
from pole.runtime.ir_parser import parse_ir


def test_literal_evaluation():
    ir_code = """
func get_number () -> Int :
  42
"""
    program = parse_ir(ir_code)
    result = interpret(program, "get_number")
    assert result == 42
    print("✓ test_literal_evaluation")


def test_binary_operations():
    ir_code = """
func add (x: Int, y: Int) -> Int :
  x + y

func multiply (x: Int, y: Int) -> Int :
  x * y
"""
    program = parse_ir(ir_code)

    assert interpret(program, "add", 2, 3) == 5
    assert interpret(program, "multiply", 4, 5) == 20
    print("✓ test_binary_operations")


def test_if_expression():
    ir_code = """
func abs (x: Int) -> Int :
  if x >= 0 then x else -x
"""
    program = parse_ir(ir_code)

    assert interpret(program, "abs", 5) == 5
    assert interpret(program, "abs", -5) == 5
    print("✓ test_if_expression")


def test_pattern_matching():
    ir_code = """
func is_zero (n: Int) -> Bool :
  match n with
  | 0 -> true
  | n -> false
"""
    program = parse_ir(ir_code)

    assert interpret(program, "is_zero", 0) == True
    assert interpret(program, "is_zero", 5) == False
    print("✓ test_pattern_matching")


def test_factorial():
    ir_code = """
@source("examples/01-factorial.pole", line=3)
@test_case(input=0, expected=1)
@test_case(input=5, expected=120)
func factorial (n: Nat) -> Nat
  requires n >= 0
  ensures result >= 1
:
  match n with
  | 0 -> 1
  | n -> n * factorial (n - 1)
"""
    program = parse_ir(ir_code)

    assert interpret(program, "factorial", 0) == 1
    assert interpret(program, "factorial", 1) == 1
    assert interpret(program, "factorial", 5) == 120
    assert interpret(program, "factorial", 10) == 3628800
    print("✓ test_factorial")


def test_factorial_from_file():
    example_file = Path(__file__).parent.parent / "examples" / "01-factorial.pole-ir"

    if not example_file.exists():
        print(f"⊘ Skipping: {example_file} not found")
        return

    with open(example_file) as f:
        ir_code = f.read()

    program = parse_ir(ir_code)

    assert interpret(program, "factorial", 0) == 1
    assert interpret(program, "factorial", 1) == 1
    assert interpret(program, "factorial", 5) == 120
    print("✓ test_factorial_from_file")


def test_let_expression():
    ir_code = """
func compute (x: Int) -> Int :
  let y = x + 5 in y * 2
"""
    program = parse_ir(ir_code)

    assert interpret(program, "compute", 10) == 30
    print("✓ test_let_expression")


def test_comparison_operators():
    ir_code = """
func is_positive (x: Int) -> Bool :
  x > 0

func is_equal (x: Int, y: Int) -> Bool :
  x == y
"""
    program = parse_ir(ir_code)

    assert interpret(program, "is_positive", 5) == True
    assert interpret(program, "is_positive", -5) == False
    assert interpret(program, "is_equal", 5, 5) == True
    assert interpret(program, "is_equal", 5, 10) == False
    print("✓ test_comparison_operators")


if __name__ == "__main__":
    print("Running interpreter tests...\n")

    test_literal_evaluation()
    test_binary_operations()
    test_if_expression()
    test_pattern_matching()
    test_factorial()
    test_factorial_from_file()
    test_let_expression()
    test_comparison_operators()

    print("\n✅ All interpreter tests passed!")
