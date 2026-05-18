# SigmaOS Automated Testing Guide

This guide explains how to run the automated regression tests and CI pipelines for SigmaOS.

## 1. Local Testing (QEMU/KVM)

To test the boot sequence and kernel initialization locally:

1. Compile the kernel using the standard build script.
2. Run the compiled binary using QEMU:

   ```bash
   qemu-system-x86_64 -kernel build/sigmaos_kernel.bin -serial stdio
   ```

3. Watch the serial output for `[BOOT] SSB: Userland ready. Boot sequence COMPLETE.`, indicating a successful boot.

## 2. Regression Suites (kselftest equivalent)

We use the `SovereignRegression` shard to perform self-tests:
***Memory Tests**: Verifies slab allocator boundaries and hugepage identity mapping.* **Security Tests**: Runs PQC attestation mocks to ensure `S-ARMOR` isolation is intact.

To invoke the regression suite from the shell:


```bash
sigma-cli diag run-tests



```

## 3. CI/CD Pipeline (GitHub Actions)

Every Pull Request triggers the CI/CD pipeline which automatically:

1. Builds the kernel for x86_64, ARM64, and RISC-V targets.
2. Boots each image in a headless QEMU instance.
3. Parses the serial output for kernel panics or test failures.
4. If any test fails, the pipeline aborts and blocks the merge.

For details on the pipeline configuration, see `.github/workflows/sigma-build.yml`.
