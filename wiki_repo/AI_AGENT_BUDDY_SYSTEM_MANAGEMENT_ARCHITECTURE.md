# AI Agent Buddy System Management Architecture in SigmaOS

## Architecture Blueprint

```
+---------------------------------------------------------------------------------+
|                       AI Physical Memory Buddy Manager                          |
|    (BuddyAllocatorGovernor, CmaReservationManager, FreeBsdZoneOptimizer)        |
+---------------------------------------------------------------------------------+
                                        |
                                        v
+---------------------------------------------------------------------------------+
|                   Physical Page Allocation & Order Calculator                   |
|       (Order 0..11, 4KB to 8MB Pages, Split/Merge Tree, Zone Watermarks)        |
+---------------------------------------------------------------------------------+
                                        |
       +--------------------------------+--------------------------------+
       |                                |                                |
       v                                v                                v
+-----------------------+   +-----------------------+   +-----------------------+
|  Buddy Free Lists     |   |  CMA Reservation Pool |   | FreeBSD VM Zone Queues|
| (Order 0..MAX_ORDER)  |   | (Unfragmented DMA)    |   | (ACTIVE/LAUNDRY/FREE) |
+-----------------------+   +-----------------------+   +-----------------------+
       |                                |                                |
       +--------------------------------+--------------------------------+
                                        |
                                        v
+---------------------------------------------------------------------------------+
|                         Physical Memory Hardware Zones                          |
|             (ZONE_DMA <16MB, ZONE_DMA32 <4GB, ZONE_NORMAL >4GB)                 |
+---------------------------------------------------------------------------------+
```

## Architectural Components

1. **Buddy Allocation Tree & Order Solver**:
   - Order-based freelists (`order 0` = 4KB, `order 11` = 8MB).
   - Recursive binary page split/merge operations eliminate physical memory fragmentation.

2. **Contiguous Memory Allocator (CMA) Pool**:
   - Reserves contiguous physical memory ranges for high-bandwidth NVMe, E1000, and GPU DMA descriptors.
   - Non-movable process pages are isolated away from CMA blocks using page-migration types (`MIGRATE_UNMOVABLE`, `MIGRATE_MOVABLE`).

3. **FreeBSD VM Zone Queues & Watermarks**:
   - Tracks page aging across `ACTIVE`, `INACTIVE`, `LAUNDRY`, and `FREE` zone queues.
   - Triggers automated background kswapd page reclamation when free memory drops below `LOW_WATERMARK`.

4. **Wiki Syncing**:
   This document is mirrored in `./wiki/` and `./wiki_repo/` for GitHub Wiki access.
