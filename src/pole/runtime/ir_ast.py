from dataclasses import dataclass
from typing import Any


@dataclass
class Type:
    pass


@dataclass
class BasicType(Type):
    name: str


@dataclass
class OptionType(Type):
    inner: Type


@dataclass
class ResultType(Type):
    ok_type: Type
    err_type: Type


@dataclass
class ListType(Type):
    element_type: Type


@dataclass
class TupleType(Type):
    element_types: list[Type]


@dataclass
class RecordType(Type):
    fields: dict[str, Type]


@dataclass
class FunctionType(Type):
    param_type: Type
    return_type: Type
    effect: str | None = None


@dataclass
class Expr:
    pass


@dataclass
class Literal(Expr):
    value: Any
    type_name: str


@dataclass
class Variable(Expr):
    name: str


@dataclass
class Lambda(Expr):
    params: list[str]
    body: Expr


@dataclass
class Application(Expr):
    func: Expr
    arg: Expr


@dataclass
class LetExpr(Expr):
    var_name: str
    value: Expr
    body: Expr


@dataclass
class IfExpr(Expr):
    condition: Expr
    then_branch: Expr
    else_branch: Expr


@dataclass
class MatchExpr(Expr):
    scrutinee: Expr
    arms: list[tuple["Pattern", Expr]]


@dataclass
class Constructor(Expr):
    name: str
    args: list[Expr]


@dataclass
class TupleExpr(Expr):
    elements: list[Expr]


@dataclass
class RecordExpr(Expr):
    fields: dict[str, Expr]


@dataclass
class FieldAccess(Expr):
    record: Expr
    field_name: str


@dataclass
class BinaryOp(Expr):
    op: str
    left: Expr
    right: Expr


@dataclass
class UnaryOp(Expr):
    op: str
    operand: Expr


@dataclass
class Pattern:
    pass


@dataclass
class WildcardPattern(Pattern):
    pass


@dataclass
class VariablePattern(Pattern):
    name: str


@dataclass
class LiteralPattern(Pattern):
    value: Any


@dataclass
class ConstructorPattern(Pattern):
    name: str
    args: list[Pattern]


@dataclass
class TuplePattern(Pattern):
    elements: list[Pattern]


@dataclass
class RecordPattern(Pattern):
    fields: dict[str, Pattern]


@dataclass
class Annotation:
    name: str
    args: dict[str, Any]


@dataclass
class FunctionDef:
    name: str
    params: list[tuple[str, Type]]
    return_type: Type
    requires: list[Expr]
    ensures: list[Expr]
    body: Expr
    annotations: list[Annotation]


@dataclass
class TypeDef:
    name: str
    definition: Type | list[tuple[str, list[Type]]]
    annotations: list[Annotation]


@dataclass
class Program:
    type_defs: list[TypeDef]
    func_defs: list[FunctionDef]
