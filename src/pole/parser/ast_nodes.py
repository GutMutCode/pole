from dataclasses import dataclass, field


@dataclass
class Location:
    line: int
    column: int


@dataclass
class SourceInfo:
    filename: str
    start: Location
    end: Location


@dataclass
class TypeField:
    name: str
    type_annotation: str
    description: str | None = None
    source_info: SourceInfo | None = None


@dataclass
class TypeDef:
    name: str
    fields: list[TypeField] = field(default_factory=list)
    source_info: SourceInfo | None = None


@dataclass
class Example:
    input_desc: str
    output_desc: str
    note: str | None = None
    source_info: SourceInfo | None = None


@dataclass
class FunctionDef:
    name: str
    purpose: str | None = None
    input_desc: str | None = None
    output_desc: str | None = None
    constraints: list[str] = field(default_factory=list)
    examples: list[Example] = field(default_factory=list)
    notes: list[str] = field(default_factory=list)
    source_info: SourceInfo | None = None


@dataclass
class Comment:
    text: str
    is_multiline: bool = False
    source_info: SourceInfo | None = None


@dataclass
class Specification:
    types: list[TypeDef] = field(default_factory=list)
    functions: list[FunctionDef] = field(default_factory=list)
    comments: list[Comment] = field(default_factory=list)
    source_info: SourceInfo | None = None
