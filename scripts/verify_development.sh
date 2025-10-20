#!/bin/bash
# Development verification script
# Runs all checks before allowing commit

set -e  # Exit on first error

echo "🔍 Running development verification..."
echo ""

# Colors
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Track failures
FAILED=0

# 1. Check dependencies
echo "1️⃣  Checking dependencies..."
if ! command -v pole &> /dev/null; then
    echo -e "${RED}✗ pole command not found${NC}"
    FAILED=1
else
    echo -e "${GREEN}✓ pole command available${NC}"
fi

if ! command -v cargo &> /dev/null; then
    echo -e "${RED}✗ cargo not found${NC}"
    FAILED=1
else
    echo -e "${GREEN}✓ cargo available${NC}"
fi
echo ""

# 2. Verify .pole spec files
echo "2️⃣  Verifying .pole specification files..."
SPEC_FAILED=0
for file in games/zomboid/specs/*.pole 2>/dev/null; do
    if [ -f "$file" ]; then
        if pole check "$file" > /dev/null 2>&1; then
            echo -e "  ${GREEN}✓${NC} $file"
        else
            echo -e "  ${RED}✗${NC} $file"
            SPEC_FAILED=1
        fi
    fi
done

if [ $SPEC_FAILED -eq 1 ]; then
    FAILED=1
fi
echo ""

# 3. Build Rust compiler (quietly)
echo "3️⃣  Building Rust compiler..."
cd compiler
if cargo build --release --bin polec > /dev/null 2>&1; then
    echo -e "${GREEN}✓ Rust compiler built successfully${NC}"
else
    echo -e "${RED}✗ Rust compiler build failed${NC}"
    FAILED=1
fi
cd ..
echo ""

# 4. Verify .pole-ir files with Rust parser
echo "4️⃣  Verifying .pole-ir files with Rust parser..."
IR_FAILED=0
for file in games/zomboid/specs/*.pole-ir examples/*.pole-ir 2>/dev/null; do
    if [ -f "$file" ]; then
        if cd compiler && cargo run --release --bin polec -- ../$file > /dev/null 2>&1; then
            echo -e "  ${GREEN}✓${NC} $file"
        else
            echo -e "  ${RED}✗${NC} $file"
            IR_FAILED=1
        fi
        cd ..
    fi
done

if [ $IR_FAILED -eq 1 ]; then
    FAILED=1
fi
echo ""

# 5. Check for TODO/FIXME without issue tracking
echo "5️⃣  Checking for untracked TODOs..."
TODO_COUNT=$(grep -r "TODO\|FIXME" games/zomboid/specs/*.pole* 2>/dev/null | wc -l || echo "0")
if [ "$TODO_COUNT" -gt 0 ]; then
    echo -e "${YELLOW}⚠ Found $TODO_COUNT TODO/FIXME comments${NC}"
    grep -n "TODO\|FIXME" games/zomboid/specs/*.pole* 2>/dev/null || true
    echo -e "${YELLOW}  Consider creating issues for these${NC}"
else
    echo -e "${GREEN}✓ No untracked TODOs${NC}"
fi
echo ""

# Final result
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
if [ $FAILED -eq 0 ]; then
    echo -e "${GREEN}✅ All verification checks passed!${NC}"
    echo -e "${GREEN}   Safe to commit.${NC}"
    exit 0
else
    echo -e "${RED}❌ Verification failed!${NC}"
    echo -e "${RED}   Please fix errors before committing.${NC}"
    exit 1
fi
