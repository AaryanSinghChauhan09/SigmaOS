# SigmaOS Sovereign cgroups v2 — Resource Accounting

## Overview

SigmaOS implements a **complete Linux cgroups v2 unified hierarchy** in 100% safe Rust with zero external dependencies. This module (`src/kernel/cgroups_v2_sovereign.rs`) mirrors the Linux kernel's cgroup v2 interface introduced in Linux 4.5 and provides:

- **CPU controller** — weight (1–10,000), bandwidth quota, throttle accounting
- **Memory controller** — hard limit, soft/high limit, OOM kill tracking
- **PID controller** — fork limits, rejection accounting
- **Freeze/Thaw** — equivalent to `cgroup.freeze`
- **Hierarchical cgroup tree** — parent/child relationships mirroring `/sys/fs/cgroup/`

## Why cgroups v2?

| Feature | cgroups v1 | cgroups v2 (SigmaOS) |
|---------|-----------|----------------------|
| Hierarchy | Multiple separate hierarchies | Single unified hierarchy |
| CPU control | `cpu` + `cpuacct` separate | Unified `cpu` controller |
| Memory | `memory` cgroup | Unified with `memory.high` soft throttle |
| Writeback control | Not supported | Supported via `io` controller |
| Delegation safety | Weak | Strong — threads vs processes separated |

## Architecture

```
/ (root cgroup)
├── system.slice/
│   ├── init.scope/
│   └── sshd.service/
├── user.slice/
│   └── user-1000.slice/
└── machine.slice/
    └── qemu-1.scope/
```

## Key Types

### `SovereignCgroupsV2Manager`

Main manager. Creates and tracks cgroup nodes in the unified hierarchy.

```rust
let mut mgr = SovereignCgroupsV2Manager::new();
mgr.create_cgroup("system", "/", 1024);
mgr.create_cgroup("web", "/system", 64);
mgr.try_fork("/system/web");          // enforces pids.max
mgr.try_alloc_memory("/system/web", 512 * 1024); // enforces memory.max
```

### `CpuAccounting`

Mirrors Linux `cpu.stat` / `cpu.weight` / `cpu.max`:

```rust
let mut cpu = CpuAccounting::new();
cpu.weight = 500;           // cpu.weight (1-10000)
cpu.quota_us = 50_000;      // cpu.max quota (50ms per period)
cpu.period_us = 100_000;    // cpu.max period (100ms)
cpu.record_user(70_000);    // Exceeds quota → nr_throttled++
println!("{:.1}%", cpu.throttle_ratio() * 100.0);
```

### `MemoryAccounting`

Mirrors Linux `memory.max` / `memory.high` / `memory.stat`:

```rust
let mut mem = MemoryAccounting::new();
mem.limit_bytes = 256 * 1024 * 1024; // 256MB hard limit
mem.high_bytes  = 200 * 1024 * 1024; // 200MB soft limit
mem.try_alloc(100 * 1024 * 1024);    // OK
mem.try_alloc(200 * 1024 * 1024);    // Exceeds limit → false
```

### `PidAccounting`

Mirrors Linux `pids.max` / `pids.current`:

```rust
let mut pids = PidAccounting::new(16);
pids.try_fork();   // true
pids.task_exit();  // decrements current
```

## Linux Parity

| Linux Interface | SigmaOS Equivalent |
|----------------|-------------------|
| `cpu.weight` | `CpuAccounting::weight` |
| `cpu.max` | `CpuAccounting::quota_us / period_us` |
| `cpu.stat` | `CpuAccounting::usage_usec / throttled_usec` |
| `memory.max` | `MemoryAccounting::limit_bytes` |
| `memory.high` | `MemoryAccounting::high_bytes` |
| `memory.events` | `MemoryAccounting::high_events / max_events / oom_kills` |
| `pids.max` | `PidAccounting::max_pids` |
| `pids.current` | `PidAccounting::current_pids` |
| `cgroup.freeze` | `CgroupNode::freeze() / thaw()` |

## Tests

6 unit tests, all passing:

- `test_cgroups_v2_hierarchy_creation` — multi-level cgroup tree
- `test_cpu_accounting_throttle` — quota enforcement and throttle ratio
- `test_memory_limit_enforcement` — hard limit blocks allocations
- `test_pid_accounting_fork_limit` — fork beyond pids.max rejected
- `test_cgroup_freeze_thaw` — frozen cgroups block new processes
- `test_cpu_weight_validation` — invalid weights (0, >10000) rejected

## Source

[`src/kernel/cgroups_v2_sovereign.rs`](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/src/kernel/cgroups_v2_sovereign.rs)
