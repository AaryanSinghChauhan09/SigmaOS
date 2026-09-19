# SigmaOS Sovereign BPF-LSM Security Engine

## Overview

SigmaOS implements a **pure-Rust sovereign BPF-LSM (Linux Security Module via eBPF) engine** (`src/security/bpf_lsm_sovereign.rs`), modeled after the BPF-LSM framework merged in Linux 5.7.

BPF-LSM empowers system administrators and security daemons to attach dynamic, programmable security hooks directly to critical kernel operations without recompiling the kernel or loading external C modules.

## Supported LSM Hooks

- **`BprmCheckSecurity`**: Binary execution authorization and SUID binary sandboxing before `execve`.
- **`FileOpen`**: Granular file path access control (restricting sensitive files like `/etc/shadow` or cryptographic keys).
- **`SocketConnect`**: Network endpoint filtering (blocking outbound C2 IP addresses and ports).
- **`TaskKill`**: Process-to-process signal dispatch verification.
- **`PtraceAccessCheck`**: Anti-debugging and memory introspection defense (protecting critical daemons and PID 1).

## Program Decisions

- `Allow`: Operation proceeds unconditionally.
- `Deny(errno)`: Operation rejected with exact POSIX-compatible error code (e.g. `-EACCES`, `-EPERM`, `-ECONNREFUSED`).
- `AuditOnly`: Operation recorded in telemetry logs without blocking.

## Test Verification

6 standalone unit tests verified in test runner suite `[16]`:
- `test_bpf_lsm_attach_detach`: Dynamic hook registration and removal.
- `test_bpf_lsm_file_open_denial`: File access blocked with `-EACCES`.
- `test_bpf_lsm_bprm_exec_protection`: Malicious executable execution blocked.
- `test_bpf_lsm_socket_filter`: Outbound C2 network connection denied with `-ECONNREFUSED`.
- `test_bpf_lsm_ptrace_defense`: PID 1 protected against unauthorized ptrace attachment.
- `test_bpf_lsm_audit_decision`: Non-blocking telemetry audit verification.
