# Comparative Analysis: Security & Isolation Models

| Operating System | Mechanism | Enforced Boundary | Failure Mode | SigmaOS Decision |
|---|---|---|---|---|
| OpenBSD | pledge / unveil | Syscalls & File Paths | SIGABRT Process Termination | Adopted dual pledge/unveil |
| FreeBSD | Jails | Host VNET & Chroot | Permission Denied Error | Adopted Jail isolation |
| Linux | Landlock / Seccomp | BPF Syscall Filters | EPERM / SIGSYS | Adopted Landlock compat |
| SigmaOS | Unified MAC Sandbox | Pledge Masks + Unveil Trees | Process Termination + Audit Log | Native Implementation |

## Tradeoff Analysis
Combining OpenBSD pledge/unveil with FreeBSD Jail semantics creates a least-privilege security model with zero capability leakage.
