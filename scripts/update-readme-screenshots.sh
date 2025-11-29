#!/bin/bash
# Update screenshot URLs in README.md with fresh cache-busting timestamps.
# Also checks if screenshots might be outdated.

set -e
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
cd "$PROJECT_ROOT"

README="README.md"
TIMESTAMP=$(date +%s)000  # Milliseconds since epoch

echo "=== Updating README Screenshot URLs ==="

if [ ! -f "$README" ]; then
    echo "ERROR: README.md not found!"
    exit 1
fi

# Update any existing timestamp parameters
# Pattern: ?ts=<digits> or &ts=<digits>
if grep -q '[?&]ts=[0-9]*' "$README"; then
    sed -i.bak "s/\([?&]ts=\)[0-9]*/\1$TIMESTAMP/g" "$README"
    rm -f "$README.bak"
    echo "Updated existing timestamp parameters to ts=$TIMESTAMP"
else
    echo "No existing timestamp parameters found in README."
    echo ""
    echo "=== CLAUDE INSTRUCTIONS ==="
    echo "To add cache-busting to screenshot URLs, change:"
    echo "  ![Screenshot](https://example.com/image.png)"
    echo "To:"
    echo "  ![Screenshot](https://example.com/image.png?ts=$TIMESTAMP)"
    echo "==========================="
fi

# Check when screenshots were last updated
echo ""
echo "Checking screenshot freshness..."

# Get last commit that touched screenshot files
SCREENSHOT_COMMIT=$(git log -1 --format="%H %ar" -- "*.png" "*.jpg" "*.gif" "assets/*.png" "assets/*.jpg" 2>/dev/null || echo "none")

if [ "$SCREENSHOT_COMMIT" != "none" ]; then
    echo "Last screenshot update: $SCREENSHOT_COMMIT"
else
    echo "No screenshot files found in repository."
fi

# Get last significant UI change
UI_COMMIT=$(git log -1 --format="%H %ar %s" -- "components/*/src/**/*.rs" "*.css" "*.html" 2>/dev/null | head -1)

if [ -n "$UI_COMMIT" ]; then
    echo "Last UI code change: $UI_COMMIT"
fi

# Compare timestamps to see if screenshots might be outdated
SCREENSHOT_TIME=$(git log -1 --format="%ct" -- "*.png" "*.jpg" "*.gif" 2>/dev/null || echo "0")
UI_TIME=$(git log -1 --format="%ct" -- "components/*/src/**/*.rs" 2>/dev/null || echo "0")

if [ "$UI_TIME" -gt "$SCREENSHOT_TIME" ] && [ "$SCREENSHOT_TIME" != "0" ]; then
    echo ""
    echo "WARNING: UI code changed after last screenshot update!"
    echo ""
    echo "=== CLAUDE INSTRUCTIONS ==="
    echo "Screenshots may be outdated. Consider updating:"
    echo ""
    echo "Screenshot conventions:"
    echo "  - Use the highest/latest user level (tt2, tt3, etc.)"
    echo "  - Expand the accordion for the most recently added feature"
    echo "  - NOT the 'About' section - show actual feature help"
    echo "  - Goal: git history shows feature evolution over time"
    echo ""
    echo "Steps:"
    echo "1. Start the dev server: ./scripts/serve.sh"
    echo "2. Navigate to http://127.0.0.1:1140"
    echo "3. Select highest user level (currently tt2)"
    echo "4. Expand help for newest feature (currently Bird/Nest messaging)"
    echo "5. Take screenshot: screencapture -w screenshot.png"
    echo "6. Save to assets/ or update external screenshot URL"
    echo "7. Update README.md with new screenshot"
    echo "==========================="
elif [ "$SCREENSHOT_TIME" = "0" ]; then
    echo ""
    echo "INFO: No screenshots found. Consider adding one to README."
fi

echo ""
echo "Done."
