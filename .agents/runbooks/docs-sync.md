# Runbook: Documentation Sync

## Purpose
Ensure README commands, build scripts, Makefile targets, and agent capability disclosures remain synchronized with actual test evidence.

## Procedure
1. Extract all bash commands documented in `README.md` and `CONTRIBUTING.md`.
2. Verify each command against `Makefile`, `Cargo.toml`, and scripts in `scripts/` or `tools/`.
3. Check `reports/capability/verified_today.md` for current verified status.
4. Flag documentation discrepancies or unverified claims.
5. Generate documentation synchronization report in `reports/docs/docs_sync_report.md`.
