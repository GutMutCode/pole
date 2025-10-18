import re
from pathlib import Path

from .ast_nodes import (
    Comment,
    Example,
    FunctionDef,
    Location,
    SourceInfo,
    Specification,
    TypeDef,
    TypeField,
)


class SpecParser:
    def __init__(self, filename: str, content: str):
        self.filename = filename
        self.content = content
        self.lines = content.splitlines()

    def parse(self) -> Specification:
        spec = Specification()
        i = 0

        while i < len(self.lines):
            line = self.lines[i].strip()

            if not line or line.startswith("//"):
                if line.startswith("//"):
                    comment = Comment(text=line[2:].strip(), is_multiline=False)
                    spec.comments.append(comment)
                i += 1
                continue

            if line.startswith("/*"):
                comment_lines = []
                while i < len(self.lines) and "*/" not in self.lines[i]:
                    comment_lines.append(self.lines[i])
                    i += 1
                if i < len(self.lines):
                    comment_lines.append(self.lines[i])
                    i += 1
                comment_text = "\n".join(comment_lines)
                comment = Comment(text=comment_text, is_multiline=True)
                spec.comments.append(comment)
                continue

            if line.startswith("type "):
                type_def, next_i = self._parse_type(i)
                spec.types.append(type_def)
                i = next_i
                continue

            if line.startswith("function "):
                func_def, next_i = self._parse_function(i)
                spec.functions.append(func_def)
                i = next_i
                continue

            i += 1

        return spec

    def _parse_type(self, start_i: int) -> tuple[TypeDef, int]:
        line = self.lines[start_i].strip()
        match = re.match(r"type\s+(\w+):", line)
        if not match:
            raise ValueError(f"Invalid type definition at line {start_i + 1}")

        type_name = match.group(1)
        fields: list[TypeField] = []

        i = start_i + 1
        in_fields_section = False

        while i < len(self.lines):
            line = self.lines[i].strip()

            if not line or line.startswith("//"):
                i += 1
                continue

            if line in ("function ", "type ") or (
                line.startswith("function ") or line.startswith("type ")
            ):
                break

            if line == "fields:":
                in_fields_section = True
                i += 1
                continue

            if in_fields_section and line.startswith("- "):
                field_text = line[2:].strip()
                parts = field_text.split(":", 1)
                if len(parts) >= 1:
                    field_name = parts[0].strip()
                    if len(parts) == 2:
                        type_and_desc = parts[1].strip()
                        desc_parts = type_and_desc.split(" - ", 1)
                        type_annotation = desc_parts[0].strip()
                        description = desc_parts[1].strip() if len(desc_parts) == 2 else None
                    else:
                        type_annotation = "unknown"
                        description = None

                    field = TypeField(
                        name=field_name, type_annotation=type_annotation, description=description
                    )
                    fields.append(field)
                i += 1
                continue

            if not line.startswith(" ") and not line.startswith("-"):
                break

            i += 1

        return TypeDef(name=type_name, fields=fields), i

    def _parse_function(self, start_i: int) -> tuple[FunctionDef, int]:
        line = self.lines[start_i].strip()
        match = re.match(r"function\s+(\w+):", line)
        if not match:
            raise ValueError(f"Invalid function definition at line {start_i + 1}")

        func_name = match.group(1)
        purpose: str | None = None
        input_desc: str | None = None
        output_desc: str | None = None
        constraints: list[str] = []
        examples: list[Example] = []
        notes: list[str] = []

        i = start_i + 1
        current_section: str | None = None
        current_example_input: str | None = None

        while i < len(self.lines):
            line = self.lines[i]
            stripped = line.strip()

            if not stripped or stripped.startswith("//"):
                i += 1
                continue

            if stripped.startswith("function ") or stripped.startswith("type "):
                break

            if stripped.startswith("purpose:"):
                purpose = stripped[8:].strip()
                current_section = None
                i += 1
                continue

            if stripped.startswith("input:"):
                input_content = stripped[6:].strip()
                if input_content:
                    input_desc = input_content
                else:
                    input_parts = []
                    i += 1
                    while i < len(self.lines):
                        line = self.lines[i]
                        if line.strip().startswith("- "):
                            input_parts.append(line.strip())
                            i += 1
                        elif not line.strip() or line.strip().startswith("//"):
                            i += 1
                        else:
                            break
                    input_desc = "\n".join(input_parts) if input_parts else None
                    continue
                i += 1
                continue

            if stripped.startswith("output:"):
                output_desc = stripped[7:].strip()
                current_section = None
                i += 1
                continue

            if stripped == "constraints:":
                current_section = "constraints"
                i += 1
                continue

            if stripped == "examples:":
                current_section = "examples"
                i += 1
                continue

            if stripped == "note:" or stripped == "notes:":
                current_section = "notes"
                i += 1
                continue

            if current_section == "constraints" and stripped.startswith("- "):
                constraints.append(stripped[2:].strip())
                i += 1
                continue

            if current_section == "examples" and stripped.startswith("- "):
                example_text = stripped[2:].strip()
                if " → " in example_text or "->" in example_text:
                    if " → " in example_text:
                        parts = example_text.split(" → ", 1)
                    else:
                        parts = example_text.split("->", 1)
                    if len(parts) == 2:
                        examples.append(
                            Example(input_desc=parts[0].strip(), output_desc=parts[1].strip())
                        )
                elif "input:" in example_text:
                    current_example_input = example_text.split("input:", 1)[1].strip()
                i += 1
                continue

            if current_section == "examples" and "output:" in stripped:
                output = stripped.split("output:", 1)[1].strip()
                if current_example_input:
                    examples.append(Example(input_desc=current_example_input, output_desc=output))
                    current_example_input = None
                i += 1
                continue

            if current_section == "notes" and stripped.startswith("- "):
                notes.append(stripped[2:].strip())
                i += 1
                continue

            if not stripped.startswith(" ") and current_section:
                current_section = None

            i += 1

        return (
            FunctionDef(
                name=func_name,
                purpose=purpose,
                input_desc=input_desc,
                output_desc=output_desc,
                constraints=constraints,
                examples=examples,
                notes=notes,
            ),
            i,
        )


def parse_file(filepath: str) -> Specification:
    path = Path(filepath)
    content = path.read_text(encoding="utf-8")
    parser = SpecParser(filename=filepath, content=content)
    return parser.parse()
