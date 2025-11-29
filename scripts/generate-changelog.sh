#!/bin/bash
# Generate CHANGELOG.md from git history
# Usage: ./scripts/generate-changelog.sh [pending_description]
# If pending_description is provided, it's added at top with <latest>

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
CHANGELOG="$PROJECT_ROOT/CHANGELOG.md"

PENDING_DESC="$1"

{
    echo "# Changelog"
    echo ""
    echo "All notable changes to tt-rs."
    echo ""

    # Group commits by date
    current_date=""

    # Add pending change if provided
    if [ -n "$PENDING_DESC" ]; then
        today=$(date +%Y-%m-%d)
        echo "## $today"
        echo ""
        echo "- $PENDING_DESC (\`<latest>\`)"
        current_date="$today"
    fi

    # Process git log
    git log --format="%ad %h %s" --date=short | while read -r line; do
        date=$(echo "$line" | cut -d' ' -f1)
        sha=$(echo "$line" | cut -d' ' -f2)
        msg=$(echo "$line" | cut -d' ' -f3-)

        if [ "$date" != "$current_date" ]; then
            echo ""
            echo "## $date"
            echo ""
            current_date="$date"
        fi

        echo "- $msg (\`$sha\`)"
    done
} > "$CHANGELOG"

echo "Generated $CHANGELOG"
