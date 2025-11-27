#!/bin/bash
# Release build script for tt-rs
# Builds optimized WASM and copies to docs/ for GitHub Pages

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
APP_CRATE="$PROJECT_ROOT/crates/tt-rs-app"
DOCS_DIR="$PROJECT_ROOT/docs"

cd "$APP_CRATE"

echo "=== Building release ==="
trunk build --release --public-url /tt-rs/

echo ""
echo "=== Copying to docs/ for GitHub Pages ==="
rm -rf "$DOCS_DIR"/*
cp -r dist/* "$DOCS_DIR"/
touch "$DOCS_DIR/.nojekyll"

echo ""
echo "=== Release build complete ==="
echo "Files in $DOCS_DIR ready for GitHub Pages deployment"
