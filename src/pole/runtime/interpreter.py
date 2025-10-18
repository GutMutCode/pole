from typing import Any

from .ir_ast import (
    Application,
    BinaryOp,
    Constructor,
    ConstructorPattern,
    Expr,
    FieldAccess,
    FunctionDef,
    IfExpr,
    LetExpr,
    Literal,
    LiteralPattern,
    MatchExpr,
    Pattern,
    Program,
    RecordExpr,
    TupleExpr,
    TuplePattern,
    UnaryOp,
    Variable,
    VariablePattern,
    WildcardPattern,
)


class InterpreterError(Exception):
    pass


class Interpreter:
    def __init__(self, program: Program):
        self.program = program
        self.functions = {f.name: f for f in program.func_defs}
        self.env: dict[str, Any] = {}

    def call_function(self, name: str, *args: Any) -> Any:
        if name not in self.functions:
            raise InterpreterError(f"Function '{name}' not found")

        func_def = self.functions[name]

        if len(args) != len(func_def.params):
            raise InterpreterError(
                f"Function '{name}' expects {len(func_def.params)} arguments, got {len(args)}"
            )

        local_env = self.env.copy()
        for (param_name, _), arg_value in zip(func_def.params, args):
            local_env[param_name] = arg_value

        old_env = self.env
        self.env = local_env

        try:
            result = self.eval_expr(func_def.body)
            return result
        finally:
            self.env = old_env

    def eval_expr(self, expr: Expr) -> Any:
        if isinstance(expr, Literal):
            return expr.value

        elif isinstance(expr, Variable):
            if expr.name in self.env:
                return self.env[expr.name]
            elif expr.name in self.functions:
                return expr.name
            raise InterpreterError(f"Variable '{expr.name}' not found")

        elif isinstance(expr, BinaryOp):
            left_val = self.eval_expr(expr.left)
            right_val = self.eval_expr(expr.right)

            if expr.op == "+":
                return left_val + right_val
            elif expr.op == "-":
                return left_val - right_val
            elif expr.op == "*":
                return left_val * right_val
            elif expr.op == "/":
                return left_val / right_val
            elif expr.op == "==":
                return left_val == right_val
            elif expr.op == "!=":
                return left_val != right_val
            elif expr.op == "<":
                return left_val < right_val
            elif expr.op == ">":
                return left_val > right_val
            elif expr.op == "<=":
                return left_val <= right_val
            elif expr.op == ">=":
                return left_val >= right_val
            elif expr.op == "and":
                return left_val and right_val
            elif expr.op == "or":
                return left_val or right_val
            elif expr.op == "=>":
                return (not left_val) or right_val
            else:
                raise InterpreterError(f"Unknown binary operator: {expr.op}")

        elif isinstance(expr, UnaryOp):
            operand_val = self.eval_expr(expr.operand)
            if expr.op == "-":
                return -operand_val
            elif expr.op == "not":
                return not operand_val
            else:
                raise InterpreterError(f"Unknown unary operator: {expr.op}")

        elif isinstance(expr, IfExpr):
            condition_val = self.eval_expr(expr.condition)
            if condition_val:
                return self.eval_expr(expr.then_branch)
            else:
                return self.eval_expr(expr.else_branch)

        elif isinstance(expr, LetExpr):
            value = self.eval_expr(expr.value)
            old_env = self.env.copy()
            self.env[expr.var_name] = value
            try:
                result = self.eval_expr(expr.body)
                return result
            finally:
                self.env = old_env

        elif isinstance(expr, MatchExpr):
            scrutinee_val = self.eval_expr(expr.scrutinee)

            for pattern, body in expr.arms:
                bindings = self.match_pattern(pattern, scrutinee_val)
                if bindings is not None:
                    old_env = self.env.copy()
                    self.env.update(bindings)
                    try:
                        result = self.eval_expr(body)
                        return result
                    finally:
                        self.env = old_env

            raise InterpreterError("Non-exhaustive pattern match")

        elif isinstance(expr, Application):
            func_val = self.eval_expr(expr.func)
            arg_val = self.eval_expr(expr.arg)

            if isinstance(func_val, str):
                return self.call_function(func_val, arg_val)
            elif callable(func_val):
                return func_val(arg_val)
            else:
                raise InterpreterError(f"Cannot apply non-function: {func_val}")

        elif isinstance(expr, Constructor):
            args = [self.eval_expr(arg) for arg in expr.args]
            return (expr.name, *args) if args else expr.name

        elif isinstance(expr, TupleExpr):
            return tuple(self.eval_expr(elem) for elem in expr.elements)

        elif isinstance(expr, RecordExpr):
            return {field: self.eval_expr(value) for field, value in expr.fields.items()}

        elif isinstance(expr, FieldAccess):
            record_val = self.eval_expr(expr.record)
            if isinstance(record_val, dict):
                if expr.field_name in record_val:
                    return record_val[expr.field_name]
                raise InterpreterError(f"Field '{expr.field_name}' not found")
            raise InterpreterError(f"Cannot access field on non-record: {record_val}")

        else:
            raise InterpreterError(f"Unknown expression type: {type(expr)}")

    def match_pattern(self, pattern: Pattern, value: Any) -> dict[str, Any] | None:
        if isinstance(pattern, WildcardPattern):
            return {}

        elif isinstance(pattern, VariablePattern):
            return {pattern.name: value}

        elif isinstance(pattern, LiteralPattern):
            if pattern.value == value:
                return {}
            return None

        elif isinstance(pattern, ConstructorPattern):
            if isinstance(value, tuple) and len(value) > 0:
                if value[0] == pattern.name:
                    if len(value) - 1 == len(pattern.args):
                        bindings = {}
                        for i, arg_pattern in enumerate(pattern.args):
                            arg_bindings = self.match_pattern(arg_pattern, value[i + 1])
                            if arg_bindings is None:
                                return None
                            bindings.update(arg_bindings)
                        return bindings
            elif isinstance(value, str) and value == pattern.name and not pattern.args:
                return {}
            return None

        elif isinstance(pattern, TuplePattern):
            if isinstance(value, tuple) and len(value) == len(pattern.elements):
                bindings = {}
                for elem_pattern, elem_value in zip(pattern.elements, value):
                    elem_bindings = self.match_pattern(elem_pattern, elem_value)
                    if elem_bindings is None:
                        return None
                    bindings.update(elem_bindings)
                return bindings
            return None

        else:
            raise InterpreterError(f"Unknown pattern type: {type(pattern)}")


def interpret(program: Program, function_name: str, *args: Any) -> Any:
    interpreter = Interpreter(program)
    return interpreter.call_function(function_name, *args)
