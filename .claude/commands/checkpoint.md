Complete checkpoint: validate, commit, deploy, and push all changes.

This command performs a full checkpoint workflow - from validation through deployment.

## Steps to Execute

### 1. Pre-commit Checks
Run `./scripts/pre-commit-checks.sh` and fix any issues:
- Build failures: fix the code
- CHANGELOG issues: update with proper SHAs and `<latest>` placeholder

### 2. Screenshot Check
If UI code has changed since last screenshot update:
- Start dev server: `./scripts/serve.sh`
- Navigate to http://127.0.0.1:1140
- Select highest user level (currently tt2)
- Open help panel and expand latest feature section (currently Bird/Nest messaging)
- Take screenshot and save to `images/screenshot1.png`
- Goal: Screenshot shows latest level with latest feature help expanded

### 3. Commit Code Changes
If there are uncommitted changes:
```bash
git add -A
git commit -m "feat/fix/docs: <description>"
```

### 4. Build Release
```bash
./scripts/build-release.sh
```
Verify validation passes (PASS for all checks).

### 5. Deploy Commit
```bash
git add docs/
git commit -m "deploy: Update live demo with <description>"
```

### 6. Push
```bash
git push
```

### 7. Verification
- Live demo: https://wrightmikea.github.io/tt-rs/
- README screenshot: Check GitHub repo page shows updated screenshot

## Important Notes

- If any step fails, stop and fix before continuing
- Screenshot requirements: latest user level, latest feature help expanded
- CHANGELOG: use `<latest>` for current commit, replace previous `<latest>` with actual SHA
- Always verify live demo loads correctly after push
