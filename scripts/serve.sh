#!/bin/bash
# Development server script for tt-rs - delegates to app component's serve.sh
set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

echo "Starting tt-rs development server..."
echo "URL: http://127.0.0.1:1140"
echo ""

exec "$PROJECT_ROOT/components/app/scripts/serve.sh"
