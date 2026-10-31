# SigmaOS Security Architecture

## Principles
1. **Least Privilege**: Components run with the minimum set of capabilities needed for their execution.
2. **Fail Closed**: In any ambiguous or corrupt state, security enforcement blocks access rather than defaulting to permissive behavior.
3. **No Custom Cryptography**: All cryptographic operations rely on standard, audited primitives (e.g., Ed25519, ChaCha20-Poly1305, post-quantum Dilithium-5).
4. **Defense in Depth**: Privilege boundaries are enforced simultaneously at compile time (Rust memory safety), syscall entry (capability checks, seccomp filters), and namespace isolation (PID, mount, network namespaces).

---

## Architecture Boundaries
- **Kernel Space**: Ring 0 execution containing core memory management, scheduler, trap handlers, and strictly verified IPC endpoints.
- **Microkernel Isolation**: Unprivileged drivers and non-essential system services execute in isolated userspace containers with restricted capability sets.
- **Userspace**: Applications run in Ring 3 unprivileged mode subject to resource limits (rlimits) and strict namespace barriers.
- **IPC Channels**: All inter-process communication routes through bounded message queues, pipes, or capability-governed shared memory segments with explicit permissions.
