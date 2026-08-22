# Resource Management & Cgroups Subsystem in SigmaOS

## Overview

SigmaOS implements a clean-room hierarchical resource controller architecture inspired by Linux **cgroups v2**, FreeBSD **rctl**, and Solaris **resource pools**.

---

## Key Modules

- [`src/resource/cgroup.rs`](file:///home/aaryansinghchauhan/SigmaOS/src/resource/cgroup.rs): Unified resource hierarchy and tree controller.
- [`src/resource/quota.rs`](file:///home/aaryansinghchauhan/SigmaOS/src/resource/quota.rs): Bandwidth, I/O, and disk usage enforcement.
- [`src/resource/rlimit.rs`](file:///home/aaryansinghchauhan/SigmaOS/src/resource/rlimit.rs): POSIX-compliant resource limits (`RLIMIT_NOFILE`, `RLIMIT_AS`, `RLIMIT_CPU`).
- [`src/kernel/sched/sigma_mlfq.rs`](file:///home/aaryansinghchauhan/SigmaOS/src/kernel/sched/sigma_mlfq.rs): MLFQ weight allocation per cgroup node.

---

## Controllers Supported

| Controller | Resource Managed | Enforcement Mechanism |
|------------|------------------|-----------------------|
| **cpu** | CPU bandwidth & share weights | MLFQ tick redistribution + quota throttling |
| **memory** | Max resident set size (RSS) & swap limit | Memory reclaim, OOM-killer hierarchy score |
| **io** | Read/write bytes/sec & IOPS | `io_uring` token bucket rate limiter |
| **pids** | Maximum concurrent process/thread count | Atomic counter on task creation |

---

## Hierarchy Example

```
/sys/fs/cgroup/
├── system.slice (Kernel daemons, SigmaPkg background tasks)
│   ├── cpu.weight = 100
│   └── memory.max = 2G
├── user.slice (Zenith Desktop session, User Apps)
│   ├── cpu.weight = 200
│   └── memory.max = 14G
└── sandbox.slice (Untrusted Web / Kuroko isolated scripts)
    ├── cpu.weight = 50
    ├── memory.max = 512M
    └── pids.max = 64
```
