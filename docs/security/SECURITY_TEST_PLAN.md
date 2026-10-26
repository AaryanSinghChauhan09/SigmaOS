# SigmaOS Security Test Plan

This plan outlines automated and manual verification procedures for validating kernel security mechanisms.

## Test Suites & Coverage Requirements

| Security Control | Verification Suite / Tool | Frequency | Success Criteria |
|---|---|---|---|
| Syscall Argument Validation | `tests/security/` & Fuzzing | Biweekly / Monthly | Rejection of malformed pointers, invalid buffers, and negative lengths |
| W^X Memory Isolation | `tests/mm/virtual.rs` | Every PR | Attempts to execute writable pages trigger fault |
| Capability Rights Enforcement | `tests/security/` | Biweekly | Operations without valid capability tokens return `EPERM` |
| Resource Limit Bounds | `tests/process/rlimits/` | Biweekly | Exceeding limits returns `ENOMEM` / `EAGAIN` without kernel panic |
| PQC Signature Verification | `tests/security/` | Quarterly | Rejection of invalid Dilithium-5 / Kyber payload signatures |
