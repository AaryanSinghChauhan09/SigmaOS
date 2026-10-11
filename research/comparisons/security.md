# Security Boundary Comparison Matrix

| Project | Isolation Model | Syscall Filtering | Path Confinement | Audit Logging |
|---|---|---|---|---|
| **OpenBSD** | Pledge & Unveil | `pledge(2)` promise flags | `unveil(2)` path rules | System log audit events |
| **Linux** | Seccomp + Landlock + LSM | Seccomp-BPF filters | Landlock VFS rules | Auditd framework |
| **FreeBSD** | Capsicum & Jails | Capability rights | Jail root directory | Auditpipe event stream |
| **SigmaOS** | Capability Sandboxing | Pledge promise bitmask | Unveil path table | Immutable `MacAuditEvent` log |
