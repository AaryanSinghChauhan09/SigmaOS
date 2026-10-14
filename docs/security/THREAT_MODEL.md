# SigmaOS Security Threat Model

## Assets

- Kernel integrity and execution context
- User process address spaces and capability tokens
- System credentials (`/etc/shadow`, PQC keystores)
- Storage volumes and filesystem structures

## Threat Vector Analysis

| Threat Vector | Mitigation Strategy | Verification / Test |
|---|---|---|
| Buffer Overflow / ROP | ASan shadow memory validation (`src/security/address_sanitizer.rs`), W^X memory paging | `cargo test --lib` ASan tests |
| Unprivileged Syscall Privilege Escalation | Capability token validation on every syscall dispatch | `tests/kernel/tier1_kernel_execution_suite.rs` |
| Malicious Package / Update Payload | Dilithium-5 PQC signature verification | `src/package/universal.rs` (`PqcPackageVerifier`) |
| Malformed Network Packet RCE | Custom TCP/IP stack with explicit bounds clamping and fuzz testing | `fuzz/fuzz_targets/` |
