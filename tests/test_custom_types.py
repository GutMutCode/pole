#!/usr/bin/env python3

import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent.parent / "src"))

from pole.runtime.ir_parser import parse_ir
from pole.verifier.type_checker import check_types


def test_record_type_parsing():
    """Test parsing of record type definitions"""
    code = """
type User = {
  name: String,
  email: String,
  age: Nat
}
"""
    program = parse_ir(code)
    assert len(program.type_defs) == 1
    assert program.type_defs[0].name == "User"
    assert len(program.type_defs[0].definition.fields) == 3
    assert "name" in program.type_defs[0].definition.fields
    assert "email" in program.type_defs[0].definition.fields
    assert "age" in program.type_defs[0].definition.fields
    print("✓ test_record_type_parsing")


def test_variant_type_parsing():
    """Test parsing of variant type definitions"""
    code = """
type ValidationError =
  | NameEmpty
  | NameTooLong
  | InvalidEmail
  | InvalidAge
"""
    program = parse_ir(code)
    assert len(program.type_defs) == 1
    assert program.type_defs[0].name == "ValidationError"
    constructors = [c[0] for c in program.type_defs[0].definition]
    assert constructors == ["NameEmpty", "NameTooLong", "InvalidEmail", "InvalidAge"]
    print("✓ test_variant_type_parsing")


def test_variant_type_with_arguments():
    """Test parsing of variant types with constructor arguments"""
    code = """
type Result =
  | Ok(String)
  | Err(String)
"""
    program = parse_ir(code)
    assert len(program.type_defs) == 1
    assert program.type_defs[0].name == "Result"
    assert len(program.type_defs[0].definition) == 2
    assert program.type_defs[0].definition[0][0] == "Ok"
    assert program.type_defs[0].definition[1][0] == "Err"
    assert len(program.type_defs[0].definition[0][1]) == 1
    assert len(program.type_defs[0].definition[1][1]) == 1
    print("✓ test_variant_type_with_arguments")


def test_type_alias_parsing():
    """Test parsing of type aliases"""
    code = """
type UserId = String
"""
    program = parse_ir(code)
    assert len(program.type_defs) == 1
    assert program.type_defs[0].name == "UserId"
    print("✓ test_type_alias_parsing")


def test_inline_record_type():
    """Test parsing inline record type (single line)"""
    code = """
type Point = { x: Int, y: Int }
"""
    program = parse_ir(code)
    assert len(program.type_defs) == 1
    assert program.type_defs[0].name == "Point"
    assert len(program.type_defs[0].definition.fields) == 2
    print("✓ test_inline_record_type")


def test_field_access_on_custom_type():
    """Test type checking of field access on custom record types"""
    code = """
type User = {
  name: String,
  age: Nat
}

func get_user_name (u: User) -> String :
  u.name
"""
    program = parse_ir(code)
    result = check_types(program)
    assert result.success, f"Type check failed: {result.errors}"
    print("✓ test_field_access_on_custom_type")


def test_multiple_custom_types():
    """Test parsing and type checking with multiple custom types"""
    code = """
type User = {
  name: String,
  age: Nat
}

type Error =
  | NotFound
  | Invalid

func get_age (u: User) -> Nat :
  u.age
"""
    program = parse_ir(code)
    assert len(program.type_defs) == 2

    result = check_types(program)
    assert result.success, f"Type check failed: {result.errors}"
    print("✓ test_multiple_custom_types")


def test_nested_record_access():
    """Test field access on nested records"""
    code = """
type Address = {
  city: String,
  country: String
}

type User = {
  name: String,
  address: Address
}

func get_city (u: User) -> Address :
  u.address
"""
    program = parse_ir(code)
    result = check_types(program)
    assert result.success, f"Type check failed: {result.errors}"
    print("✓ test_nested_record_access")


def test_invalid_field_access():
    """Test that invalid field access is detected"""
    code = """
type User = {
  name: String
}

func get_age (u: User) -> Nat :
  u.age
"""
    program = parse_ir(code)
    result = check_types(program)
    assert not result.success
    assert any("not found" in e.message.lower() for e in result.errors)
    print("✓ test_invalid_field_access")


def test_type_annotations_preserved():
    """Test that annotations on type definitions are preserved"""
    code = """
@generated_from("User type from spec")
type User = {
  name: String
}
"""
    program = parse_ir(code)
    assert len(program.type_defs) == 1
    assert len(program.type_defs[0].annotations) == 1
    assert program.type_defs[0].annotations[0].name == "generated_from"
    print("✓ test_type_annotations_preserved")


if __name__ == "__main__":
    test_record_type_parsing()
    test_variant_type_parsing()
    test_variant_type_with_arguments()
    test_type_alias_parsing()
    test_inline_record_type()
    test_field_access_on_custom_type()
    test_multiple_custom_types()
    test_nested_record_access()
    test_invalid_field_access()
    test_type_annotations_preserved()
    print("\n✅ All custom type tests passed!")
