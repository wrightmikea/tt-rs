#!/bin/bash
# Check CHANGELOG.md for completeness and proper formatting.
# Verifies commits are documented and <latest> placeholder is used correctly.

set -e
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
cd "$PROJECT_ROOT"

CHANGELOG="CHANGELOG.md"
ERRORS=0
WARNINGS=0

echo "=== Checking CHANGELOG.md ==="

if [ ! -f "$CHANGELOG" ]; then
    echo "ERROR: CHANGELOG.md not found!"
    exit 1
fi

# Count <latest> placeholders
LATEST_COUNT=$(grep -c '<latest>' "$CHANGELOG" 2>/dev/null || echo "0")

echo "Found $LATEST_COUNT <latest> placeholder(s)"

if [ "$LATEST_COUNT" -gt 1 ]; then
    echo "WARNING: Multiple <latest> placeholders found!"
    echo "Only the most recent entry should have <latest>."
    echo ""
    echo "=== CLAUDE INSTRUCTIONS ==="
    echo "Replace older <latest> placeholders with actual commit SHAs:"
    echo ""
    grep -n '<latest>' "$CHANGELOG" | head -5
    echo ""
    echo "Use 'git log --oneline' to find the correct SHAs."
    echo "==========================="
    WARNINGS=$((WARNINGS + 1))
fi

# Get recent commits that should be in changelog
echo ""
echo "Checking for missing commits..."

# Get commits from the last week that aren't merge commits
RECENT_COMMITS=$(git log --since="7 days ago" --no-merges --format="%h %s" -- . ':!docs/' 2>/dev/null || echo "")

if [ -n "$RECENT_COMMITS" ]; then
    MISSING=""
    while IFS= read -r line; do
        SHA=$(echo "$line" | cut -d' ' -f1)
        MSG=$(echo "$line" | cut -d' ' -f2-)

        # Check if SHA or message fragment is in changelog
        if ! grep -q "$SHA" "$CHANGELOG" 2>/dev/null; then
            # Also check for <latest> as a valid placeholder for the most recent
            if ! grep -q '<latest>' "$CHANGELOG" 2>/dev/null || [ "$(git log -1 --format='%h')" != "$SHA" ]; then
                # Check if message fragment is there (first 30 chars)
                MSG_FRAG=$(echo "$MSG" | cut -c1-30)
                if ! grep -q "$MSG_FRAG" "$CHANGELOG" 2>/dev/null; then
                    MISSING="$MISSING\n  $SHA: $MSG"
                fi
            fi
        fi
    done <<< "$RECENT_COMMITS"

    if [ -n "$MISSING" ]; then
        echo ""
        echo "WARNING: Some recent commits may not be in CHANGELOG:"
        echo -e "$MISSING"
        echo ""
        echo "=== CLAUDE INSTRUCTIONS ==="
        echo "Consider adding these commits to CHANGELOG.md if they are user-visible changes."
        echo "Use <latest> for the current commit, actual SHAs for older commits."
        echo ""
        echo "Example format:"
        echo "  - feat: Description of change (\`<latest>\`)"
        echo "  - fix: Another change (\`abc1234\`)"
        echo "==========================="
        WARNINGS=$((WARNINGS + 1))
    else
        echo "All recent commits appear to be documented."
    fi
else
    echo "No recent commits found (or all are docs/ only)."
fi

# Check for today's date section
TODAY=$(date +%Y-%m-%d)
if grep -q "## $TODAY" "$CHANGELOG"; then
    echo ""
    echo "Today's date section ($TODAY) exists in CHANGELOG."
else
    echo ""
    echo "INFO: No section for today ($TODAY) in CHANGELOG."
    echo "If you're making changes today, add a new date section."
fi

# Summary
echo ""
echo "=== Summary ==="
if [ $ERRORS -gt 0 ]; then
    echo "FAIL: $ERRORS error(s), $WARNINGS warning(s)"
    exit 1
elif [ $WARNINGS -gt 0 ]; then
    echo "WARN: $WARNINGS warning(s) - review suggestions above"
    exit 0
else
    echo "PASS: CHANGELOG looks good!"
    exit 0
fi
