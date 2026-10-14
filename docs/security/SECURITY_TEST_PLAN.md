# SigmaOS Security Test Plan

## Overview

Security verification in SigmaOS combines static auditing, fuzzing, standalone unit tests, and integration verification.

## Recurring Test Procedures

1. **ASan Shadow Memory Error Detection**:
   - Run `rustc --test --edition=2021 src/security/address_sanitizer.rs` to verify redzone overflow detection and use-after-free tracking.
2. **Dilithium-5 PQC Verification**:
   - Run `rustc --test --edition=2021 --cfg 'feature="standalone_test"' src/package/universal.rs` to verify post-quantum signature checks.
3. **Fuzz Smoke Matrix**:
   - Execute cargo fuzz targets under `fuzz/fuzz_targets/` (Syscall argument parsing, VFS path parsing, IPC message parsing).
