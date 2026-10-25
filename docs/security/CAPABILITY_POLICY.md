# SigmaOS Capability & Privilege Policy

## Capabilities Overview

Processes in SigmaOS carry bitwise capability sets:

- `CAP_SYS_ADMIN`: Administrative system actions, module loading, reboot.
- `CAP_NET_ADMIN`: Network interface configuration, firewall rules.
- `CAP_CHROOT`: Changing filesystem root.
- `CAP_KILL`: Sending signals to processes owned by other user IDs.
- `CAP_PTRACE`: Tracing or inspecting process memory.

## Policy Rules

1. Every privileged syscall checks `has_capability(cap)` before executing.
2. Ineligible privilege attempts return `-EPERM` and trigger an auditable security log event.
3. Capabilities are dropped upon process `execve()` unless explicitly retained via capability bounding sets.
