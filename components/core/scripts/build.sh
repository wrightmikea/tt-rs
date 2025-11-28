#!/bin/bash
set -e
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$(dirname "$SCRIPT_DIR")"
echo "Building core component..."
cargo build --target wasm32-unknown-unknown
cargo clippy --target wasm32-unknown-unknown -- -D warnings
cargo fmt --check
echo "Core component built successfully"
