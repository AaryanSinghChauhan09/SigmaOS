# AI Agents Clock Replacement & Timekeeping Specification for SigmaOS

## Abstract
This specification defines the Clock (Second-Chance) page replacement algorithm, user-defined page eviction policies, CPU cache replacement algorithms, and real-time clock timekeeping infrastructure for AI agents operating within or developing memory/kernel subsystems for SigmaOS.

---

## 1. Virtual Memory Clock (Second-Chance) Page Replacement

SigmaOS uses the CLOCK (Second-Chance) algorithm (`SimpleVMM::perform_clock_replacement_step`) for virtual memory page frame eviction under memory pressure.

```
 [ Active Pages Circular Buffer: active_pages_for_clock ]
                       ┌───┐
                       │P0 │ (accessed = true -> reset to false, 2nd chance)
                       └───┘
                         ▲
                         │ [ Clock Hand Pointer ]
                       ┌───┐
                       │P1 │ (accessed = false -> EVICTED to zram / swap)
                       └───┘
```

### 1.1 CLOCK Algorithm Execution Flow
1. **Clock Hand Scan**:
   - The clock hand (`clock_hand`) traverses the circular array of active mapped virtual addresses (`active_pages_for_clock`).
2. **Accessed Bit Evaluation**:
   - **Accessed = 1**: The page is granted a "second chance". Its `accessed` bit is reset to `false`, and the clock hand advances to the next entry.
   - **Accessed = 0**: The page is selected for eviction.
3. **zram Swap Compression**:
   - Evicted physical pages are compressed via zram (`ZramPage`) and stored in the compressed in-memory zram pool (`zram_pool`), avoiding disk I/O bottlenecks.
   - The page table entry is cleared (`None`) and unmapped.
4. **On-Demand Page Restore**:
   - Accessing a zram-swapped page triggers a Page Fault handler that decompresses the page in-place and re-maps it back into the active page table.

---

## 2. User-Defined & CPU Cache Replacement Policies

### 2.1 User-Defined Page Replacement
- Modules can register dynamic page eviction policies via `IUserPageReplacement` in `src/kernel/user_defined.rs`:
  - **LFU (Least-Frequently-Used)**: Tracks page access frequency counters.
  - **MFU (Most-Frequently-Used)**: Evicts pages with highest access frequency.

### 2.2 CPU Cache Replacement Policies
- L1/L2/L3 hardware CPU cache controllers (`src/kernel/mm/cpu_cache.rs`) implement:
  - **LRU (Least Recently Used)**: Evicts cache lines with oldest access timestamp.
  - **pLRU (Pseudo-LRU)**: Tree-based binary decision tree for fast $O(1)$ eviction.

---

## 3. Real-Time Clock & System Timekeeping

### 3.1 Hardware Timekeeping Architecture
- Timekeeping interfaces in `src/time/clock.rs` manage:
  - High-resolution timers (`SimpleTimer`, `TimerTable`).
  - Monotonic tick counters (`tick_timers`) for kernel task scheduling.
  - WDK Windows Driver Kit timer structures (`WdkTimer`).
  - Asynchronous event timers via `kqueue` (`EVFILT_TIMER`).

---

## 4. Operational Directives for AI Agents

1. **Lock-Free Clock Updates**:
   - AI agents modifying memory management units MUST ensure `accessed` bit toggles use atomic operations or safe page-table mutability guards.
2. **zram Pool Monitoring**:
   - Before evicting pages, agents should query available zram memory capacity to prevent compression pool overflow.
3. **Timer Callbacks**:
   - High-resolution timer callbacks MUST NOT perform blocking disk writes or heavy allocations inside software interrupt context.

---

## 5. Wiki Synchronization

This document is synchronized across all documentation hubs via `./scripts/sync_wiki.sh`:
- `WIKI/AI_AGENTS_CLOCK_REPLACEMENT_SPEC.md`
- `wiki/AI_AGENTS_CLOCK_REPLACEMENT_SPEC.md`
- `wiki_repo/AI_AGENTS_CLOCK_REPLACEMENT_SPEC.md`

---

*Specification Version: 1.0.0 — SigmaOS Virtual Memory & Timekeeping Architecture*
