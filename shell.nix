{ pkgs ? import <nixpkgs> {} }:

let
  pole-wrapper = pkgs.writeShellScriptBin "pole" ''
    export PYTHONPATH="$PWD/src:$PYTHONPATH"
    exec python -m pole.cli.main "$@"
  '';
  
  # Python with required packages
  pythonEnv = pkgs.python311.withPackages (ps: with ps; [
    openai
    anthropic
  ]);
in
pkgs.mkShell {
  buildInputs = with pkgs; [
    pythonEnv
    pole-wrapper
    
    # Rust toolchain for compiler development
    rustc
    cargo
    rust-analyzer
    rustfmt
    clippy
    
    # LLVM for backend development (Phase 5 M2+)
    llvm_17
    libffi
    libxml2
    zlib
    
    # SDL2 for FFI testing (Phase 6.1 M4)
    SDL2
    SDL2.dev
    pkg-config
  ];

  shellHook = ''
    # Set PYTHONPATH to include src directory
    export PYTHONPATH="$PWD/src:$PYTHONPATH"
    
    # LLVM environment variables
    export LLVM_SYS_170_PREFIX="${pkgs.llvm_17.dev}"
    export LIBCLANG_PATH="${pkgs.llvm_17.lib}/lib"
    export LD_LIBRARY_PATH="${pkgs.llvm_17.lib}/lib:$LD_LIBRARY_PATH"

    echo "✓ Pole environment loaded (NixOS)"
    echo "  Python: $(python --version)"
    echo "  Rust: $(rustc --version)"
    echo "  LLVM: $(llvm-config --version)"
    echo ""
    echo "Available commands:"
    echo "  pole check <file>              - Validate specification file"
    echo "  pole build <file> [--mock]     - Generate IR from specification"
    echo "  pole run <ir-file> <fn> [args] - Run IR function"
    echo "  pole test <ir-file>            - Run tests in IR file"
    echo ""
    echo "Rust compiler commands:"
    echo "  cd compiler && cargo build     - Build Rust compiler"
    echo "  cd compiler && cargo test      - Run Rust tests"
    echo "  cd compiler && cargo bench     - Run benchmarks"
    echo ""
    echo "Quick test:"
    echo "  pole run examples/01-factorial.pole-ir factorial 5"
  '';
}
