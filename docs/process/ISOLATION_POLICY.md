# SigmaOS Process Isolation Policy (`docs/process/ISOLATION_POLICY.md`)

## 1. Scope & Objective

This document defines process boundary isolation across PID, Mount, Network, UTS namespaces, user credentials, and capability limits in SigmaOS.

## 2. Namespace Isolation Boundaries

SigmaOS process isolation incorporates modular namespace contexts:

1. **PID Namespace**: Isolates the process ID space. Processes inside a child PID namespace have local PIDs (e.g. starting at `PID 1`), while mapped to distinct global PIDs in the parent namespace.
2. **Mount Namespace**: Provides isolated virtual filesystem mount points and root directory contexts (`chroot`/`pivot_root`).
3. **Network Namespace**: Isolates network interfaces, IP addresses, routing tables, and socket binding domains.
4. **UTS Namespace**: Isolates hostname and NIS domain configuration.
5. **User Namespace**: Maps local user and group IDs (`UID`/`GID`), enabling unprivileged user ID 0 mapping inside container domains.

```text
Global Namespace
 ├── Global PID 100
 └── Child PID Namespace Container
      ├── Local PID 1 (Mapped to Global PID 100)
      ├── Isolated Mount Tree
      └── Isolated Net Stack
```

## 3. Privilege & Capability Boundaries

- Cross-namespace or privileged operations require specific kernel capability tokens (e.g., `CAP_SYS_ADMIN`, `CAP_NET_ADMIN`, `CAP_SYS_PTRACE`).
- Signals cannot cross user/namespace boundaries unless sent by root (`CAP_KILL`) or matching credentials.

## 4. Verification

Namespace isolation boundaries and cross-namespace checks are tested in `tests/process/namespaces/namespace_test.rs`.
