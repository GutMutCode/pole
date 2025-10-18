from dataclasses import dataclass
from typing import Any

from pole.runtime.ir_ast import (
    Application,
    BasicType,
    BinaryOp,
    Constructor,
    Expr,
    FieldAccess,
    FunctionDef,
    FunctionType,
    IfExpr,
    LetExpr,
    ListType,
    Literal,
    MatchExpr,
    OptionType,
    Pattern,
    Program,
    RecordExpr,
    RecordType,
    ResultType,
    TupleExpr,
    TupleType,
    Type,
    TypeDef,
    UnaryOp,
    Variable,
)


@dataclass
class TypeError:
    message: str
    location: str | None = None


@dataclass
class TypeCheckResult:
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


class TypeChecker:
    def __init__(self, program: Program):
        self.program = program
        self.type_env: dict[str, Type] = {}
        self.function_types: dict[str, FunctionType] = {}
        self.custom_types: dict[str, TypeDef] = {}
        self.errors: list[TypeError] = []

        self._initialize_builtins()
        self._collect_type_definitions()
        self._collect_function_signatures()

    def _initialize_builtins(self):
        pass

    def _collect_type_definitions(self):
        for type_def in self.program.type_defs:
            self.custom_types[type_def.name] = type_def

    def _collect_function_signatures(self):
        for func_def in self.program.func_defs:
            if len(func_def.params) == 0:
                func_type = FunctionType(
                    param_type=BasicType(name="Unit"),
                    return_type=func_def.return_type,
                )
            elif len(func_def.params) == 1:
                func_type = FunctionType(
                    param_type=func_def.params[0][1], return_type=func_def.return_type
                )
            else:
                param_types = [p[1] for p in func_def.params]
                func_type = FunctionType(
                    param_type=TupleType(element_types=param_types),
                    return_type=func_def.return_type,
                )

            self.function_types[func_def.name] = func_type

    def check(self) -> TypeCheckResult:
        self.errors = []

        for func_def in self.program.func_defs:
            self._check_function(func_def)

        return TypeCheckResult(success=len(self.errors) == 0, errors=self.errors)

    def _check_function(self, func_def: FunctionDef):
        local_env = self.type_env.copy()

        for param_name, param_type in func_def.params:
            local_env[param_name] = param_type

        old_env = self.type_env
        self.type_env = local_env

        try:
            body_type = self._infer_type(func_def.body)

            if not self._types_compatible(body_type, func_def.return_type):
                self.errors.append(
                    TypeError(
                        message=f"Function '{func_def.name}' body type {self._type_to_string(body_type)} does not match declared return type {self._type_to_string(func_def.return_type)}",
                        location=func_def.name,
                    )
                )
        finally:
            self.type_env = old_env

    def _infer_type(self, expr: Expr) -> Type:
        if isinstance(expr, Literal):
            return self._literal_type(expr)

        elif isinstance(expr, Variable):
            if expr.name in self.type_env:
                return self.type_env[expr.name]
            elif expr.name in self.function_types:
                return self.function_types[expr.name]
            else:
                self.errors.append(
                    TypeError(message=f"Undefined variable '{expr.name}'", location=expr.name)
                )
                return BasicType(name="Unknown")

        elif isinstance(expr, BinaryOp):
            left_type = self._infer_type(expr.left)
            right_type = self._infer_type(expr.right)

            if expr.op in ["+", "-", "*", "/"]:
                if self._is_numeric_type(left_type) and self._is_numeric_type(right_type):
                    return left_type
                else:
                    self.errors.append(
                        TypeError(
                            message=f"Binary operator '{expr.op}' requires numeric types, got {self._type_to_string(left_type)} and {self._type_to_string(right_type)}"
                        )
                    )
                    return BasicType(name="Unknown")

            elif expr.op in ["==", "!=", "<", ">", "<=", ">="]:
                return BasicType(name="Bool")

            elif expr.op in ["and", "or", "=>"]:
                return BasicType(name="Bool")

            else:
                return BasicType(name="Unknown")

        elif isinstance(expr, UnaryOp):
            operand_type = self._infer_type(expr.operand)

            if expr.op == "-":
                if self._is_numeric_type(operand_type):
                    return operand_type
                else:
                    self.errors.append(
                        TypeError(
                            message=f"Unary operator '-' requires numeric type, got {self._type_to_string(operand_type)}"
                        )
                    )
                    return BasicType(name="Unknown")

            elif expr.op == "not":
                return BasicType(name="Bool")

            else:
                return BasicType(name="Unknown")

        elif isinstance(expr, IfExpr):
            cond_type = self._infer_type(expr.condition)

            if not self._types_compatible(cond_type, BasicType(name="Bool")):
                self.errors.append(
                    TypeError(
                        message=f"If condition must be Bool, got {self._type_to_string(cond_type)}"
                    )
                )

            then_type = self._infer_type(expr.then_branch)
            else_type = self._infer_type(expr.else_branch)

            if not self._types_compatible(then_type, else_type):
                self.errors.append(
                    TypeError(
                        message=f"If branches have incompatible types: {self._type_to_string(then_type)} and {self._type_to_string(else_type)}"
                    )
                )

            return then_type

        elif isinstance(expr, LetExpr):
            value_type = self._infer_type(expr.value)

            old_env = self.type_env.copy()
            self.type_env[expr.var_name] = value_type

            body_type = self._infer_type(expr.body)

            self.type_env = old_env

            return body_type

        elif isinstance(expr, MatchExpr):
            scrutinee_type = self._infer_type(expr.scrutinee)

            if not expr.arms:
                self.errors.append(TypeError(message="Match expression must have at least one arm"))
                return BasicType(name="Unknown")

            first_arm_type = None
            for pattern, body in expr.arms:
                arm_type = self._infer_type(body)

                if first_arm_type is None:
                    first_arm_type = arm_type
                elif not self._types_compatible(arm_type, first_arm_type):
                    self.errors.append(
                        TypeError(
                            message=f"Match arms have incompatible types: {self._type_to_string(first_arm_type)} and {self._type_to_string(arm_type)}"
                        )
                    )

            return first_arm_type if first_arm_type else BasicType(name="Unknown")

        elif isinstance(expr, Application):
            func_type = self._infer_type(expr.func)
            arg_type = self._infer_type(expr.arg)

            if isinstance(func_type, FunctionType):
                if not self._types_compatible(arg_type, func_type.param_type):
                    self.errors.append(
                        TypeError(
                            message=f"Function argument type mismatch: expected {self._type_to_string(func_type.param_type)}, got {self._type_to_string(arg_type)}"
                        )
                    )

                return func_type.return_type
            else:
                self.errors.append(
                    TypeError(
                        message=f"Cannot apply non-function type: {self._type_to_string(func_type)}"
                    )
                )
                return BasicType(name="Unknown")

        elif isinstance(expr, Constructor):
            return BasicType(name=expr.name)

        elif isinstance(expr, TupleExpr):
            element_types = [self._infer_type(elem) for elem in expr.elements]
            return TupleType(element_types=element_types)

        elif isinstance(expr, RecordExpr):
            field_types = {field: self._infer_type(value) for field, value in expr.fields.items()}
            return RecordType(fields=field_types)

        elif isinstance(expr, FieldAccess):
            record_type = self._infer_type(expr.record)
            record_type = self._resolve_type(record_type)

            if isinstance(record_type, RecordType):
                if expr.field_name in record_type.fields:
                    return record_type.fields[expr.field_name]
                else:
                    self.errors.append(
                        TypeError(message=f"Field '{expr.field_name}' not found in record type")
                    )
                    return BasicType(name="Unknown")
            else:
                self.errors.append(
                    TypeError(
                        message=f"Cannot access field on non-record type: {self._type_to_string(record_type)}"
                    )
                )
                return BasicType(name="Unknown")

        else:
            return BasicType(name="Unknown")

    def _literal_type(self, literal: Literal) -> Type:
        type_map = {
            "Int": "Int",
            "Nat": "Nat",
            "Float64": "Float64",
            "Bool": "Bool",
            "String": "String",
            "Unit": "Unit",
        }

        return BasicType(name=type_map.get(literal.type_name, "Unknown"))

    def _is_numeric_type(self, t: Type) -> bool:
        if isinstance(t, BasicType):
            return t.name in ["Int", "Nat", "Float64"]
        return False

    def _types_compatible(self, t1: Type, t2: Type) -> bool:
        if isinstance(t1, BasicType) and isinstance(t2, BasicType):
            if t1.name == "Unknown" or t2.name == "Unknown":
                return True
            if t1.name == "Nat" and t2.name == "Int":
                return True
            if t1.name == "Int" and t2.name == "Nat":
                return True
            return t1.name == t2.name

        elif isinstance(t1, OptionType) and isinstance(t2, OptionType):
            return self._types_compatible(t1.inner, t2.inner)

        elif isinstance(t1, ResultType) and isinstance(t2, ResultType):
            return self._types_compatible(t1.ok_type, t2.ok_type) and self._types_compatible(
                t1.err_type, t2.err_type
            )

        elif isinstance(t1, ListType) and isinstance(t2, ListType):
            return self._types_compatible(t1.element_type, t2.element_type)

        elif isinstance(t1, TupleType) and isinstance(t2, TupleType):
            if len(t1.element_types) != len(t2.element_types):
                return False
            return all(
                self._types_compatible(e1, e2) for e1, e2 in zip(t1.element_types, t2.element_types)
            )

        elif isinstance(t1, FunctionType) and isinstance(t2, FunctionType):
            return self._types_compatible(t1.param_type, t2.param_type) and self._types_compatible(
                t1.return_type, t2.return_type
            )

        else:
            return False

    def _resolve_type(self, t: Type) -> Type:
        """Resolve custom type names to their definitions"""
        if isinstance(t, BasicType) and t.name in self.custom_types:
            type_def = self.custom_types[t.name]
            if isinstance(type_def.definition, RecordType):
                return type_def.definition
            elif isinstance(type_def.definition, Type):
                return self._resolve_type(type_def.definition)
            else:
                return t
        return t

    def _type_to_string(self, t: Type) -> str:
        if isinstance(t, BasicType):
            return t.name

        elif isinstance(t, OptionType):
            return f"Option<{self._type_to_string(t.inner)}>"

        elif isinstance(t, ResultType):
            return f"Result<{self._type_to_string(t.ok_type)}, {self._type_to_string(t.err_type)}>"

        elif isinstance(t, ListType):
            return f"List<{self._type_to_string(t.element_type)}>"

        elif isinstance(t, TupleType):
            types = ", ".join(self._type_to_string(e) for e in t.element_types)
            return f"({types})"

        elif isinstance(t, RecordType):
            fields = ", ".join(f"{k}: {self._type_to_string(v)}" for k, v in t.fields.items())
            return f"{{{fields}}}"

        elif isinstance(t, FunctionType):
            return f"{self._type_to_string(t.param_type)} -> {self._type_to_string(t.return_type)}"

        else:
            return "Unknown"


def check_types(program: Program) -> TypeCheckResult:
    checker = TypeChecker(program)
    return checker.check()
