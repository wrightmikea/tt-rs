# Development Learnings

This document captures solutions to issues encountered during development.

## Clippy False Positive: Dead Code Warnings for WASM Projects

**Problem:** Running `cargo clippy --all-targets --all-features -- -D warnings` reports false positive `dead_code` warnings for methods that are used in tests or will be used in WASM runtime.

**Cause:** When clippy runs without the wasm32 target, it doesn't see cfg-gated code as "used", leading to false positives.

**Solution:** Run clippy with the WASM target:

```bash
cargo clippy --target wasm32-unknown-unknown --all-features -- -D warnings
```

**Reference:** [wasm-bindgen issue #1297](https://github.com/rustwasm/wasm-bindgen/issues/1297)

---

## GitHub Pages Deployment for Trunk/WASM Projects

**Problem:** Live demo shows blank screen with 404 errors for `.wasm`, `.js`, and `.css` files.

**Cause:** By default, `trunk build` generates asset paths relative to `/` (root). GitHub Pages serves project sites at `/<repo-name>/`, so paths like `/tt-rs-xxx.wasm` resolve to `https://username.github.io/tt-rs-xxx.wasm` instead of `https://username.github.io/tt-rs/tt-rs-xxx.wasm`.

**Solution:** Use the `--public-url` flag when building for GitHub Pages:

```bash
trunk build --release --public-url /tt-rs/
```

This prefixes all asset paths with `/tt-rs/`, so they resolve correctly.

**Deployment steps:**

1. Build with correct public URL:
   ```bash
   trunk build --release --public-url /tt-rs/
   ```

2. Copy dist to docs:
   ```bash
   rm -rf docs/*
   cp -r dist/* docs/
   touch docs/.nojekyll
   ```

3. Commit and push

4. Enable GitHub Pages in repository settings:
   - Source: Deploy from branch
   - Branch: main
   - Folder: /docs

---

## Dedicated Development Port

**Problem:** Port conflicts when developing multiple projects simultaneously.

**Solution:** Assign a dedicated port per project.

For tt-rs, use port **1140**:

```bash
trunk serve --port 1140
```

To kill any existing server on that port:

```bash
lsof -ti:1140 | xargs kill -9 2>/dev/null
```

---

## Rust Edition 2024 Benefits

**Decision:** Use Rust edition 2024 (latest stable) instead of older editions.

**Benefits:**

1. **Improved Linter Checking:** Edition 2024 enables more precise lint detection and clearer error messages
2. **Language Ergonomics:** New features like improved async handling, better pattern matching exhaustiveness
3. **Future Compatibility:** Code written for 2024 edition follows current Rust idioms and best practices
4. **Stricter Defaults:** Better default warnings catch more potential issues at compile time

**Note:** Some tools (like trunk) may not fully support edition 2024 yet. If encountering issues, check tool versions and update as needed rather than downgrading the edition.

---

## sw-checklist Modularity Standards

**Tool:** `sw-checklist` enforces modularity to prevent tech debt accumulation.

**Thresholds:**

| Metric | Warning | Fail |
|--------|---------|------|
| Lines per function | > 25 | > 50 |
| Functions per module | > 4 | > 7 |
| Modules per crate | > 4 | > 7 |
| Crates per project | > 4 | > 7 |

**Rationale:** Leave room for future features. When you're at the threshold, the next feature forces a refactor.

### Workspace Refactoring Strategy

When a single-crate project exceeds thresholds, restructure as a Rust workspace:

1. **Create workspace root `Cargo.toml`:**
   ```toml
   [workspace]
   resolver = "2"
   members = ["crates/crate-name", ...]

   [workspace.package]
   version = "0.1.0"
   edition = "2024"
   ```

2. **Split by responsibility:**
   - `*-core`: Core traits and types (no dependencies)
   - `*-number`, `*-text`, etc.: Domain widgets
   - `*-ui`: Presentation components
   - `*-app`: WASM entry point (ties everything together)

3. **Use workspace inheritance:**
   ```toml
   [package]
   name = "crate-name"
   version.workspace = true
   edition.workspace = true
   ```

### Function Splitting Strategy

When a module has too many functions:

1. **Group by cohesion:** Accessors, constructors, operations, trait implementations
2. **Create focused modules:** `accessors.rs`, `arithmetic.rs`, `widget_impl.rs`
3. **Keep public API simple:** Re-export from `lib.rs`

### Example Split (Number Widget)

Before: `number.rs` with 18+ functions

After:
```
tt-rs-number/src/
├── lib.rs          # Re-exports public API
├── number.rs       # Struct and constructors (4 functions)
├── accessors.rs    # Getters and apply_to (7 functions)
├── arithmetic.rs   # Pure math functions (6 functions)
├── operator.rs     # ArithOperator enum (2 functions)
└── widget_impl.rs  # Widget trait impl (7 functions)
```

---

## Trunk with Rust Workspaces

**Problem:** `trunk serve` reports "could not find the root package of the target crate" with workspace projects.

**Cause:** Trunk needs explicit configuration when the Cargo.toml at project root is a workspace, not a package.

**Solution:** In `index.html`, specify the exact crate path:

```html
<link data-trunk rel="rust" href="crates/tt-rs-app/Cargo.toml" data-wasm-opt="z">
```

If trunk still fails, create a `Trunk.toml` at project root:

```toml
[build]
target = "index.html"

[[proxy]]
backend = "http://localhost:1140/"
```

**Solution:** Keep `index.html`, `favicon.ico`, and `assets/` within the app crate directory (`crates/tt-rs-app/`). Run trunk from that crate directory. Use project scripts to automate this.

---

## Project Scripts Best Practice

**Principle:** Always use project scripts for building and serving. Repeated use prevents regressions and makes builds repeatable and reproducible.

**Benefits:**

1. **Reproducibility:** Same commands run every time, no manual errors
2. **Documentation:** Scripts self-document the build process
3. **Regression Prevention:** Tested scripts catch issues early
4. **Onboarding:** New developers can build immediately

**Standard Scripts:**

```
scripts/
├── serve.sh        # Development server
├── build-all.sh    # Build + test + clippy + fmt
└── build-release.sh # Production build for deployment
```

**serve.sh** - Development server:
```bash
#!/bin/bash
set -e
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
APP_CRATE="$PROJECT_ROOT/crates/tt-rs-app"

lsof -ti:1140 | xargs kill -9 2>/dev/null || true
cd "$APP_CRATE"
trunk serve --port 1140
```

**build-all.sh** - Quality checks:
```bash
#!/bin/bash
set -e
cd "$(dirname "$0")/.."

cargo build --target wasm32-unknown-unknown
cargo test
cargo clippy --target wasm32-unknown-unknown --all-features -- -D warnings
cargo fmt --all -- --check
```

**Usage in development:**
```bash
# Start dev server
./scripts/serve.sh

# Run all quality checks
./scripts/build-all.sh

# Build for deployment
./scripts/build-release.sh
```
