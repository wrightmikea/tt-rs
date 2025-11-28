#!/bin/bash
# Release build script for tt-rs
# Builds optimized WASM and copies to docs/ for GitHub Pages

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
APP_CRATE="$PROJECT_ROOT/components/app/crates/tt-rs-app"
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
echo "=== Validating deployment ==="
# Verify that all asset paths include /tt-rs/ prefix
if grep -q 'href="/tt-rs/' "$DOCS_DIR/index.html" && \
   grep -q 'from '\''/tt-rs/' "$DOCS_DIR/index.html"; then
    echo "PASS: Asset paths correctly use /tt-rs/ prefix"
else
    echo "FAIL: Asset paths missing /tt-rs/ prefix!"
    echo "This will cause 404 errors on GitHub Pages."
    echo "Check that --public-url /tt-rs/ was used."
    exit 1
fi

# Verify .nojekyll exists
if [ -f "$DOCS_DIR/.nojekyll" ]; then
    echo "PASS: .nojekyll file present"
else
    echo "FAIL: .nojekyll file missing!"
    exit 1
fi

# Verify WASM file exists
if ls "$DOCS_DIR"/*.wasm 1>/dev/null 2>&1; then
    echo "PASS: WASM file present"
else
    echo "FAIL: No .wasm file found!"
    exit 1
fi

echo ""
echo "=== Release build complete ==="
echo "Files in $DOCS_DIR ready for GitHub Pages deployment"
echo ""
echo "Next steps:"
echo "  1. git add docs/"
echo "  2. git commit"
echo "  3. git push"
echo "  4. Verify at https://wrightmikea.github.io/tt-rs/"
