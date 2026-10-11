# Pull Request Proposal: Transparent Huge Page (THP) khugepaged Defrag & Runtime Promotion Daemon

**PR Title:** `feat(mm): Transparent Huge Page (THP) khugepaged Defrag & Runtime Page Promotion Daemon`
**PR Branch:** `feature/thp-defrag-khugepaged-daemon`
**Target Branch:** `main`
**Status:** Specification & Native Implementation Complete
**Component Scope:** `src/memory/huge_pages.rs`, `src/memory/mod.rs`, `docs/roadmap/`

---

## 1. Executive Summary & Problem Statement

Transparent Huge Pages (THP) allow operating system kernels to transparently promote 4KiB base physical pages into 2MiB / 1GiB superpages, reducing Translation Lookaside Buffer (TLB) misses and accelerating memory-intensive workloads. While basic 2MiB/1GiB page allocation was present in `src/memory/huge_pages.rs`, Linux `khugepaged` parity requires background asynchronous page scanning, active page promotion based on access frequency and `MADV_HUGEPAGE` hints, memory compaction for fragmented zones, and sysfs defrag mode controls (`always`, `defer`, `defer+madvise`, `madvise`, `never`).

This PR proposal specifies the implementation of the **THP khugepaged Defrag & Runtime Page Promotion Daemon** in `src/memory/huge_pages.rs`.

---

## 2. Key Architectural Subsystems

| # | Architectural Subsystem | Specification / Structure | Primary Functionality |
|---|------------------------|---------------------------|-----------------------|
| 1 | **THP Defrag Modes** | `ThpDefragMode` | Configures compaction behavior (`Always`, `Defer`, `DeferMadvise`, `Madvise`, `Never`) |
| 2 | **Promotion Policy** | `ThpPagePromotionPolicy` | Defines scan rates, sleep intervals, access frequency thresholds, and collapse limits |
| 3 | **Zone Compactor** | `ThpMemoryCompactor` | Compacts 4KiB page fragmentation into contiguous 2MiB physical memory blocks |
| 4 | **khugepaged Daemon** | `ThpKhugepagedDefragDaemon` | Background daemon scanning, evaluating access frequency, and collapsing 4KiB pages into 2MiB THPs |

---

## 3. Technical Implementation Details

The THP khugepaged defrag daemon is implemented in `src/memory/huge_pages.rs`:

```rust
pub struct ThpKhugepagedDefragDaemon {
    pub policy: ThpPagePromotionPolicy,
    pub compactor: ThpMemoryCompactor,
    pub huge_page_engine: SovereignHugePageEngine,
    pub pages_collapsed: u64,
    pub alloc_success: u64,
    pub alloc_fail: u64,
    pub scan_count: u64,
}

impl ThpKhugepagedDefragDaemon {
    pub fn evaluate_and_promote_page_range(
        &mut self,
        base_phys_2mb: u64,
        access_counts: &[u32; PAGES_PER_2MB_HUGE_PAGE],
        has_madv_hugepage: bool,
    ) -> Option<u64> {
        // Evaluates access counters and MADV_HUGEPAGE hints to coalesce 512 4KiB pages into a 2MiB THP
        ...
    }

    pub fn trigger_khugepaged_scan_and_defrag(
        &mut self,
        fragmented_pages: &mut [u64],
    ) -> usize {
        // Compacts memory zone fragmentation and promotes newly formed contiguous 2MiB blocks
        ...
    }
}
```

---

## 4. Verification & Testing

- Standalone unit tests in `src/memory/huge_pages.rs` (`test_thp_khugepaged_daemon_and_compaction`).
- Automated verification via `./run_sigma_tests.sh` (`test_huge_pages_test`).
- Tested 4KiB to 2MiB page collapse, compaction of fragmented physical address runs, and defrag mode policy enforcement.

---

## 5. Architectural Alignment

Registered under Tier 1 (`Foundation & Hardware Control`) of the SigmaOS 3-Tier Architecture, maintaining zero-dependency memory management and sub-millisecond superpage promotion latency.
