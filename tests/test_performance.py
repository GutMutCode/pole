import sys
import time
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent.parent / "src"))

from pole.runtime.interpreter import interpret
from pole.runtime.ir_parser import parse_ir


def benchmark(name: str, func, iterations: int = 1):
    """Run a benchmark and print results"""
    start = time.time()
    result = None
    for _ in range(iterations):
        result = func()
    elapsed = time.time() - start

    if iterations == 1:
        print(f"  {name}: {elapsed * 1000:.2f}ms")
    else:
        print(f"  {name}: {elapsed * 1000:.2f}ms total, {elapsed * 1000 / iterations:.3f}ms avg")

    return result, elapsed


def test_factorial_performance():
    """Benchmark factorial computation"""
    print("\n=== Factorial Performance ===")

    ir_code = """
func factorial (n: Nat) -> Nat :
  match n with
  | 0 -> 1
  | n -> n * factorial (n - 1)
"""

    program = parse_ir(ir_code)

    result, _ = benchmark("Parse IR", lambda: parse_ir(ir_code))

    result, _ = benchmark("Factorial(10)", lambda: interpret(program, "factorial", 10))
    assert result == 3628800

    result, _ = benchmark("Factorial(20)", lambda: interpret(program, "factorial", 20))
    assert result == 2432902008176640000

    _, elapsed = benchmark(
        "Factorial(15) x 100", lambda: interpret(program, "factorial", 15), iterations=100
    )

    avg_per_call = elapsed / 100
    print(f"  Average per call: {avg_per_call * 1000:.3f}ms")

    if avg_per_call < 0.001:
        print("  ✓ Performance: Excellent (< 1ms per call)")
    elif avg_per_call < 0.01:
        print("  ✓ Performance: Good (< 10ms per call)")
    else:
        print("  ⚠ Performance: Could be improved (> 10ms per call)")


def test_parsing_performance():
    """Benchmark IR parsing"""
    print("\n=== IR Parsing Performance ===")

    ir_code = """
func factorial (n: Nat) -> Nat :
  match n with
  | 0 -> 1
  | n -> n * factorial (n - 1)

func fibonacci (n: Nat) -> Nat :
  match n with
  | 1 -> 1
  | 2 -> 1
  | n -> fibonacci (n - 1) + fibonacci (n - 2)
"""

    _, elapsed = benchmark("Parse IR x 1000", lambda: parse_ir(ir_code), iterations=1000)

    avg_per_parse = elapsed / 1000
    print(f"  Average per parse: {avg_per_parse * 1000:.3f}ms")

    if avg_per_parse < 0.001:
        print("  ✓ Parsing: Excellent (< 1ms per parse)")
    elif avg_per_parse < 0.01:
        print("  ✓ Parsing: Good (< 10ms per parse)")
    else:
        print("  ⚠ Parsing: Could be improved (> 10ms per parse)")


def test_recursion_depth():
    """Test deep recursion performance"""
    print("\n=== Recursion Depth Performance ===")

    ir_code = """
func sum_to_n (n: Nat) -> Nat :
  match n with
  | 0 -> 0
  | n -> n + sum_to_n (n - 1)
"""

    program = parse_ir(ir_code)

    result, elapsed = benchmark("Sum to 100", lambda: interpret(program, "sum_to_n", 100))
    assert result == 5050
    print(f"  Result: {result}")

    result, elapsed = benchmark("Sum to 200", lambda: interpret(program, "sum_to_n", 200))
    assert result == 20100
    print(f"  Result: {result}")

    if elapsed < 0.01:
        print("  ✓ Deep recursion: Excellent")
    elif elapsed < 0.1:
        print("  ✓ Deep recursion: Good")
    else:
        print("  ⚠ Deep recursion: Could be improved")


def test_pattern_matching_performance():
    """Test pattern matching performance"""
    print("\n=== Pattern Matching Performance ===")

    ir_code = """
func classify (n: Int) -> Int :
  match n with
  | 0 -> 0
  | 1 -> 1
  | 2 -> 2
  | 3 -> 3
  | 4 -> 4
  | 5 -> 5
  | n -> 99
"""

    program = parse_ir(ir_code)

    _, elapsed = benchmark(
        "Pattern match x 1000",
        lambda: interpret(program, "classify", 3),
        iterations=1000,
    )

    avg_per_match = elapsed / 1000
    print(f"  Average per match: {avg_per_match * 1000:.3f}ms")

    if avg_per_match < 0.0001:
        print("  ✓ Pattern matching: Excellent")
    elif avg_per_match < 0.001:
        print("  ✓ Pattern matching: Good")
    else:
        print("  ⚠ Pattern matching: Could be improved")


if __name__ == "__main__":
    print("=" * 60)
    print("Pole Performance Tests")
    print("=" * 60)

    test_factorial_performance()
    test_parsing_performance()
    test_recursion_depth()
    test_pattern_matching_performance()

    print("\n" + "=" * 60)
    print("✅ All performance tests completed!")
    print("=" * 60)
