import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent.parent / "src"))

from pole.common.errors import (
    ErrorContext,
    PoleError,
    SourceLocation,
    format_error_with_context,
)


def test_source_location():
    loc = SourceLocation(file="test.pole", line=10, column=5)
    assert str(loc) == "test.pole:10:5"

    loc_no_col = SourceLocation(file="test.pole", line=10)
    assert str(loc_no_col) == "test.pole:10"

    print("✓ test_source_location")


def test_error_context():
    loc = SourceLocation(file="test.pole", line=5, column=10)
    context = ErrorContext(
        source_location=loc,
        source_line="  let x = undefined_variable + 1",
        highlight_start=10,
        highlight_length=18,
    )

    formatted = context.format()
    assert "test.pole:5:10" in formatted
    assert "let x = undefined_variable + 1" in formatted
    assert "^^^^^^^^^^^^^^^^^^" in formatted

    print("✓ test_error_context")


def test_pole_error():
    loc = SourceLocation(file="test.pole", line=5, column=10)
    context = ErrorContext(
        source_location=loc,
        source_line="  let x = undefined_variable + 1",
        highlight_start=10,
        highlight_length=18,
    )

    error = PoleError(
        message='Undefined variable "undefined_variable"',
        location=loc,
        context=context,
        suggestion='Did you mean "undefined_var"?',
    )

    error_str = str(error)
    assert "Undefined variable" in error_str
    assert "test.pole:5:10" in error_str
    assert "Suggestion:" in error_str

    print("✓ test_pole_error")


def test_format_error_with_context():
    source_lines = [
        "function factorial:",
        "  purpose: Calculate factorial",
        "  input: n (integer)",
        "  output: result",
        "  let x = undefined_variable + 1",
    ]

    error_msg = format_error_with_context(
        message='Undefined variable "undefined_variable"',
        file="test.pole",
        line_num=5,
        source_lines=source_lines,
        column=10,
        highlight_length=18,
        suggestion="Check if the variable is defined",
    )

    assert "Undefined variable" in error_msg
    assert "test.pole:5" in error_msg
    assert "let x = undefined_variable + 1" in error_msg

    print("✓ test_format_error_with_context")


if __name__ == "__main__":
    print("Running error system tests...\n")

    test_source_location()
    test_error_context()
    test_pole_error()
    test_format_error_with_context()

    print("\n✅ All error system tests passed!")
