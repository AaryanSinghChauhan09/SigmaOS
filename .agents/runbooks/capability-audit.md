# Runbook: Capability Audit

## Purpose
Audit `CAPABILITY_MATRIX.toml`, `FEATURE_STATUS.toml`, and README claims against actual source code references and test evidence.

## Procedure
1. Parse all entries in `CAPABILITY_MATRIX.toml` and `FEATURE_STATUS.toml`.
2. Cross-reference declared symbols/modules against `src/` files.
3. Check if corresponding test suites exist in `tests/` or `./run_sigma_tests.sh`.
4. Classify status into standard vocabulary:
   - `unimplemented`: No code
   - `model-only`: Unit-tested, no boot path
   - `integration-pending`: Code present, unverified end-to-end
   - `qemu-verified`: Passed QEMU boot test
   - `hardware-verified`: Verified on physical hardware
   - `blocked`: Known dependency blocker
5. Output `reports/capability/truth_table.json` and update `reports/capability/verified_today.md`.
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
