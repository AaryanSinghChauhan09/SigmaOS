# SigmaOS Security Testing & Verification Plan

## Test Execution Matrix

### 1. Negative Syscall Boundary Testing
- Pass invalid / non-canonical memory pointers to `read`, `write`, `sys_mmap`.
- Verify kernel returns `EFAULT` without crashing or panicking.

### 2. Capability Bypassing Negative Tests
- Attempt execution of `mount` or `insmod` from process without `CAP_SYS_ADMIN`.
- Verify operation returns `EPERM` and logs audit entry.

### 3. W^X Violation Testing
- Attempt code execution on user stack / writable heap memory page.
- Verify page fault interrupt triggers process termination (`SIGSEGV`).
