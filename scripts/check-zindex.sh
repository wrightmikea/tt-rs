#!/bin/bash
# Validate z-index hierarchy in CSS
# This prevents regressions where z-index might be accidentally changed
#
# Z-INDEX HIERARCHY (lowest to highest):
#   1. Copy source stacks: 1 (behind everything)
#   2. Boxes/containers: 10
#   3. Values (numbers, text): 20
#   4. Comparison (scales): 30
#   5. Agents (robot, bird, nest): 40
#   6. Tools (vacuum, wand): 50
#   7. Tooltips: 100
#   8. Dragging widgets: 200
#
# KEY PRINCIPLE: z-index must be set in CSS, NOT inline in Rust code.
# This ensures proper layering based on widget type.

CSS_FILE="components/app/crates/tt-rs-app/assets/styles/main.css"

echo "Checking z-index hierarchy..."

# Check that CSS variables are defined
check_var() {
    local var="$1"
    local expected="$2"
    # Use grep -e to explicitly treat pattern as pattern, not option
    local actual=$(grep -e "$var:" "$CSS_FILE" | head -1 | sed 's/.*: *\([0-9-]*\);.*/\1/')

    if [ "$actual" != "$expected" ]; then
        echo "ERROR: $var expected $expected but found '$actual'"
        return 1
    fi
    echo "  ✓ $var = $expected"
    return 0
}

errors=0

check_var "z-copy-source-behind" "1" || ((errors++))
check_var "z-containers" "10" || ((errors++))
check_var "z-values" "20" || ((errors++))
check_var "z-comparison" "30" || ((errors++))
check_var "z-agents" "40" || ((errors++))
check_var "z-tools" "50" || ((errors++))
check_var "z-tooltip-hover" "100" || ((errors++))
check_var "z-dragging" "200" || ((errors++))

# Check that copy-source-stack has z-index rule in CSS
if grep -A2 '\.copy-source-stack {' "$CSS_FILE" | grep -q "z-index"; then
    echo "  ✓ .copy-source-stack has z-index rule in CSS"
else
    echo "ERROR: .copy-source-stack missing z-index rule in CSS"
    ((errors++))
fi

# Check that draggable z-index is NOT set inline in Rust format strings
# Look for z-index inside string literals (format! strings), not in comments
# Pattern: quote followed by anything then z-index: then digits
DRAGGABLE_FILE="components/dnd/crates/tt-rs-drag/src/draggable.rs"
if grep -e '".*z-index: *[0-9{}]' "$DRAGGABLE_FILE" >/dev/null 2>&1; then
    echo "ERROR: draggable.rs has inline z-index in string - should use CSS classes"
    ((errors++))
else
    echo "  ✓ draggable.rs uses CSS for z-index (no inline styles)"
fi

COPY_SOURCE_FILE="components/dnd/crates/tt-rs-drag/src/copy_source.rs"
if grep -e '".*z-index: *[0-9{}]' "$COPY_SOURCE_FILE" >/dev/null 2>&1; then
    echo "ERROR: copy_source.rs has inline z-index in string - should use CSS classes"
    ((errors++))
else
    echo "  ✓ copy_source.rs uses CSS for z-index (no inline styles)"
fi

echo ""
if [ $errors -eq 0 ]; then
    echo "✓ All z-index checks passed"
    exit 0
else
    echo "✗ $errors z-index check(s) failed"
    exit 1
fi
