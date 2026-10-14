# Sovereign Texinfo / GNU Info Pages Navigation Tree

## Summary
The Texinfo / GNU Info documentation system in SigmaOS provides structured, node-based navigation across all system specifications, architectural components, and development guides. All nodes are indexed into a unified GitHub Wiki tree with bidirectional link routing (`Top`, `Up`, `Next`, `Prev`).

---

## Node Directory Structure

```
* Top: (dir)
  ├── Kernel Micro-architecture
  ├── Packaging System
  ├── Sandboxing & Security
  └── Developer Ecosystem
```

---

## Node Details

### Node: `Top`
- **Navigation:** Up: `[(dir)]` | Next: `[Kernel Architecture]` | Prev: `[None]`
- **Description:** Welcome to the GNU Info Texinfo manual tree for SigmaOS.
- **Menu:**
  - `Kernel Architecture`: SMP, eBPF, Buddy Allocator & Lockless Memory Queues
  - `Packaging System`: Universal package format, DPLL SAT solver & zero-copy CAS store
  - `Sandboxing Security`: OpenBSD pledge/unveil, FreeBSD Capsicum & Linux Landlock v5

### Node: `Kernel Architecture`
- **Navigation:** Up: `[Top]` | Next: `[Packaging System]` | Prev: `[Top]`
- **Description:** Detailed documentation on 64-bit SMP multi-core dispatch, eBPF JIT compiler, slab allocator, and zero-allocation kernel ring buffers.

### Node: `Packaging System`
- **Navigation:** Up: `[Top]` | Next: `[Sandboxing Security]` | Prev: `[Kernel Architecture]`
- **Description:** Specifications for multi-format package ingestion, DPLL SAT dependency resolution, Dilithium-5 PQC signatures, and boot environment rollbacks.

### Node: `Sandboxing Security`
- **Navigation:** Up: `[Top]` | Next: `[Developer Ecosystem]` | Prev: `[Packaging System]`
- **Description:** Security governor details including OpenBSD `pledge`/`unveil`, FreeBSD Capsicum capability rights, and Linux Landlock v5 LSM filesystem isolation.
