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
