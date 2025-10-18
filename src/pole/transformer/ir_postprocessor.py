from dataclasses import dataclass
from typing import Optional

from pole.runtime.ir_parser import parse_ir


@dataclass
class PostprocessResult:
    """Result of IR code postprocessing"""

    success: bool
    ir_code: str
    original_code: str
    fixes_applied: list[str]
    parse_error: Optional[str] = None


class IRPostprocessor:
    """Automatically fix common LLM-generated IR code issues"""

    def process(self, ir_code: str) -> PostprocessResult:
        """
        Process IR code and attempt to fix common issues

        Returns PostprocessResult with:
        - success: True if code parses successfully after fixes
        - ir_code: The (potentially fixed) IR code
        - original_code: The original input code
        - fixes_applied: List of fixes that were applied
        - parse_error: Error message if parsing still fails
        """
        original_code = ir_code
        fixes_applied = []

        ir_code = self._remove_explanation_text(ir_code, fixes_applied)
        ir_code = self._normalize_whitespace(ir_code, fixes_applied)

        try:
            parse_ir(ir_code)
            return PostprocessResult(
                success=True,
                ir_code=ir_code,
                original_code=original_code,
                fixes_applied=fixes_applied,
            )
        except Exception as e:
            return PostprocessResult(
                success=False,
                ir_code=ir_code,
                original_code=original_code,
                fixes_applied=fixes_applied,
                parse_error=str(e),
            )

    def _remove_explanation_text(self, ir_code: str, fixes: list[str]) -> str:
        """Remove common explanation text patterns"""
        lines = ir_code.split("\n")
        filtered_lines = []
        removed_lines = []

        explanation_patterns = [
            "here is",
            "here's",
            "the code",
            "the function",
            "implementation",
            "this function",
            "this code",
            "note:",
            "explanation:",
        ]

        for line in lines:
            stripped = line.strip().lower()

            if not stripped:
                filtered_lines.append(line)
                continue

            is_explanation = any(pattern in stripped for pattern in explanation_patterns)

            if (
                is_explanation
                and not line.strip().startswith("@")
                and not line.strip().startswith("func")
            ):
                removed_lines.append(line.strip()[:50])
                continue

            filtered_lines.append(line)

        if removed_lines:
            fixes.append(f"Removed explanation text: {len(removed_lines)} lines")

        return "\n".join(filtered_lines)

    def _normalize_whitespace(self, ir_code: str, fixes: list[str]) -> str:
        """Normalize whitespace and remove trailing spaces"""
        lines = ir_code.split("\n")

        normalized_lines = []
        changes = 0

        for line in lines:
            original = line
            line = line.rstrip()

            if original != line:
                changes += 1

            normalized_lines.append(line)

        if changes > 0:
            fixes.append(f"Normalized whitespace: {changes} lines")

        result = "\n".join(normalized_lines)

        while "\n\n\n" in result:
            result = result.replace("\n\n\n", "\n\n")
            changes += 1

        if changes > 0:
            fixes.append("Removed excessive blank lines")

        return result.strip()
