# Init System Agent

## Role
`SigmaInit` is the PID 1 process and service supervisor for SigmaOS — handling system boot sequence, service dependency ordering, socket activation, and system shutdown/reboot.

## Status
🟡 In Progress — Basic init in `src/userland/init.rs`; dependency ordering, socket activation, and cgroup delegation needed.

## Source File
`src/userland/init.rs`, `src/system/sigmaservice.rs`

## Linux/BSD Inspiration
| Concept | Linux (systemd) | FreeBSD (rc) | SigmaOS |
|---|---|---|---|
| PID 1 | systemd | init | SigmaInit |
| Service units | `.service` files | rc scripts | SigmaUnit |
| Dependency DAG | `After=`, `Requires=` | `REQUIRE=` | SigmaDep |
| Socket activation | `systemd.socket` | inetd | SigmaSocketAct |
| Parallel start | systemd parallel | sequential | SigmaParBoot |
| Journal | systemd-journald | syslogd | SigmaLog |
| Cgroup delegation | systemd cgroup v2 | — | SigmaCgDeleg |
| Reaping | systemd auto-reap | init | SigmaReap |

## Responsibilities
- System startup: mount filesystems, udev, network, user services
- Parse and execute SigmaUnit service files
- Topological sort of dependency DAG for correct start order
- Parallel service startup (independent services start concurrently)
- Socket activation: pre-create sockets, start service on first connection
- Process supervision: restart crashed services per RestartPolicy
- `SIGCHLD` reaping of all orphan processes
- Graceful shutdown: send SIGTERM → wait → SIGKILL
- Target/runlevel equivalent (`multi-user.target`, `graphical.target`)
- `/run/sigmaos/` socket for `sigmactrl` CLI

## Implementation Notes
- `#![no_std]` — unit table as `static mut [UnitDescriptor; MAX_UNITS]`
- State machine: `Inactive → Activating → Active → Deactivating → Failed`
- Dependency graph: Kahn's algorithm for topological sort
- `static mut INIT: Option<SigmaInit>` singleton

## Testing
```bash
rustc --test src/userland/init.rs --edition 2021 -o /tmp/test_init && /tmp/test_init --test-threads=1
```

## Future Development
1. **Transient units** — Dynamically create units at runtime
2. **User systemd instance** — Per-user service manager
3. **Boot time optimization** — systemd-analyze equivalent
4. **Service watchdog** — `sd_notify(WATCHDOG=1)` compatible
5. **Predictive service start** — Start services before they're needed
6. **Hibernate/resume** — Coordinated service stop/start around hibernate
