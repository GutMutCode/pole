#!/usr/bin/env python3

import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent.parent / "src"))

from pole.parser.spec_parser import SpecParser
from pole.validator.spec_validator import SpecificationValidator
from pole.transformer.llm_client import MockLLMClient
from pole.transformer.llm_transformer import transform_specification
from pole.runtime.ir_parser import parse_ir
from pole.verifier.type_checker import check_types


def test_factorial_spec_to_ir_mock():
    """Test .pole → .pole-ir transformation with mock LLM"""
    spec_file = Path(__file__).parent.parent / "examples" / "01-factorial.pole"
    spec_content = spec_file.read_text()

    parser = SpecParser(str(spec_file), spec_content)
    spec = parser.parse()

    validator = SpecificationValidator()
    result = validator.validate(spec)
    assert not result.has_errors(), f"Spec validation failed: {result}"

    llm_client = MockLLMClient()
    ir_code = transform_specification(spec, str(spec_file), llm_client)

    assert ir_code is not None
    assert len(ir_code) > 0
    assert "func" in ir_code

    program = parse_ir(ir_code)
    assert program is not None

    type_result = check_types(program)
    assert type_result.success, f"Type check failed: {type_result}"

    print("✓ test_factorial_spec_to_ir_mock")


def test_fibonacci_spec_to_ir_mock():
    """Test .pole → .pole-ir transformation with mock LLM"""
    spec_file = Path(__file__).parent.parent / "examples" / "02-fibonacci.pole"
    spec_content = spec_file.read_text()

    parser = SpecParser(str(spec_file), spec_content)
    spec = parser.parse()

    validator = SpecificationValidator()
    result = validator.validate(spec)
    assert not result.has_errors(), f"Spec validation failed: {result}"

    llm_client = MockLLMClient()
    ir_code = transform_specification(spec, str(spec_file), llm_client)

    assert ir_code is not None
    assert len(ir_code) > 0
    assert "func" in ir_code

    program = parse_ir(ir_code)
    assert program is not None

    type_result = check_types(program)
    assert type_result.success, f"Type check failed: {type_result}"

    print("✓ test_fibonacci_spec_to_ir_mock")


def test_spec_validation_consistency():
    """Verify all spec files pass validation"""
    examples_dir = Path(__file__).parent.parent / "examples"
    spec_files = [
        "01-factorial.pole",
        "02-fibonacci.pole",
        "03-user-validation.pole",
        "04-simple-math.pole",
        "05-is-even.pole",
        "07-max.pole",
    ]

    validator = SpecificationValidator()

    for spec_file in spec_files:
        spec_path = examples_dir / spec_file
        spec_content = spec_path.read_text()

        parser = SpecParser(str(spec_path), spec_content)
        spec = parser.parse()

        result = validator.validate(spec)

        assert not result.has_errors(), f"{spec_file} validation failed: {result}"

    print("✓ test_spec_validation_consistency")


if __name__ == "__main__":
    test_factorial_spec_to_ir_mock()
    test_fibonacci_spec_to_ir_mock()
    test_spec_validation_consistency()
    print("\n✅ All LLM pipeline tests passed!")
