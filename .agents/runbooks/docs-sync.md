# Runbook: Documentation and Onboarding Agent (`docs-sync.md`)

## Objective
Ensure README commands, developer guides, and onboarding runbooks match actual Makefile and script targets, and update "Verified Today" documentation.

## Execution Sequence
1. **Command Sync:**
   - Cross-reference commands in `README.md` with `Makefile` and `scripts/`.
2. **Verified Today Sync:**
   - Update `reports/capability/verified_today.md` following capability audits.
3. **Contributor Path Test:**
   - Verify clean clone -> build -> test -> runbook execution path works with zero missing dependencies.
