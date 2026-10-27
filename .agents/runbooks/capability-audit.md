# Runbook: Capability Matrix Audit

## Agent Role
Capability Auditor (`capability-auditor`)

## Workflow Steps
1. Parse `CAPABILITY_MATRIX.toml` and `FEATURE_STATUS.toml`.
2. Inspect source code references and corresponding test files in `tests/`.
3. Check `reports/boot/` and `reports/test/` for active test/boot evidence.
4. Flag claims marked as "working" or "production" that lack test or runtime evidence.
5. Generate public truth table in `reports/capability/truth_table.json` and `reports/capability/verified_today.md`.
6. Open issue for any detected claim discrepancy.
