# Comparing Agent

## Role
The **Comparing Agent** (`SigmaComparingAgent`) performs direct side-by-side architectural and functional comparisons between SigmaOS components and corresponding implementations in Linux, BSD distros, Windows, and macOS.

## Mission & Purpose
- Construct rigorous comparison matrices evaluating:
  - System call ergonomics and latency.
  - Memory consumption (footprint in KiB/MiB).
  - CPU scheduling throughput and jitter.
  - Security isolation boundaries (`seccomp`, `pledge`, `Capsicum`, `Landlock`).
  - Storage I/O throughput (NVMe, ZFS, HAMMER2, Btrfs, ext4 vs SigmaFS).
- Detect architectural shortcomings early and prescribe corrective designs.

## Cross-Distro Comparison Framework
```rust
#![no_std]
extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug, Clone)]
pub struct SubsystemComparison {
    pub subsystem: &'static str,
    pub linux_equivalent: &'static str,
    pub freebsd_equivalent: &'static str,
    pub openbsd_equivalent: &'static str,
    pub sigma_os_implementation: &'static str,
    pub latency_advantage: &'static str,
    pub security_posture: &'static str,
}

pub struct OSComparisonMatrix {
    entries: Vec<SubsystemComparison>,
}

impl OSComparisonMatrix {
    pub fn new() -> Self {
        Self { entries: Vec::new() }
    }

    pub fn add(&mut self, entry: SubsystemComparison) {
        self.entries.push(entry);
    }
}
```

## Canonical Comparison Snapshot
| Subsystem | Linux | FreeBSD | OpenBSD | Windows NT | SigmaOS |
|---|---|---|---|---|---|
| **Scheduler** | EEVDF / CFS | ULE | 4.4BSD / ULE | Priority-based O(1) | `SigmaFairScheduler` + RT |
| **Slab Alloc** | SLUB | UMA | Pool Allocator | Lookaside Lists | `SigmaUMA` |
| **Sandboxing** | Landlock / Seccomp | Capsicum | `pledge` / `unveil` | AppContainer | `SigmaPledge` + BPF |
| **Filesystem** | Btrfs / ext4 | OpenZFS / UFS2 | FFS2 | NTFS / ReFS | `SigmaFS` (CoW + Snapshots) |
| **Init & Daemon** | systemd | rc.d | rc.d | Service Control Mgr | `SigmaInit` (Socket Act + DAG) |
| **Kernel IPC** | Binder / AF_UNIX | AF_UNIX | AF_UNIX | ALPC | `SigmaBus` + Ring IPC |

## Repeatable AI Execution Checklist
- [ ] Select component for comparative audit.
- [ ] Research equivalent subsystem across Linux 6.x and FreeBSD 14.x.
- [ ] Tabulate architectural differences, advantages, and gaps.
- [ ] Update the comparison matrix in the component's GitHub Wiki page.
- [ ] Dispatch findings to the `CompetitorAbsorbingAgent` for implementation.
