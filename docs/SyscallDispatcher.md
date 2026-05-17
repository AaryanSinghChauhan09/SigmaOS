# Syscall Dispatcher & Functions Improvements

This document defines the architectural and security improvements implemented in the SigmaOS Syscall Dispatcher (S-SYSCALL).

### Dispatch Mechanics
1. **Modular Syscall Registry Database**: Dynamic registration of system call handlers mapping directly to functions.
2. **O(1) Array Lookup Fast-Path**: Using IA32_LSTAR or ARM SVC direct indices without branch penalties.
3. **Sub-nanosecond Tracing Hooks**: Low-overhead syscall tracing vectors capturing execution time, caller ID, and parameters.
4. **Fallback Mitigation**: `sys_ni_syscall` intercepts all undefined parameters gracefully, returning `ENOSYS`.

### Security Boundary Enforcement
5. **Sandboxing Enforcement**: Strict namespace and permission bounds checks at user/kernel transitions.
6. **Parameter Sanitization**: All Ring-3 pointer arguments are verified against the task's valid VMA ranges.
7. **Capability-Based Routing**: Syscalls reject invalid operations inherently if the executing process lacks capabilities.
8. **Register Scrubbing**: Information-leak prevention by scrubbing non-return registers on transition back to Ring-3.

### Core Handlers
9. **sys_write / sys_read**: POSIX-compliant file stream interactions.
10. **sys_socket**: BSD-compatible network stack socket creation APIs.
11. **sys_pkg_install**: Sovereign App Store integration at the syscall level.
