# Kernel Security Threat Model

## Attack Vectors and Mitigation Strategies

### 1. Malformed Syscall Arguments
- **Threat**: User-supplied pointers or lengths causing kernel panic or out-of-bounds read/write.
- **Mitigation**: Bounds checking, `validate_user_ptr` copy-in/copy-out routines, and automated fuzz testing (`fuzz/fuzz_targets/ipc_parser.rs`).

### 2. Privilege Escalation
- **Threat**: Unprivileged process invoking administrative syscalls or modifying kernel data structures.
- **Mitigation**: Capability token checking at syscall entrypoints and seccomp syscall filtering.

### 3. Buffer Overflows & Use-After-Free
- **Threat**: Heap memory corruption in kernel allocators.
- **Mitigation**: Address Sanitizer (`src/security/address_sanitizer.rs`), bounded page tables, double-free detection in `PhysicalMemoryManager`.

### 4. Malicious Driver / Module Loading
- **Threat**: Untrusted kernel module insertion.
- **Mitigation**: Dilithium-5 PQC signature verification on module loading.
