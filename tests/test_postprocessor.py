import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent.parent / "src"))

from pole.transformer.ir_postprocessor import IRPostprocessor


def test_postprocessor_valid_code():
    """Test that valid IR code passes through unchanged"""
    postprocessor = IRPostprocessor()

    valid_code = """
@source("test.pole")
func add(x: Int, y: Int) -> Int
  requires true
:
  x + y
""".strip()

    result = postprocessor.process(valid_code)

    assert result.success
    assert result.parse_error is None
    assert len(result.fixes_applied) >= 0


def test_postprocessor_remove_explanation():
    """Test removal of explanation text"""
    postprocessor = IRPostprocessor()

    code_with_explanation = """
Here is the implementation:

@source("test.pole")
func add(x: Int, y: Int) -> Int
  requires true
:
  x + y

This function adds two numbers.
""".strip()

    result = postprocessor.process(code_with_explanation)

    assert result.success
    assert any("explanation" in fix.lower() for fix in result.fixes_applied)
    assert "Here is" not in result.ir_code
    assert "This function" not in result.ir_code


def test_postprocessor_normalize_whitespace():
    """Test whitespace normalization"""
    postprocessor = IRPostprocessor()

    code_with_whitespace = """
@source("test.pole")   
func add(x: Int, y: Int) -> Int   
  requires true   
:
  x + y   


""".strip()

    result = postprocessor.process(code_with_whitespace)

    assert result.success
    assert any("whitespace" in fix.lower() for fix in result.fixes_applied)
    assert not result.ir_code.endswith("   ")


def test_postprocessor_invalid_syntax():
    """Test that invalid syntax is detected"""
    postprocessor = IRPostprocessor()

    invalid_code = """
@source("test.pole")
func (x: Int) -> Int
:
  x + 1
""".strip()

    result = postprocessor.process(invalid_code)

    assert not result.success
    assert result.parse_error is not None


def test_postprocessor_multiple_fixes():
    """Test that multiple fixes can be applied"""
    postprocessor = IRPostprocessor()

    messy_code = """
Here is the code:

@source("test.pole")   
func add(x: Int, y: Int) -> Int   
  requires true   
:
  x + y   



Note: This adds two integers.
""".strip()

    result = postprocessor.process(messy_code)

    assert result.success
    assert len(result.fixes_applied) > 0
    assert "Here is" not in result.ir_code
    assert "Note:" not in result.ir_code


def test_postprocessor_preserves_annotations():
    """Test that annotations are preserved"""
    postprocessor = IRPostprocessor()

    code = """
@source("test.pole", line=1)
@test_case(input=2, expected=2)
@test_case(input=5, expected=120)
func factorial(n: Nat) -> Nat
  requires n >= 0
:
  match n with
  | 0 -> 1
  | n -> n * factorial(n - 1)
""".strip()

    result = postprocessor.process(code)

    assert result.success
    assert "@source" in result.ir_code
    assert "@test_case" in result.ir_code


if __name__ == "__main__":
    test_postprocessor_valid_code()
    test_postprocessor_remove_explanation()
    test_postprocessor_normalize_whitespace()
    test_postprocessor_invalid_syntax()
    test_postprocessor_multiple_fixes()
    test_postprocessor_preserves_annotations()
    print("All tests passed!")
