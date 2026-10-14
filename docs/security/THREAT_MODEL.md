# SigmaOS Threat Model & Boundary Analysis

## System Boundaries & Threat Vectors

### 1. Userland -> Kernel Boundary (Syscalls)
- **Threat**: Buffer overflow, non-canonical address dereference, NULL-pointer injection, capability forgery.
- **Mitigation**: Standardized `Posix450SyscallDispatcher` argument validation, copy-from-user boundary verification, capability token validation on entrypoint.

### 2. Process -> Process Boundary (IPC & Shared Memory)
- **Threat**: Privilege escalation, arbitrary memory inspection, unauthorized signal injection.
- **Mitigation**: Process-local capability checks, sandboxing via Landlock/Unveil mechanisms, signal masking policy.

### 3. File System & Storage Boundary
- **Threat**: Path traversal (`../`), unauthorized file descriptor inheritance, symlink races.
- **Mitigation**: VFS path normalization, atomic file open flags (`O_CLOEXEC`), permission engine evaluation.
