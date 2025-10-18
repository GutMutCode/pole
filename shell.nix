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
  ];

  shellHook = ''
    # Set PYTHONPATH to include src directory
    export PYTHONPATH="$PWD/src:$PYTHONPATH"

    echo "✓ Pole environment loaded (NixOS)"
    echo "  Python: $(python --version)"
    echo ""
    echo "Available commands:"
    echo "  pole check <file>              - Validate specification file"
    echo "  pole build <file> [--mock]     - Generate IR from specification"
    echo "  pole run <ir-file> <fn> [args] - Run IR function"
    echo "  pole test <ir-file>            - Run tests in IR file"
    echo ""
    echo "Quick test:"
    echo "  pole run examples/01-factorial.pole-ir factorial 5"
  '';
}
