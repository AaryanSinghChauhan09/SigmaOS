# Sovereign AI Agent Zones Operation Management Specification

This document specifies mandatory guidelines, architectural principles, physical memory zoning invariants, FreeBSD UMA (Universal Memory Allocator) zone management protocols, watermark-driven reclamation rules, and double-free protection standards for autonomous AI engineering agents (Jules, Sentinel, Palette, Bolt) working within the SigmaOS memory management subsystem (`src/memory/zone.rs`, `src/memory/manager.rs`).

---

## 1. Physical Memory Addressing Zones Architecture

SigmaOS partitions physical memory address space into four distinct memory zones to accommodate hardware bus addressing limits:

1. **`ZONE_DMA` (0 – 16 MiB)**:
   - Reserved exclusively for legacy ISA devices, floppy controllers, and 24-bit physical DMA bus controllers.
   - Physical allocations in this zone must satisfy strict 16-bit addressing constraints.
2. **`ZONE_DMA32` (16 MiB – 4 GiB)**:
   - Designated for 32-bit PCI device DMA buffers (e.g., Intel E1000 NICs, legacy AHCI SATA controllers, USB 2.0 EHCI controllers).
   - Serves 32-bit hardware devices lacking 64-bit IOMMU physical address translation support.
3. **`ZONE_NORMAL` (4 GiB – Architecture Ceiling)**:
   - Primary physical memory pool directly mapped into kernel virtual address space for process page tables, kernel heap allocations, file system page caches, and driver shards.
4. **`ZONE_HIGHMEM` (Architectural High Addresses)**:
   - Physical memory regions not permanently mapped into kernel space (primarily applicable on 32-bit architectures or constrained virtual address spaces). Pages in this zone are mapped dynamically on-demand via kmap operations.

---

## 2. FreeBSD UMA (Universal Memory Allocator) Zone Management (`src/memory/zone.rs`)

SigmaOS implements a FreeBSD-inspired Universal Memory Allocator (UMA) providing type-stable object caching and slab management:

1. **Zone Creation (`uma_zcreate`)**:
   - `uma_zcreate(name, item_size, max_items)` initializes a type-stable object cache zone for kernel resources (e.g., `ThreadControlBlock`, `VnodeZone`, `FileDescriptorZone`).
   - Every zone allocates virtual memory cushion padding between slab blocks to guarantee address space isolation and prevent slab boundary corruption.
2. **Type-Stable Allocation (`uma_zalloc`) & Deallocation (`uma_zfree`)**:
   - Object requests attempt allocation from existing slab free-lists first.
   - If active slabs are full and `allocated_items < max_items`, the allocator grows the zone by allocating a new 4KiB page slab.
   - Deallocations check slab address range bounds (`has_address`) and enforce double-free validation.
3. **Zone Draining (`uma_zdrain`)**:
   - Under memory pressure, calling `uma_zdrain()` iterates across all registered UMA zones, identifying and releasing completely free 4KiB slabs back to the physical page frame allocator while retaining partially populated slabs for type stability.

---

## 3. Watermark-Driven Page Reclamation (`kswapd`)

1. **Memory Watermarks**:
   - **`Watermark::High`**: Normal operating zone capacity. Asynchronous page reclamation is idle.
   - **`Watermark::Low`**: Free physical pages drop below `Low`. Triggers asynchronous background page reclamation (`kswapd` thread) to reclaim page cache, expire inode dentries, and invoke `uma_zdrain()`.
   - **`Watermark::Min`**: Critical physical memory exhaustion. Direct sync page reclamation is enforced on calling threads, blocking new process allocations until free memory rises above `Watermark::Min`.

---

## 4. Double-Free Protection & Allocation Safety Invariants

1. **Boundary Checking**:
   - `Slab::free` must verify that the target address lies strictly within `[start_address, start_address + size)`.
2. **Duplicate Release Prevention**:
   - Before returning an item slot to `free_slots`, the slab allocator must confirm `!free_slots.contains(&address)`. Attempting to free an already-free item must return `Err("Double free or invalid release")`.
3. **Zone Capacity Caps**:
   - `allocated_items` must never exceed `max_items`. Requests exceeding `max_items` must return `None` rather than exhausting kernel address space.

---

## 5. AI Agent Zones Operation Management Directives Summary

1. **Respect Hardware DMA Address Limits**: Direct DMA allocations for 32-bit PCI peripherals to `ZONE_DMA32` and 24-bit ISA controllers to `ZONE_DMA`.
2. **Use Type-Stable UMA Zones**: Allocate fixed-size kernel structures via `uma_zcreate`/`uma_zalloc` rather than raw dynamic heap pointers.
3. **Drain Slabs Under Memory Pressure**: Invoke `uma_zdrain()` during memory reclamation cycles to release empty slabs.
4. **Enforce Double-Free Protection**: Ensure all custom zone and slab implementations enforce explicit address bounds and free-slot duplicate checks.
