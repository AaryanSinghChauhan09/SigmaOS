# Sovereign Security Sandbox

The Sovereign Sandbox is the core of SigmaOS's defense-in-depth architecture. It completely replaces discretionary access control (like standard Linux file permissions) with a strict Capability-Based Access Control (CBAC) model.

## Capability-Based Access Control
Every process in SigmaOS is assigned a Profile upon creation. A Profile contains a bitmask of capabilities:
- `CAP_FS_READ`, `CAP_FS_WRITE`
- `CAP_NET_BIND`, `CAP_NET_BROADCAST`
- `CAP_IPC_SEND`, `CAP_IPC_RECV`
- `CAP_HARDWARE_IO`, `CAP_SYS_ADMIN`

## Enforcement
By default, the Sandbox drops all privileges when a process calls `execve`. System calls into the VFS, Network Stack, and Device Manager route through `sandbox_check_capability()` before execution. If a process attempts an unauthorized action, it is blocked, audited, and potentially terminated.
