# SigmaOS Security Test Plan

## Security Testing Methodology

1. **Syscall Negative Testing**: Test invalid pointers, null pointers, negative buffer lengths, and unauthorized capabilities across all syscalls (`tests/security/capability_test.rs`).
2. **Fuzzing Campaigns**: Continuous fuzzing of VFS path normalization, ELF parsing, IPC messages, and network packet handlers (`fuzz/fuzz_targets/`).
3. **Capability Sandbox Verification**: Validate that pledged/unveiled process sandboxes block restricted file paths and syscalls.
4. **Memory Safety Audits**: Automated checks for missing bounds validation or unsafe block justifications.
