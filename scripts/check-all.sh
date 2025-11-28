#!/bin/bash
# Run sw-checklist on each component and capture results
# Goal: Get FAIL and WARN counts to 0 and keep them there
set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

COMPONENTS=(core widgets dnd app)
TOTAL_FAIL=0
TOTAL_WARN=0

echo "Running sw-checklist on all components..."
echo ""

for component in "${COMPONENTS[@]}"; do
    COMPONENT_DIR="$PROJECT_ROOT/components/$component"
    TARGET_DIR="$COMPONENT_DIR/target"
    OUTPUT_FILE="$TARGET_DIR/sw-checklist-results.txt"

    # Ensure target directory exists
    mkdir -p "$TARGET_DIR"

    echo "=== Checking $component ==="

    # Run sw-checklist and capture output
    cd "$COMPONENT_DIR"
    sw-checklist . > "$OUTPUT_FILE" 2>&1 || true

    # Extract summary line and counts
    SUMMARY=$(grep "^Summary:" "$OUTPUT_FILE" || echo "Summary: 0 passed, 0 failed, 0 warnings")
    FAIL_COUNT=$(echo "$SUMMARY" | grep -o '[0-9]* failed' | grep -o '[0-9]*')
    WARN_COUNT=$(echo "$SUMMARY" | grep -o '[0-9]* warning' | grep -o '[0-9]*')

    # Default to 0 if parsing failed
    FAIL_COUNT=${FAIL_COUNT:-0}
    WARN_COUNT=${WARN_COUNT:-0}

    # Update totals
    TOTAL_FAIL=$((TOTAL_FAIL + FAIL_COUNT))
    TOTAL_WARN=$((TOTAL_WARN + WARN_COUNT))

    # Show summary for this component
    echo "  $SUMMARY"
    echo "  Results saved to: $OUTPUT_FILE"
    echo ""
done

echo "================================================================================"
echo "TOTALS ACROSS ALL COMPONENTS"
echo "================================================================================"
echo "  FAIL: $TOTAL_FAIL"
echo "  WARN: $TOTAL_WARN"
echo ""

if [ "$TOTAL_FAIL" -eq 0 ] && [ "$TOTAL_WARN" -eq 0 ]; then
    echo "✓ All checks passed with no warnings!"
    exit 0
elif [ "$TOTAL_FAIL" -eq 0 ]; then
    echo "⚠ No failures, but $TOTAL_WARN warning(s) to address"
    exit 0
else
    echo "✗ $TOTAL_FAIL failure(s) and $TOTAL_WARN warning(s) need attention"
    exit 1
fi
