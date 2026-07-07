# SigmaOS sigmad Service Manager

> Lightweight PID 1 init system with runit/OpenRC-inspired supervision trees.

## Overview

`sigmad` is the SigmaOS service manager and PID 1 process. It replaces systemd with a leaner, dependency-aware supervision tree that supports automatic restarts, health checks, and fast boot ordering.

## Service Types

| Type     | Behavior                                    |
|----------|---------------------------------------------|
| Oneshot  | Run once, mark complete                     |
| Daemon   | Long-running process, supervised            |
| Forking  | Forks child, parent exits                   |
| Notify   | Sends readiness notification to sigmad      |
| Timer    | Periodic execution on schedule              |

## Restart Policies

| Policy      | When Restarts                              |
|-------------|---------------------------------------------|
| Never       | Service stays in Failed state               |
| OnFailure   | Only on non-zero exit                       |
| Always      | Restarts on any exit (normal or error)      |
| OnAbnormal  | Restarts on signal/crash, not clean exit    |

## Default Boot Services

```
sigma-logd ──────────────────┐
                             ├──→ sigma-ai-agent
sigma-dbus ──┬───────────────┘
             ├──→ sigma-networkd
             └──→ sigma-zenith-compositor
```

| Service                | Type   | Dependencies              | Restart    |
|------------------------|--------|---------------------------|------------|
| sigma-dbus             | Daemon | —                         | Always     |
| sigma-logd             | Daemon | —                         | Always     |
| sigma-networkd         | Daemon | sigma-dbus                | OnFailure  |
| sigma-ai-agent         | Daemon | sigma-dbus, sigma-logd    | OnFailure  |
| sigma-zenith-compositor| Notify | sigma-dbus, sigma-logd    | OnAbnormal |

## Implementation

- **Source**: `kernel/init/sigma_init.rs`
- **Language**: Rust (`no_std`)
- **Key APIs**:
  - `SupervisionTree::boot_order()` — topological service ordering
  - `SupervisionTree::start_service(name)` — start with dependency check
  - `SupervisionTree::handle_failure(name)` — restart policy enforcement
  - `create_default_services()` — default SigmaOS boot configuration

## Health Checks

Services can define a `health_check_cmd` that sigmad runs periodically. If the health check fails, sigmad treats it as a service failure and applies the restart policy.

## Design Principles

1. **Dependency-first boot** — no service starts before its dependencies
2. **Fast restart** — configurable delay between restart attempts
3. **Bounded retries** — max_restarts prevents infinite restart loops
4. **PID tracking** — sigmad tracks each service's PID for clean shutdown
