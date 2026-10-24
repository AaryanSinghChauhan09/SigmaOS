# Runbook: Build and Test Sentinel Execution (.agents/runbooks/build-test.md)

1. Clean checkout: `git checkout clean` or `cargo clean`.
2. Run standard verification pipeline:
   - `cargo check --lib`
   - `cargo test --lib`
   - `./run_sigma_tests.sh`
   - `make check`
   - `make test`
   - `make format`
3. Retry intermittent failures up to 3 times to distinguish flaky tests from real regressions.
4. Record output digest to `reports/build/latest.json` and `reports/test/summary.json`.
