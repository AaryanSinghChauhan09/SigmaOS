# AI Agent Service Manager & Init Management Guidelines

## 1. Overview & Architecture
This document details AI agent procedures for maintaining system initialization, systemd unit compatibility, Void Linux runit service supervision, daemon lifecycle management (`sigmctl`/`siginit`), and structured logging (`journald`) in SigmaOS (`src/init/systemd_init.rs`, `src/distro/void_runit.rs`).

---

## 2. Operational Directives for AI Agents

### 2.1 Service Supervision & Lifecycle
- **Dependency Ordering**: Service units must be started according to dependency graphs (`Wants`, `Requires`, `After`, `Before`) and state transition loops.
- **Automatic Daemon Restart**: Failed services configured with `Restart=always` or `Restart=on-failure` must be monitored and restarted with exponential backoff limits.

### 2.2 Structured System Logging
- **Binary & Ring-Buffer Logging**: Logging streams must write structured log events (timestamp, priority, unit name, message) to in-memory ring buffers and disk journals.
- **Service Control CLI**: The `sigmctl` control CLI must provide standard start, stop, restart, status, and log tailing commands.

---

## 3. Related Files
- `src/init/systemd_init.rs`
- `src/distro/void_runit.rs`
- `docs/LINUX_DISTRO_PARITY_CHECKLIST.md`
