#!/bin/bash
# Build script for tt-rs
# Builds all crates, runs tests, clippy, and fmt checks

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

cd "$PROJECT_ROOT"

echo "=== Building all crates ==="
cargo build --target wasm32-unknown-unknown

echo ""
echo "=== Running tests ==="
cargo test

echo ""
echo "=== Running clippy ==="
cargo clippy --target wasm32-unknown-unknown --all-features -- -D warnings

echo ""
echo "=== Checking formatting ==="
cargo fmt --all -- --check

echo ""
echo "=== All checks passed ==="
