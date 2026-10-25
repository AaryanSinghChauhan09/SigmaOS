# SigmaOS Kernel Security Test Plan

## Test Taxonomy for Security

1. **Negative Pointer & Buffer Tests**:
   - Verify negative pointer addresses passed to syscalls return `-EFAULT`.
2. **Capability Denial Tests**:
   - Verify unprivileged process calls to `CAP_SYS_ADMIN` paths return `-EPERM`.
3. **Pledge & Unveil Boundary Tests**:
   - Verify processes attempting un-pledged syscalls or unveiled paths receive `SIGABRT` / `-EACCES`.
4. **Fuzzing**:
   - Monthly fuzzing of syscall dispatcher, ELF parser, VFS paths, and network packet handlers using `fuzz/` targets.
