#!/bin/bash
# Build script for tt-rs - delegates to each component's build.sh
set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

cd "$PROJECT_ROOT"

echo "Building all components..."
COMPONENTS=(core widgets dnd app)
for component in "${COMPONENTS[@]}"; do
    echo ""
    echo "=== Building $component ==="
    "$PROJECT_ROOT/components/$component/scripts/build.sh"
done

echo ""
echo "=== All components built successfully ==="
