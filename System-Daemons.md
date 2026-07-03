# System Daemons (sigmad)

SigmaOS system daemons run as capability-restricted userspace processes.

---

## Daemon Overview

| Daemon | Language | Purpose | Status |
|--------|----------|---------|--------|
| `sigmad-health` | Go | System health monitoring | ✅ |
| `sigmad-netd` | Go | Network configuration (DHCP, DNS) | ✅ |
| `sigmad-vault` | Go | Secret management, TPM2 integration | ✅ |
| `sigmad-metrics` | Go | Prometheus-compatible metrics | ✅ |
| `sigmad-watchdog` | Go | Process supervision + restart | ✅ |
| `sigmad-updater` | Rust | A/B transactional updates | ✅ |
| `sigmad-pkg` | Nim | Package manager daemon | 🔄 |
| `sigma-agent` | Rust | AI/LLM inference daemon | 🔄 |

---

## sigma-bus Integration

All daemons communicate with the kernel via sigma-bus IPC channels:

```
Daemon           sigma-bus channel         Kernel
sigmad-netd   ←→  IPC_CH_NET_RX (0x20)  ←→ NIC driver
sigmad-health ←   IPC_CH_HOTPLUG (0x10) ←  Hotplug manager
sigmad-vault  ←   IPC_CH_SECURITY (0x80)←  pledge auditor
```

---

## sigmad-updater

The A/B transactional updater (`sigmad/updater/main.rs`):

```bash
# Check current slot status
sigma-updater status

# Apply an update
sigma-updater apply /sigma/updates/manifest.toml

# Roll back to previous slot
sigma-updater rollback
```

See [Transactional Updates](Transactional-Updates) for full details.

---

## sigmad-health

Monitors:
- CPU usage, memory pressure, disk I/O
- Process liveness (restart failed processes)
- Network connectivity
- sigma-bus channel saturation

```bash
# View health status
sigma-health status

# View metrics
sigma-health metrics --format prometheus
```

---

## sigmad-watchdog

Supervises system daemons and user services:

```toml
# /etc/sigma/services/nginx.toml
[service]
name    = "nginx"
command = "/usr/sbin/nginx"
restart = "always"
pledge  = ["stdio", "rpath", "inet"]
```

```bash
sigma-watchdog start nginx
sigma-watchdog status nginx
sigma-watchdog stop nginx
```

---

## Service Definition Format

```toml
# /etc/sigma/services/my-service.toml
[service]
name        = "my-service"
description = "My background service"
command     = "/usr/bin/my-service --config /etc/my-service.conf"
user        = "nobody"
restart     = "on-failure"   # always | on-failure | never
delay_ms    = 1000           # restart delay

[security]
pledge  = ["stdio", "rpath", "inet"]
unveil  = ["/etc/my-service.conf:r", "/tmp:rwc"]
cgroup  = "services"
```

---

## Startup Order

```
sigma_kernel_main()
  └─ process_manager_init()      # PID 0 (idle), PID 1 (init)
       └─ /sbin/sigma-init       # reads /etc/sigma/services/
            ├─ sigmad-health     # PID 2
            ├─ sigmad-netd       # PID 3 (DHCP at boot)
            ├─ sigmad-vault      # PID 4
            ├─ sigmad-watchdog   # PID 5 (supervises the rest)
            └─ ... user services
```

---

*Sources: `sigmad/`, `kernel/core/process_manager.rs`, `kernel/core/ipc/SovereignIPC.rs`*
