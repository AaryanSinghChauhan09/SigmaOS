# SigmaOS Memory Management

## Physical Memory — Buddy Allocator

Manages physical pages in power-of-2 blocks (order 0=4KB, order 9=2MB):

*   **Allocation**: Find smallest free block >= requested order, split if needed
*   **Deallocation**: Merge with buddy if both free -> promotes to higher order

## Virtual Memory — 4-Level Paging (x86\_64)

    Virtual Address (48-bit):
      [47:39] PML4 (9 bits) → PML4 table
      [38:30] PDPT (9 bits) → Page Directory Pointer
      [29:21] PD   (9 bits) → Page Directory
      [20:12] PT   (9 bits) → Page Table
      [11:0]  Page offset (12 bits)

## Memory Zones

| Zone | Range | Purpose |
|------|-------|---------|
| DMA | 0–16 MB | Legacy ISA DMA |
| DMA32 | 16 MB–4 GB | 32-bit PCI DMA |
| Normal | >4 GB | General use |
| Movable | Configurable | Hot-plug |

## Slab Allocator

Kernel object caches — pre-formatted page slabs for frequent small allocations. O(1) alloc/dealloc, excellent cache locality.

## Key Algorithms

### kswapd

    Trigger: free < low_watermark
      → Scan LRU (active_anon, inactive_anon, active_file, inactive_file)
      → Age pages: active → inactive on second chance miss
      → Reclaim clean file pages
      → Swap dirty anonymous pages
      → Return pages to buddy
    Stop: free > high_watermark

### KSM (Kernel Same-page Merging)

1.  Hash page content (SHA-256)
2.  Compare with existing merged pages
3.  If match → remap both to single CoW page
4.  On write → allocate new private page

### Copy-on-Write (Fork)

    fork():
      Clone parent page table (mark all read-only + CoW)
      Parent & child share all physical pages

    First write:
      Page fault → detect CoW flag
      Allocate new page → copy content
      Update PTE: remove CoW, set writable

## NUMA Topology

*   First-touch: memory on fault-node
*   NUMA balancing: hot pages migrated to local node
*   Interleaved allocation for shared data structures

## OOM Killer

    score = (memory_used / total_memory) * 1000 + adjustments
    Kill process with highest score

## Memory Overcommit Modes

| Mode | Behavior |
|------|---------|
| 0 | Heuristic (default) |
| 1 | Always allow |
| 2 | Never overcommit |
