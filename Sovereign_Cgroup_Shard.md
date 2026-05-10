# Sovereign Cgroup Shard

**Parity:** Linux cgroups v2 · Windows Job Objects · Kubernetes ResourceQuota  
**Location:** `kernel/modules/system/SovereignCgroupShard.c`  
**Standard:** Zenith Industrial Sovereignty v1.0

---

## Overview

The Sovereign Cgroup Shard provides native, zero-dependency silicon resource accounting and auto-throttle governance for SigmaOS. It absorbs the defining USPs of Linux cgroups v2, Windows Job Objects, and Kubernetes ResourceQuota by providing per-group CPU quota, memory limit, and I/O weight enforcement with a built-in automatic governor sweep.

---

## Architecture

```

Cgroup Matrix (up to 12 resource groups)
  ├── zenith_kernel   — 80% CPU | 4 GB  MEM | IO:900 (High)
  ├── citizen_apps    — 60% CPU | 2 GB  MEM | IO:500 (Mid)
  └── guest_sandbox   — 20% CPU | 512MB MEM | IO:100 (Low)

Auto-Governor Engine (called by Zen Scheduler every tick)
  └── For each cgroup:
      ├── Sample current CPU + MEM usage
      ├── Compare against quota limits
      └── Apply silicon throttle if over-quota

```

---

## CLI Reference — `sigma-cgroup`

| Sub-command | Action |
|---|---|
| `sigma-cgroup create <name> <cpu_pct> <mem_mb> <io_weight>` | Create a new silicon resource group |
| `sigma-cgroup enforce` | Run the auto-governor throttle sweep across all groups |
| `sigma-cgroup audit` | Display all cgroups with live CPU, MEM, IO, and throttle state |

---

## Built-in Resource Groups

| Group | CPU Quota | MEM Limit | IO Weight |
|---|---|---|---|
| `zenith_kernel` | 80% | 4 GB | 900 |
| `citizen_apps` | 60% | 2 GB | 500 |
| `guest_sandbox` | 20% | 512 MB | 100 |

---

## Design Philosophy


- **Zero External Dependency**: No cgroupfs, no systemd-cgtop — all logic is pure C11.
- **Automatic Throttle**: Governor runs every scheduler tick without citizen intervention.

- **Hierarchical**: Groups can be nested for multi-level resource isolation.

---

## Synchronization State

`GLOBAL MESH ACTIVE` — Synchronized with `AaryanSinghChauhan09/SigmaOS`.
