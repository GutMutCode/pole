import re
from typing import Any

from .ir_ast import (
    Annotation,
    Application,
    BasicType,
    BinaryOp,
    Constructor,
    ConstructorPattern,
    Expr,
    FieldAccess,
    FunctionDef,
    FunctionType,
    IfExpr,
    Lambda,
    LetExpr,
    ListType,
    Literal,
    LiteralPattern,
    MatchExpr,
    OptionType,
    Pattern,
    Program,
    RecordExpr,
    RecordPattern,
    RecordType,
    ResultType,
    TupleExpr,
    TuplePattern,
    TupleType,
    Type,
    TypeDef,
    UnaryOp,
    Variable,
    VariablePattern,
    WildcardPattern,
)


class IRParser:
    def __init__(self, source: str):
        self.source = source
        self.lines = source.splitlines()
        self.pos = 0

    def parse(self) -> Program:
        type_defs = []
        func_defs = []

        while self.pos < len(self.lines):
            line = self.lines[self.pos].strip()

            if not line or line.startswith("//"):
                self.pos += 1
                continue

            if line.startswith("@"):
                annotations = self._parse_annotations()
                self.pos += 1
                line = self.lines[self.pos].strip()

                if line.startswith("type "):
                    type_def = self._parse_type_def(annotations)
                    type_defs.append(type_def)
                elif line.startswith("func "):
                    func_def = self._parse_function_def(annotations)
                    func_defs.append(func_def)
                else:
                    self.pos += 1
            elif line.startswith("type "):
                type_def = self._parse_type_def([])
                type_defs.append(type_def)
            elif line.startswith("func "):
                func_def = self._parse_function_def([])
                func_defs.append(func_def)
            else:
                self.pos += 1

        return Program(type_defs=type_defs, func_defs=func_defs)

    def _parse_annotations(self) -> list[Annotation]:
        annotations = []
        while self.pos < len(self.lines):
            line = self.lines[self.pos].strip()
            if not line.startswith("@"):
                break

            match = re.match(r"@(\w+)(?:\((.*)\))?", line)
            if match:
                name = match.group(1)
                args_str = match.group(2)
                args = self._parse_annotation_args(args_str) if args_str else {}
                annotations.append(Annotation(name=name, args=args))

            self.pos += 1

        self.pos -= 1
        return annotations

    def _parse_annotation_args(self, args_str: str) -> dict[str, Any]:
        args = {}
        if not args_str:
            return args

        for part in args_str.split(","):
            part = part.strip()
            if "=" in part:
                key, value = part.split("=", 1)
                key = key.strip()
                value = value.strip()

                if value.startswith('"') and value.endswith('"'):
                    args[key] = value[1:-1]
                elif value.startswith("{") and value.endswith("}"):
                    args[key] = value
                elif value == "true":
                    args[key] = True
                elif value == "false":
                    args[key] = False
                else:
                    try:
                        args[key] = int(value)
                    except ValueError:
                        try:
                            args[key] = float(value)
                        except ValueError:
                            args[key] = value

        return args

    def _parse_type_def(self, annotations: list[Annotation]) -> TypeDef:
        line = self.lines[self.pos].strip()
        self.pos += 1

        match = re.match(r"type (\w+) = (.+)", line)
        if match:
            name = match.group(1)
            type_def_str = match.group(2).strip()

            if type_def_str.startswith("{"):
                definition = self._parse_record_type_inline(type_def_str)
            else:
                definition = []

            return TypeDef(name=name, definition=definition, annotations=annotations)

        match = re.match(r"type (\w+) =", line)
        if match:
            name = match.group(1)
            constructors = []

            while self.pos < len(self.lines):
                line = self.lines[self.pos].strip()
                if not line or not line.startswith("|"):
                    break

                cons_match = re.match(r"\| (\w+)(?:\((.*)\))?", line)
                if cons_match:
                    cons_name = cons_match.group(1)
                    cons_args_str = cons_match.group(2)
                    cons_args = []
                    if cons_args_str:
                        for arg in cons_args_str.split(","):
                            cons_args.append(self._parse_type(arg.strip()))
                    constructors.append((cons_name, cons_args))

                self.pos += 1

            return TypeDef(name=name, definition=constructors, annotations=annotations)

        raise ValueError(f"Invalid type definition: {line}")

    def _parse_record_type_inline(self, type_str: str) -> RecordType:
        type_str = type_str.strip()
        if not (type_str.startswith("{") and type_str.endswith("}")):
            raise ValueError(f"Invalid record type: {type_str}")

        type_str = type_str[1:-1].strip()
        fields = {}

        if type_str:
            for field_str in type_str.split(","):
                field_str = field_str.strip()
                if ":" in field_str:
                    field_name, field_type_str = field_str.split(":", 1)
                    fields[field_name.strip()] = self._parse_type(field_type_str.strip())

        return RecordType(fields=fields)

    def _parse_type(self, type_str: str) -> Type:
        type_str = type_str.strip()

        if type_str in ["Int", "Nat", "Float64", "Bool", "String", "Unit"]:
            return BasicType(name=type_str)

        if type_str.startswith("Option<") and type_str.endswith(">"):
            inner_str = type_str[7:-1]
            return OptionType(inner=self._parse_type(inner_str))

        if type_str.startswith("Result<") and type_str.endswith(">"):
            inner_str = type_str[7:-1]
            parts = inner_str.split(",", 1)
            if len(parts) == 2:
                return ResultType(
                    ok_type=self._parse_type(parts[0].strip()),
                    err_type=self._parse_type(parts[1].strip()),
                )

        if type_str.startswith("List<") and type_str.endswith(">"):
            inner_str = type_str[5:-1]
            return ListType(element_type=self._parse_type(inner_str))

        if type_str.startswith("(") and type_str.endswith(")"):
            inner_str = type_str[1:-1]
            parts = [p.strip() for p in inner_str.split(",")]
            return TupleType(element_types=[self._parse_type(p) for p in parts])

        return BasicType(name=type_str)

    def _parse_function_def(self, annotations: list[Annotation]) -> FunctionDef:
        line = self.lines[self.pos].strip()
        self.pos += 1

        match = re.match(r"func (\w+)\s*\((.*?)\) -> (.+?)(?:\s*:)?$", line)
        if not match:
            raise ValueError(f"Invalid function definition: {line}")

        name = match.group(1)
        params_str = match.group(2)
        return_type_str = match.group(3).strip()

        params = []
        if params_str.strip():
            for param in params_str.split(","):
                param = param.strip()
                if ":" in param:
                    param_name, param_type = param.split(":", 1)
                    params.append((param_name.strip(), self._parse_type(param_type.strip())))

        return_type = self._parse_type(return_type_str.strip())

        requires = []
        ensures = []

        while self.pos < len(self.lines):
            line = self.lines[self.pos].strip()
            if line.startswith("requires "):
                req_expr = self._parse_simple_expr(line[9:].strip())
                requires.append(req_expr)
                self.pos += 1
            elif line.startswith("ensures "):
                ens_expr = self._parse_simple_expr(line[8:].strip())
                ensures.append(ens_expr)
                self.pos += 1
            elif line == ":":
                self.pos += 1
                break
            else:
                break

        body = self._parse_expr()

        return FunctionDef(
            name=name,
            params=params,
            return_type=return_type,
            requires=requires,
            ensures=ensures,
            body=body,
            annotations=annotations,
        )

    def _parse_expr(self) -> Expr:
        if self.pos >= len(self.lines):
            return Literal(value=None, type_name="Unit")

        line = self.lines[self.pos].strip()
        self.pos += 1

        if line.startswith("match "):
            return self._parse_match_expr(line)
        elif line.startswith("if "):
            return self._parse_if_expr(line)
        elif line.startswith("let "):
            return self._parse_let_expr(line)
        else:
            expr = self._parse_simple_expr(line)

            parts = line.split()
            if len(parts) >= 2 and not any(
                op in line for op in ["+", "-", "*", "/", "==", "!=", "<", ">", "<=", ">=", "=>"]
            ):
                if not line.startswith("(") and "(" not in parts[0]:
                    func_name = parts[0]
                    arg_str = " ".join(parts[1:])
                    return Application(
                        func=Variable(name=func_name), arg=self._parse_simple_expr(arg_str)
                    )

            return expr

    def _parse_simple_expr(self, expr_str: str) -> Expr:
        expr_str = expr_str.strip()

        if not expr_str:
            return Literal(value=None, type_name="Unit")

        if expr_str.startswith("-") and " " not in expr_str and not expr_str[1:].isdigit():
            return UnaryOp(op="-", operand=self._parse_simple_expr(expr_str[1:]))

        if expr_str.isdigit() or (expr_str.startswith("-") and expr_str[1:].isdigit()):
            return Literal(value=int(expr_str), type_name="Int")

        if expr_str in ["true", "false"]:
            return Literal(value=expr_str == "true", type_name="Bool")

        if expr_str.startswith('"') and expr_str.endswith('"'):
            return Literal(value=expr_str[1:-1], type_name="String")

        if expr_str == "()":
            return Literal(value=None, type_name="Unit")

        binary_ops = [
            (" + ", "+"),
            (" - ", "-"),
            ("==", "=="),
            ("!=", "!="),
            ("<=", "<="),
            (">=", ">="),
            (" < ", "<"),
            (" > ", ">"),
            (" * ", "*"),
            (" / ", "/"),
            (" % ", "%"),
        ]

        for op_str, op_name in binary_ops:
            if op_str != " > " or " => " not in expr_str:
                paren_depth = 0
                for i in range(len(expr_str) - len(op_str) + 1):
                    if expr_str[i] == "(":
                        paren_depth += 1
                    elif expr_str[i] == ")":
                        paren_depth -= 1
                    elif paren_depth == 0 and expr_str[i : i + len(op_str)] == op_str:
                        left_part = expr_str[:i].strip()
                        right_part = expr_str[i + len(op_str) :].strip()
                        return BinaryOp(
                            op=op_name,
                            left=self._parse_simple_expr(left_part),
                            right=self._parse_simple_expr(right_part),
                        )

        if "(" in expr_str and expr_str.endswith(")"):
            paren_pos = expr_str.index("(")
            func_name = expr_str[:paren_pos].strip()
            args_str = expr_str[paren_pos + 1 : -1].strip()

            if func_name and func_name[0].isupper():
                args = (
                    [self._parse_simple_expr(a.strip()) for a in args_str.split(",")]
                    if args_str
                    else []
                )
                return Constructor(name=func_name, args=args)
            elif func_name:
                arg = self._parse_simple_expr(args_str)
                return Application(func=Variable(name=func_name), arg=arg)

        if " => " in expr_str:
            parts = expr_str.split(" => ", 1)
            return BinaryOp(
                op="=>",
                left=self._parse_simple_expr(parts[0].strip()),
                right=self._parse_simple_expr(parts[1].strip()),
            )

        if "." in expr_str and not expr_str[0].isdigit():
            parts = expr_str.split(".", 1)
            return FieldAccess(
                record=self._parse_simple_expr(parts[0].strip()),
                field_name=parts[1].strip(),
            )

        return Variable(name=expr_str)

    def _parse_match_expr(self, line: str) -> MatchExpr:
        match_obj = re.match(r"match (.+?) with", line)
        if not match_obj:
            raise ValueError(f"Invalid match expression: {line}")

        scrutinee = self._parse_simple_expr(match_obj.group(1).strip())
        arms = []

        while self.pos < len(self.lines):
            line = self.lines[self.pos].strip()
            if not line.startswith("|"):
                break

            arm_match = re.match(r"\| (.+?) -> (.+)", line)
            if arm_match:
                pattern_str = arm_match.group(1).strip()
                expr_str = arm_match.group(2).strip()

                pattern = self._parse_pattern(pattern_str)
                expr = self._parse_simple_expr(expr_str)
                arms.append((pattern, expr))

            self.pos += 1

        return MatchExpr(scrutinee=scrutinee, arms=arms)

    def _parse_pattern(self, pattern_str: str) -> Pattern:
        pattern_str = pattern_str.strip()

        if pattern_str == "_":
            return WildcardPattern()

        if pattern_str.isdigit() or (pattern_str.startswith("-") and pattern_str[1:].isdigit()):
            return LiteralPattern(value=int(pattern_str))

        if pattern_str in ["true", "false"]:
            return LiteralPattern(value=pattern_str == "true")

        if pattern_str.startswith('"') and pattern_str.endswith('"'):
            return LiteralPattern(value=pattern_str[1:-1])

        if pattern_str[0].isupper() and "(" in pattern_str:
            paren_pos = pattern_str.index("(")
            cons_name = pattern_str[:paren_pos].strip()
            args_str = pattern_str[paren_pos + 1 : -1].strip()
            args = [self._parse_pattern(a.strip()) for a in args_str.split(",")] if args_str else []
            return ConstructorPattern(name=cons_name, args=args)

        if pattern_str[0].isupper():
            return ConstructorPattern(name=pattern_str, args=[])

        return VariablePattern(name=pattern_str)

    def _parse_if_expr(self, line: str) -> IfExpr:
        if "else" in line:
            else_pos = line.find(" else ")
            if_part = line[:else_pos]
            else_part = line[else_pos + 6 :].strip()

            match = re.match(r"if (.+?) then (.+)", if_part)
            if not match:
                raise ValueError(f"Invalid if expression: {line}")

            condition = self._parse_simple_expr(match.group(1).strip())
            then_branch = self._parse_simple_expr(match.group(2).strip())

            if else_part.startswith("if "):
                else_branch = self._parse_if_expr(else_part)
            else:
                else_branch = self._parse_simple_expr(else_part)

            return IfExpr(condition=condition, then_branch=then_branch, else_branch=else_branch)
        else:
            match = re.match(r"if (.+?) then", line)
            if not match:
                raise ValueError(f"Invalid if expression: {line}")

            condition = self._parse_simple_expr(match.group(1).strip())

            then_line = self.lines[self.pos].strip()
            self.pos += 1
            then_branch = self._parse_simple_expr(then_line)

            else_line = self.lines[self.pos].strip()
            if else_line == "else":
                self.pos += 1
                else_line = self.lines[self.pos].strip()
                self.pos += 1
            elif else_line.startswith("else"):
                else_line = else_line[4:].strip()
                self.pos += 1

            else_branch = self._parse_simple_expr(else_line)

            return IfExpr(condition=condition, then_branch=then_branch, else_branch=else_branch)

    def _parse_let_expr(self, line: str) -> LetExpr:
        match = re.match(r"let (\w+) = (.+?) in (.+)", line)
        if not match:
            raise ValueError(f"Invalid let expression: {line}")

        var_name = match.group(1).strip()
        value = self._parse_simple_expr(match.group(2).strip())
        body = self._parse_simple_expr(match.group(3).strip())

        return LetExpr(var_name=var_name, value=value, body=body)


def parse_ir(source: str) -> Program:
    parser = IRParser(source)
    return parser.parse()
