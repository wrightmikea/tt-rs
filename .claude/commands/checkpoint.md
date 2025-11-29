Complete checkpoint: validate, commit, deploy, and push all changes.

This command ensures CHANGELOG, README.md, screenshot1.png, and live demo are all up-to-date.

## Steps to Execute

### 1. CHANGELOG Sync
Compare `git log --oneline` with `CHANGELOG.md`:
- Add any missing commits to the current date section
- Replace previous `<latest>` with its actual SHA
- Use `<latest>` for the newest commit entry
- Commits should be in reverse chronological order (newest first)

### 2. Pre-commit Checks
Run `./scripts/pre-commit-checks.sh` and fix any issues:
- Build failures: fix the code
- CHANGELOG issues: update with proper SHAs and `<latest>` placeholder

### 3. Screenshot Check
If UI code has changed since last screenshot update:
- Start dev server: `./scripts/serve.sh`
- Navigate to http://127.0.0.1:1140
- Select highest user level (currently tt2)
- Open help panel and expand latest feature section (currently Bird/Nest messaging)
- Get current epoch: `date +%s`
- Delete old screenshot: `rm images/screenshot-*.png`
- Take screenshot and save to `images/screenshot-<epoch>.png`
- Update README.md link to point to new screenshot filename
- Goal: Screenshot shows latest level with latest feature help expanded

### 4. Commit Code Changes
If there are uncommitted changes:
```bash
git add -A
git commit -m "feat/fix/docs: <description>"
```

### 5. Build Release
```bash
./scripts/build-release.sh
```
Verify validation passes (PASS for all checks).

### 6. Deploy Commit
```bash
git add docs/
git commit -m "deploy: Update live demo with <description>"
```

### 7. Push
```bash
git push
```

### 8. Verification
After push, verify:
- [ ] Live demo works: https://wrightmikea.github.io/tt-rs/
- [ ] README screenshot shows current UI (tt2 with messaging help)
- [ ] CHANGELOG includes all recent commits with correct SHAs
- [ ] Footer "Changes" link shows updated changelog

## Checklist Summary

Before every push, ensure:
- [ ] CHANGELOG.md has all commits (compare with `git log --oneline`)
- [ ] Screenshot (images/screenshot-*.png) shows latest level (tt2) with latest feature help expanded
- [ ] Live demo (docs/) is rebuilt with `./scripts/build-release.sh`
- [ ] All links in README.md and footer are working

## Important Notes

- If any step fails, stop and fix before continuing
- Screenshot requirements: latest user level, latest feature help expanded
- CHANGELOG: use `<latest>` for current commit, replace previous `<latest>` with actual SHA
- Always verify live demo loads correctly after push
