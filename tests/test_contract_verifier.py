import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent.parent / "src"))

from pole.runtime.ir_parser import parse_ir
from pole.verifier.contract_verifier import (
    ContractVerifier,
    ContractViolation,
    verify_contracts,
)


def test_precondition_success():
    ir_code = """
func factorial (n: Nat) -> Nat
  requires n >= 0
  ensures result >= 1
:
  match n with
  | 0 -> 1
  | n -> n * factorial (n - 1)
"""
    program = parse_ir(ir_code)
    verifier = ContractVerifier(program)

    success, result, violations = verifier.verify_function_contract("factorial", 5)

    assert success
    assert result == 120
    assert len(violations) == 0

    print("✓ test_precondition_success")


def test_precondition_failure():
    ir_code = """
func factorial (n: Nat) -> Nat
  requires n >= 0
  ensures result >= 1
:
  match n with
  | 0 -> 1
  | n -> n * factorial (n - 1)
"""
    program = parse_ir(ir_code)
    verifier = ContractVerifier(program)

    success, result, violations = verifier.verify_function_contract("factorial", -5)

    assert not success
    assert len(violations) > 0
    assert "Precondition failed" in violations[0]

    print("✓ test_precondition_failure")


def test_postcondition_verification():
    ir_code = """
func always_positive (n: Int) -> Int
  requires n >= 0
  ensures result > 0
:
  n + 1
"""
    program = parse_ir(ir_code)
    verifier = ContractVerifier(program)

    success, result, violations = verifier.verify_function_contract("always_positive", 5)

    assert success
    assert result == 6
    assert len(violations) == 0

    print("✓ test_postcondition_verification")


def test_postcondition_failure():
    ir_code = """
func broken_func (n: Int) -> Int
  requires n >= 0
  ensures result > 100
:
  n + 1
"""
    program = parse_ir(ir_code)
    verifier = ContractVerifier(program)

    success, result, violations = verifier.verify_function_contract("broken_func", 5)

    assert not success
    assert result == 6
    assert len(violations) > 0
    assert "Postcondition failed" in violations[0]

    print("✓ test_postcondition_failure")


def test_verify_contracts_function():
    ir_code = """
func add (x: Int, y: Int) -> Int
  requires x >= 0
  requires y >= 0
  ensures result >= 0
:
  x + y
"""
    program = parse_ir(ir_code)

    success, result = verify_contracts(program, "add", 5, 10)

    assert success
    assert result == 15

    print("✓ test_verify_contracts_function")


def test_verify_contracts_violation():
    ir_code = """
func add (x: Int, y: Int) -> Int
  requires x >= 0
  requires y >= 0
  ensures result >= 0
:
  x + y
"""
    program = parse_ir(ir_code)

    try:
        verify_contracts(program, "add", -5, 10)
        assert False, "Should have raised ContractViolation"
    except ContractViolation as e:
        assert "Contract violation" in str(e)
        assert "Precondition failed" in str(e)

    print("✓ test_verify_contracts_violation")


def test_multiple_contracts():
    ir_code = """
func safe_subtract (x: Int, y: Int) -> Int
  requires x >= y
  ensures result >= 0
:
  x - y
"""
    program = parse_ir(ir_code)
    verifier = ContractVerifier(program)

    success, result, violations = verifier.verify_function_contract("safe_subtract", 10, 5)

    assert success
    assert result == 5

    success, result, violations = verifier.verify_function_contract("safe_subtract", 5, 10)

    assert not success
    assert "Precondition failed" in violations[0]

    print("✓ test_multiple_contracts")


def test_factorial_contracts():
    ir_code = """
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
    verifier = ContractVerifier(program)

    all_passed, violations = verifier.verify_all_test_cases("factorial")

    assert all_passed, f"Violations: {violations}"
    assert len(violations) == 0

    print("✓ test_factorial_contracts")


if __name__ == "__main__":
    print("Running contract verifier tests...\n")

    test_precondition_success()
    test_precondition_failure()
    test_postcondition_verification()
    test_postcondition_failure()
    test_verify_contracts_function()
    test_verify_contracts_violation()
    test_multiple_contracts()
    test_factorial_contracts()

    print("\n✅ All contract verifier tests passed!")
