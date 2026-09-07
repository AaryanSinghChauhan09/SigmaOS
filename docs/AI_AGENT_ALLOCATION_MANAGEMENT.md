# AI Agent Allocation Management in SigmaOS

## Overview
SigmaOS features an autonomous, multi-resource Allocation Management Subsystem supervised by AI Agents (**Sentinel** 🛡️, **Bolt** ⚡, **Palette** 🎨). This document defines operational directives, memory policies, and API interfaces for AI agents allocating physical memory frames, system resource shares (CPU, memory, max processes, I/O bandwidth), and process resource limits (`rlimit`).

AI agents interact directly with `src/resource/sovereign_allocator.rs` (`SovereignMultiResourceAllocator`), `src/memory/pmm_vmm.rs` (`BitmapFrameAllocator`), and `src/resource/rlimit.rs` (`RlimitManager`).

---

## 1. Core Allocation Subsystems & Frameworks

### 1.1 Multi-Resource Domain Allocator (`SovereignMultiResourceAllocator`)
Implemented in `src/resource/sovereign_allocator.rs`, providing hierarchical resource domain allocation and enforcement:
```rust
pub enum AllocatorResourceType {
    CpuShares,
    MemoryPages,
    MaxProcessCount,
    IoBandwidth,
    NetworkSocketCount,
    OpenFileDescriptors,
}
```
* **Resource Limits**: Soft limits (`soft_limit`) trigger warning signals; hard limits (`hard_limit`) strictly reject requests (`request_allocation`).
* **Allocation Requests**: Agents invoke `allocate_resource(domain_id, res_type, amount)` and `free_resource(domain_id, res_type, amount)`.

### 1.2 Physical Frame Allocator (`BitmapFrameAllocator`)
Implemented in `src/memory/pmm_vmm.rs`. Manages 4 KiB physical memory frames via a physical memory bitmap initialized from the boot memory map (`init_from_memory_map`).
* **Page Allocation**: Agents monitor frame consumption to prevent physical memory exhaustion.

### 1.3 Process Resource Limits (`rlimit`)
Implemented in `src/resource/rlimit.rs`. Enforces POSIX process limits (`RLIMIT_NOFILE`, `RLIMIT_AS`, `RLIMIT_NPROC`, `RLIMIT_STACK`, `RLIMIT_CORE`).

---

## 2. AI Agent Operational Directives & Workflows

### 2.1 Dynamic Workload Allocation & Rebalancing
1. **Workload Profiling**:
   AI agents monitor latency requirements (`WorkloadLatencyClass`). Under heavy AI/3D compilation tasks, **Bolt** ⚡ dynamically increases domain `CpuShares` and `MemoryPages` limits.
2. **Soft Limit Threshold Warnings**:
   When domain usage exceeds 80% of its soft limit, agents trigger memory compaction or cache trimming (`SovereignCacheFlow`).
3. **Hard Limit Rejection & Emergency Fallback**:
   If an allocation request exceeds `hard_limit`, `SovereignMultiResourceAllocator` returns an `Err("Limit exceeded")`. Agents handle the failure by shedding background jobs or throttling thread pools.

---

## 3. Compliance & Security Protocol Rules

1. **Isolation Invariants**:
   No userland process domain may consume resources allocated to `RootDomain` or core kernel memory pools.
2. **Zero-Allocation Hot Path Policy**:
   Performance-critical interrupt handlers and IPC channels must use pre-allocated buffers (`src/klib/`) without runtime heap allocations.
3. **Audit Trails**:
   All resource quota modifications and limit overruns are logged to `ChainedAuditTrailLedger` for ISO 27001 compliance.

---

## 4. Sample Agent Commands & CLI Interactions

```bash
# Query active resource allocation limits for domain
sigma-alloc status --domain 1

# Update memory soft/hard limits for user process domain
sigma-alloc set-limit --domain 2 --resource MemoryPages --soft 1048576 --hard 2097152

# Inspect physical frame allocator usage bitmap
sigma-alloc pmm-stats
```
