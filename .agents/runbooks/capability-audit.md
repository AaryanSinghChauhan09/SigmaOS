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
