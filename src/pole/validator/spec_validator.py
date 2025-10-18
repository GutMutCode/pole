import re
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent.parent.parent))

from pole.parser.ast_nodes import FunctionDef, Specification, TypeDef  # type: ignore

from .validation_result import (
    IssueSeverity,
    IssueType,
    ValidationIssue,
    ValidationResult,
)


class SpecificationValidator:
    AMBIGUOUS_PATTERNS = [
        (r"\befficiently\b", "성능 목표가 구체적이지 않음"),
        (r"\bfast\b", "성능 목표가 구체적이지 않음"),
        (r"\bquickly\b", "성능 목표가 구체적이지 않음"),
        (r"\bappropriate\b", "선택 기준이 불명확"),
        (r"\bsuitable\b", "선택 기준이 불명확"),
        (r"\bhandle safely\b", "구체적 동작이 미정의"),
        (r"\bmanage properly\b", "구체적 동작이 미정의"),
    ]

    def validate(self, spec: Specification) -> ValidationResult:
        result = ValidationResult(is_valid=True)

        for func in spec.functions:
            self._validate_function(func, result)

        for type_def in spec.types:
            self._validate_type(type_def, result)

        result.is_valid = not result.has_errors()
        return result

    def _validate_function(self, func: FunctionDef, result: ValidationResult) -> None:
        location = f"function {func.name}"

        if not func.purpose:
            result.issues.append(
                ValidationIssue(
                    severity=IssueSeverity.ERROR,
                    issue_type=IssueType.MISSING_FIELD,
                    location=location,
                    message="함수의 목적(purpose)이 정의되지 않았습니다.",
                    question="이 함수가 무엇을 하는 함수인가요?",
                )
            )

        if not func.input_desc:
            result.issues.append(
                ValidationIssue(
                    severity=IssueSeverity.ERROR,
                    issue_type=IssueType.MISSING_FIELD,
                    location=location,
                    message="함수의 입력(input)이 정의되지 않았습니다.",
                    question="이 함수는 어떤 입력을 받나요? (매개변수와 타입)",
                )
            )

        if not func.output_desc:
            result.issues.append(
                ValidationIssue(
                    severity=IssueSeverity.ERROR,
                    issue_type=IssueType.MISSING_FIELD,
                    location=location,
                    message="함수의 출력(output)이 정의되지 않았습니다.",
                    question="이 함수는 무엇을 반환하나요? (반환 타입)",
                )
            )

        if not func.examples or len(func.examples) == 0:
            result.issues.append(
                ValidationIssue(
                    severity=IssueSeverity.WARNING,
                    issue_type=IssueType.MISSING_EXAMPLES,
                    location=location,
                    message="예제(examples)가 없습니다. 최소 1개 이상의 예제를 추가하는 것을 권장합니다.",
                    question="정상 동작하는 예제를 하나 제공해주세요. (예: input → output)",
                )
            )

        for constraint in func.constraints:
            self._check_ambiguity(constraint, location, result)

            if "overflow" in constraint.lower() and "handle" in constraint.lower():
                result.issues.append(
                    ValidationIssue(
                        severity=IssueSeverity.WARNING,
                        issue_type=IssueType.AMBIGUOUS_EXPRESSION,
                        location=f"{location} - constraints",
                        message=f'"{constraint}" - 오버플로우 처리 방법이 명시되지 않았습니다.',
                        question="오버플로우가 발생하면 어떻게 처리해야 하나요?",
                        options=[
                            "에러 반환 (안전하지만 사용자가 처리 필요)",
                            "임의 정밀도 정수 사용 (느리지만 정확)",
                            "최대값으로 클램핑 (빠르지만 부정확)",
                        ],
                    )
                )

            if "valid" in constraint.lower() and (
                "email" in constraint.lower() or "format" in constraint.lower()
            ):
                result.issues.append(
                    ValidationIssue(
                        severity=IssueSeverity.WARNING,
                        issue_type=IssueType.AMBIGUOUS_EXPRESSION,
                        location=f"{location} - constraints",
                        message=f'"{constraint}" - 검증 기준이 불명확합니다.',
                        question="어느 수준까지 검증해야 하나요?",
                        options=[
                            "@ 기호 포함 여부만 확인 (간단)",
                            "정규식으로 기본 형식 검증 (보통)",
                            "RFC 5322 완전 준수 (엄격)",
                        ],
                    )
                )

    def _validate_type(self, type_def: TypeDef, result: ValidationResult) -> None:
        location = f"type {type_def.name}"

        if not type_def.fields or len(type_def.fields) == 0:
            result.issues.append(
                ValidationIssue(
                    severity=IssueSeverity.ERROR,
                    issue_type=IssueType.MISSING_FIELD,
                    location=location,
                    message="타입의 필드가 정의되지 않았습니다.",
                    question="이 타입은 어떤 필드를 가지고 있나요?",
                )
            )

        for field in type_def.fields:
            if not field.type_annotation or field.type_annotation == "unknown":
                result.issues.append(
                    ValidationIssue(
                        severity=IssueSeverity.ERROR,
                        issue_type=IssueType.MISSING_FIELD,
                        location=f"{location}.{field.name}",
                        message=f"필드 '{field.name}'의 타입이 정의되지 않았습니다.",
                        question=f"필드 '{field.name}'의 타입은 무엇인가요?",
                    )
                )

    def _check_ambiguity(self, text: str, location: str, result: ValidationResult) -> None:
        for pattern, description in self.AMBIGUOUS_PATTERNS:
            if re.search(pattern, text, re.IGNORECASE):
                result.issues.append(
                    ValidationIssue(
                        severity=IssueSeverity.WARNING,
                        issue_type=IssueType.AMBIGUOUS_EXPRESSION,
                        location=location,
                        message=f'"{text}" - {description}',
                        question=f"이 부분을 좀 더 구체적으로 설명해주시겠습니까?",
                    )
                )


def validate_specification(spec: Specification) -> ValidationResult:
    validator = SpecificationValidator()
    return validator.validate(spec)
