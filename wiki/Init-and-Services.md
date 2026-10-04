# Init and Services

SigmaOS implements a clean, fast, dependency-aware init system and service manager written entirely in Rust. It replaces systemd's C codebase with a safer, leaner alternative while maintaining full compatibility with existing service units, offering parallel startup, socket activation, cgroup v2 resource isolation, and AI-driven failure recovery.

---

## Architecture Overview

```
 Kernel → /sbin/sigma-init (PID 1)
               │
               ├── Stage 1: Early init (mount /proc /sys /dev)
               ├── Stage 2: udev rules + firmware load
               ├── Stage 3: Network + cryptsetup
               └── Stage 4: Service graph execution
                                │
                    ┌───────────┴───────────┐
                    │   SigmaServiced        │
                    │  (src/init/)           │
                    │                        │
                    │  Dependency resolver   │
                    │  Socket activation     │
                    │  Cgroup v2 scopes      │
                    │  Health watchdog       │
                    │  AI failure recovery   │
                    └────────────────────────┘
```

---

## Stages

### Stage 1 — Kernel Init
- Mount essential pseudo-filesystems: `/proc`, `/sys`, `/dev`, `/run`
- Set up `devtmpfs` and `mdev` (or udev)
- Load compiled-in drivers (ACPI, NVME, framebuffer)

### Stage 2 — Hardware Setup
- udev rule processing: GPU, NIC, USB, sound devices
- Firmware blobs loaded via `request_firmware`
- Input device detection and keymap loading

### Stage 3 — Storage & Crypto
- LUKS2 decrypt (TPM2 or passphrase)
- LVM activation, RAID assembly
- Btrfs / SigmaFS mount with subvolume selection
- `/tmp` as tmpfs, `/var` as overlay if needed

### Stage 4 — Service Execution
- Parallel service graph with proper `After=` / `Requires=` handling
- Target milestones: `basic.target` → `network.target` → `graphical.target`
- Socket activation: services started on first connection
- Boot time target: **< 3 seconds** to `graphical.target` on NVMe systems

---

## Service Unit Format

SigmaOS service units are TOML (systemd `.service` syntax also supported):

```toml
[Unit]
description = "SigmaOS Network Manager"
after = ["basic.target"]
requires = ["dbus.service"]

[Service]
exec_start = "/usr/bin/sigma-netd"
restart = "on-failure"
restart_sec = 2

[Cgroup]
cpu_quota_percent = 20
memory_max_mb = 256

[Watchdog]
interval_sec = 30
action = "restart"     # restart | kill | ai-recover
```

---

## Socket Activation

Services are not started until their socket receives a connection:

```
Client connects to /run/sigma/dbus.socket
          │
          ▼
sigma-init creates socket, listens
          │
          ▼ (first connection)
Spawns dbus-daemon, passes socket via fd inheritance
```

---

## Cgroup v2 Integration

Every service runs in its own cgroup v2 scope:

```
/sys/fs/cgroup/
  └── sigma.slice/
      ├── sigma-netd.service/
      │     cpu.max = "200000 1000000"   (20% of 1 core)
      │     memory.max = 268435456       (256MB)
      └── sigma-display.service/
            cpu.max = "max"
            memory.max = max
```

---

## AI-Driven Failure Recovery

When a service fails, the AI runtime (`src/ai/`) analyses:
1. Crash dump / stderr output
2. System resource state at time of crash
3. Historical failure patterns

Then takes action:
- `restart`: simple restart with backoff
- `reconfigure`: adjust service parameters and restart
- `isolate`: move service to degraded cgroup, alert user
- `rollback`: restore previous service binary from snapshot

---

## Boot Time Comparison

| OS | Boot to Login (NVMe) | Boot to Login (HDD) |
|----|---------------------|---------------------|
| Linux Mint 22 | 8.2s | 22s |
| Omarchy | 5.1s | 16s |
| Arch Linux | 4.3s | 14s |
| **SigmaOS** | **< 3s** | **< 10s** |

---

## Source Files

| File | Description |
|------|-------------|
| `src/init/` | PID 1 init, stage runner |
| `src/kernel/acpi_pm.rs` | ACPI power management during init |
| `src/kernel/cgroup_v2_controller.rs` | Cgroup v2 service scopes |
| `src/system/` | System service helpers |

---

## AI Agent Maintenance Instructions

> **For AI agents maintaining this page:**
> - Source: `src/init/`, `src/system/`
> - Update boot time table with measured values on new hardware
> - Document new service unit fields as they are added to the parser
> - Keep cgroup v2 resource limit examples accurate
