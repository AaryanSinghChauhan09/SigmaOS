# SigmaOS Kernel Security Test Plan

## Overview
This document specifies automated security verification suites across memory safety, syscall bounds checks, capability enforcement, and fuzz testing.

---

## Security Verification Test Suites

### 1. Syscall Bounds & Pointer Validation Tests
- **Target**: `src/kernel/syscall/`
- **Execution**: Negative pointer tests (`NULL`, unmapped address, kernel memory pointers from userspace).

### 2. Capability Denial Regression Tests
- **Target**: Privilege checks in VFS, networking, process management.
- **Execution**: Verify that unprivileged processes receive `EPERM` when requesting privileged operations.

### 3. Fuzzing Target Matrix
- **Target**: Syscall argument parsing, ELF binary loader, packet deserialization.
- **Execution**: `cargo fuzz` smoke tests in CI.
