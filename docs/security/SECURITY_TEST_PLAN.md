# Security Test Plan & Verification Directives

## Negative Testing Requirement
No security feature or capability check is considered complete without negative tests proving that unauthorized actions are rejected.

---

## Required Security Test Categories

1. **Capability Escalation Rejection**:
   - Verify unprivileged process attempting `sys_mount` receives `EPERM`.
   - Tested in `tests/security/capability_test.rs`.

2. **VFS Path Traversal & Unveil Enforcement**:
   - Verify process restricted by `unveil("/tmp", "rw")` cannot access `/etc/passwd`.
   - Verified via VFS path parser fuzzing and unit tests.

3. **Syscall Argument Validation**:
   - Table-driven tests passing null pointers, out-of-bound addresses, and negative buffer lengths into syscall handlers.

4. **Package Signature Verification**:
   - Test PQC verifier rejection of tampered packages with mismatched Dilithium signatures.
