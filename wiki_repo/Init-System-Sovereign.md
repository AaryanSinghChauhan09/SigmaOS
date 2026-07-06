# Init System — Sovereign Init

> SigmaOS v15.0 "Zenith" — Init System Reference

## Overview

SigmaOS uses the **Sovereign Init** system as PID 1 — a lightweight, parallel service manager written in Rust. It replaces systemd with a dependency-aware, journal-integrated init that boots the system in under 3 seconds on reference hardware.

---

## Architecture

```
sigma_init.rs (PID 1)
├── sigma_service.rs   — Service definition parser
├── sigma_journal.rs   — Structured log aggregator
└── Service Supervisor
    ├── Dependency Graph  (topological sort for parallel launch)
    ├── Process Table      (track PIDs, restart policies)
    └── Signal Handler     (SIGCHLD → reap, SIGTERM → shutdown)
```

---

## Boot Phases

| Phase | Actions |
|---|---|
| 0 — Kernel | `sigma_irq`, `sigma_vmm`, `sigma_vfs` initialization |
| 1 — Early | Mount `/`, `/tmp`, `/proc`, `/sys` |
| 2 — Basic | Start `sigma-journal`, `sigma-udev` |
| 3 — Network | Start `sigma-networkd`, `sigma-resolved` |
| 4 — Services | Start user-enabled services in parallel |
| 5 — Login | Start `sigma-login` (TTY) or `zenith-dm` (GUI) |

---

## Service File Format

Services live in `/etc/sigma/services/` with `.service` extension.

```ini
[Unit]
Name        = nginx
Description = Nginx HTTP Server
Requires    = network.service
After       = network.service

[Service]
Type           = simple
ExecStart      = /usr/bin/nginx -g "daemon off;"
ExecStop       = /usr/bin/nginx -s stop
RestartOnFail  = true
RestartDelay   = 5
WatchdogSec    = 30
User           = www-data
Group          = www-data
MemoryMax      = 512M
CPUWeight      = 50

[Install]
WantedBy = multi-user.target
```

### Service Types

| Type | Behavior |
|---|---|
| `simple` | Supervisor tracks ExecStart PID |
| `forking` | Supervisor tracks secondary PID (from PidFile) |
| `oneshot` | Runs once, no persistent process |
| `notify` | Process sends ready notification via socket |

---

## CLI Reference

```bash
# Start a service
sigma-init start nginx

# Stop a service
sigma-init stop nginx

# Restart a service
sigma-init restart nginx

# Enable on boot
sigma-init enable nginx

# Disable on boot
sigma-init disable nginx

# Check status
sigma-init status nginx

# List all services
sigma-init list

# Show boot timing
sigma-init analyze-boot
```

---

## Journal

The `sigma_journal` subsystem collects structured log entries from all services:

```bash
# View all logs
sigma-journal

# Filter by service
sigma-journal --service nginx

# Filter by log level
sigma-journal --level error

# Follow live
sigma-journal -f

# Export as JSON
sigma-journal --format json > logs.json
```

Log entry format:
```json
{
  "timestamp": 1720271234567,
  "service": "nginx",
  "level": 3,
  "message": "worker process 1234 exited with status 0"
}
```

---

## Timers

Timers are services with a `.timer` file that replaces cron:

```ini
# /etc/sigma/services/backup.timer
[Timer]
OnCalendar = daily
Persistent = true

[Install]
WantedBy = timers.target
```

---

## Differences from systemd

| Feature | systemd | Sovereign Init |
|---|---|---|
| Binary format | ELF + dbus | Single Rust binary |
| D-Bus dependency | Required | None |
| Cgroup control | systemd manages | Kernel-native cgroups |
| Journal format | Binary | Structured JSON |
| Boot speed | ~5s (typical) | <3s (target) |
| Memory footprint | ~20MB | ~2MB |
