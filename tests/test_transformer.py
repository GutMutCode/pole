import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent.parent / "src"))

from pole.parser.spec_parser import SpecParser
from pole.transformer.llm_client import MockLLMClient
from pole.transformer.llm_transformer import (
    SpecificationTransformer,
    transform_specification,
)


def test_transform_function_basic():
    spec_text = """
function factorial:
  purpose: Calculate the factorial of a number
  input: n (non-negative integer)
  output: factorial of n
  
  example: factorial(0) = 1
  example: factorial(5) = 120
"""
    parser = SpecParser("test.pole", spec_text)
    spec = parser.parse()
    mock_client = MockLLMClient()

    ir_code = transform_specification(spec, "test.pole", mock_client)

    assert "func factorial" in ir_code
    assert "@source" in ir_code
    assert "@test_case" in ir_code


def test_transform_type_basic():
    spec_text = """
type User:
  - name: string
  - age: integer
  - email: string
"""
    parser = SpecParser("test.pole", spec_text)
    spec = parser.parse()
    mock_client = MockLLMClient()
    transformer = SpecificationTransformer(mock_client)

    ir_code = transformer.transform(spec, "test.pole")

    assert "type User" in ir_code


def test_clean_markdown_blocks():
    spec_text = """
function add:
  purpose: Add two numbers
  input: a, b (integers)
  output: sum of a and b
"""
    parser = SpecParser("test.pole", spec_text)
    spec = parser.parse()

    mock_response = """```
func add(a: Int, b: Int) -> Int:
  return a + b
```"""

    mock_client = MockLLMClient()
    mock_client.mock_response = mock_response

    transformer = SpecificationTransformer(mock_client)
    ir_code = transformer.transform(spec, "test.pole")

    assert not ir_code.startswith("```")
    assert not ir_code.endswith("```")
    assert "func add" in ir_code


def test_transform_with_constraints():
    spec_text = """
function divide:
  purpose: Divide two numbers
  input: numerator, denominator (numbers)
  output: result of division
  
  constraint: denominator must not be zero
  
  example: divide(10, 2) = 5
"""
    parser = SpecParser("test.pole", spec_text)
    spec = parser.parse()
    mock_client = MockLLMClient()

    ir_code = transform_specification(spec, "test.pole", mock_client)

    assert "func divide" in ir_code


def test_transform_example_file():
    example_file = Path(__file__).parent.parent / "examples" / "01-factorial.pole"

    if not example_file.exists():
        print(f"Skipping: {example_file} not found")
        return

    with open(example_file) as f:
        spec_text = f.read()

    parser = SpecParser("test.pole", spec_text)
    spec = parser.parse()
    mock_client = MockLLMClient()

    ir_code = transform_specification(spec, str(example_file), mock_client)

    assert "func factorial" in ir_code
    assert "@source" in ir_code
    print("\n=== Generated IR for 01-factorial.pole ===")
    print(ir_code)


if __name__ == "__main__":
    print("Running transformer tests...\n")

    test_transform_function_basic()
    print("✓ test_transform_function_basic")

    test_transform_type_basic()
    print("✓ test_transform_type_basic")

    test_clean_markdown_blocks()
    print("✓ test_clean_markdown_blocks")

    test_transform_with_constraints()
    print("✓ test_transform_with_constraints")

    test_transform_example_file()
    print("✓ test_transform_example_file")

    print("\n✅ All transformer tests passed!")
