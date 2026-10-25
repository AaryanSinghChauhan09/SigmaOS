# SigmaOS Syscall ABI & Validation Contract

## Overview

The Syscall ABI Contract defines formal requirements for all syscall entry points, argument validation, capability checks, pointer safety, error propagation, and userland compatibility.

## Rules & Principles

1. **Syscall Registration & Numbering**:
   - Syscall numbers are fixed and immutable.
   - Unimplemented or deprecated syscalls return `-ENOSYS`.

2. **Pointer Copy-In / Copy-Out Safety**:
   - All userland pointers passed to syscalls must be validated for range, alignment, and W^X boundaries before access.
   - Faults during copy-in/copy-out must return `-EFAULT` without corrupting kernel data structures.

3. **Argument Validation**:
   - Buffer lengths, flags, file descriptors, and enum values must be strictly validated.
   - Negative or oversized lengths return `-EINVAL` or `-EBADF`.

4. **Capability & Privilege Checks**:
   - Operations altering process isolation, hardware devices, or network interfaces require active capability tokens (`CAP_SYS_ADMIN`, `CAP_NET_ADMIN`).
   - Insufficient privileges return `-EPERM` or `-EACCES` and log a security denial event.

5. **Error Code Consistency**:
   - POSIX-compliant typed error codes (`EPERM`, `ENOENT`, `EIO`, `E2BIG`, `EBADF`, `EAGAIN`, `ENOMEM`, `EFAULT`, `EINVAL`).
   - No silent failures or undocumented panics in syscall dispatch.

6. **Signal Interaction & Restartability**:
   - Syscalls interrupted by signals return `-EINTR` or auto-restart if `SA_RESTART` is set on the signal handler.

7. **Compatibility Policy**:
   - Breaking ABI changes require a formal ADR (`docs/adr/`) and migration policy.
