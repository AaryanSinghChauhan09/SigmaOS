# 🔗 AI Agents Chained Allocation Management Specification (`docs/AI_AGENTS_CHAINED_ALLOCATION_MANAGEMENT.md`)

This specification defines Memory Descriptor List (MDL) scatter-gather chaining, intrusive linked list allocation chains, slab page chaining, and lockless queue node linking for autonomous AI agents (Bolt ⚡, Palette 🎨, Sentinel 🛡️) in SigmaOS.

---

## 1. Memory Descriptor List (MDL) Chaining (`src/process/kernel_data.rs`)

AI agents manage scatter-gather DMA memory descriptor chains:
- **`MemoryDescriptorList`**: Describes non-contiguous physical page frame ranges backing virtual buffer allocations.
- **MDL Link Chains (`link_mdl`)**: Connects multiple MDL descriptors into a unified scatter-gather chain for zero-copy I/O operations.
- **Scatter-Gather DMA**: Hardware controllers process chained MDLs without intermediate contiguous re-allocations.

---

## 2. Intrusive & Doubly Linked Allocation Chains (`src/klib/linked_list.rs`)

- **Intrusive `LinkedList<T>`**: Zero-allocation node linking embedded directly within data structures.
- **Slab Page Allocation Chains**: Slab allocators maintain partial, full, and empty slab page chains using intrusive pointer links.
- **Circular Doubly Linked Lists**: WDK-inspired doubly linked lists (`LIST_ENTRY` parity) managing active task and APC queues.

---

## 3. Allocation Chain Invariants & Memory Safety

- **Node Lifetime & Ownership**: Intrusive nodes are owned exclusively by the containing chain.
- **Non-Paged Pool Invariants**: Chained allocation pointers belonging to interrupt or DPC queues must reside strictly in `NonPagedPool`.
- **Lock-Free Concurrency**: Atomic pointer CAS operations maintain lock-free singly linked queues during concurrent multi-core operations.

---

## 4. AI Agent Chained Allocation Responsibilities

- **⚡ Bolt**: Profiles MDL scatter-gather DMA throughput, measures slab allocation chain traversal times, and optimizes intrusive node cache locality.
- **🎨 Palette**: Visualizes memory descriptor chains, slab allocation page distributions, and active task list graphs in diagnostic views.
- **🛡️ Sentinel**: Audits MDL buffer boundaries, verifies intrusive link pointer integrity, and detects cyclic link corruption or dangling pointers.
