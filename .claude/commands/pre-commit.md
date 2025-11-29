Run pre-commit checks and address any issues found.

Execute `./scripts/pre-commit-checks.sh` and then:

1. Review all FAIL and WARN items reported by the script
2. Follow any CLAUDE INSTRUCTIONS printed by the script
3. For each issue found:
   - If it's a build failure: fix the code
   - If live demo is outdated: run `./scripts/build-release.sh && git add docs/ && git commit -m 'deploy: ...'`
   - If CHANGELOG needs updates: add missing commits with proper SHA references
   - If documentation is stale: review and update as needed

4. **Screenshot Requirements** (when screenshots need updating):
   - Use the HIGHEST/LATEST user level (currently tt2, later tt3, tt4, etc.)
   - Expand the accordion for the MOST RECENTLY ADDED feature in that level
   - Do NOT show the "About" section - show actual feature help content
   - Goal: Git history of screenshots shows feature evolution over time
   - Example: Currently show tt2 level with Bird/Nest messaging help expanded

5. After addressing issues, run `./scripts/pre-commit-checks.sh` again to verify
6. Report the final status (PASS/WARN/FAIL)

Note: This command is designed to be run before commits to ensure:
- Build passes
- Live demo is current
- CHANGELOG is complete
- Documentation is reasonably fresh
- Screenshots reflect latest features
