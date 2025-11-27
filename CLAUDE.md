# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

tt-rs ("Cartoon-oriented Talking Programming Application") is a Rust/WebAssembly reimplementation of ToonTalk, a visual programming environment for learning. Users create programs by training robots that watch and learn from their actions.

This is a derived work based on ToonTalk by Ken Kahn. See COPYRIGHT and LICENSE files.

## IMPORTANT: Always Use Project Scripts

**Always use project scripts for building and serving.** This prevents regressions and ensures reproducible builds.

```bash
# Development server (DEDICATED PORT: 1140)
./scripts/serve.sh           # Start dev server at http://127.0.0.1:1140

# Quality checks (run before every commit)
./scripts/build-all.sh       # Build + test + clippy + fmt

# Production build for GitHub Pages
./scripts/build-release.sh   # Build and copy to docs/
```

**Before starting development:** Read [learnings.md](documentation/learnings.md) for solutions to common issues.

## Build & Development Commands

```bash
# Prerequisites (one-time setup)
rustup target add wasm32-unknown-unknown
cargo install trunk wasm-bindgen-cli

# Manual commands (prefer scripts above)
cargo test               # Run all tests
cargo test test_name     # Run specific test

# Kill any existing server on dedicated port
lsof -ti:1140 | xargs kill -9 2>/dev/null

# Quality (run before every commit)
cargo clippy --target wasm32-unknown-unknown --all-features -- -D warnings
cargo fmt --all
sw-checklist .           # Check modularity and standards
```

**Note:** This project uses port 1140 exclusively to avoid conflicts with other projects.

**Important:** Run clippy with `--target wasm32-unknown-unknown` to avoid false dead_code warnings.

## Workspace Architecture

This project uses a Rust workspace with focused crates for modularity:

```
tt-rs/
├── Cargo.toml          # Workspace definition
├── scripts/            # Build and serve scripts
│   ├── serve.sh        # Development server
│   ├── build-all.sh    # Quality checks
│   └── build-release.sh # Production build
├── crates/
│   ├── tt-rs-core/     # Core Widget trait, WidgetId, MatchResult (2 modules)
│   ├── tt-rs-number/   # Number widget with rational arithmetic (6 modules)
│   ├── tt-rs-ui/       # UI components - Footer (2 modules)
│   └── tt-rs-app/      # Main WASM app entry point (2 modules)
│       ├── index.html  # Trunk entry point
│       ├── favicon.ico
│       └── assets/     # CSS stylesheets
├── docs/               # GitHub Pages deployment
└── documentation/      # Project documentation
```

### Crate Responsibilities

- **tt-rs-core**: Widget trait, WidgetId, MatchResult - no dependencies
- **tt-rs-number**: Number widget (rational arithmetic, pattern matching)
- **tt-rs-ui**: Reusable UI components (Footer with build info)
- **tt-rs-app**: Application entry point, ties everything together, contains web assets

### Modularity Guidelines (sw-checklist)

- Functions: max 50 LOC (warn >25)
- Modules: max 7 functions (warn >4)
- Crates: max 7 modules (warn >4)
- Projects: max 7 crates (warn >4)

Keep modules focused to leave room for future features.

## Technology Stack

- **Rust 2024 edition** - Latest stable with improved linting and language features
- **Yew** for reactive UI components
- **Trunk** for WASM bundling (run from tt-rs-app crate)
- **Three.js** (planned) for 3D graphics
- **SVG/CSS** for 2D graphics and animations

## Development Guidelines

### Pre-Commit Checklist

Run `./scripts/build-all.sh` or manually:

1. `cargo test` - all tests pass
2. `cargo clippy --target wasm32-unknown-unknown --all-features -- -D warnings`
3. `cargo fmt --all`
4. `sw-checklist .` - check modularity standards

### Code Quality

- Zero clippy warnings
- All public APIs have doc comments
- Rust 2024 edition idioms
- Use inline format args: `format!("{name}")` not `format!("{}", name)`

## DEPLOYMENT CHECKLIST - MANDATORY

**DO NOT BREAK THE LIVE DEMO.** Always use `./scripts/build-release.sh` for deployments.

### Common Mistakes (DO NOT REPEAT)

1. **Missing `--public-url /tt-rs/`** - GitHub Pages serves at `/<repo>/` subdirectory
   - Symptom: Blank page, 404 errors for .wasm, .js, .css files
   - Solution: Always use `./scripts/build-release.sh` which includes validation
   - NEVER run `trunk build --release` directly for deployment

2. **Missing `.nojekyll` file** - GitHub Jekyll processing breaks WASM
   - Symptom: Files with underscores don't load
   - Solution: Script automatically creates this file

3. **Forgetting to copy to docs/** - Build goes to dist/, not docs/
   - Solution: Script handles this automatically

### Deployment Steps

```bash
# 1. ALWAYS use the release script
./scripts/build-release.sh

# 2. The script will:
#    - Build with --public-url /tt-rs/
#    - Copy to docs/
#    - Create .nojekyll
#    - VALIDATE paths contain /tt-rs/ prefix
#    - FAIL if validation fails

# 3. Commit and push
git add docs/
git commit -m "Deploy to GitHub Pages"
git push

# 4. Verify live demo works at:
#    https://wrightmikea.github.io/tt-rs/
```

### Before Any Deployment

- [ ] Read [learnings.md](documentation/learnings.md) if encountering issues
- [ ] Use `./scripts/build-release.sh` (NOT manual trunk commands)
- [ ] Verify script validation passes
- [ ] Test locally if possible
- [ ] Check live demo after push

## Documentation

- [architecture.md](documentation/architecture.md) - System design
- [prd.md](documentation/prd.md) - Product requirements
- [design.md](documentation/design.md) - Technical design
- [plan.md](documentation/plan.md) - Implementation roadmap
- [learnings.md](documentation/learnings.md) - **Solutions to issues encountered** (read this first!)
