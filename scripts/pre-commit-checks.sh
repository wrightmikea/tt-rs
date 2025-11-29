#!/bin/bash
# Pre-commit checks for tt-rs.
# Run this before committing to catch common issues and get guidance.
#
# Usage: ./scripts/pre-commit-checks.sh [--fix]
#   --fix: Attempt to auto-fix issues where possible

set -e
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
cd "$PROJECT_ROOT"

FIX_MODE=false
if [ "$1" = "--fix" ]; then
    FIX_MODE=true
fi

ERRORS=0
WARNINGS=0
INSTRUCTIONS=""

add_instruction() {
    INSTRUCTIONS="$INSTRUCTIONS\n$1"
}

echo "========================================"
echo "  tt-rs Pre-Commit Checks"
echo "========================================"
echo ""

# 1. Check for uncommitted changes to understand scope
echo "--- Checking scope of changes ---"
CHANGED_FILES=$(git diff --name-only HEAD 2>/dev/null || echo "")
STAGED_FILES=$(git diff --cached --name-only 2>/dev/null || echo "")
ALL_CHANGES="$CHANGED_FILES $STAGED_FILES"

HAS_CODE_CHANGES=false
HAS_DOCS_CHANGES=false
HAS_UI_CHANGES=false

if echo "$ALL_CHANGES" | grep -q "\.rs$"; then
    HAS_CODE_CHANGES=true
    echo "  Code changes detected"
fi
if echo "$ALL_CHANGES" | grep -qE "(\.md$|documentation/)"; then
    HAS_DOCS_CHANGES=true
    echo "  Documentation changes detected"
fi
if echo "$ALL_CHANGES" | grep -qE "(components/.*src.*\.rs$|\.css$|\.html$)"; then
    HAS_UI_CHANGES=true
    echo "  UI changes detected"
fi

# 2. Run build checks if code changed
if [ "$HAS_CODE_CHANGES" = true ]; then
    echo ""
    echo "--- Running build checks ---"
    if ./scripts/build-all.sh > /dev/null 2>&1; then
        echo "  PASS: Build succeeded"
    else
        echo "  FAIL: Build failed!"
        ERRORS=$((ERRORS + 1))
        add_instruction "Fix build errors before committing."
    fi
fi

# 3. Check live demo status
echo ""
echo "--- Checking live demo status ---"
if ./scripts/check-live-demo.sh 2>/dev/null; then
    echo "  (see output above)"
else
    WARNINGS=$((WARNINGS + 1))
    if [ "$HAS_CODE_CHANGES" = true ]; then
        add_instruction "Run: ./scripts/build-release.sh && git add docs/ && git commit -m 'deploy: ...' && git push"
    fi
fi

# 4. Check CHANGELOG
echo ""
echo "--- Checking CHANGELOG ---"
if ./scripts/check-changelog.sh 2>/dev/null; then
    echo "  (see output above)"
else
    WARNINGS=$((WARNINGS + 1))
fi

# 5. Check screenshots if UI changed
if [ "$HAS_UI_CHANGES" = true ]; then
    echo ""
    echo "--- Checking screenshots ---"
    ./scripts/update-readme-screenshots.sh 2>/dev/null || true
fi

# 6. Check documentation freshness
echo ""
echo "--- Checking documentation freshness ---"

# Check README last updated
README_TIME=$(git log -1 --format="%ct" -- "README.md" 2>/dev/null || echo "0")
CODE_TIME=$(git log -1 --format="%ct" -- "components/" 2>/dev/null || echo "0")

if [ "$CODE_TIME" -gt "$README_TIME" ] && [ "$README_TIME" != "0" ]; then
    DAYS_OLD=$(( (CODE_TIME - README_TIME) / 86400 ))
    if [ "$DAYS_OLD" -gt 3 ]; then
        echo "  WARNING: README.md not updated in $DAYS_OLD days since last code change"
        WARNINGS=$((WARNINGS + 1))
        add_instruction "Review README.md for outdated information."
    else
        echo "  README.md is reasonably current"
    fi
fi

# Check plan.md status
if [ -f "documentation/plan.md" ]; then
    PLAN_TIME=$(git log -1 --format="%ct" -- "documentation/plan.md" 2>/dev/null || echo "0")
    if [ "$CODE_TIME" -gt "$PLAN_TIME" ] && [ "$PLAN_TIME" != "0" ]; then
        DAYS_OLD=$(( (CODE_TIME - PLAN_TIME) / 86400 ))
        if [ "$DAYS_OLD" -gt 7 ]; then
            echo "  INFO: plan.md not updated in $DAYS_OLD days"
            add_instruction "Consider updating plan.md with completed/new items."
        fi
    fi
fi

# 7. Summary and instructions
echo ""
echo "========================================"
echo "  Summary"
echo "========================================"
echo ""

if [ $ERRORS -gt 0 ]; then
    echo "RESULT: FAIL - $ERRORS error(s), $WARNINGS warning(s)"
    echo ""
    echo "=== CLAUDE INSTRUCTIONS ==="
    echo "Fix the errors above before committing."
    if [ -n "$INSTRUCTIONS" ]; then
        echo -e "$INSTRUCTIONS"
    fi
    echo "==========================="
    exit 1
elif [ $WARNINGS -gt 0 ]; then
    echo "RESULT: WARN - $WARNINGS warning(s)"
    echo ""
    echo "=== CLAUDE INSTRUCTIONS ==="
    echo "Consider addressing these warnings:"
    if [ -n "$INSTRUCTIONS" ]; then
        echo -e "$INSTRUCTIONS"
    fi
    echo ""
    echo "You may proceed with commit if warnings are acceptable."
    echo "==========================="
    exit 0
else
    echo "RESULT: PASS - All checks passed!"
    echo ""
    echo "Ready to commit."
    exit 0
fi
