from dataclasses import dataclass
from typing import Optional


@dataclass
class SourceLocation:
    file: str
    line: int
    column: int = 0

    def __str__(self) -> str:
        if self.column > 0:
            return f"{self.file}:{self.line}:{self.column}"
        return f"{self.file}:{self.line}"


@dataclass
class ErrorContext:
    source_location: Optional[SourceLocation]
    source_line: Optional[str] = None
    highlight_start: int = 0
    highlight_length: int = 0

    def format(self) -> str:
        lines = []

        if self.source_location:
            lines.append(f"  at {self.source_location}")
            lines.append("")

        if self.source_line:
            lines.append(f"  {self.source_line}")
            if self.highlight_length > 0:
                pointer = " " * self.highlight_start + "^" * self.highlight_length
                lines.append(f"  {pointer}")

        return "\n".join(lines)


class PoleError(Exception):
    def __init__(
        self,
        message: str,
        location: Optional[SourceLocation] = None,
        context: Optional[ErrorContext] = None,
        suggestion: Optional[str] = None,
    ):
        super().__init__(message)
        self.message = message
        self.location = location
        self.context = context
        self.suggestion = suggestion

    def __str__(self) -> str:
        lines = [f"Error: {self.message}"]

        if self.context:
            lines.append("")
            lines.append(self.context.format())

        if self.suggestion:
            lines.append("")
            lines.append(f"💡 Suggestion: {self.suggestion}")

        return "\n".join(lines)


class ParseError(PoleError):
    pass


class ValidationError(PoleError):
    pass


class IRParseError(PoleError):
    pass


class TypeCheckError(PoleError):
    pass


class RuntimeError(PoleError):
    pass


class TransformError(PoleError):
    pass


def format_error_with_context(
    message: str,
    file: str,
    line_num: int,
    source_lines: list[str],
    column: int = 0,
    highlight_length: int = 0,
    suggestion: Optional[str] = None,
) -> str:
    location = SourceLocation(file=file, line=line_num, column=column)

    source_line = None
    if 0 <= line_num - 1 < len(source_lines):
        source_line = source_lines[line_num - 1].rstrip()

    context = ErrorContext(
        source_location=location,
        source_line=source_line,
        highlight_start=column,
        highlight_length=highlight_length,
    )

    error = PoleError(message=message, location=location, context=context, suggestion=suggestion)
    return str(error)
