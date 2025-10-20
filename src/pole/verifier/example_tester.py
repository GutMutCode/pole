from dataclasses import dataclass
from typing import Any

from pole.runtime.interpreter import Interpreter
from pole.runtime.ir_ast import FunctionDef, Program


@dataclass
class TestCase:
    function_name: str
    input_value: Any
    expected_output: Any
    source_annotation: dict[str, Any]


@dataclass
class TestResult:
    test_case: TestCase
    passed: bool
    actual_output: Any | None = None
    error: str | None = None


@dataclass
class TestReport:
    total: int
    passed: int
    failed: int
    results: list[TestResult]

    @property
    def success_rate(self) -> float:
        return (self.passed / self.total * 100) if self.total > 0 else 0.0

    def __str__(self) -> str:
        lines = [
            "=" * 60,
            "Test Report",
            "=" * 60,
            f"Total: {self.total}",
            f"Passed: {self.passed} ({self.success_rate:.1f}%)",
            f"Failed: {self.failed}",
            "",
        ]

        if self.failed > 0:
            lines.append("Failed Tests:")
            lines.append("-" * 60)
            for result in self.results:
                if not result.passed:
                    tc = result.test_case
                    lines.append(f"Function: {tc.function_name}")
                    lines.append(f"  Input: {tc.input_value}")
                    lines.append(f"  Expected: {tc.expected_output}")
                    lines.append(
                        f"  Actual: {result.actual_output if result.error is None else f'ERROR: {result.error}'}"
                    )
                    lines.append("")

        lines.append("=" * 60)
        return "\n".join(lines)


class ExampleTester:
    def __init__(self, program: Program):
        self.program = program
        self.interpreter = Interpreter(program)

    def extract_test_cases(self) -> list[TestCase]:
        test_cases = []

        for func_def in self.program.func_defs:
            for annotation in func_def.annotations:
                if annotation.name == "test_case":
                    test_case = self._parse_test_case_annotation(func_def, annotation.args)
                    if test_case:
                        test_cases.append(test_case)

        return test_cases

    def _parse_test_case_annotation(
        self, func_def: FunctionDef, args: dict[str, Any]
    ) -> TestCase | None:
        if "input" not in args or "expected" not in args:
            return None

        input_value = self._parse_value(args["input"])
        expected_output = self._parse_value(args["expected"])

        return TestCase(
            function_name=func_def.name,
            input_value=input_value,
            expected_output=expected_output,
            source_annotation=args,
        )

    def _parse_value(self, value: Any) -> Any:
        """Parse string values to appropriate types (for Rust parser compatibility)"""
        if not isinstance(value, str):
            return value

        # Try int
        try:
            return int(value)
        except ValueError:
            pass

        # Try float
        try:
            return float(value)
        except ValueError:
            pass

        # Try bool
        if value.lower() == "true":
            return True
        elif value.lower() == "false":
            return False

        # Return as string
        return value

    def run_test(self, test_case: TestCase) -> TestResult:
        try:
            input_args = (
                test_case.input_value
                if isinstance(test_case.input_value, tuple)
                else (test_case.input_value,)
            )

            actual_output = self.interpreter.call_function(test_case.function_name, *input_args)

            passed = actual_output == test_case.expected_output

            return TestResult(test_case=test_case, passed=passed, actual_output=actual_output)

        except Exception as e:
            return TestResult(test_case=test_case, passed=False, actual_output=None, error=str(e))

    def run_all_tests(self) -> TestReport:
        test_cases = self.extract_test_cases()
        results = []

        for test_case in test_cases:
            result = self.run_test(test_case)
            results.append(result)

        passed = sum(1 for r in results if r.passed)
        failed = sum(1 for r in results if not r.passed)

        return TestReport(total=len(results), passed=passed, failed=failed, results=results)

    def verify_examples(self) -> bool:
        report = self.run_all_tests()
        return report.failed == 0


def test_program(program: Program, verbose: bool = True) -> TestReport:
    tester = ExampleTester(program)
    report = tester.run_all_tests()

    if verbose:
        print(report)

    return report
