#!/bin/bash
set -e
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
COMPONENT_DIR="$(dirname "$SCRIPT_DIR")"
APP_CRATE="$COMPONENT_DIR/crates/tt-rs-app"
lsof -ti:1140 | xargs kill -9 2>/dev/null || true
cd "$APP_CRATE"
trunk serve --port 1140
