# Daily Health Runbook

## Purpose
Keep build, unit tests, integration tests, and boot status healthy.

## Workflow
1. Execute `cargo check --lib`.
2. Execute `cargo test --lib`.
3. Execute `./run_sigma_tests.sh`.
4. Inspect build artifacts and boot matrix outputs.
5. Record status report and failure triage logs in `research/reports/daily/`.
