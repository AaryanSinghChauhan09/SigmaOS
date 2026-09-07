# SigmaOS AI Agent Demand Techniques Operation Management Guide

This guide defines operational protocols, page fault handling standards, and resource allocation guidelines for AI agents managing on-demand operating system techniques across SigmaOS.

---

## 1. Overview of On-Demand Techniques in SigmaOS

SigmaOS utilizes on-demand resource allocation and execution paradigms to minimize boot latency, reduce memory footprint, and optimize energy efficiency:

1. **Demand Paging & Swapping (`DemandPagingSwapEngine`):** Physical memory frames are allocated lazily when a process accesses an unmapped virtual memory address, triggering a `PageNotPresent` page fault (#PF). Excess memory pages are swapped out on demand to disk swap slots.
2. **Copy-on-Write (CoW) Page Duplication:** During process creation (`fork()`), parent and child share physical pages marked read-only. Duplicate physical frames are allocated on demand only when a write operation occurs (`WriteProtectionViolation`).
3. **On-Demand GPU Workload Offloading (`NvidiaOnDemand`):** Dynamic power management profile that keeps the discrete GPU in low-power idle state until heavy rendering/compute PIDs are offloaded on demand.
4. **On-Demand Package & Module Mounting (`TinyCoreModularTczLoader`):** Applications and driver extensions remain uncompressed in loopback images (`.tcz` / `.AppImage`) and are mounted into VFS namespaces on demand when executed.

---

## 2. On-Demand Memory Management Protocol

AI agents modifying or managing virtual memory subsystems MUST observe these rules:

1. **Zeroed Anonymous Frame Allocation:** On-demand anonymous page fault allocations MUST return zero-filled physical frames to prevent secret memory leakage across process boundaries.
2. **Atomic Swap Slot Tracking:** Allocating and freeing disk swap slots in `DemandPagingSwapEngine` MUST execute atomically under lock-free or spinlock protection.
3. **Lazy Vector/FPU Context Saving:** CPU FPU/SSE/AVX register state saving MUST occur lazily on demand upon the first `#NM` (Device Not Available) exception in a thread quantum.

---

## 3. Demand Paging & Fault Fault Handler Interface Pattern

```rust
pub enum PageFaultReason {
    PageNotPresent,
    WriteProtectionViolation,
    UserAccessViolation,
}

impl DemandPagingSwapEngine {
    pub fn handle_page_fault(&mut self, fault_vaddr: u64, reason: PageFaultReason) -> Result<(), &'static str> {
        match reason {
            PageFaultReason::PageNotPresent => {
                // 1. Allocate physical frame lazily
                // 2. Update page table entry (PTE) with PRESENT | WRITABLE
                Ok(())
            }
            PageFaultReason::WriteProtectionViolation => {
                // Handle Copy-on-Write (CoW) frame duplication
                Ok(())
            }
            PageFaultReason::UserAccessViolation => Err("SIGSEGV: Invalid memory access"),
        }
    }
}
```
