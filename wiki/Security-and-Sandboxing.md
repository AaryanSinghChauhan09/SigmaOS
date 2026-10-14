# SigmaOS Security and Sandboxing Framework

## Overview

SigmaOS implements a unified, multi-layered security sandboxing architecture combining the security models of Linux, OpenBSD, FreeBSD, and macOS into a cohesive Rust security governor.

---

## Sandboxing Mechanisms Supported

### 1. OpenBSD Pledge & Unveil
- **Pledge:** Restricts system call capabilities for processes (e.g., `stdio`, `rpath`, `wpath`, `cpath`, `inet`, `dns`, `exec`).
- **Unveil:** Restricts filesystem access visibility, hiding paths outside the unveiled hierarchy with specific permission flags (`r`, `w`, `x`, `c`).

### 2. FreeBSD Capsicum & Jails
- **Capsicum:** Capability mode putting file descriptors into sandboxed capability contexts with restricted rights (`cap_rights_limit`).
- **Jails:** Lightweight virtualization with VNET virtual networking stack isolation per jail.

### 3. Linux Landlock v5 & Seccomp BPF
- **Landlock v5:** Unprivileged access control allowing applications to restrict their own file and network operations.
- **Seccomp BPF:** JIT-compiled BPF filter program auditing syscall numbers and arguments, defaulting to `SECCOMP_RET_KILL_PROCESS`.

### 4. macOS App Sandbox & SELinux
- **App Sandbox:** Entitlement-based resource delegation.
- **SELinux:** Type Enforcement (TE) and Multi-Level Security (MLS) security contexts (`user:role:type:level`).

---

## Fail-Closed Policy Enforcement

All sandboxing operations follow a **fail-closed** default policy:
- Deny by default if permission or capability is not explicitly granted.
- Audit all denial events with structured security logs.
- Memory allocation failures or invalid pointers produce immediate, safe isolation traps without compromising kernel memory.
