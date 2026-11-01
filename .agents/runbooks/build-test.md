# Runbook: Build and Test Sentinel (`build-test.md`)

## Objective
Verify repository compilation, unit test suites, integration test scripts, and formatting checks cleanly across the workspace.

## Execution Sequence
1. **Repository Check:**
   ```bash
   cargo check --lib
   ```
2. **Unit Test Suite:**
   ```bash
   cargo test --lib
   ```
3. **Integration Test Suite:**
   ```bash
   ./run_sigma_tests.sh
   ```
4. **Makefile Verification:**
   ```bash
   make check
   make test
   make format
   ```
5. **Flaky Test Protocol:**
   - If a test fails, rerun up to 3 times to distinguish environmental flakiness from deterministic regression.
   - Record failures in `reports/test/summary.json`.

## Expected Outputs
- `reports/build/latest.json`: Compiler exit code, duration, artifact hashes.
- `reports/test/summary.json`: Pass/fail count, duration, failure classifications.
