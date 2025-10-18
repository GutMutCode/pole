"""
Test IR Parser Improvements for Phase 4.1 & 4.2

This test suite verifies that LLM-generated code can be parsed
and executed without manual fixes.
"""

import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent.parent / "src"))

from pole.runtime.ir_parser import parse_ir
from pole.runtime.interpreter import Interpreter


def test_function_without_space_before_paren():
    """LLM often generates func name(args) without space"""
    code = """
func is_even(n: Int) -> Bool:
  n % 2 == 0
"""
    prog = parse_ir(code)
    assert len(prog.func_defs) == 1
    assert prog.func_defs[0].name == "is_even"


def test_modulo_operator_parsing():
    """Test % operator is parsed correctly"""
    code = """
func mod_test(n: Int) -> Bool:
  n % 2 == 0
"""
    prog = parse_ir(code)
    interp = Interpreter(prog)

    assert interp.call_function("mod_test", 4) == True
    assert interp.call_function("mod_test", 5) == False


def test_nested_if_expression():
    """Test nested if...else if...else"""
    code = """
func classify(n: Int) -> String:
  if n == 0 then "zero" else if n > 0 then "positive" else "negative"
"""
    prog = parse_ir(code)
    interp = Interpreter(prog)

    assert interp.call_function("classify", 0) == "zero"
    assert interp.call_function("classify", 5) == "positive"
    assert interp.call_function("classify", -3) == "negative"


def test_complex_expression_with_function_call():
    """Test n * factorial (n - 1) parses correctly"""
    code = """
func factorial(n: Nat) -> Nat:
  match n with
  | 0 -> 1
  | n -> n * factorial (n - 1)
"""
    prog = parse_ir(code)
    interp = Interpreter(prog)

    assert interp.call_function("factorial", 0) == 1
    assert interp.call_function("factorial", 5) == 120


def test_comment_support():
    """Test // comments are ignored"""
    code = """
// This is a comment
func add(x: Int) -> Int:
  // Another comment
  x + 1
"""
    prog = parse_ir(code)
    assert len(prog.func_defs) == 1
    assert prog.func_defs[0].name == "add"


def test_operator_precedence():
    """Test operators are parsed with correct precedence"""
    code = """
func test(a: Int, b: Int, c: Int) -> Int:
  a + b * c
"""
    prog = parse_ir(code)
    interp = Interpreter(prog)

    # Should be: a + (b * c), not (a + b) * c
    assert interp.call_function("test", 1, 2, 3) == 7  # 1 + (2*3) = 7


def test_multiple_operators_in_expression():
    """Test multiple operators in one expression"""
    code = """
func check(n: Int) -> Bool:
  n + 1 == 5
"""
    prog = parse_ir(code)
    interp = Interpreter(prog)

    assert interp.call_function("check", 4) == True
    assert interp.call_function("check", 3) == False
