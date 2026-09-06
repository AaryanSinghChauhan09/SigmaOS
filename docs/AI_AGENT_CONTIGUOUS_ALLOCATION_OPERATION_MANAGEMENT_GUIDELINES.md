# AI Agent Contiguous Allocation Operation Management Guidelines

## Purpose
These guidelines define operational protocols, implementation patterns, and safety guardrails for AI coding agents requesting or coalescing contiguous memory allocations in SigmaOS.

---

## Directives for AI Agents

1. **DMA & ISA Device Alignment**:
   - Always request contiguous physical memory from the reserved CMA pool (`allocate_contiguous`) for DMA/hardware buffers.
   - Fall back to real-time DMA defragmentation coalescing if initial contiguous searches fail.

2. **CMA Reservation Pattern**:
```rust
// Request 16 contiguous physical pages (64KB) for DMA
if let Ok(phys_addr) = cma.allocate_contiguous(16) {
    // Perform DMA operation
    cma.release_contiguous(phys_addr, 16)?;
}
```

3. **Testing and Verification**:
   - Execute `./run_sigma_tests.sh` to confirm CMA and DMA contiguous memory allocation unit tests pass.

---

## Related Files
- `src/kernel/memory/sigma_buddy.rs`
- `src/drivers/unified_dma.rs`
- `docs/AI_AGENT_CONTIGUOUS_ALLOCATION_OPERATION_MANAGEMENT_ARCHITECTURE.md`
- `wiki/AI_AGENT_CONTIGUOUS_ALLOCATION_OPERATION_MANAGEMENT.md`
