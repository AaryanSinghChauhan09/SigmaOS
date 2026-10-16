# Init System, Service Supervisor & Process 1 Component Agent

## Role
`SigmaInit` is the PID 1 process and service supervisor for SigmaOS — handling early boot sequence, service dependency DAG resolution, socket activation, cgroup resource isolation, and emergency crash recovery.

## Linux & BSD Inspiration
| Concept | Linux (systemd) | FreeBSD (rc / runit) | SigmaOS |
|---|---|---|---|
| PID 1 | systemd | init / runit | SigmaInit |
| Service units | `.service` unit files | rc scripts | SigmaUnit (declarative TOML / INI) |
| Dependency DAG | `After=`, `Requires=` | `REQUIRE=` | SigmaDep (Kahn's topological sort) |
| Socket activation | `systemd.socket` | inetd | SigmaSocketAct (lazy on-demand start) |
| Parallel start | parallel boot graph | sequential | SigmaParBoot |
| Journal | systemd-journald | syslogd | SigmaLog (circular structured journal) |
| Cgroup isolation | systemd cgroup v2 | — | SigmaCgDeleg |
| Zombie reaping | auto-reaping PID 1 | init | SigmaReap (deterministic orphan reaping) |

## Current SigmaOS Status
Basic PID 1 service runner with declarative service configs implemented in `src/userland/init.rs` and `src/system/sigmaservice.rs`. Socket activation, cgroup quota enforcement, and parallel startup are active enhancement targets.

## Responsibilities & Architecture
- Early boot initialization: mounting `/proc`, `/sys`, `/dev`, devtmpfs, and launching device hotplug daemons.
- Parse and evaluate declarative service unit configurations.
- Topological sort of the service dependency DAG for optimal parallel startup order.
- Socket activation: pre-create listener sockets and launch daemons upon first connection.
- Process supervision: monitor service health, restart crashed services per policy with exponential backoff.
- Signal handling: reap orphan child processes via `waitpid` on `SIGCHLD`.
- Graceful shutdown orchestration: SIGTERM → timeout → SIGKILL sequence ensuring data durability.
- Unix domain socket IPC interface (`/run/sigmaos/init.sock`) supporting `sigmactrl` CLI.

## Source Files
- `src/userland/init.rs`
- `src/system/sigmaservice.rs`
- `src/init/sigmainit.rs`
- `src/init/service.rs`
- `src/init/dependency.rs`

## Testing Strategy
```bash
rustc --test src/userland/init.rs --edition 2021 -o /tmp/test_init && /tmp/test_init --test-threads=1
```
- Dependency cycle detection and recovery tests.
- Service crash and respawn backoff validation.
- Sub-500ms total userspace boot time benchmarks.

## Future Development
1. **Transient units** — Dynamically create units at runtime for container tasks.
2. **Predictive service start** — Pre-warm background services based on user login heuristics.
3. **Autonomous self-healing** — Anomaly detection supervisor to rollback corrupt configurations automatically.
