# Runbook: Build and Test Sentinel

## Purpose
Execute repository-wide build checks, standalone module test suites, and generate a machine-readable test manifest.

## Procedure
1. Clean temporary artifacts if requested: `make clean`
2. Run standard Rust check: `cargo check --lib`
3. Run standard unit tests: `cargo test --lib`
4. Run native SigmaOS test runner script: `./run_sigma_tests.sh`
5. Collect test exit codes, duration, and failure excerpts.
6. Write test manifest to `reports/test/test_manifest.json`.
7. If failures occur, classify as: compilation_error, test_assertion_failure, script_missing, or timeout.
