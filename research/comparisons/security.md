# Comparative Analysis: Security & Sandboxing

| Metric / Feature | OpenBSD | Linux | FreeBSD | SigmaOS Target |
|---|---|---|---|---|
| Syscall Restrictions | `pledge` | Seccomp-BPF | Capsicum | Pledge promises |
| File Path Restrictions | `unveil` | Landlock | Capsicum rights | Unveil path tree |
| Process Isolation | `pledge` / `unveil` | Namespaces / Cgroups | Jails | Unified MAC Sandbox |
