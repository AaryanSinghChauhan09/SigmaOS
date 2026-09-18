# SigmaOS AI Agent Buddy System Management Guidelines

## 1. Overview
SigmaOS implements a low-level physical memory buddy system allocator (`sigma_buddy` in `src/kernel/memory.rs`) managed autonomously by AI memory agents (such as `BuddyAllocatorGovernor`, `CmaReservationManager`, and `FreeBsdZoneQueueOptimizer`). These guidelines define power-of-two page order calculations, buddy page splitting and coalescing, Contiguous Memory Allocator (CMA) reservations, FreeBSD VM zone queue transitions, watermark evaluations, and page-migration routing for AI agents in SigmaOS.

## 2. Core Buddy System Management Principles

### 2.1 Page Order Calculation & Splitting / Merging
- **Order Calculation**: Page allocations request sizes in power-of-two blocks ($2^{\text{order}}$ pages, where $\text{order} \in [0, 11]$).
- **Buddy Page Splitting**: When an order-$k$ block is unavailable, a higher-order block ($k+1$) is split into two equal "buddy" blocks.
- **Buddy Page Coalescing**: Upon deallocation, if a block's adjacent "buddy" block is also free, they are recursively merged back into a higher-order block to eliminate memory fragmentation.

### 2.2 Contiguous Memory Allocator (CMA) Reservations
- **CMA Glue**: Specialized DMA hardware drivers (e.g. NVMe, E1000 NICs, GPUs) request unfragmented contiguous physical memory blocks via CMA glue routines.
- **Fallback Migration**: Non-movable pages are routed away from CMA pools (`MIGRATE_UNMOVABLE` vs `MIGRATE_RECLAIMABLE` vs `MIGRATE_MOVABLE`), ensuring CMA regions remain free for high-priority DMA allocations.

### 2.3 FreeBSD VM Zone Queue Transitions
- **Zone Queues**: Physical memory pages transition through FreeBSD-inspired VM zone queues: `ACTIVE`, `INACTIVE`, `LAUNDRY`, and `FREE`.
- **Watermark Evaluation**: AI agents monitor page watermarks (`MIN`, `LOW`, `HIGH`). When free memory drops below `LOW_WATERMARK`, background kswapd page reclamation is triggered.

### 2.4 Ancient ISA DMA & Memory Alignment Bounds
- **16MB ISA DMA Boundary**: Legacy device allocations enforce physical address limits below 16MB (`ZONE_DMA`).
- **32-Bit PCI DMA Boundary**: PCI devices enforce 4GB address boundaries (`ZONE_DMA32`).
- **64-Bit Direct Addressing**: Modern 64-bit devices allocate from `ZONE_NORMAL`.

---
*Maintained by the SigmaOS Memory Management Steering Committee & SIG-Kernel.*
