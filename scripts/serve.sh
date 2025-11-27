#!/bin/bash
# Development server script for tt-rs
# Runs trunk serve from the tt-rs-app crate directory on dedicated port 1140

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
APP_CRATE="$PROJECT_ROOT/crates/tt-rs-app"

# Kill any existing server on port 1140
lsof -ti:1140 | xargs kill -9 2>/dev/null || true

echo "Starting tt-rs development server..."
echo "URL: http://127.0.0.1:1140"
echo ""

cd "$APP_CRATE"
trunk serve --port 1140
