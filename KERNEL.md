# SigmaOS Kernel Documentation

This document covers the internals of the SigmaOS kernel.

***

## Table of Contents

1.  [Kernel Entry Point](#kernel-entry-point)
2.  [Memory Management](#memory-management)
3.  [Scheduler](#scheduler)
4.  [System Calls](#system-calls)
5.  [Interrupt Handling](#interrupt-handling)
6.  [Device Drivers](#device-drivers)
7.  [Virtual Filesystem (VFS)](#virtual-filesystem-vfs)
8.  [IPC](#ipc)
9.  [Process Model](#process-model)
10. [Kernel Hardening](#kernel-hardening)

***

## Kernel Entry Point

The kernel entry is at `src/kernel/main.rs`:

```rust
// src/kernel/main.rs (simplified)
#[no_std]
#[no_main]

fn sigma_main(boot_info: &BootInfo) -> ! {
    // 1. Initialise memory management
    memory::init(boot_info);

    // 2. Set up interrupt handlers
    irq::init();

    // 3. Initialise kernel subsystems
    scheduler::init();
    vfs::init();
    networking::init();
    security::init();

    // 4. Load initial userspace (init process)
    process::spawn_init();

    // 5. Enable interrupts and enter scheduler
    scheduler::run_forever()
}
```

Boot sequence:

1.  Bootloader (GRUB2 / sigma-boot) loads kernel to physical memory
2.  Early assembly stub sets up 64-bit mode, page tables, stack
3.  `sigma_main` initialises all subsystems in order
4.  PID 1 (sigma-init) is spawned in userspace

***

## Memory Management

### Physical Memory: BuddyAllocator

`src/klib/buddy_allocator.rs`

The buddy allocator manages physical pages. Pages are grouped in power-of-2 orders:

    Order 0: 4 KB  (1 page)
    Order 1: 8 KB  (2 pages)
    Order 2: 16 KB (4 pages)
    ...
    Order 9: 2 MB  (512 pages)
    Order 10: 4 MB (1024 pages)

**Allocation** (O(log n)):

1.  Find smallest order ≥ requested
2.  If exact order free list empty, split a higher-order block
3.  Return one half, put the "buddy" on its freelist

**Deallocation** (O(log n)):

1.  Mark page free at its order
2.  Check if buddy is also free
3.  If so, coalesce and recurse at order+1

```rust
use crate::klib::buddy_allocator::BuddyAllocator;

let mut alloc = BuddyAllocator::new(phys_start, phys_end);
let page = alloc.alloc_order(0)?;  // 4 KB
let big  = alloc.alloc_order(9)?;  // 2 MB
alloc.free(page, 0);
```

### Slab Allocator

`src/kernel/slab_allocator.rs`

On top of the buddy allocator, the slab allocator provides O(1) allocation for fixed-size kernel objects:

*   Each slab cache serves objects of one size
*   Objects are pre-constructed and cached between uses
*   Per-CPU magazines for lock-free hot-path alloc/free

```rust
// Create a cache for 64-byte kernel objects
let cache = SlabCache::new("proc_descriptor", 64);
let obj = cache.alloc()?;
cache.free(obj);
```

### Virtual Memory: Paging

`src/klib/paging.rs`

x86\_64 4-level page tables:

*   PML4 → PDPT → PD → PT → Physical page
*   Each table has 512 entries of 8 bytes
*   Maps 48-bit virtual address space (256 TB)

Key operations:

```rust
// Map virtual page to physical frame
paging::map_page(virt_addr, phys_addr, PageFlags::PRESENT | PageFlags::WRITABLE);

// Unmap and flush TLB
paging::unmap_page(virt_addr);
invlpg(virt_addr);
```

Hardware-enforced protections:

*   **NX/XD bit** — non-executable data pages (W^X)
*   **SMEP** — kernel cannot execute userspace pages
*   **SMAP** — kernel cannot read/write userspace without explicit STAC/CLAC
*   **PCID** — Process Context Identifiers (avoid full TLB flush on context switch)

### Custom Vec Optimisation

`src/klib/vec.rs` (from `bolt/vec-string-bulk-copy-opt` branch)

The standard `extend_from_slice` was updated to use `copy_from_slice` for bulk byte copies, avoiding element-by-element iteration:

```rust
// Before (slow):
for &item in slice {
    self.push(item);
}

// After (fast bulk copy):
let len = self.len();
self.reserve(slice.len());
unsafe {
    let dst = self.as_mut_ptr().add(len);
    dst.copy_from_nonoverlapping(slice.as_ptr(), slice.len());
    self.set_len(len + slice.len());
}
```

***

## Scheduler

`src/kernel/sched/`

SigmaOS uses a hybrid scheduler:

### CFS (Completely Fair Scheduler)

Normal processes use virtual runtime (vruntime) to ensure proportional CPU time:

    vruntime += actual_runtime × (NICE_0_LOAD / task_weight)

The task with the lowest vruntime runs next (red-black tree ordered by vruntime).

### EDF (Earliest Deadline First)

Real-time tasks use EDF: the task with the soonest deadline runs next. Guaranteed deadline meeting for tasks where `sum(utilisation) ≤ 1`.

### BORE Integration

CachyOS-inspired BORE (Burst-Oriented Response Enhancer):

*   Tracks burst score per task
*   Gives interactive tasks priority boost after CPU-bound bursts
*   Reduces scheduling latency for desktop workloads

### NUMA Scheduling

`src/kernel/numa_scheduler.rs`

*   Task placement prefers the NUMA node whose memory the task uses most
*   Migration threshold: only migrate if imbalance > 25%
*   NUMA balancing: periodic page migration to improve locality

***

## System Calls

`src/kernel/syscall/`

SigmaOS implements both a native Sigma syscall ABI and POSIX compatibility:

### Native Sigma Syscalls

    sigma_read(fd, buf, len) → ssize_t
    sigma_write(fd, buf, len) → ssize_t
    sigma_open(path, flags, mode) → fd
    sigma_close(fd) → void
    sigma_mmap(addr, len, prot, flags, fd, off) → *void
    sigma_pledge(promises, execpromises) → int
    sigma_unveil(path, permissions) → int
    sigma_cap_enter() → int
    sigma_cap_rights_limit(fd, rights) → int
    sigma_jail_create(path, hostname) → jailid
    sigma_spawn(path, args, env) → pid

### POSIX Compatibility Layer

POSIX syscalls are translated to Sigma equivalents:

*   `open(2)` → `sigma_open` with flag translation
*   `mmap(2)` → `sigma_mmap`
*   `clone(2)` → `sigma_spawn` with namespace flags

***

## Interrupt Handling

`src/kernel/irq/`

Interrupt Descriptor Table (IDT) entries for:

*   CPU exceptions (0–31): divide-by-zero, page fault, GP fault, etc.
*   Hardware IRQs (32–47): PIC/APIC-remapped hardware interrupts
*   Software interrupts (48+): syscall entry points

```rust
// Register a hardware IRQ handler
irq::register_handler(IRQ_TIMER, timer_interrupt_handler);
irq::register_handler(IRQ_KEYBOARD, keyboard_handler);
irq::register_handler(IRQ_NIC, network_interrupt_handler);
```

Page fault handler performs:

1.  Read CR2 (faulting address)
2.  Check if address is in a valid VMA
3.  If copy-on-write: allocate new page, copy, update PTE
4.  If stack growth: extend stack VMA
5.  Otherwise: deliver SIGSEGV to process

***

## Device Drivers

`src/drivers/` and `src/kernel/drivers/`

Driver interface (`src/kernel/driver.rs`):

```rust
pub trait SigmaDriver {
    fn probe(&mut self, device: &PciDevice) -> bool;
    fn init(&mut self) -> Result<(), DriverError>;
    fn read(&self, buf: &mut [u8]) -> Result<usize, DriverError>;
    fn write(&self, buf: &[u8]) -> Result<usize, DriverError>;
    fn ioctl(&mut self, cmd: u32, arg: usize) -> Result<i64, DriverError>;
}
```

Implemented drivers:

*   NVMe block device
*   virtio-net (QEMU network)
*   virtio-blk (QEMU disk)
*   USB HID keyboard/mouse (`src/klib/...`)
*   PCI scanner + BAR mapping
*   VGA framebuffer (direct)

***

## Virtual Filesystem (VFS)

`src/kernel/vfs/`

The VFS layer abstracts all filesystem operations:

```rust
pub trait FileSystem {
    fn lookup(&self, parent: InodeId, name: &str) -> Result<InodeId, VfsError>;
    fn read_dir(&self, dir: InodeId) -> Result<Vec<DirEntry>, VfsError>;
    fn open(&self, inode: InodeId, flags: OpenFlags) -> Result<FileHandle, VfsError>;
    fn read(&self, fh: &FileHandle, buf: &mut [u8], offset: u64) -> Result<usize, VfsError>;
    fn write(&self, fh: &FileHandle, buf: &[u8], offset: u64) -> Result<usize, VfsError>;
    fn create(&self, parent: InodeId, name: &str, mode: u32) -> Result<InodeId, VfsError>;
    fn unlink(&self, parent: InodeId, name: &str) -> Result<(), VfsError>;
}
```

Mount table management:

*   Each mount point stores: device, filesystem type, mount flags, superblock
*   Lookups traverse the dcache (directory entry cache) before hitting disk
*   Negative caching for fast "file not found" responses

***

## IPC

`src/kernel/ipc.rs`

SigmaOS provides multiple IPC mechanisms:

| Mechanism | Use Case | Latency |
|-----------|---------|---------|
| Capability channels | Microkernel message passing | ~1 µs |
| Unix sockets | POSIX compatibility | ~5 µs |
| Shared memory | Large data sharing | ~100 ns |
| io\_uring rings | Async I/O batching | ~500 ns |
| Signals | Async notifications | ~2 µs |

***

## Process Model

`src/kernel/process.rs`

Process descriptor fields:

*   `pid` — process ID
*   `ppid` — parent PID
*   `uid`, `gid` — credentials
*   `pledge_set` — declared syscall classes
*   `cap_mode` — Capsicum capability mode flag
*   `jail_id` — jail membership (0 = host)
*   `vmas` — virtual memory areas
*   `fd_table` — file descriptor table with capability rights

***

## Kernel Hardening

### KASLR

At boot, the kernel is loaded at a random physical base address (entropy from RDRAND or RDTSC). All internal pointers are offset accordingly.

### W^X Enforcement

No page is both writable and executable simultaneously. `mprotect(PROT_WRITE | PROT_EXEC)` returns `EACCES` by default.

### Retguard

On function entry:

1.  Save return address to a shadow stack page (randomly placed)
2.  XOR return address on stack with a per-process cookie

On function return:

1.  Verify return address matches shadow stack copy
2.  Mismatch → kernel panic (stack-smashing detected)

### Stack Canaries

All kernel functions compiled with `-fstack-protector-strong` equivalent (Rust's stack protection). Random 64-bit canary placed between local variables and saved return address.
