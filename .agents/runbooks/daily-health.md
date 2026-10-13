# Runbook: Daily Health & Build Verification

## Purpose
Ensure workspace compilation, unit tests, and boot status remain clean daily.

## Procedure
1. Run workspace check: `cargo check --lib`
2. Run unit tests: `cargo test --lib`
3. Execute master test suite: `./run_sigma_tests.sh`
4. Record build/test results in `research/reports/daily/`
5. Triage any test or build regressions immediately.
