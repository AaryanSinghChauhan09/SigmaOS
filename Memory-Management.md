# SigmaOS Memory Management

The memory manager (`kernel/core/sigma_mm.rs`) provides physical and virtual
memory management with ASLR and W^X enforcement built in.

---

## Physical Memory: Buddy Allocator

Manages physical page frames in 2^n block sizes (4 KB to 8 MB).

```
Order 0:  1 page  =    4 KB
Order 1:  2 pages =    8 KB
Order 2:  4 pages =   16 KB
...
Order 10: 1024 pages = 4 MB
Order 11: 2048 pages = 8 MB (max block)
```

Operations:
- `alloc(order)` → returns physical frame number, O(log n) worst case
- `free(frame, order)` → coalesces with buddy if both free, O(log n)
- Automatic coalescing: when two buddies are both free → merged into larger block

```c
// Kernel C ABI
sigma_slab_init();              // initialize both buddy + slab
uint64_t free_pages = sigma_mm_free_pages();
uint64_t used_pages = sigma_mm_used_pages();
```

---

## Object Memory: Slab Allocator

Fast O(1) allocation for fixed-size kernel objects (8 bytes to 1024 bytes).

| Size Class | Objects/Slab | Use Case |
|-----------|-------------|---------|
| 8 bytes | 512 | Small flags, counters |
| 16 bytes | 512 | Short descriptors |
| 32 bytes | 512 | IPC messages |
| 64 bytes | 512 | File descriptors |
| 128 bytes | 512 | Task descriptors |
| 256 bytes | 512 | Network packets |
| 512 bytes | 512 | DMA headers |
| 1024 bytes | 512 | Large kernel objects |

```c
void* obj = sigma_slab_alloc(64);   // allocate 64-byte object
sigma_slab_free(obj);               // return to slab
```

---

## Virtual Memory: ASLR + W^X

Every `mmap()` call randomizes the base address with 42-bit entropy:

```
rand = xorshift64() & 0x3FF_FFFF_F000  (42-bit, page-aligned)
base = hint_base | rand
```

### W^X Enforcement

SigmaOS never allows a page to be both writable and executable:

```rust
// Attempted: exec+write → REJECTED
mmap(hint, size, PROT_WRITE | PROT_EXEC)  // → Err(MmError::WxViolation)

// Allowed: exec+read (no write)
mmap(hint, size, PROT_READ | PROT_EXEC)   // → Ok(randomized_addr)
```

This prevents a whole class of memory corruption exploits.

### Virtual Memory Areas

Each process has a `VmSpace` tracking up to 256 VMAs:

```rust
// Map a region (with ASLR + W^X check)
let addr = vm_space.mmap(hint, size, VmaPerm::Rx)?;  // read+exec

// Unmap
vm_space.munmap(addr, size);

// Page fault handler
vm_space.handle_page_fault(fault_addr, write_access)?;
```

---

## Memory Layout (x86-64)

```
0x0000_0000_0000_0000 ─ 0x0000_7FFF_FFFF_FFFF   User space (128 TB)
   ├── ASLR-randomized text, data, stack
   └── [stack top] → randomized per process

0xFFFF_8000_0000_0000 ─ 0xFFFF_FFFF_FFFF_FFFF   Kernel space (128 TB)
   ├── 0xFFFF_8000_0000_0000  Physical direct map
   ├── 0xC000_0000            Slab allocator pool
   ├── 0xFFFF_A000_0000_0000  Kernel text + data
   ├── 0xFFFF_C000_0000_0000  Kernel heap
   └── 0xFFFF_FF00_0000_0000  MMIO / device memory
```

---

## Performance Targets

| Operation | Target | Algorithm |
|-----------|--------|-----------|
| `slab_alloc(64)` | < 50 ns | O(1) free-list |
| `buddy_alloc(order=0)` | < 200 ns | O(log n) |
| `mmap(hint, size)` | < 500 ns | ASLR + VMA insert |
| Page fault (valid) | < 1 µs | VMA lookup |

---

## Source

`kernel/core/sigma_mm.rs` — 370 lines, `#![no_std]`, no external crates.

*See also: [Scheduler](Scheduler) · [Kernel Developer Handbook](../docs/KERNEL_DEVELOPER_HANDBOOK.md)*
