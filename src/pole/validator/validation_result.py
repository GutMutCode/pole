from dataclasses import dataclass, field
from enum import Enum


class IssueSeverity(Enum):
    ERROR = "error"
    WARNING = "warning"
    INFO = "info"


class IssueType(Enum):
    MISSING_FIELD = "missing_field"
    AMBIGUOUS_EXPRESSION = "ambiguous_expression"
    INCOMPLETE_CONSTRAINT = "incomplete_constraint"
    MISSING_EXAMPLES = "missing_examples"
    CONFLICTING_REQUIREMENTS = "conflicting_requirements"


@dataclass
class ValidationIssue:
    severity: IssueSeverity
    issue_type: IssueType
    location: str
    message: str
    question: str | None = None
    options: list[str] = field(default_factory=list)


@dataclass
class ValidationResult:
    is_valid: bool
    issues: list[ValidationIssue] = field(default_factory=list)

    def has_errors(self) -> bool:
        return any(issue.severity == IssueSeverity.ERROR for issue in self.issues)

    def has_warnings(self) -> bool:
        return any(issue.severity == IssueSeverity.WARNING for issue in self.issues)

    def get_errors(self) -> list[ValidationIssue]:
        return [issue for issue in self.issues if issue.severity == IssueSeverity.ERROR]

    def get_warnings(self) -> list[ValidationIssue]:
        return [issue for issue in self.issues if issue.severity == IssueSeverity.WARNING]
