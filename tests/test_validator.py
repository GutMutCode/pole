from pathlib import Path
import sys

sys.path.insert(0, str(Path(__file__).parent.parent / "src"))

from pole.parser.ast_nodes import FunctionDef, Specification, TypeDef, TypeField  # type: ignore
from pole.validator.spec_validator import validate_specification  # type: ignore
from pole.validator.validation_result import IssueSeverity, IssueType  # type: ignore
from pole.parser.spec_parser import parse_file  # type: ignore


def test_validate_complete_function() -> None:
    func = FunctionDef(
        name="test_func",
        purpose="test purpose",
        input_desc="x (integer)",
        output_desc="result (integer)",
        constraints=["x >= 0"],
        examples=[],
    )
    spec = Specification(functions=[func])

    result = validate_specification(spec)

    errors = result.get_errors()
    assert len(errors) == 0


def test_validate_missing_purpose() -> None:
    func = FunctionDef(
        name="test_func",
        purpose=None,
        input_desc="x (integer)",
        output_desc="result (integer)",
    )
    spec = Specification(functions=[func])

    result = validate_specification(spec)

    errors = result.get_errors()
    assert len(errors) == 1
    assert errors[0].issue_type == IssueType.MISSING_FIELD
    assert "purpose" in errors[0].message


def test_validate_missing_input() -> None:
    func = FunctionDef(
        name="test_func",
        purpose="test purpose",
        input_desc=None,
        output_desc="result (integer)",
    )
    spec = Specification(functions=[func])

    result = validate_specification(spec)

    errors = result.get_errors()
    assert any("input" in error.message for error in errors)


def test_validate_missing_output() -> None:
    func = FunctionDef(
        name="test_func",
        purpose="test purpose",
        input_desc="x (integer)",
        output_desc=None,
    )
    spec = Specification(functions=[func])

    result = validate_specification(spec)

    errors = result.get_errors()
    assert any("output" in error.message for error in errors)


def test_validate_missing_examples_warning() -> None:
    func = FunctionDef(
        name="test_func",
        purpose="test purpose",
        input_desc="x (integer)",
        output_desc="result (integer)",
        examples=[],
    )
    spec = Specification(functions=[func])

    result = validate_specification(spec)

    warnings = result.get_warnings()
    assert any(warning.issue_type == IssueType.MISSING_EXAMPLES for warning in warnings)


def test_detect_ambiguous_efficiently() -> None:
    func = FunctionDef(
        name="test_func",
        purpose="test purpose",
        input_desc="x (integer)",
        output_desc="result (integer)",
        constraints=["compute efficiently"],
    )
    spec = Specification(functions=[func])

    result = validate_specification(spec)

    warnings = result.get_warnings()
    assert any(
        warning.issue_type == IssueType.AMBIGUOUS_EXPRESSION and "efficiently" in warning.message
        for warning in warnings
    )


def test_detect_overflow_ambiguity() -> None:
    func = FunctionDef(
        name="test_func",
        purpose="test purpose",
        input_desc="x (integer)",
        output_desc="result (integer)",
        constraints=["handle overflow safely"],
    )
    spec = Specification(functions=[func])

    result = validate_specification(spec)

    warnings = result.get_warnings()
    overflow_warnings = [w for w in warnings if "overflow" in w.message.lower()]
    assert len(overflow_warnings) > 0
    assert len(overflow_warnings[0].options) > 0


def test_detect_email_validation_ambiguity() -> None:
    func = FunctionDef(
        name="test_func",
        purpose="test purpose",
        input_desc="email (string)",
        output_desc="result (boolean)",
        constraints=["email must be valid format"],
    )
    spec = Specification(functions=[func])

    result = validate_specification(spec)

    warnings = result.get_warnings()
    email_warnings = [w for w in warnings if "valid" in w.message.lower()]
    assert len(email_warnings) > 0


def test_validate_type_missing_fields() -> None:
    type_def = TypeDef(name="User", fields=[])
    spec = Specification(types=[type_def])

    result = validate_specification(spec)

    errors = result.get_errors()
    assert any("필드" in error.message for error in errors)


def test_validate_type_field_missing_type() -> None:
    field = TypeField(name="age", type_annotation="unknown", description=None)
    type_def = TypeDef(name="User", fields=[field])
    spec = Specification(types=[type_def])

    result = validate_specification(spec)

    errors = result.get_errors()
    assert any("타입" in error.message and "age" in error.message for error in errors)


def test_validate_factorial_example() -> None:
    spec = parse_file("examples/01-factorial.pole")
    result = validate_specification(spec)

    warnings = result.get_warnings()
    assert any("overflow" in w.message.lower() for w in warnings)


def test_validate_fibonacci_example() -> None:
    spec = parse_file("examples/02-fibonacci.pole")
    result = validate_specification(spec)

    warnings = result.get_warnings()
    assert any("efficiently" in w.message.lower() for w in warnings)


def test_validate_user_validation_example() -> None:
    spec = parse_file("examples/03-user-validation.pole")
    result = validate_specification(spec)

    warnings = result.get_warnings()
    email_warnings = [
        w for w in warnings if "email" in w.message.lower() or "valid" in w.message.lower()
    ]
    assert len(email_warnings) > 0
