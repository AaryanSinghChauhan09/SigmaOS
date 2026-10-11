# Kernel Security Test Plan

## Verification Matrix

| Area | Strategy | Automation | Frequency |
|---|---|---|---|
| Syscall bounds | Fuzzing & negative input tests | `cargo fuzz` / `tests/` | Continuous / Monthly |
| Memory safety | Address Sanitizer & shadow state | `src/security/address_sanitizer.rs` | Every PR |
| PQC verification | Signature validation tests | `PqcPackageVerifier` unit tests | Every PR |
| Privilege checks | Negative capability tests | Syscall unit tests | Biweekly |
