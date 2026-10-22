# Daily Health Runbook

## Objective
Keep build, unit tests, integration tests, and boot status healthy across all supported architectures.

## Steps
1. Run `cargo check --lib` and `cargo test --lib`.
2. Run `./run_sigma_tests.sh`.
3. Check boot matrix for build regressions or test flakiness.
4. Record daily triage status report in `research/reports/daily/`.
