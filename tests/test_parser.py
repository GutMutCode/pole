from pathlib import Path
import sys

sys.path.insert(0, str(Path(__file__).parent.parent / "src"))

from pole.parser.spec_parser import SpecParser, parse_file  # type: ignore


def test_parse_simple_function() -> None:
    content = """
function factorial:
  purpose: calculate factorial
  input: n (integer)
  output: n! (integer)
"""
    parser = SpecParser(filename="test.pole", content=content)
    spec = parser.parse()

    assert len(spec.functions) == 1
    func = spec.functions[0]
    assert func.name == "factorial"
    assert func.purpose == "calculate factorial"
    assert func.input_desc == "n (integer)"
    assert func.output_desc == "n! (integer)"


def test_parse_function_with_constraints() -> None:
    content = """
function factorial:
  purpose: calculate factorial
  input: n (non-negative integer)
  output: n! (integer)
  
  constraints:
    - n >= 0
    - handle overflow safely if n is too large
"""
    parser = SpecParser(filename="test.pole", content=content)
    spec = parser.parse()

    assert len(spec.functions) == 1
    func = spec.functions[0]
    assert len(func.constraints) == 2
    assert func.constraints[0] == "n >= 0"
    assert func.constraints[1] == "handle overflow safely if n is too large"


def test_parse_function_with_examples() -> None:
    content = """
function factorial:
  purpose: calculate factorial
  input: n (integer)
  output: n! (integer)
  
  examples:
    - 0 → 1
    - 5 → 120
"""
    parser = SpecParser(filename="test.pole", content=content)
    spec = parser.parse()

    assert len(spec.functions) == 1
    func = spec.functions[0]
    assert len(func.examples) == 2
    assert func.examples[0].input_desc == "0"
    assert func.examples[0].output_desc == "1"
    assert func.examples[1].input_desc == "5"
    assert func.examples[1].output_desc == "120"


def test_parse_type_definition() -> None:
    content = """
type User:
  fields:
    - name: string - user's name
    - email: string - email address
    - age: integer - user's age
"""
    parser = SpecParser(filename="test.pole", content=content)
    spec = parser.parse()

    assert len(spec.types) == 1
    type_def = spec.types[0]
    assert type_def.name == "User"
    assert len(type_def.fields) == 3
    assert type_def.fields[0].name == "name"
    assert type_def.fields[0].type_annotation == "string"
    assert type_def.fields[0].description == "user's name"


def test_parse_comments() -> None:
    content = """
// This is a comment
function factorial:
  purpose: calculate factorial
"""
    parser = SpecParser(filename="test.pole", content=content)
    spec = parser.parse()

    assert len(spec.comments) == 1
    assert spec.comments[0].text == "This is a comment"
    assert spec.comments[0].is_multiline is False


def test_parse_factorial_example() -> None:
    spec = parse_file("examples/01-factorial.pole")

    assert len(spec.functions) == 1
    func = spec.functions[0]
    assert func.name == "factorial"
    assert func.purpose == "calculate factorial of given integer"
    assert "n >= 0" in func.constraints
    assert len(func.examples) >= 3


def test_parse_fibonacci_example() -> None:
    spec = parse_file("examples/02-fibonacci.pole")

    assert len(spec.functions) == 1
    func = spec.functions[0]
    assert func.name == "fibonacci"
    assert "n >= 1" in func.constraints


def test_parse_user_validation_example() -> None:
    spec = parse_file("examples/03-user-validation.pole")

    assert len(spec.types) == 1
    assert spec.types[0].name == "User"
    assert len(spec.functions) == 1
    assert spec.functions[0].name == "validate_user"
