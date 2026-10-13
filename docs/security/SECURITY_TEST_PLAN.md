# Security Test Plan & Regression Audit

## Verification Tiers

Security controls in SigmaOS are verified across three distinct testing tiers:

1. **Negative Syscall & Pointer Tests:**
   - Test passing null pointers, invalid address ranges, kernel-space pointers, and misaligned buffers into syscall entrypoints.
   - Verify that all invalid inputs return appropriate error codes (`EFAULT`, `EINVAL`) without causing kernel panic or memory leakage.

2. **Sandboxing & Isolation Tests:**
   - Verify `pledge` and `unveil` restrictions by attempting unauthorized file paths and system calls.
   - Validate namespace isolation (UTS, Mount, Network, PID) to ensure processes cannot inspect or alter resource states outside their namespace.

3. **Fuzzing & Input Parsing Audits:**
   - Continuous fuzzing of ELF binary header parsers, network packet decoders, and VFS file path normalization.
   - Fuzz targets located in `fuzz/fuzz_targets/` must pass clean fuzz runs during monthly triage.
