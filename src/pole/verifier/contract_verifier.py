import sys
from pathlib import Path
from typing import Any

sys.path.insert(0, str(Path(__file__).parent.parent.parent))

from pole.common.errors import PoleError, RuntimeError as PoleRuntimeError
from pole.runtime.interpreter import Interpreter
from pole.runtime.ir_ast import FunctionDef, Program


class ContractViolation(PoleRuntimeError):
    pass


class ContractVerifier:
    def __init__(self, program: Program):
        self.program = program
        self.interpreter = Interpreter(program)

    def verify_function_contract(self, func_name: str, *args: Any) -> tuple[bool, Any, list[str]]:
        """
        Verify a function's contracts (requires/ensures).

        Returns:
            (success, result, violations)
            - success: True if all contracts satisfied
            - result: function return value (if executed)
            - violations: list of contract violation messages
        """
        func_def = self.interpreter.functions.get(func_name)
        if not func_def:
            raise ValueError(f"Function '{func_name}' not found")

        violations = []

        local_env = {}
        for (param_name, _), arg_value in zip(func_def.params, args):
            local_env[param_name] = arg_value

        old_env = self.interpreter.env
        self.interpreter.env = local_env.copy()

        try:
            for i, requires_expr in enumerate(func_def.requires):
                try:
                    result = self.interpreter.eval_expr(requires_expr)
                    if not result:
                        violations.append(
                            f"Precondition failed (requires #{i + 1}): {self._expr_to_string(requires_expr)}"
                        )
                except Exception as e:
                    violations.append(f"Precondition evaluation error (requires #{i + 1}): {e}")

            if violations:
                return (False, None, violations)

            result = self.interpreter.eval_expr(func_def.body)

            self.interpreter.env["result"] = result

            for i, ensures_expr in enumerate(func_def.ensures):
                try:
                    check_result = self.interpreter.eval_expr(ensures_expr)
                    if not check_result:
                        violations.append(
                            f"Postcondition failed (ensures #{i + 1}): {self._expr_to_string(ensures_expr)}"
                        )
                except Exception as e:
                    violations.append(f"Postcondition evaluation error (ensures #{i + 1}): {e}")

            success = len(violations) == 0
            return (success, result, violations)

        finally:
            self.interpreter.env = old_env

    def _expr_to_string(self, expr) -> str:
        """Simple expression to string converter for error messages"""
        from pole.runtime.ir_ast import BinaryOp, Literal, Variable

        if isinstance(expr, Literal):
            return str(expr.value)
        elif isinstance(expr, Variable):
            return expr.name
        elif isinstance(expr, BinaryOp):
            left = self._expr_to_string(expr.left)
            right = self._expr_to_string(expr.right)
            return f"{left} {expr.op} {right}"
        else:
            return str(expr)

    def verify_all_test_cases(self, func_name: str) -> tuple[bool, list[str]]:
        """
        Verify contracts for all @test_case annotations of a function.

        Returns:
            (all_passed, violations)
        """
        func_def = self.interpreter.functions.get(func_name)
        if not func_def:
            raise ValueError(f"Function '{func_name}' not found")

        all_violations = []

        for annotation in func_def.annotations:
            if annotation.name == "test_case":
                input_value = annotation.args.get("input")
                expected = annotation.args.get("expected")

                if input_value is None or expected is None:
                    continue

                args = input_value if isinstance(input_value, tuple) else (input_value,)

                success, result, violations = self.verify_function_contract(func_name, *args)

                if not success:
                    all_violations.append(f"Test case {annotation.args}: {', '.join(violations)}")
                elif result != expected:
                    all_violations.append(
                        f"Test case {annotation.args}: Expected {expected}, got {result}"
                    )

        return (len(all_violations) == 0, all_violations)


def verify_contracts(program: Program, func_name: str, *args: Any) -> tuple[bool, Any]:
    """
    Verify function contracts and execute.

    Returns:
        (success, result)

    Raises:
        ContractViolation if contracts are violated
    """
    verifier = ContractVerifier(program)
    success, result, violations = verifier.verify_function_contract(func_name, *args)

    if not success:
        violation_msg = "\n".join(f"  - {v}" for v in violations)
        raise ContractViolation(f"Contract violation in function '{func_name}':\n{violation_msg}")

    return (success, result)
