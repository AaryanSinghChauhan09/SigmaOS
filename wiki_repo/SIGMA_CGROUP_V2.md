# SigmaOS cgroup v2 — Unified Resource Hierarchy

## Overview

SigmaOS implements a Linux cgroup v2-compatible unified hierarchy for process resource management. No dependency on Linux kernel interfaces — fully sovereign Rust implementation.

**Location:** `src/kernel/sigma_cgroup_v2.rs`

---

## Unified Hierarchy

Unlike cgroup v1 (multiple per-controller hierarchies), cgroup v2 uses a **single unified tree**:

```
/sys/fs/cgroup/          ← root
├── system.slice/
│   ├── sshd.service/
│   └── nginx.service/
└── user.slice/
    └── user-1000.slice/
        └── session-1.scope/
```

---

## Controllers

| Controller | Interface Files | Purpose |
|-----------|----------------|---------|
| `cpu` | `cpu.weight`, `cpu.max`, `cpu.stat` | CPU scheduling |
| `memory` | `memory.max`, `memory.current`, `memory.high` | Memory limits |
| `pids` | `pids.max`, `pids.current` | Process count |
| `freezer` | `cgroup.freeze` | Freeze/thaw all tasks |
| `io` | `io.max`, `io.stat` | Block I/O rate |

---

## API Reference

```rust
let mut cg = SigmaCgroupV2::new();

// Create hierarchy
let sys = cg.mkdir(1, "system.slice").unwrap();
let sshd = cg.mkdir(sys, "sshd.service").unwrap();

// Set CPU weight (1-10000, default 100)
cg.get_mut(sshd).unwrap().cpu.weight = 200; // 2× normal

// Set memory limit (512MB)
cg.get_mut(sshd).unwrap().mem.memory_max = 512 * 1024 * 1024;

// Attach PID 1234 to sshd.service cgroup
cg.attach_task(1234, sshd).unwrap();

// Charge memory allocation
cg.charge_memory(1234, 65536).unwrap();

// Freeze all tasks in cgroup
cg.freeze(sshd).unwrap();
```

---

## CPU Bandwidth Control

```rust
// Allow cgroup to use 50% of one CPU
node.cpu.quota_us = 50_000;   // 50ms quota
node.cpu.period_us = 100_000; // per 100ms period
```

---

## Comparison with Linux cgroup v2

| Feature | Linux cgroup v2 | SigmaOS |
|---------|----------------|---------|
| Unified hierarchy | Yes | Yes |
| CPU weight (1-10000) | Yes | Yes |
| Memory accounting | Yes | Yes |
| PID limits | Yes | Yes |
| Freezer | Yes | Yes |
| OOM killer integration | Yes | Planned |
| BPF programs | Yes | Planned |
| no_std | No | **Yes** |

---

## Resource Enforcement

The kernel scheduler calls `charge_cpu_ns()` on every context switch. The memory allocator calls `charge_memory()` on every allocation. When limits are exceeded, operations return `Err`.
