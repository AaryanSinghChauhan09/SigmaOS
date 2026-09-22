# SigmaOS cgroups v2 Memory Controller: Gap Analysis & Actionable Roadmap

## Executive Summary

The cgroups memory controller in **SigmaOS** bridges Linux cgroups v2 resource accounting (`src/memory/cgroups.rs`, `src/resource/cgroup.rs`) with FreeBSD RACCT/RCTL resource limits and OpenBSD login class boundaries (`login.conf`). This document presents an exhaustive gap analysis comparing SigmaOS cgroups memory control against enterprise Linux cgroups v2 (`mm/memcontrol.c`) and FreeBSD RCTL (`sys/kern/kern_rctl.c`) paradigms, followed by a 3-phase strategic development roadmap.

---

## 1. Existing Memory cgroup Architecture in SigmaOS

SigmaOS currently implements hierarchical memory accounting and basic OOM policies in `src/memory/cgroups.rs` (`MemCgroupManager`):

| Component | Implementation File | Capabilities Provided |
| :--- | :--- | :--- |
| **Hierarchical Memory Cgroups** | `src/memory/cgroups.rs` | Tree structure (`MemCgroup`) with `parent_id` tracking, unique ID allocation, and root `/` cgroup initialization. |
| **Recursive Memory Charging** | `src/memory/cgroups.rs` | `charge_memory` traversing up the cgroup hierarchy to ensure parent and child limits (`cgroup.limit`) are respected, with transaction rollback on charge failure. |
| **Memory Uncharging** | `src/memory/cgroups.rs` | `uncharge_memory` releasing memory allocations up the cgroup parent chain. |
| **OOM Policy Selector** | `src/memory/cgroups.rs` | `trigger_oom_killer_with_policy` supporting `KillHeuristicProcess` (largest footprint), `KillYoungest` (highest PID), and `PanicSystem`. |

---

## 2. Exhaustive Gap Analysis vs. Linux cgroups v2, FreeBSD RCTL & OpenBSD

While basic hard limits (`limit`) and OOM triggers exist in SigmaOS, several critical operational gaps remain when benchmarked against Linux cgroups v2, FreeBSD RCTL, and OpenBSD resource controls:

```
                  ┌──────────────────────────────────────────────────────────┐
                  │      SigmaOS cgroups v2 Memory Controller Subsystem      │
                  └────────────────────────────┬─────────────────────────────┘
                                               │
      ┌────────────────────────────────────────┼────────────────────────────────────────┐
      ▼                                        ▼                                        ▼
┌───────────────────────────┐    ┌───────────────────────────┐    ┌───────────────────────────┐
│ Multi-Tier Watermarks     │    │  Pressure Stall (PSI)     │    │ FreeBSD RCTL Action Triggers│
│ GAP: Lacks memory.min,    │    │  GAP: Lacks memory.pressure│   │ GAP: Lacks signal/deny/   │
│ memory.low & memory.high  │    │  stall duration tracking  │    │ devctl action policies    │
└───────────────────────────┘    └───────────────────────────┘    └───────────────────────────┘
      │                                        │                                        │
      ▼                                        ▼                                        ▼
┌───────────────────────────┐    ┌───────────────────────────┐    ┌───────────────────────────┐
│ Memory Swap Limits        │    │ OpenBSD Login Class Limits│    │  Kernel Memory (kmem)     │
│ GAP: Lacks memory.swap.max│    │ GAP: Lacks datasize-max   │    │  Accounting               │
│ and memory.swap.high      │    │ and stacksize-max controls│    │ GAP: Page table & slab    │
│ controls                  │    │                           │    │ charge tracking missing   │
└───────────────────────────┘    └───────────────────────────┘    └───────────────────────────┘
```

### 2.1. Linux cgroups v2 Multi-Tier Memory Limits (`min`, `low`, `high`, `max`)
* **Linux Baseline**: Linux cgroups v2 replaces flat hard limits with a 4-tier protection model:
  - `memory.min`: Hard protection limit; memory below this threshold is never reclaimed by `kswapd`.
  - `memory.low`: Soft protection limit; memory is reclaimed only if no unprotected cgroups are available.
  - `memory.high`: Throttle limit; processes crossing `high` are slowed down and forced into direct memory reclamation.
  - `memory.max`: Hard ceiling; crossing `max` triggers direct reclaim and OOM execution.
* **SigmaOS Gap**: `MemCgroup` in `src/memory/cgroups.rs` features only a single flat `limit` field, without tiered protection (`min`/`low`) or throttling (`high`).

### 2.2. Pressure Stall Information (PSI - `memory.pressure`)
* **Linux Baseline**: Linux cgroups v2 tracks memory pressure stalls via `memory.pressure` (some/full stall percentages over 10s, 60s, 300s windows), allowing userland container supervisors (`systemd`, `containerd`) to detect memory starvation before OOM kills occur.
* **SigmaOS Gap**: SigmaOS does not track task stall durations spent waiting for free page frames or swap completion.

### 2.3. Swap Memory Limits (`memory.swap.max`, `memory.swap.high`)
* **Linux Baseline**: Linux cgroups v2 isolates swap space per container using `memory.swap.max` and `memory.swap.high`, preventing a single memory-hogging process from filling the entire swap partition.
* **SigmaOS Gap**: Memory charging in `src/memory/cgroups.rs` tracks physical RAM allocations without separate swap tracking or combined `memory.zswap.max` caps.

### 2.4. FreeBSD RCTL Action Policies (`rctl.conf`)
* **FreeBSD Baseline**: FreeBSD RACCT/RCTL allows fine-grained rules combining subjects (user, process, jail), resources (`memoryuse`, `vmemoryuse`, `swapuse`), and actions (`deny`, `log`, `devctl`, `sigterm`, `sigkill`).
* **SigmaOS Gap**: OOM handling in `src/memory/cgroups.rs` immediately terminates tasks via `trigger_oom_killer` without support for non-destructive actions (`log`, `sigterm` warnings, `deny` allocations).

### 2.5. OpenBSD `login.conf` Process Limits (`datasize-max`, `stacksize-max`)
* **FreeBSD / OpenBSD Baseline**: OpenBSD enforces per-login class resource boundaries (`datasize-max`, `stacksize-max`, `memoryuse-max`) during process `execve`, ensuring process heap and stack growth cannot exceed user class limits.
* **SigmaOS Gap**: Memory cgroups operate at the container level without binding per-user login classes (`/etc/login.conf`) to process stack and heap allocations.

### 2.6. Kernel Memory (`kmem`) & Page Table Accounting
* **Linux Baseline**: Linux cgroups v2 accounts for kernel-space overheads (page tables, slab cache objects, socket buffers) directly against the process's `memory.current` total (`kmem` accounting).
* **SigmaOS Gap**: `charge_memory` accounts for userland physical page allocations, but ignores kernel object allocations (`BsdZoneAllocator` in `src/memory/zone.rs`).

---

## 3. Actionable Strategic Development Roadmap

To bridge these gaps, the following 3-phase strategic development roadmap will be executed:

### Phase 1: Multi-Tier Threshold Controls & Swap Limits (Months 1–3)
1. **4-Tier Limits Model (`memory.min`, `low`, `high`, `max`)**:
   - Update `MemCgroup` in `src/memory/cgroups.rs` to include `memory_min`, `memory_low`, `memory_high`, and `memory_max`.
   - Implement throttling on `memory_high` breaches and soft protection on `memory_low` during `kswapd` page reclamation.
2. **Dedicated Swap Limit Accounting (`memory.swap.max`)**:
   - Add `swap_usage` and `swap_max` fields to `MemCgroup` to constrain ZRAM and disk swap consumption per cgroup.

### Phase 2: Pressure Stall Information (PSI) & Kernel Memory Accounting (Months 3–6)
1. **Memory PSI Metrics Engine (`memory.pressure`)**:
   - Track microsecond stall times spent by processes waiting for memory frame allocation.
   - Calculate `some` and `full` 10s/60s/300s pressure moving averages.
2. **Kernel Memory (`kmem`) Slab Charging**:
   - Hook `BsdZoneAllocator` (`src/memory/zone.rs`) into `charge_memory` to account for kernel slab objects.

### Phase 3: FreeBSD RCTL Actions & OpenBSD Login Class Limits (Months 6–12)
1. **FreeBSD-Style RCTL Action Rule Engine**:
   - Add flexible action triggers (`RctlAction::Deny`, `RctlAction::Log`, `RctlAction::SigTerm`, `RctlAction::SigKill`).
2. **OpenBSD `login.conf` Resource Boundary Binding**:
   - Enforce process heap (`datasize`) and stack limits during process spawn based on `/etc/login.conf`.

---

## 4. Verification and Benchmark Plan

| Verification Task | Test Target | Success Criterion |
| :--- | :--- | :--- |
| **4-Tier Limit Enforcement** | `test_cgroup_4tier_limits` | Throttles process at `memory.high` and triggers OOM at `memory.max` |
| **Swap Limit Isolation** | `test_cgroup_swap_max` | Fails swap allocation when cgroup `swap_usage` reaches `swap_max` |
| **PSI Memory Pressure** | `test_memory_psi_calculation` | Accurately records stall duration percentages under high allocation contention |
| **RCTL Action Trigger** | `test_rctl_deny_action` | Rejects memory allocation cleanly when rule action is set to `Deny` |
