# AI Agent Development Instructions for Security & Memory Protection Subsystems (`src/security/`)

This directory implements memory exploit mitigations, binary protection standards, ASLR/KASLR address space randomization, stack canaries, non-executable memory stacks (`NX`/`DEP`), control flow integrity (`CFI`), OpenBSD KARL (Kernel Address Randomized Link), input bounds validation, and process privilege containment for SigmaOS.

## Subsystem Architecture & Directives

1. **Memory Exploit Mitigations & Binary Protection (`binary_protection.rs`, `kernel_hardening.rs`, `openbsd_karl.rs`)**
   - Enforce Non-Executable Stacks and Pages (`NX` / `DEP`) across all userland processes and dynamic kernel allocations.
   - Enforce Stack Canary Guards (`-fstack-protector-strong` parity) on function prologue/epilogue frames to catch stack frame buffer overwrites before return address unwinding.
   - KASLR & KARL: Randomize kernel base addresses and re-link kernel symbol locations on every boot to neutralize fixed-address ROP/JOP chain exploits.

2. **Input Bounds Validation & Buffer Management (`input_validation.rs` & `vulnerability.rs`)**
   - All string and byte slice operations must enforce explicit length checks (`len() <= MAX_ALLOWED_LEN`) before copy operations.
   - Strictly prohibit unbounded memory copies (`memcpy` without explicit boundary bounds). Use safe Rust slice copies (`copy_from_slice`) or safe wrapper APIs.
   - Sanitize all external IPC payloads and device driver ioctl input buffers.

3. **Syscall Filtering & Sandboxing (`seccomp.rs`, `seccomp_ebpf.rs`, `pledge.rs`, `unveil.rs`)**
   - Restrict process capabilities using eBPF seccomp syscall filters and OpenBSD `pledge`/`unveil` path restrictions to mitigate post-exploitation privilege escalation.

4. **Verification & Audit**
   - Validate code changes with `cargo check --lib` and run relevant security unit tests before submitting.
