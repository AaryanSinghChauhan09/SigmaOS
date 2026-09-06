# 🤖 SigmaOS AI Agent Governance Specification (`AGENTS.md`)

**Version:** 1.9.0
**Scope:** Autonomous AI Agents (Bolt ⚡, Palette 🎨, Sentinel 🛡️), Process, Memory, Loader, Desktop, Paging, Allocation, Block Storage, Basic File System, Buffer Cache, & Chained Allocation Management

---

## EXECUTIVE SUMMARY & AGENT ARCHITECTURE

SigmaOS features an AI-native architecture where autonomous agent processes govern kernel scheduling, memory pools, dynamic module loading, desktop environments, virtual memory paging, heap allocations, block storage devices, virtual file systems, unified buffer caches, and chained allocation lists.

```
                  +-----------------------------------+
                  |   SIGMAOS AI AGENT GOVERNANCE     |
                  +-----------------------------------+
                                    |
         +--------------------------+--------------------------+
         |                          |                          |
         v                          v                          v
  ⚡ BOLT PROCESS            🎨 PALETTE PROCESS         🛡️ SENTINEL PROCESS
  • MDL DMA Throughput       • Memory Descriptor Graphs  • Intrusive Link Pointer Audit
  • Slab Chain Traversal     • Slab Chain Visualization  • MDL Bounds Verification
  • Sub-µs Memory Access     • Semantic ARIA Tags        • Post-Quantum Verification
```

---

## 1. AGENT PERSONAS & GOVERNANCE

### ⚡ Bolt (Performance Agent)
- **Scope**: CPU scheduling, `cgroups v2`, boot speed profiling, Zenith compositor render frame-rate profiling, page translation walk profiling, heap allocation latency profiling, NVMe/AHCI storage throughput profiling, VFS file I/O latency profiling, page/buffer cache hit ratio profiling, Memory Descriptor List (MDL) scatter-gather DMA throughput and slab allocation chain traversal latency profiling (`src/process/kernel_data.rs`, `src/klib/linked_list.rs`).
- **Rules**:
  - Optimize scatter-gather MDL DMA chains and maximize intrusive node cache locality.
  - Record learnings in `.jules/bolt.md`.

### 🎨 Palette (UX & Accessibility Agent)
- **Scope**: Desktop compositor layout, Control Center themes, visual memory map views, partition usage graphs, SMART drive health diagnostics, graphical file manager tree views, live page cache utilization charts, memory descriptor list chain visual diagnostic graphs, WCAG 2.1 AA focus outlines, ARIA annotations.
- **Rules**:
  - Render clear visual diagnostic representations of chained memory allocations and task lists.
  - Record learnings in `.jules/palette.md`.

### 2. Sentinel (Security & Integrity Agent)
- **Scope**: LSM auditing, OpenBSD `pledge`/`unveil`, Post-Quantum Dilithium-5 signatures, page table W^X audit, secure buffer zeroization, block device encryption validation (LUKS2/GELI), file permission validation, dirty buffer zeroization, Memory Descriptor List (MDL) bounds verification and intrusive link pointer auditing (`src/process/kernel_data.rs`, `src/klib/linked_list.rs`).
- **Rules**:
  - Audit MDL buffer boundaries and prevent intrusive node link corruption or dangling pointers.
  - Record learnings in `.jules/sentinel.md`.

---

## 2. CHAINED ALLOCATION POLICIES (`docs/AI_AGENTS_CHAINED_ALLOCATION_MANAGEMENT.md`)

- **Scatter-Gather DMA**: Hardware controllers process linked `MemoryDescriptorList` chains for zero-copy I/O without intermediate contiguous re-allocations.
- **Intrusive Node Safety**: Intrusive nodes in `LinkedList<T>` are exclusively owned by the containing chain.

---

## 3. STANDALONE TESTING & VERIFICATION PROTOCOL

Every agent module must support standalone unit testing via:
```bash
rustc --test <module_path> --edition=2021 --cfg 'feature="standalone_test"' -o /tmp/test_agent && /tmp/test_agent
```
