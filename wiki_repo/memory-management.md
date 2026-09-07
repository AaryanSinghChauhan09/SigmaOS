# SigmaOS Memory Management

## Overview

SigmaOS memory management (`src/memory/`, `src/kernel/memory.rs`, `src/klib/paging.rs`) provides:

- Physical memory management via bitmap-based frame allocator
- Virtual address space management with 4-level page tables (x86_64)
- Kernel heap allocation via custom slab+buddy allocator
- Userspace heap via brk/mmap syscall interfaces
- NUMA-aware allocation policies

## Physical Memory Manager

### Frame Allocator (`src/klib/paging.rs`)

The physical frame allocator maintains a bitmap of all available physical frames (4KB each):

```rust
pub struct BitmapFrameAllocator {
    bitmap: &'static mut [u64],    // Each bit = one 4KB frame
    total_frames: usize,
    free_frames: usize,
    next_free_hint: usize,         // Speed up allocation
}

impl BitmapFrameAllocator {
    pub fn allocate(&mut self) -> Option<PhysAddr> { ... }
    pub fn allocate_contiguous(&mut self, n: usize) -> Option<PhysAddr> { ... }
    pub fn free(&mut self, addr: PhysAddr) { ... }
    pub fn free_count(&self) -> usize { self.free_frames }
}
```

### Memory Regions

At boot, the memory map (from UEFI/BIOS) is parsed to identify:

| Region Type | Description |
|------------|-------------|
| Available | Free RAM for kernel use |
| Reserved | BIOS/UEFI reserved regions |
| ACPI Reclaimable | ACPI tables (can be freed after ACPI init) |
| MMIO | Memory-mapped I/O (PCI BARs, etc.) |
| Framebuffer | Video memory |
| Kernel | Kernel code/data (permanent) |

## Virtual Memory Manager

### Page Table Layout (x86_64)

```
Virtual Address (48-bit canonical):
  Bits 47-39: PML4 index (512 entries)
  Bits 38-30: PDPT index (512 entries)
  Bits 29-21: PD index   (512 entries)
  Bits 20-12: PT index   (512 entries)
  Bits 11- 0: Page offset (4096 bytes)
```

### Page Flags

| Flag | Description |
|------|-------------|
| `PRESENT` | Page is mapped and valid |
| `WRITABLE` | Page can be written |
| `USER` | Accessible from user mode |
| `WRITE_THROUGH` | Write-through cache policy |
| `NO_CACHE` | Disable caching (MMIO) |
| `ACCESSED` | Set by CPU on access |
| `DIRTY` | Set by CPU on write |
| `HUGE_PAGE` | 2MB (PD) or 1GB (PDPT) page |
| `GLOBAL` | Page shared across all address spaces (kernel) |
| `NO_EXECUTE` | Instruction fetch not allowed (NX bit) |

### Address Space Layout

```
Kernel Virtual Address Space (upper half, canonical):
  0xFFFF800000000000 - 0xFFFF87FFFFFFFFFF  Physical memory direct map (8TB)
  0xFFFF888000000000 - 0xFFFF88FFFFFFFFFF  vmalloc/ioremap space
  0xFFFFC90000000000 - 0xFFFFE8FFFFFFFFFF  struct page array
  0xFFFFFE0000000000 - 0xFFFFFEFFFFFFFFFF  Kernel heap (slab)
  0xFFFFFFFF80000000 - 0xFFFFFFFFFFFFFFFF  Kernel text/data

User Virtual Address Space (lower half):
  0x0000000000001000 - 0x00007FFFFFFFFFFF  User space
  0x0000000000001000                        First valid user address
  0x0000700000000000 - 0x00007FFFFFFFFFFF  User stack (grows down)
  0x0000600000000000 - 0x00006FFFFFFFFFFF  mmap region
  0x0000000000400000 - 0x00005FFFFFFFFFFF  User heap (grows up)
```

## Kernel Heap Allocator

SigmaOS uses a two-tier kernel heap:

### Slab Allocator (Fixed-size objects)

For small, frequently-allocated objects (< 4KB):

```rust
pub struct SlabAllocator<const OBJECT_SIZE: usize> {
    partial_slabs: LinkedList<Slab>,
    full_slabs: LinkedList<Slab>,
    empty_slabs: LinkedList<Slab>,
}
```

Size classes: 8, 16, 32, 64, 128, 256, 512, 1024, 2048, 4096 bytes.

### Buddy Allocator (Large allocations)

For larger allocations (≥ 4KB), the buddy system provides O(log n) allocation:

```
Order 0: 4KB  (page)
Order 1: 8KB  (2 pages)
Order 2: 16KB (4 pages)
...
Order 10: 4MB (1024 pages)
```

## Memory Protection

### KASLR (Kernel Address Space Layout Randomization)

The kernel is loaded at a random base address at each boot to prevent address-based attacks. Entropy comes from the UEFI RNG protocol or RDRAND.

### SMEP and SMAP

- **SMEP** (Supervisor Mode Execution Prevention): Prevents kernel from executing user-space pages
- **SMAP** (Supervisor Mode Access Prevention): Prevents kernel from accessing user-space without explicit `stac`/`clac` instructions

### Memory Tagging (Planned)

ARM64 Memory Tagging Extension (MTE) support is planned for detecting use-after-free and buffer overflow errors at the hardware level.

## Huge Pages

SigmaOS supports 2MB and 1GB huge pages via Transparent Huge Pages (THP):

```rust
// Map a 2MB huge page
let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE | PageTableFlags::HUGE_PAGE;
mapper.map_2mb(page, frame_2mb, flags)?;
```

THP can be enabled/disabled per-process:
```bash
echo always > /sys/kernel/mm/transparent_hugepage/enabled
echo madvise > /sys/kernel/mm/transparent_hugepage/enabled
```

## Swap / Paging

SigmaOS supports swap space for memory overcommit:

1. Swap partition on disk (`/dev/sda3`)
2. Swap file on a filesystem
3. zswap — compressed in-memory swap cache
4. zram — compressed RAM block device

```bash
# Enable swap
sigma-swap enable /dev/sda3

# Check swap status
sigma-mem status
```

## Memory Debugging

### Kernel Address Sanitizer (KASAN)

In debug builds, KASAN detects out-of-bounds accesses and use-after-free:

```toml
# Cargo.toml
[profile.dev]
sanitize = ["address"]
```

### Memory Statistics

```rust
// Get kernel memory statistics
let stats = MEMORY_STATS.get();
println!("Total: {}MB, Free: {}MB, Cached: {}MB",
    stats.total_mb, stats.free_mb, stats.cached_mb);
```

### /proc/meminfo equivalent

```bash
sigma-sysinfo memory
# Output:
# MemTotal:    16384 MB
# MemFree:      8192 MB
# MemAvailable: 10240 MB
# Cached:        2048 MB
# SwapTotal:     4096 MB
# SwapFree:      4096 MB
```

## NUMA Support

For multi-socket systems, SigmaOS implements NUMA-aware allocation:

```rust
pub struct NumaPolicy {
    pub mode: NumaMode,
    pub preferred_node: Option<u8>,
    pub node_mask: u64,
}

pub enum NumaMode {
    Default,       // Allocate on current CPU's node
    Bind,          // Strictly allocate on specific node
    Interleave,    // Round-robin across nodes
    Preferred,     // Prefer a node, fall back if needed
}
```
