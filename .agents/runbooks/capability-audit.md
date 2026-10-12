# Runbook: Capability Auditor (`capability-audit`)

## Purpose
Prevent "aspirational completion" by validating every claim in `README.md`, `CAPABILITY_MATRIX.toml`, and `FEATURE_STATUS.toml` against actual evidence in `reports/`.

## Execution Steps
1. Parse capability files:
   - Read `CAPABILITY_MATRIX.toml` and `FEATURE_STATUS.toml`.
2. Cross-check against source code and test suite:
   - Search for associated unit tests in `tests/` or source code in `src/`.
   - Search for runtime logs in `reports/boot/` or `reports/test/`.
3. Audit Status Integrity:
   - If a feature is marked `Supported` or `Integrated` but lacks QEMU runtime logs under `reports/boot/`, downgrade claim or flag as `unverified`.
   - If a feature claim is older than 90 days, mark as `needs-revalidation`.
4. Generate Evidence Outputs:
   - Update `reports/capability/truth_table.json` containing complete mapping of claim -> evidence link -> status.
   - Update `reports/capability/verified_today.md` formatted as a markdown dashboard.
