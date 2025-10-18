import sys
import os
from typing import Any

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

from .ir_parser import IRParser as PythonIRParser
from .ir_ast import Program


class IRParser:
    """
    Hybrid IR Parser that uses Rust when available, falls back to Python.

    This provides a transparent interface - callers don't need to know
    whether Rust or Python is being used underneath.
    """

    def __init__(self, source: str, force_python: bool = False):
        self.source = source
        self.use_rust = RUST_AVAILABLE and not force_python

    def parse(self) -> Program:
        """Parse IR source into a Program AST."""
        if self.use_rust:
            return self._parse_rust()
        else:
            return self._parse_python()

    def _parse_rust(self) -> Program:
        """Parse using Rust (fast path)."""
        result_dict = pole_compiler.parse_ir(self.source)
        return self._dict_to_program(result_dict)

    def _parse_python(self) -> Program:
        """Parse using Python (fallback)."""
        parser = PythonIRParser(self.source)
        return parser.parse()

    def _dict_to_program(self, data: dict) -> Program:
        """Convert Rust parser output (dict) to Python AST classes."""
        from .ir_ast import (
            Program,
            FunctionDef,
            Annotation,
            Type,
            BasicType,
            OptionType,
            ResultType,
            ListType,
            TupleType,
            Expr,
            Literal,
            Variable,
            BinaryOp,
            Application,
            IfExpr,
            LetExpr,
            MatchExpr,
            Pattern,
            LiteralPattern,
            VariablePattern,
            WildcardPattern,
            ConstructorPattern,
        )

        def convert_type(t: dict) -> Type:
            kind = t["kind"]
            if kind == "Basic":
                return BasicType(name=t["name"])
            elif kind == "Option":
                return OptionType(inner=convert_type(t["inner"]))
            elif kind == "Result":
                return ResultType(ok_type=convert_type(t["ok"]), err_type=convert_type(t["err"]))
            elif kind == "List":
                return ListType(element_type=convert_type(t["inner"]))
            elif kind == "Tuple":
                return TupleType(element_types=[convert_type(e) for e in t["elements"]])
            else:
                raise ValueError(f"Unknown type kind: {kind}")

        def convert_expr(e: dict) -> Expr:
            expr_type = e["type"]
            if expr_type == "Literal":
                return Literal(value=e["value"], type_name="auto")
            elif expr_type == "Variable":
                return Variable(name=e["name"])
            elif expr_type == "BinaryOp":
                return BinaryOp(
                    op=e["op"], left=convert_expr(e["left"]), right=convert_expr(e["right"])
                )
            elif expr_type == "Application":
                return Application(
                    func=convert_expr(e["function"]), arg=convert_expr(e["argument"])
                )
            elif expr_type == "If":
                return IfExpr(
                    condition=convert_expr(e["condition"]),
                    then_branch=convert_expr(e["then"]),
                    else_branch=convert_expr(e["else"]),
                )
            elif expr_type == "Let":
                return LetExpr(
                    var_name=e["name"], value=convert_expr(e["value"]), body=convert_expr(e["body"])
                )
            elif expr_type == "Match":
                cases = [
                    (convert_pattern(c["pattern"]), convert_expr(c["body"])) for c in e["cases"]
                ]
                return MatchExpr(scrutinee=convert_expr(e["scrutinee"]), arms=cases)
            else:
                raise ValueError(f"Unknown expression type: {expr_type}")

        def convert_pattern(p: dict) -> Pattern:
            pat_type = p["type"]
            if pat_type == "Literal":
                return LiteralPattern(value=p["value"])
            elif pat_type == "Variable":
                return VariablePattern(name=p["name"])
            elif pat_type == "Wildcard":
                return WildcardPattern()
            elif pat_type == "Constructor":
                return ConstructorPattern(
                    name=p["name"], args=[convert_pattern(arg) for arg in p["args"]]
                )
            else:
                raise ValueError(f"Unknown pattern type: {pat_type}")

        functions = []
        for func_dict in data.get("functions", []):
            params = [(p["name"], convert_type(p["type"])) for p in func_dict["parameters"]]

            annotations = [
                Annotation(name=ann["name"], args=ann.get("args", {}))
                for ann in func_dict.get("annotations", [])
            ]

            requires = [convert_expr(req) for req in func_dict.get("requires", [])]

            ensures = [convert_expr(ens) for ens in func_dict.get("ensures", [])]

            func = FunctionDef(
                name=func_dict["name"],
                params=params,
                return_type=convert_type(func_dict["return_type"]),
                requires=requires,
                ensures=ensures,
                body=convert_expr(func_dict["body"]),
                annotations=annotations,
            )
            functions.append(func)

        return Program(type_defs=[], func_defs=functions)


def parse_ir(source: str, force_python: bool = False) -> Program:
    """
    Parse Pole IR source code into an AST.

    Uses Rust parser when available (10-100x faster), falls back to Python.

    Args:
        source: IR source code as string
        force_python: If True, use Python parser even if Rust is available

    Returns:
        Program AST
    """
    parser = IRParser(source, force_python=force_python)
    return parser.parse()
