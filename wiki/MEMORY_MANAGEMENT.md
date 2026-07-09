# SigmaOS Memory Management

## Overview

SigmaOS implements a sophisticated memory management system with multiple allocators for different use cases. This document describes the memory management architecture and implementation details.

## Memory Management Components

### 1. Buddy Allocator

The buddy allocator manages physical memory using the buddy system algorithm.

**Location**: `kernel/mm/buddy_allocator.rs`

**Features**:
- Linked list free lists per order (0-10)
- Block splitting and merging
- Static frame table for tracking
- No heap allocations
- Support for up to 1GB memory (262,144 frames)

**Data Structures**:

```rust
pub struct BuddyAllocator {
    initialized: bool,
    total_frames: u64,
    free_frames: u64,
    allocated_frames: u64,
    max_order: u8,
    free_lists: [Option<u64>; MAX_ORDER + 1],
    frame_table: [PhysicalFrame; MAX_FRAMES],
    block_table: [BuddyBlock; MAX_FRAMES],
    base_address: u64,
}

pub struct PhysicalFrame {
    pub pfn: u64,
    pub order: u8,
    pub allocated: bool,
    pub reserved: bool,
}

pub struct BuddyBlock {
    pub order: u8,
    pub free: bool,
    pub split: bool,
    pub next: u64,
    pub prev: u64,
}
```

**API**:

```rust
pub unsafe fn sigma_buddy_init(base_addr: u64, total_mem: u64) -> i32;
pub unsafe fn sigma_buddy_alloc(order: u8) -> u64;
pub unsafe fn sigma_buddy_free(addr: u64, order: u8) -> i32;
pub unsafe fn sigma_buddy_get_free() -> u64;
pub unsafe fn sigma_buddy_get_total() -> u64;
```

**Allocation Algorithm**:

1. Find free block of requested order or larger
2. If larger, split blocks until reaching requested order
3. Mark block as allocated
4. Update frame table

**Free Algorithm**:

1. Mark block as free
2. Try to merge with buddy block
3. If buddy is free, merge and repeat
4. Add merged block to free list

### 2. Slab Allocator

The slab allocator manages kernel object allocation efficiently.

**Location**: `kernel/mm/slab_allocator.rs`

**Features**:
- Per-object-type caches
- Efficient small object allocation
- Cache management
- Reduced fragmentation

**Data Structures**:

```rust
pub struct SlabCache {
    name: [u8; 32],
    object_size: usize,
    align: usize,
    objects_per_slab: usize,
    free_slabs: *mut Slab,
    partial_slabs: *mut Slab,
    full_slabs: *mut Slab,
}

pub struct Slab {
    cache: *mut SlabCache,
    free_objects: *mut *mut u8,
    objects: [u8; 0],
}
```

**API**:

```rust
pub unsafe fn sigma_slab_init() -> i32;
pub unsafe fn sigma_slab_alloc(size: usize, align: usize) -> *mut u8;
pub unsafe fn sigma_slab_free(ptr: *mut u8, size: usize);
```

### 3. Page Table Walker

The page table walker manages virtual memory and page tables.

**Location**: `kernel/mm/page_table_walker.rs`

**Features**:
- Page table traversal
- Page mapping/unmapping
- Permission management
- Support for 4-level page tables

**Data Structures**:

```rust
pub struct PageTableEntry {
    pub present: bool,
    pub writable: bool,
    pub user_accessible: bool,
    pub write_through: bool,
    pub cache_disable: bool,
    pub accessed: bool,
    pub dirty: bool,
    pub global: bool,
    pub available: u8,
    pub frame: u64,
}

pub struct PageTable {
    pub entries: [PageTableEntry; 512],
}
```

**API**:

```rust
pub unsafe fn sigma_pt_walk(cr3: u64, virt_addr: u64) -> Option<PageTableEntry>;
pub unsafe fn sigma_pt_map(cr3: u64, virt_addr: u64, phys_addr: u64, flags: u64) -> bool;
pub unsafe fn sigma_pt_unmap(cr3: u64, virt_addr: u64) -> bool;
```

## Memory Layout

### Physical Memory Layout

```
0x0000000000000000 - 0x0000000000800000 : Reserved (BIOS, etc.)
0x0000000000800000 - 0x0000000001000000 : Kernel code
0x0000000001000000 - 0x0000000002000000 : Kernel data
0x0000000002000000 - 0x0000000040000000 : Available memory
```

### Virtual Memory Layout

```
0xFFFFFFFF80000000 - 0xFFFFFFFFFFFFFFFF : Kernel space
0x0000000000000000 - 0x00007FFFFFFFFFFF : User space
```

## Memory Allocation Strategy

### Kernel Memory

Kernel memory is allocated using:
1. **Buddy allocator** for large allocations (pages)
2. **Slab allocator** for small objects (structures, buffers)
3. **Static allocation** for fixed-size data

### User Memory

User memory is allocated using:
1. **mmap system call** for memory regions
2. **brk system call** for heap expansion
3. **Anonymous mappings** for private memory

## Memory Protection

SigmaOS implements memory protection through:

1. **Page permissions**: Read, write, execute flags
2. **User/supervisor mode**: Separate kernel and user space
3. **Capability checks**: Memory access requires capabilities
4. **NX bit**: No-execute bit for data pages

## Memory Optimization

### Zero-Allocation Optimizations

SigmaOS includes zero-allocation optimizations in `kernel/core/sigma_zero_alloc.rs`:

```rust
pub struct ZeroAllocString<const N: usize> {
    data: [u8; N],
    len: usize,
}

pub unsafe fn sigma_sprintf(buf: *mut u8, fmt: *const u8, args: VaList) -> i32;
```

### Memory Pooling

Temporary buffer pool for short-lived allocations:

```rust
pub struct ZeroAllocPool {
    buffers: [[u8; 4096]; 16],
    used: [bool; 16],
}
```

## Memory Debugging

### Memory Leak Detection

Track allocations and frees:

```rust
pub unsafe fn sigma_mem_track_alloc(ptr: *mut u8, size: usize);
pub unsafe fn sigma_mem_track_free(ptr: *mut u8);
pub unsafe fn sigma_mem_dump_leaks();
```

### Memory Corruption Detection

Use guard pages and canaries:

```rust
pub unsafe fn sigma_mem_add_guard(ptr: *mut u8, size: usize);
pub unsafe fn sigma_mem_check_guard(ptr: *mut u8) -> bool;
```

## Memory Statistics

### Tracking Memory Usage

```rust
pub unsafe fn sigma_buddy_get_allocated() -> u64;
pub unsafe fn sigma_buddy_get_free() -> u64;
pub unsafe fn sigma_buddy_get_total() -> u64;
```

### Memory Pressure Handling

When memory is low:
1. Trigger garbage collection
2. Compress inactive pages
3. Swap to disk (future)
4. Kill low-priority processes (future)

## Future Enhancements

### Planned Features

1. **Swap support**: Page out to disk
2. **Memory compression**: zswap/zram
3. **Huge pages**: 2MB and 1GB pages
4. **NUMA support**: Multi-socket systems
5. **Memory hotplug**: Add/remove memory dynamically

### Research Areas

1. **Automatic memory management**: Rust-style ownership in C
2. **Persistent memory**: NVDIMM support
3. **Heterogeneous memory**: HBM, CXL support
4. **Memory tagging**: ARM MTE-style tagging

## Best Practices

### For Kernel Developers

1. Use slab allocator for small objects
2. Use buddy allocator for page-sized allocations
3. Prefer static allocation where possible
4. Always check allocation return values
5. Free memory when no longer needed

### For Userland Developers

1. Use appropriate allocation sizes
2. Reuse buffers when possible
3. Avoid fragmentation
4. Use memory pools for frequent allocations
5. Profile memory usage

## Troubleshooting

### Out of Memory

**Symptoms**: Allocation failures, system slowdown

**Solutions**:
1. Check memory usage statistics
2. Look for memory leaks
3. Reduce memory footprint
4. Increase available memory

### Memory Corruption

**Symptoms**: Crashes, unexpected behavior

**Solutions**:
1. Enable memory debugging
2. Check for buffer overflows
3. Verify pointer arithmetic
4. Use guard pages

### Fragmentation

**Symptoms**: High memory usage, allocation failures

**Solutions**:
1. Use appropriate allocator
2. Reduce allocation size variance
3. Compact memory
4. Use memory pools

## References

- [Buddy System Algorithm](https://en.wikipedia.org/wiki/Buddy_memory_allocation)
- [Slab Allocator](https://en.wikipedia.org/wiki/Slab_allocation)
- [x86_64 Paging](https://wiki.osdev.org/Paging)
