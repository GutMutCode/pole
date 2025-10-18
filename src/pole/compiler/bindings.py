"""
Python bindings for Pole Rust compiler

Phase 5: Native compiler integration via PyO3

This module provides Python access to the Rust-implemented LLVM compiler.
The actual compiler is built in the `compiler/` directory as a Rust crate.

Usage:
    from pole.compiler.bindings import compile_to_native

    result = compile_to_native(
        ir_code="func factorial(n: Nat): Nat { ... }",
        output_path="factorial",
        target="x86_64-linux"
    )
"""

try:
    from pole_compiler import compile_ir, optimize_ir, CompilerConfig

    __all__ = ["compile_ir", "optimize_ir", "CompilerConfig", "compile_to_native"]

    def compile_to_native(ir_code: str, output_path: str, target: str = "native") -> dict:
        """
        Compile Pole IR to native executable

        Args:
            ir_code: Pole IR source code
            output_path: Output executable path
            target: Target triple (e.g., "x86_64-linux", "aarch64-macos")

        Returns:
            dict with compilation results and statistics
        """
        config = CompilerConfig(target=target, optimization_level=2)
        return compile_ir(ir_code, output_path, config)

except ImportError as e:
    import warnings

    warnings.warn(
        f"Rust compiler not available: {e}\n"
        "Install with: cd compiler && maturin develop\n"
        "Phase 5 features (native compilation) will not work.",
        ImportWarning,
    )

    def compile_to_native(*args, **kwargs):
        raise RuntimeError(
            "Rust compiler not installed. See compiler/README.md for build instructions."
        )

    __all__ = ["compile_to_native"]
