# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

tt-rs ("Cartoon-oriented Talking Programming Application") is a Rust/WebAssembly reimplementation of ToonTalk, a visual programming environment for learning. Users create programs by training robots that watch and learn from their actions.

This is a derived work based on ToonTalk by Ken Kahn. See COPYRIGHT and LICENSE files.

## Build & Development Commands

```bash
# Prerequisites (one-time setup)
rustup target add wasm32-unknown-unknown
cargo install trunk wasm-bindgen-cli

# Development server (DEDICATED PORT: 1140)
trunk serve --port 1140  # Dev server at http://127.0.0.1:1140
cargo test               # Run all tests
cargo test test_name     # Run specific test
cargo watch -x test      # Watch mode

# Kill any existing server on dedicated port
lsof -ti:1140 | xargs kill -9 2>/dev/null

# Production
trunk build --release    # Build optimized WASM

# Quality (run before every commit)
cargo clippy --target wasm32-unknown-unknown --all-features -- -D warnings
cargo fmt --all
cargo doc --open         # Generate documentation
```

**Note:** This project uses port 1140 exclusively to avoid conflicts with other projects.

**Important:** Run clippy with `--target wasm32-unknown-unknown` to avoid false dead_code warnings. See [wasm-bindgen issue #1297](https://github.com/rustwasm/wasm-bindgen/issues/1297).

## Architecture

### Core Domain Concepts

- **Widgets** - Base trait for all visual objects (numbers, boxes, robots, birds/nests, scales)
- **Robots** - Programs trained by demonstration; they watch user actions and learn to repeat them
- **Birds/Nests** - Actor-model message passing (birds carry messages to nests)
- **Boxes** - Container widgets holding other widgets
- **Scales** - Comparison widgets

### Module Structure (planned)

```
src/
├── domain/        # Core business logic (widgets, robots, patterns)
├── execution/     # Robot execution engine, scheduling
├── presentation/  # Yew components for UI
├── rendering/     # Three.js/SVG graphics bindings
├── storage/       # LocalStorage/IndexedDB persistence
├── audio/         # Text-to-speech, sound effects
└── bindings/      # JavaScript interop (minimal)
```

### Technology Stack

- **Rust** compiled to WebAssembly
- **Yew** for reactive UI components
- **Three.js** (via wasm-bindgen) for 3D graphics
- **SVG/CSS** for 2D graphics and animations

## Development Guidelines

### Code Quality Standards

- Zero clippy warnings (use `-D warnings`)
- All public APIs must have doc comments
- Rust 2024 edition idioms
- Use inline format args: `format!("{name}")` not `format!("{}", name)`
- Files under 500 lines (prefer 200-300)
- Functions under 50 lines (prefer 10-30)
- Maximum 3 TODO comments per file; convert persistent TODOs to GitHub issues

### Rust/WASM Specific

- All business logic in Rust; JavaScript only for WASM loading
- Use `wasm-bindgen` for JS interop, `web-sys` for DOM
- Write tests in Rust using `wasm-bindgen-test`, not JavaScript
- Use `thiserror` for error types with descriptive messages

### Pre-Commit Checklist

1. `cargo test` - all tests pass
2. `cargo clippy --target wasm32-unknown-unknown --all-features -- -D warnings` - zero warnings
3. `cargo fmt --all` - code formatted
4. Update documentation if features changed

## Reference Implementation

The original JavaScript ToonTalk Reborn is at https://github.com/ToonTalk/ToonTalk with wiki documentation. When implementing features, refer to that codebase for behavior reference.

## Documentation

- [architecture.md](documentation/architecture.md) - System design and module structure
- [prd.md](documentation/prd.md) - Product requirements and user stories
- [design.md](documentation/design.md) - Technical design decisions
- [plan.md](documentation/plan.md) - Implementation roadmap
- [process.md](documentation/process.md) - Development workflow
- [ai_agent_instructions.md](documentation/ai_agent_instructions.md) - AI coding assistant guidelines
