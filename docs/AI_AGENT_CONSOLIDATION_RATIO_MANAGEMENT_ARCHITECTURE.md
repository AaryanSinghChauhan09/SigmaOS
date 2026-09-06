# AI Agent Consolidation Ratio Management Architecture

## Executive Overview

Consolidation Ratio Management in SigmaOS governs the density of virtual machines (VMs), containers, and process workloads hosted on physical hardware. Implemented across `src/virtualization/vm_manager.rs`, `src/distro/sovereign_distro_dominance.rs`, and `src/filesystem/defragmenter.rs`, SigmaOS optimizes hardware utilization using VirtIO memory ballooning (`set_memory_balloon`), Kernel Samepage Merging (KSM) deduplication, memory overcommit management, and free space defragmentation consolidation built with zero-dependency Rust primitives (`#![no_std]`).

This document serves as the architectural reference for AI coding agents inspecting, calculating, or tuning consolidation ratios in SigmaOS.

---

## Consolidation Subsystem Architecture

```
                                +-----------------------------------+
                                |    Physical Hardware Resource     |
                                |      M_physical / CPU_cores       |
                                +-----------------------------------+
                                                  |
                                                  v
                                +-----------------------------------+
                                |    Consolidation Manager          |
                                |  (src/virtualization/vm_manager)  |
                                +-----------------------------------+
                                 /                |                \
                                /                 |                 \
            +-----------------------+   +-------------------+   +-----------------------+
            | VirtIO Memory Balloon |   | KSM Deduplication |   | Disk Free Space Cons. |
            | set_memory_balloon    |   | Identical Pages   |   | defragmenter.rs       |
            +-----------------------+   +-------------------+   +-----------------------+
                                \                 |                 /
                                 \                |                /
                                  v               v               v
                                +-----------------------------------+
                                |  Target Consolidation Ratio       |
                                | C_ratio = (Sum M_alloc) / M_phys  |
                                +-----------------------------------+
```

### Core Consolidation Components

1. **VirtIO Memory Ballooning (`src/virtualization/vm_manager.rs`)**:
   - `set_memory_balloon(vm_id, target_mb)`: Dynamically inflates or deflates VM memory footprints via VirtIO balloon drivers. Reclaimed guest pages are returned to the host memory pool to increase VM density per physical host.

2. **Memory Overcommit Formula**:
   - Consolidation Ratio ($C_{ratio}$):

$$C_{ratio} = \frac{\sum_{i=1}^{N} M_{allocated}(i)}{M_{physical}}$$

   - Safe overcommit threshold: $1.5 \le C_{ratio} \le 3.0$ for general virtualized workloads.

3. **Disk Free Space Consolidation (`src/filesystem/defragmenter.rs`)**:
   - Relocates fragmented data clusters into contiguous extents to maximize continuous free storage blocks for sparse disk image allocation.

---

## Zero-Allocation Guardrails

AI agents tuning consolidation ratios must adhere to these zero-allocation constraints:
- Ballooning calculations manipulate atomic page counters in $O(1)$.
- KSM hash comparisons evaluate memory page signatures without heap buffer allocations.

---

## Related Architectural References
- `src/virtualization/vm_manager.rs` - Master VM manager and VirtIO balloon driver.
- `src/distro/sovereign_distro_dominance.rs` - Virtualization dominance metrics.
- `src/filesystem/defragmenter.rs` - Disk space consolidation.
- `docs/AI_AGENT_BALLOONING_MANAGEMENT_GUIDE.md` - VirtIO ballooning guidelines.
