import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent.parent / "src"))

from pole.runtime.ir_parser import parse_ir
from pole.verifier.type_checker import check_types


def test_simple_function_type_check():
    ir_code = """
func add (x: Int, y: Int) -> Int :
  x + y
"""
    program = parse_ir(ir_code)
    result = check_types(program)

    assert result.success
    assert len(result.errors) == 0
    print("✓ test_simple_function_type_check")


def test_factorial_type_check():
    ir_code = """
func factorial (n: Nat) -> Nat :
  match n with
  | 0 -> 1
  | n -> n * factorial (n - 1)
"""
    program = parse_ir(ir_code)
    result = check_types(program)

    assert result.success
    print("✓ test_factorial_type_check")


def test_if_expression_type_check():
    ir_code = """
func abs (x: Int) -> Int :
  if x >= 0 then x else -x
"""
    program = parse_ir(ir_code)
    result = check_types(program)

    assert result.success
    print("✓ test_if_expression_type_check")


def test_let_expression_type_check():
    ir_code = """
func compute (x: Int) -> Int :
  let y = x + 5 in y * 2
"""
    program = parse_ir(ir_code)
    result = check_types(program)

    assert result.success
    print("✓ test_let_expression_type_check")


def test_type_mismatch_in_return():
    ir_code = """
func bad_func (x: Int) -> Bool :
  x + 1
"""
    program = parse_ir(ir_code)
    result = check_types(program)

    assert not result.success
    assert len(result.errors) > 0
    assert "does not match declared return type" in result.errors[0].message
    print("✓ test_type_mismatch_in_return")


def test_type_mismatch_in_if_branches():
    ir_code = """
func bad_if (x: Int) -> Int :
  if x > 0 then x else true
"""
    program = parse_ir(ir_code)
    result = check_types(program)

    assert not result.success
    assert any("incompatible types" in err.message for err in result.errors)
    print("✓ test_type_mismatch_in_if_branches")


def test_undefined_variable():
    ir_code = """
func bad_var () -> Int :
  undefined_variable + 1
"""
    program = parse_ir(ir_code)
    result = check_types(program)

    assert not result.success
    assert any("Undefined variable" in err.message for err in result.errors)
    print("✓ test_undefined_variable")


def test_non_bool_condition():
    ir_code = """
func bad_cond (x: Int) -> Int :
  if x then 1 else 0
"""
    program = parse_ir(ir_code)
    result = check_types(program)

    assert not result.success
    assert any("condition must be Bool" in err.message for err in result.errors)
    print("✓ test_non_bool_condition")


def test_factorial_from_file():
    example_file = Path(__file__).parent.parent / "examples" / "01-factorial.pole-ir"

    if not example_file.exists():
        print(f"⊘ Skipping: {example_file} not found")
        return

    with open(example_file) as f:
        ir_code = f.read()

    program = parse_ir(ir_code)
    result = check_types(program)

    print(f"\n=== Type checking {example_file.name} ===")
    if result.success:
        print("✓ Type check passed")
    else:
        print(result)

    assert result.success
    print("✓ test_factorial_from_file")


def test_comparison_operators():
    ir_code = """
func is_positive (x: Int) -> Bool :
  x > 0

func is_equal (x: Int, y: Int) -> Bool :
  x == y
"""
    program = parse_ir(ir_code)
    result = check_types(program)

    assert result.success
    print("✓ test_comparison_operators")


def test_multiple_functions():
    ir_code = """
func add (x: Int, y: Int) -> Int :
  x + y

func square (x: Int) -> Int :
  x * x

func use_square (x: Int) -> Int :
  square x
"""
    program = parse_ir(ir_code)
    result = check_types(program)

    assert result.success
    print("✓ test_multiple_functions")


if __name__ == "__main__":
    print("Running type checker tests...\n")

    test_simple_function_type_check()
    test_factorial_type_check()
    test_if_expression_type_check()
    test_let_expression_type_check()
    test_type_mismatch_in_return()
    test_type_mismatch_in_if_branches()
    test_undefined_variable()
    test_non_bool_condition()
    test_factorial_from_file()
    test_comparison_operators()
    test_multiple_functions()

    print("\n✅ All type checker tests passed!")
