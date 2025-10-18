import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent.parent / "src"))

from pole.runtime.ir_parser import parse_ir
from pole.verifier.example_tester import ExampleTester, test_program


def test_extract_test_cases():
    ir_code = """
@test_case(input=0, expected=1)
@test_case(input=5, expected=120)
func factorial (n: Nat) -> Nat :
  match n with
  | 0 -> 1
  | n -> n * factorial (n - 1)
"""
    program = parse_ir(ir_code)
    tester = ExampleTester(program)

    test_cases = tester.extract_test_cases()

    assert len(test_cases) == 2
    assert test_cases[0].function_name == "factorial"
    assert test_cases[0].input_value == 0
    assert test_cases[0].expected_output == 1
    assert test_cases[1].input_value == 5
    assert test_cases[1].expected_output == 120

    print("✓ test_extract_test_cases")


def test_run_single_test():
    ir_code = """
@test_case(input=5, expected=120)
func factorial (n: Nat) -> Nat :
  match n with
  | 0 -> 1
  | n -> n * factorial (n - 1)
"""
    program = parse_ir(ir_code)
    tester = ExampleTester(program)

    test_cases = tester.extract_test_cases()
    result = tester.run_test(test_cases[0])

    assert result.passed
    assert result.actual_output == 120
    assert result.error is None

    print("✓ test_run_single_test")


def test_run_all_tests():
    ir_code = """
@test_case(input=0, expected=1)
@test_case(input=1, expected=1)
@test_case(input=5, expected=120)
func factorial (n: Nat) -> Nat :
  match n with
  | 0 -> 1
  | n -> n * factorial (n - 1)
"""
    program = parse_ir(ir_code)
    report = test_program(program, verbose=False)

    assert report.total == 3
    assert report.passed == 3
    assert report.failed == 0
    assert report.success_rate == 100.0

    print("✓ test_run_all_tests")


def test_failed_test():
    ir_code = """
@test_case(input=5, expected=999)
func add_one (n: Int) -> Int :
  n + 1
"""
    program = parse_ir(ir_code)
    report = test_program(program, verbose=False)

    assert report.total == 1
    assert report.passed == 0
    assert report.failed == 1
    assert report.results[0].actual_output == 6
    assert report.results[0].test_case.expected_output == 999

    print("✓ test_failed_test")


def test_verify_examples():
    ir_code = """
@test_case(input=0, expected=1)
@test_case(input=5, expected=120)
func factorial (n: Nat) -> Nat :
  match n with
  | 0 -> 1
  | n -> n * factorial (n - 1)
"""
    program = parse_ir(ir_code)
    tester = ExampleTester(program)

    assert tester.verify_examples() == True

    print("✓ test_verify_examples")


def test_factorial_from_file():
    example_file = Path(__file__).parent.parent / "examples" / "01-factorial.pole-ir"

    if not example_file.exists():
        print(f"⊘ Skipping: {example_file} not found")
        return

    with open(example_file) as f:
        ir_code = f.read()

    program = parse_ir(ir_code)
    report = test_program(program, verbose=False)

    print(f"\n=== Testing {example_file.name} ===")
    print(f"Total: {report.total}, Passed: {report.passed}, Failed: {report.failed}")

    assert report.total > 0
    assert report.passed == report.total

    print("✓ test_factorial_from_file")


def test_report_formatting():
    ir_code = """
@test_case(input=5, expected=999)
@test_case(input=10, expected=1000)
func broken_func (n: Int) -> Int :
  n + 1
"""
    program = parse_ir(ir_code)
    report = test_program(program, verbose=False)

    report_str = str(report)

    assert "Test Report" in report_str
    assert "Total: 2" in report_str
    assert "Passed: 0" in report_str
    assert "Failed: 2" in report_str
    assert "Function: broken_func" in report_str

    print("✓ test_report_formatting")


if __name__ == "__main__":
    print("Running example tester tests...\n")

    test_extract_test_cases()
    test_run_single_test()
    test_run_all_tests()
    test_failed_test()
    test_verify_examples()
    test_factorial_from_file()
    test_report_formatting()

    print("\n✅ All example tester tests passed!")
