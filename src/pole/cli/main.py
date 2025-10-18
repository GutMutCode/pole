#!/usr/bin/env python3

import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent.parent.parent))

from pole.parser.spec_parser import SpecParser
from pole.runtime.interpreter import interpret
from pole.runtime.ir_parser import parse_ir
from pole.transformer.llm_client import MockLLMClient, OpenRouterClient
from pole.transformer.llm_transformer import transform_specification
from pole.validator.spec_validator import SpecificationValidator
from pole.verifier.example_tester import test_program
from pole.verifier.type_checker import check_types


def cmd_check(file_path: str):
    """Validate a Pole specification file"""
    print(f"Checking {file_path}...")

    if not Path(file_path).exists():
        print(f"Error: File '{file_path}' not found")
        sys.exit(1)

    with open(file_path) as f:
        content = f.read()

    parser = SpecParser(file_path, content)
    spec = parser.parse()

    validator = SpecificationValidator()
    result = validator.validate(spec)

    print()
    print(result)
    print()

    if result.has_errors():
        print("✗ Validation failed")
        sys.exit(1)
    elif result.has_warnings():
        print("⚠ Validation passed with warnings")
        sys.exit(0)
    else:
        print("✓ Validation passed")
        sys.exit(0)


def cmd_build(file_path: str, output: str | None = None, use_mock: bool = False):
    """Build IR from a Pole specification file"""
    print(f"Building {file_path}...")

    if not Path(file_path).exists():
        print(f"Error: File '{file_path}' not found")
        sys.exit(1)

    with open(file_path) as f:
        content = f.read()

    parser = SpecParser(file_path, content)
    spec = parser.parse()

    validator = SpecificationValidator()
    result = validator.validate(spec)

    if result.has_errors():
        print("Error: Specification has validation errors")
        print(result)
        sys.exit(1)

    if use_mock:
        print("Using mock LLM client (no API call)")
        llm_client = MockLLMClient()
    else:
        try:
            llm_client = OpenRouterClient()
        except ValueError as e:
            print(f"Error: {e}")
            print("Use --mock flag to use mock LLM client for testing")
            sys.exit(1)

    ir_code = transform_specification(spec, file_path, llm_client)

    if output:
        output_path = Path(output)
    else:
        output_path = Path(file_path).with_suffix(".pole-ir")

    with open(output_path, "w") as f:
        f.write(ir_code)

    print(f"✓ IR generated: {output_path}")

    program = parse_ir(ir_code)
    type_result = check_types(program)

    if not type_result.success:
        print()
        print("Warning: Generated IR has type errors:")
        print(type_result)

    sys.exit(0)


def cmd_run(file_path: str, function: str, *args):
    """Run a function from an IR file"""
    print(f"Running {function} from {file_path}...")

    if not Path(file_path).exists():
        print(f"Error: File '{file_path}' not found")
        sys.exit(1)

    with open(file_path) as f:
        ir_code = f.read()

    program = parse_ir(ir_code)

    parsed_args = []
    for arg in args:
        try:
            parsed_args.append(int(arg))
        except ValueError:
            try:
                parsed_args.append(float(arg))
            except ValueError:
                if arg.lower() == "true":
                    parsed_args.append(True)
                elif arg.lower() == "false":
                    parsed_args.append(False)
                else:
                    parsed_args.append(arg)

    try:
        result = interpret(program, function, *parsed_args)
        print()
        print(f"Result: {result}")
        sys.exit(0)
    except Exception as e:
        print(f"Error: {e}")
        sys.exit(1)


def cmd_test(file_path: str):
    """Run tests from an IR file"""
    print(f"Testing {file_path}...")

    if not Path(file_path).exists():
        print(f"Error: File '{file_path}' not found")
        sys.exit(1)

    with open(file_path) as f:
        ir_code = f.read()

    program = parse_ir(ir_code)

    type_result = check_types(program)
    if not type_result.success:
        print()
        print("Type check failed:")
        print(type_result)
        sys.exit(1)

    print()
    report = test_program(program, verbose=True)

    if report.failed == 0:
        sys.exit(0)
    else:
        sys.exit(1)


def print_help():
    """Print help message"""
    print(
        """
Pole - LLM-optimized programming language

Usage:
  pole check <file>              Validate a specification file
  pole build <file> [options]    Generate IR from specification
  pole run <ir-file> <function> [args...]  Run a function
  pole test <ir-file>            Run all tests in IR file

Options:
  --output, -o <file>   Output file for build command
  --mock                Use mock LLM (no API call) for build command

Examples:
  pole check examples/01-factorial.pole
  pole build examples/01-factorial.pole --mock
  pole run examples/01-factorial.pole-ir factorial 5
  pole test examples/01-factorial.pole-ir
"""
    )


def main():
    if len(sys.argv) < 2:
        print_help()
        sys.exit(1)

    command = sys.argv[1]

    if command == "check":
        if len(sys.argv) < 3:
            print("Error: 'check' requires a file argument")
            print_help()
            sys.exit(1)
        cmd_check(sys.argv[2])

    elif command == "build":
        if len(sys.argv) < 3:
            print("Error: 'build' requires a file argument")
            print_help()
            sys.exit(1)

        file_path = sys.argv[2]
        output = None
        use_mock = False

        i = 3
        while i < len(sys.argv):
            if sys.argv[i] in ["--output", "-o"]:
                if i + 1 < len(sys.argv):
                    output = sys.argv[i + 1]
                    i += 2
                else:
                    print("Error: --output requires an argument")
                    sys.exit(1)
            elif sys.argv[i] == "--mock":
                use_mock = True
                i += 1
            else:
                print(f"Error: Unknown option '{sys.argv[i]}'")
                sys.exit(1)

        cmd_build(file_path, output, use_mock)

    elif command == "run":
        if len(sys.argv) < 4:
            print("Error: 'run' requires <file> <function> arguments")
            print_help()
            sys.exit(1)
        cmd_run(sys.argv[2], sys.argv[3], *sys.argv[4:])

    elif command == "test":
        if len(sys.argv) < 3:
            print("Error: 'test' requires a file argument")
            print_help()
            sys.exit(1)
        cmd_test(sys.argv[2])

    elif command in ["--help", "-h", "help"]:
        print_help()
        sys.exit(0)

    else:
        print(f"Error: Unknown command '{command}'")
        print_help()
        sys.exit(1)


if __name__ == "__main__":
    main()
