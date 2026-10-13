# Security and Sandboxing Specification

This wiki page defines the technical sandboxing capabilities, process capability sets, and privilege isolation mechanisms implemented in SigmaOS.

## Sandboxing Matrix

| Sandboxing Paradigm | Source Inspiration | Key Capabilities | Implementation Module |
|---|---|---|---|
| OpenBSD Pledge & Unveil | OpenBSD | Syscall promise masks (`stdio`, `rpath`, `wpath`, `cpath`, `inet`, `exec`) and path-hiding views | `src/security/pledge_unveil.rs`, `src/security/bsd_hardening.rs` |
| FreeBSD Capsicum | FreeBSD | File descriptor capabilities (`cap_rights_limit`) and capability-mode sandboxing | `src/security/capsicum.rs`, `src/security/capsicum_wrapper.rs` |
| Linux Landlock LSM | Linux 6.12+ | Unprivileged filesystem access rulesets (`LandlockRuleset`) and network port binding restrictions | `src/security/landlock.rs`, `src/security/landlock_v4.rs` |
| Seccomp BPF | Linux / Android | JIT-compiled eBPF filter programs over system call numbers and arguments | `src/security/seccomp.rs`, `src/security/seccomp_ebpf.rs` |
| macOS App Sandbox | macOS | Typed portal entitlement mediation and sandbox profile enforcement | `src/security/capability_sandbox.rs` |

## Threat Boundaries

- **User-to-Kernel Boundary:** Enforced via x86_64 Syscall/Sysret dispatcher, SMAP/SMEP, and memory permission checks.
- **Inter-Process Boundary:** Enforced via namespaces (PID, Mount, NET, UTS, IPC, User, Cgroup), Capsicum process descriptors (`pdfork`), and FreeBSD Jail network isolation.
- **File System Boundary:** Enforced via Landlock v5 rulesets, OpenBSD `unveil()`, and `fscrypt` directory encryption.
