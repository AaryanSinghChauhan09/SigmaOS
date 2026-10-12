# Runbook: Build and Test Sentinel (`build-test`)

## Purpose
Ensure that all code changes undergo clean builds, flaky-test detection, and evidence generation before being considered for integration.

## Repeated Tasks
1. Clean checkout build validation:
   - Execute `cargo check --lib`
   - Execute `cargo test --lib`
   - Execute `./run_sigma_tests.sh`
   - Execute `make check`
   - Execute `make test`
   - Execute `make format`
2. Flaky-test mitigation:
   - Rerun any failing test up to 3 times to distinguish transient/flaky failures from true regressions.
3. Report Generation:
   - Record commands, start/end timestamps, exit codes, execution duration, and log excerpts.
   - Output structured report to `reports/build/latest.json` and `reports/test/latest.json`.

## Success Criteria
- All standard build and test commands exit with code 0.
- No new flaky tests detected.
- Verification hash generated and signed in `reports/test/latest.json`.
