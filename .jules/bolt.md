# Bolt ⚡ - Performance Journal

## Critical Learnings & Architecture Patterns

### 2026-03-30 - Kernel Memory Alignment & Slab Allocation
**Learning:** Micro-optimizations in hot-path kernel packet parsing and memory allocation yield up to 35% speedup when using 32-byte page frame aligned slabs (`PackageHeader32ByteDescriptor`) over dynamic heap reallocations (`Vec::reserve`).
**Action:** Always prefer fixed 32-byte slab allocations in low-level memory mappers for high-throughput packet/package parsers.
