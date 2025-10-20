#!/usr/bin/env python3
"""
Automatic development workflow enforcer
Ensures LLM follows the development checklist

Usage:
    python scripts/auto_development.py zombie.pole

This script will:
1. Generate TODO checklist
2. Execute each step
3. Validate results
4. Report status
"""

import argparse
import subprocess
import sys
from pathlib import Path
from typing import List, Tuple


class DevelopmentStep:
    def __init__(self, name: str, command: str, validation: str = ""):
        self.name = name
        self.command = command
        self.validation = validation
        self.completed = False
        self.output = ""


def create_checklist(spec_file: str, integration_test: str = "") -> List[DevelopmentStep]:
    """Create development checklist for a spec file

    Args:
        spec_file: Path to .pole specification file
        integration_test: Optional path to integration test file (e.g., examples/67-test-player.pole-ir)
    """

    base_name = Path(spec_file).stem
    ir_file = spec_file.replace(".pole", ".pole-ir")

    steps = [
        DevelopmentStep("Check dependencies", f"ls {spec_file}", "File must exist"),
        DevelopmentStep(
            "Read .pole syntax examples",
            "cat examples/03-user-validation.pole",
            "Learn type and function syntax",
        ),
        DevelopmentStep(
            "Read .pole-ir syntax examples",
            "cat examples/08-simple-record.pole-ir",
            "Learn IR type syntax",
        ),
        DevelopmentStep("Test pole CLI", "pole --version", "Verify pole is available"),
        DevelopmentStep("Validate spec file", f"pole check {spec_file}", "Spec must be valid"),
        DevelopmentStep(
            "Generate IR", f"pole build {spec_file} --output {ir_file}", "IR must be generated"
        ),
        DevelopmentStep(
            "Verify IR with Rust parser",
            f"cd compiler && cargo run --release --bin polec -- ../{ir_file}",
            "Rust parser must succeed",
        ),
        DevelopmentStep("Run test cases", f"pole test {ir_file}", "All tests must pass"),
    ]

    # Add integration test steps if specified
    if integration_test:
        steps.extend(
            [
                DevelopmentStep(
                    "Write integration test",
                    f"echo 'Manual step: Write {integration_test}'",
                    "Integration test file must be created",
                ),
                DevelopmentStep(
                    "Run integration test",
                    f"pole test {integration_test}",
                    "Integration tests must pass",
                ),
            ]
        )

    return steps


def execute_step(step: DevelopmentStep, auto_fix: bool = False) -> Tuple[bool, str]:
    """Execute a development step and return (success, output)"""

    print(f"⏳ {step.name}...")
    print(f"   Command: {step.command}")

    max_retries = 3 if auto_fix else 1

    for attempt in range(max_retries):
        if attempt > 0:
            print(f"   🔄 Retry attempt {attempt}/{max_retries - 1}")

        try:
            result = subprocess.run(
                step.command, shell=True, capture_output=True, text=True, timeout=60
            )

            step.output = result.stdout + result.stderr

            if result.returncode == 0:
                step.completed = True
                print(f"✅ {step.name} - SUCCESS")
                if attempt > 0:
                    print(f"   ✨ Auto-fixed on attempt {attempt + 1}")
                return True, step.output
            else:
                print(f"❌ {step.name} - FAILED")
                print(f"   Error: {step.output[:200]}")

                # Auto-fix logic
                if auto_fix and attempt < max_retries - 1:
                    print(f"   🔧 Attempting auto-fix...")
                    fix_success = attempt_auto_fix(step)
                    if fix_success:
                        print(f"   ✅ Auto-fix applied")
                        continue
                    else:
                        print(f"   ⚠️ Auto-fix failed, retrying...")
                        continue

                return False, step.output

        except subprocess.TimeoutExpired:
            print(f"⏰ {step.name} - TIMEOUT")
            return False, "Command timed out"
        except Exception as e:
            print(f"💥 {step.name} - ERROR: {e}")
            return False, str(e)

    return False, "Max retries exceeded"


def attempt_auto_fix(step: DevelopmentStep) -> bool:
    """Attempt to automatically fix common errors"""

    error_output = step.output.lower()

    # Fix 1: pole check syntax errors
    if "pole check" in step.command and "invalid type definition" in error_output:
        print("   📖 Reading syntax examples for reference...")
        # Could invoke LLM here to fix syntax
        return False  # Placeholder

    # Fix 2: IR generation failures
    if "pole build" in step.command and "failed" in error_output:
        print("   📝 Attempting manual IR generation...")
        # Could write IR manually based on examples
        return False  # Placeholder

    # Fix 3: Common typos
    if "file not found" in error_output or "no such file" in error_output:
        print("   🔍 Checking for file existence...")
        return False  # Placeholder

    return False


def run_workflow(spec_file: str, auto_fix: bool = False, integration_test: str = "") -> bool:
    """Run the complete development workflow"""

    print(f"🚀 Starting development workflow for: {spec_file}")
    print("=" * 60)

    checklist = create_checklist(spec_file, integration_test)

    for i, step in enumerate(checklist, 1):
        print(f"\nStep {i}/{len(checklist)}:")
        success, output = execute_step(step)

        if not success:
            print("\n" + "=" * 60)
            print("❌ WORKFLOW FAILED")
            print(f"Failed at: {step.name}")
            print(f"Validation: {step.validation}")
            print("\nSuggested fixes:")

            if "pole check" in step.command:
                print("  - Review examples/03-user-validation.pole for syntax")
                print("  - Check type definitions use 'type Name:' format")
                print("  - Ensure fields section exists")
            elif "pole build" in step.command:
                print("  - LLM may have failed - try manual IR writing")
                print("  - Check src/pole/transformer/llm_transformer.py prompt")
                print("  - Use examples/08-simple-record.pole-ir as reference")
            elif "cargo run" in step.command:
                print("  - Rust parser is authoritative - fix IR syntax errors")
                print("  - Check record types use '=' not ':'")
                print("  - Ensure match expressions are properly formatted")
            elif "pole test" in step.command:
                print("  - Add @test_case annotations to functions")
                print("  - Ensure test logic is correct")

            return False

    print("\n" + "=" * 60)
    print("✅ WORKFLOW COMPLETED SUCCESSFULLY")
    print(f"All {len(checklist)} steps passed!")
    print("\nSafe to commit:")
    print(f"  git add {spec_file} {spec_file.replace('.pole', '.pole-ir')}")
    print(f'  git commit -m "feat: Add {Path(spec_file).stem} specification"')

    return True


def main():
    parser = argparse.ArgumentParser(description="Automatic development workflow enforcer")
    parser.add_argument("spec_file", help="Path to .pole specification file")
    parser.add_argument(
        "--auto-fix", action="store_true", help="Attempt automatic fixes (experimental)"
    )
    parser.add_argument(
        "--integration-test",
        default="",
        help="Path to integration test file (e.g., examples/67-test-player.pole-ir)",
    )

    args = parser.parse_args()

    if not Path(args.spec_file).exists():
        print(f"❌ File not found: {args.spec_file}")
        return 1

    success = run_workflow(args.spec_file, args.auto_fix, args.integration_test)

    return 0 if success else 1


if __name__ == "__main__":
    sys.exit(main())
