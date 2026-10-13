# SigmaOS Security Test Plan

## Negative Testing Standard
No security control or privilege boundary is marked complete without negative tests demonstrating rejection of unauthorized access.

## Automated Verification Targets
- **Syscall Argument Fuzzing:** `fuzz/fuzz_targets/ipc_parser.rs` and `fuzz/fuzz_targets/vfs_path_parser.rs`.
- **Capability Violations:** `tests/distro_inspection_and_security_tests.rs`.
- **Address Sanitizer Detection:** `src/security/address_sanitizer.rs` unit test suite.
