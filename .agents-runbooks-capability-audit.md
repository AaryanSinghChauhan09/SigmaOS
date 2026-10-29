# Runbook: Capability Auditor Execution (.agents/runbooks/capability-audit.md)

1. Parse `CAPABILITY_MATRIX.toml` and `FEATURE_STATUS.toml`.
2. Cross-reference source code presence, test files, and logs in `reports/`.
3. Highlight claims labeled `Integrated` or `Supported` lacking linked test/boot evidence.
4. Output truth table to `reports/capability/truth_table.json` and dashboard to `reports/capability/verified_today.md`.
