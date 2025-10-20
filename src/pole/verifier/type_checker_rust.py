"""
Hybrid type checker using Rust when available, falling back to Python.

This provides transparent type checking - callers don't need to know
whether Rust or Python is being used underneath.
"""

import sys
import os
from dataclasses import dataclass
from typing import Optional

try:
    rust_lib_path = os.path.join(
        os.path.dirname(os.path.dirname(os.path.dirname(os.path.dirname(__file__)))),
        "compiler",
        "target",
        "release",
    )
    if rust_lib_path not in sys.path:
        sys.path.insert(0, rust_lib_path)

    import pole_compiler

    RUST_AVAILABLE = True
except ImportError:
    RUST_AVAILABLE = False
    pole_compiler = None

from .type_checker import TypeChecker as PythonTypeChecker, TypeError, TypeCheckResult
from pole.runtime.ir_ast import Program


@dataclass
class TypeCheckResultRust:
    """Result from Rust type checker (converted to Python types)"""

    success: bool
    errors: list[TypeError]

    def __str__(self) -> str:
        if self.success:
            return "✓ Type check passed"

        lines = ["✗ Type check failed:", ""]
        for error in self.errors:
            if error.location:
                lines.append(f"  [{error.location}] {error.message}")
            else:
                lines.append(f"  {error.message}")

        return "\n".join(lines)


def check_types(program: Program, force_python: bool = False) -> TypeCheckResult:
    """
    Type check a Pole IR program.

    Uses Rust type checker when available (faster, more accurate),
    falls back to Python type checker.

    Args:
        program: IR Program AST
        force_python: If True, use Python type checker even if Rust is available

    Returns:
        TypeCheckResult with success status and any errors
    """
    use_rust = RUST_AVAILABLE and not force_python

    if use_rust:
        return _check_types_rust(program)
    else:
        return _check_types_python(program)


def _check_types_rust(program: Program) -> TypeCheckResult:
    """Type check using Rust (fast path)"""
    # Serialize program to IR string, then let Rust parse and check
    # This is a workaround - ideally we'd pass the AST directly
    # But PyO3 bindings for full AST would be complex

    # For now, we need to reconstruct the IR source
    # This is available if we cached it during parsing
    # Otherwise, we fall back to Python

    # Try to get the source from program metadata
    if hasattr(program, "_ir_source"):
        ir_source = program._ir_source
    else:
        # Fall back to Python if we don't have the source
        return _check_types_python(program)

    try:
        result_dict = pole_compiler.check_types_py(ir_source)

        errors = []
        for err_dict in result_dict.get("errors", []):
            errors.append(TypeError(message=err_dict["message"], location=err_dict.get("location")))

        return TypeCheckResult(success=result_dict["success"], errors=errors)
    except Exception as e:
        # If Rust fails, fall back to Python
        print(f"Warning: Rust type checker failed ({e}), using Python fallback")
        return _check_types_python(program)


def _check_types_python(program: Program) -> TypeCheckResult:
    """Type check using Python (fallback)"""
    checker = PythonTypeChecker(program)
    return checker.check()


def check_types_with_source(ir_source: str, force_python: bool = False) -> TypeCheckResult:
    """
    Type check IR source code directly.

    This is the preferred method when you have the IR source string,
    as it allows Rust to parse and check in one step.

    Args:
        ir_source: IR source code as string
        force_python: If True, use Python even if Rust is available

    Returns:
        TypeCheckResult
    """
    use_rust = RUST_AVAILABLE and not force_python

    if use_rust:
        try:
            result_dict = pole_compiler.check_types_py(ir_source)

            errors = []
            for err_dict in result_dict.get("errors", []):
                errors.append(
                    TypeError(message=err_dict["message"], location=err_dict.get("location"))
                )

            return TypeCheckResult(success=result_dict["success"], errors=errors)
        except Exception as e:
            # Fall back to Python
            print(f"Warning: Rust type checker failed ({e}), using Python fallback")
            from pole.runtime.ir_parser_rust import parse_ir

            program = parse_ir(ir_source, force_python=True)
            return _check_types_python(program)
    else:
        # Parse with Python and check
        from pole.runtime.ir_parser_rust import parse_ir

        program = parse_ir(ir_source, force_python=True)
        return _check_types_python(program)
