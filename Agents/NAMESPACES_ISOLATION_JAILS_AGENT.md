# Process Namespaces & Jail Isolation Component Agent

## Role
`SigmaNamespace` implements Linux-compatible process isolation primitives and BSD-style jail confinement for SigmaOS — providing isolated views of system resources across PID, Mount, Network, UTS, IPC, and User namespaces.

## Linux & BSD Inspiration
| Concept | Linux | FreeBSD | OpenBSD | SigmaOS |
|---|---|---|---|---|
| PID Namespace | `clone(CLONE_NEWPID)` | — | — | SigmaPidNS (isolated PID 1 tree) |
| Mount Namespace | `clone(CLONE_NEWNS)` | Jail VFS | unveil | SigmaMntNS (isolated mount table) |
| Network Namespace | `clone(CLONE_NEWNET)` | VIMAGE | — | SigmaNetNS (virtual interfaces & routes) |
| UTS Namespace | `clone(CLONE_NEWUTS)` | Jail hostname | — | SigmaUtsNS (hostname/domain) |
| User Namespace | `clone(CLONE_NEWUSER)` | — | pledge | SigmaUserNS (unprivileged root mapping) |
| IPC Namespace | `clone(CLONE_NEWIPC)` | Jail SysV | — | SigmaIpcNS (isolated MQ/SHM) |
| Jail Confinement | Landlock | FreeBSD Jails | pledge / unveil | SigmaJail (capability-bound sandbox) |

## Current SigmaOS Status
Process hierarchy tracking and basic filesystem chroot isolation implemented. Network namespaces and unprivileged user namespace UID/GID mappings are active enhancement targets.

## Responsibilities & Architecture
- Creation and joining of PID, Mount, Network, UTS, IPC, and User namespaces via `clone()`, `unshare()`, and `setns()`.
- Isolated filesystem views with per-namespace mount tables and copy-on-write mounts.
- Per-container network stacks with virtual Ethernet pairs (`veth`) and private loopback devices.
- Unprivileged user namespaces enabling rootless container runtimes (Podman compatibility).
- BSD Jail and OpenBSD pledge/unveil inspired restriction masks limiting syscall exposure per sandbox.
- Namespace file descriptors under `/proc/PID/ns/` for introspection and dynamic joining.

## Source Files
- `src/namespace/`
- `src/security/jail.rs`
- `src/process/`

## Testing Strategy
```bash
rustc --test src/security/jail.rs --edition 2021 -o /tmp/test_jail && /tmp/test_jail --test-threads=1
```
- Namespace escape prevention tests.
- Mount propagation and private/shared bind-mount isolation checks.
- User namespace UID/GID map verification.

## Future Development
1. **Rootless Podman execution** — Seamless container startup without root privileges.
2. **Landlock LSM integration** — Fine-grained unprivileged filesystem access control.
3. **Cross-namespace IPC bridges** — Secure zero-copy message channels between isolated jails.
