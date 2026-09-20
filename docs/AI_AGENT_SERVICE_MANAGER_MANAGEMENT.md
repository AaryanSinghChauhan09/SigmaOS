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

---

## 4. Systemd Services Architecture, Socket Activation & Hardening

SigmaOS implements comprehensive systemd service compatibility and multi-init bridging (`SystemdEngine` in `src/init/systemd_init.rs`):

### 4.1 Systemd Unit Types & Socket Activation
* **Unit Types**: Supports `.service`, `.target`, `.socket`, `.timer`, `.path`, `.mount`, `.device`, `.slice`, `.scope`, `.swap` unit types.
* **Socket Activation**: `SystemdSocketActivationManager` binds sockets (`ListenStream`, `ListenDatagram`, Unix sockets) and triggers target service startup upon incoming connections.
* **Transient Services**: `generate_transient_service()` spawns dynamic, ephemeral services (`systemd-run`).

### 4.2 Cgroup v2 Slice Resource Governance
* **Slice Governor**: `SystemdCgroupSliceGovernor` enforces CPU weights and memory limits across system slices (`system.slice`, `user.slice`, `app.slice`).

### 4.3 Security Sandbox & Hardening Analysis
* **Security Auditor**: `SystemdSecurityAuditor` and `audit_systemd_service_security()` evaluate service hardening profiles (`NoNewPrivileges`, `ProtectSystem`, `ProtectHome`, `PrivateTmp`, `MemoryDenyWriteExecute`, OpenBSD `Pledge`/`Unveil`) and assign a security exposure rating (`OK`, `EXPOSED`, `UNSAFE`).
* **BSD Parallel Stage Solver**: `BsdRcParallelStageSolver` resolves dependency order for parallel rc.d stage execution.
