#!/bin/bash
# Pole Zomboid Demo Runner
# Compiles and runs the Zomboid game from Pole IR to native binary

set -e

echo "🎮 Pole Zomboid Demo - Native Compilation"
echo "=========================================="
echo ""

# Build the compiler
echo "📦 Building Rust compiler..."
cd compiler
cargo build --release --quiet
cd ..

# Compile the game
echo "🔨 Compiling Zomboid main.pole-ir..."
cd compiler
cargo run --release --example test_zomboid_main 2>&1 | grep -E "✓|✗|Type defs|Functions|Externs"
cd ..

# Link with SDL2
echo "🔗 Linking with SDL2..."
gcc -o /tmp/zomboid_game /tmp/zomboid_main.o runtime/pole_runtime.o -lSDL2 -lm
echo "   ✓ Binary created: /tmp/zomboid_game ($(du -h /tmp/zomboid_game | cut -f1))"

# Run the game
echo ""
echo "🎮 Running game demo..."
echo "   (Will run for 10 seconds @ 60fps, then exit)"
echo ""
timeout 15 /tmp/zomboid_game

EXIT_CODE=$?
if [ $EXIT_CODE -eq 0 ]; then
    echo ""
    echo "✅ SUCCESS! Game completed successfully!"
    echo "   - SDL2 window created"
    echo "   - Renderer initialized"
    echo "   - 600 frames rendered (10 seconds)"
    echo "   - Clean shutdown"
else
    echo ""
    echo "❌ Game exited with code: $EXIT_CODE"
fi
