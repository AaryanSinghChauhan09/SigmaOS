# SigmaOS Memory Subsystem Policy

## Memory-Management Objective

> **Maintain SigmaOS memory management as a safe, observable, testable subsystem: allocate, map, protect, swap, limit, reclaim, and release memory without leaks, races, corruption, or undefined behavior.**

SigmaOS connects memory models directly to a validated boot-to-shell runtime path, turning experimental memory structures into an evidence-backed, maintainable kernel subsystem.

---

## Authoritative Memory Management Contract

SigmaOS memory management enforces one single authoritative lifecycle:

```text
boot -> physical allocator init -> paging init -> virtual memory init ->
allocation -> mapping -> protection -> swap -> reclaim -> free -> cleanup
```

### Stage Behaviors and Requirements

| Stage | Required Behavior | Invariants & Guarantees |
|---|---|---|
| **Boot** | Memory map discovered from hardware/firmware, reserved regions excluded | E820 / EFI memory map parsed; kernel/firmware regions marked unallocatable. |
| **Physical Allocation** | Frames allocated/freed with bounded capacity | Lock-free or race-free frame allocation; double-free protection; typed exhaustion errors. |
| **Paging** | Page tables initialized, mappings race-safe | 4-level x86_64 paging structure; page-table entries protected against lost-updates. |
| **Virtual Memory** | User/kernel mappings isolated | User and kernel address space boundaries strictly enforced; virtual region limits defined. |
| **Protection** | Permissions enforced on every mapping | Read, Write, Execute, User/Supervisor permissions enforced; W^X (Write XOR Execute) policy. |
| **Swap** | Swap-out / swap-in failure-safe | Bounded swap slot bitmap; race-free disk enqueue; priority queue processing; zero lost swaps. |
| **Reclaim** | Defined reclaim policy under memory pressure | LRU/mGLRU page compaction; background kswapd flushing; loss-less frame reclamation. |
| **Free** | Frames and accounting released exactly once | Double-free prevention via reference counting and bitmap sanity checks; zero leaks. |

---

## Memory Policies

### Physical Allocator Class
- **Hybrid Buddy + Slab Allocator**: Bounded buddy allocator for contiguous physical frame allocation backed by slab cache pools for fine-grained kernel object allocations.
- **Frame Allocator Policy**: Bitmap-tracked physical memory manager (`src/memory/physical.rs`, `src/memory/frame.rs`) with O(1) free-list search and bounded capacity checks.
- **Paging Policy**: 4-level page table management (`src/mm/paging.rs`, `src/mm/page_table.rs`) with canonical address validation, TLB shootdown hooks, and race-free page table mutation.
- **Virtual Memory Policy**: VMA tracking (`src/mm/virtual.rs`) supporting demand paging, copy-on-write (COW), kernel/user isolation, and W^X memory protection.
- **Swap Policy**: Priority-queued swap slots (`src/swap/core.rs`, `src/swap/enqueue.rs`) supporting bounded slot allocation, zram/zswap compression stages, and deterministic disk persistence.
- **Reclaim Policy**: Multi-generational LRU aging, active/inactive page list scanning, and page compaction (`src/memory/kswapd.rs`, `src/memory/compaction.rs`).
- **OOM Policy**: Explicit OOM score evaluation and typed error handling (`src/memory/oom_killer.rs`). OOM is a designed state returning `Err(MemoryError::OutOfMemory)`, preventing accidental kernel panics.
- **NUMA & Hotplug Behavior**: ACPI SLIT/SRAT awareness (`src/memory/acpi_slit.rs`) and physical frame hotplug registration with NUMA node affinity binding.

---

## Periodic Task Calendar

| Frequency | Memory Task | Output | Required PR |
|---|---|---|---|
| **Every PR** | Build, lint, unit, and integration tests | CI result | `feat`, `fix`, `test`, `docs`, or `chore` PR |
| **Daily** | CI/nightly triage | Root cause + fix | `fix(memory)` or `fix(ci)` PR |
| **Weekly** | Memory subsystem review | Gap list + status | `docs(memory)` PR |
| **Weekly** | Linux/BSD memory study | Study note | `docs(studies)` PR |
| **Biweekly** | Allocator audit | Leak/OOM findings | `fix(memory)` / `test(memory)` PR |
| **Biweekly** | Paging and protection audit | Mapping findings | `fix(paging)` PR |
| **Monthly** | Boot-to-shell memory validation | Boot log + smoke report | `test(memory)` PR |
| **Monthly** | Memory stress/fuzz triage | Crash reports + fixes | `fix(memory)` PR |
| **Monthly** | Swap rehearsal | Swap enforcement report | `test(swap)` PR |
| **Quarterly** | Memory architecture review | ADR + roadmap | `docs(adr)` PR |
| **Quarterly** | Security review | Threat-model update | `security(memory)` PR |
| **Quarterly** | Release readiness review | Release checklist | `docs(release)` PR |

---

## First 12 Memory PR Roadmap

| # | PR | Purpose |
|---:|---|---|
| 1 | `docs(memory): add lifecycle state machine` | Establish authoritative memory states |
| 2 | `docs(memory): add frame and page-table policy` | Define allocation, reuse, and limits |
| 3 | `feat(memory): add bounded physical manager` | Prevent unbounded kernel growth |
| 4 | `feat(memory): add race-free frame allocator` | Ensure safe frame lifecycle |
| 5 | `test(memory): add paging/protection conformance tests` | Validate core lifecycle |
| 6 | `feat(memory): define reclaim and OOM policy` | Eliminate corruption leaks |
| 7 | `feat(memory): add page-table contract and integrity metric` | Make paging observable |
| 8 | `test(mm): add preemption and starvation tests` | Validate scheduling behavior |
| 9 | `feat(swap): define swap-out/swap-in policy` | Make signals deterministic |
| 10 | `test(swap): add CPU, memory, fd, and process-limit tests` | Enforce resource limits |
| 11 | `test(namespaces): add address-space isolation tests` | Validate isolation boundaries |
| 12 | `test(boot): add boot-to-shell memory smoke test` | Connect memory management to runtime |

---

## Memory Health Metrics

| Metric | Target | Verification Method |
|---|---|---|
| **Memory CI pass rate** | 100% | GitHub Actions CI |
| **Boot-to-shell pass rate** | 100% | QEMU smoke test runner |
| **Orphan frames after stress test** | 0 | Physical manager audit |
| **Double-free incidents** | 0 | Frame tracking sanity assertions |
| **Frame reuse races** | 0 | Concurrency unit tests |
| **Memory corruption incidents** | 0 | Page table & protection tests |
| **Declared limits with enforcement tests** | 100% | VMA/Swap limit test suite |
| **Memory claims linked to evidence** | 100% | PR evidence checklist audit |
| **Security-sensitive PRs reviewed** | 100% | Security gate evaluation |
