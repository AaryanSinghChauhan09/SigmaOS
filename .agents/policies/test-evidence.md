# Agent Test Evidence Policy

## Evidence Standards

1. **Objective Verification**: No capability claim may be upgraded without attaching a machine-readable test or boot report.
2. **Reproducible Commands**: Evidence artifacts must record command string, working directory, exit code, execution time, and toolchain version.
3. **Flaky Test Retries**: Transient test failures must be rerun up to 3 times before being classified as persistent failures.
4. **Report Storage**: Reports must be written to designated `reports/` subdirectories.
