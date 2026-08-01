# Core Kernel Architecture

This document covers the foundational kernel components of SigmaOS: memory management (physical & virtual), scheduling, and driver integration.

---

## Physical Memory: Buddy Allocator

SigmaOS uses a **Buddy Allocator** for physical frame management (`src/kernel/memory.rs`). It manages memory in power-of-two page blocks (4KB to 8MB), supporting:

- `allocate(size)` — Find and split the smallest available block.
- `deallocate(block)` — Return and attempt to merge with buddy blocks.
- `get_free_memory()` — Query total free physical memory.

---

## Virtual Memory Manager (Paging)

The VMM builds on top of physical memory with a fully OOP page table abstraction:

```rust
pub struct PageTableEntry(u64); // 64-bit x86 PTE
pub struct PageTable { entries: [PageTableEntry; 512] } // 4KB aligned, 512 entries
pub struct VirtualMemoryManager { root_directory: NonNull<PageTable> }
```

### PageFlags
| Flag | Bit | Purpose |
|---|---|---|
| `PRESENT` | 0 | Page is mapped |
| `WRITABLE` | 1 | Page is writable |
| `USER_ACCESSIBLE` | 2 | Accessible from ring 3 |
| `NO_EXECUTE` | 63 | Execute-disable |

### Key Methods
- `map_page(vaddr, paddr, flags)` — Maps a virtual to physical address
- `unmap_page(vaddr)` — Removes a mapping
- `translate(vaddr)` — Resolves a virtual address to physical

---

## Round-Robin Scheduler

The enhanced `RoundRobinScheduler` (`src/kernel/roundrobin.rs`) features:

- **Priority-aware time slices**: Higher-priority processes get proportionally more CPU time (Realtime: 8x, High: 4x, Normal: 2x, Low: 1x).
- **CPU Context tracking** via `CpuContext` — saves all 18 x86_64 registers.
- **Voluntary yielding** — A process can call `yield_current()` to immediately relinquish the CPU.
- **Context save/restore** — `save_context(rsp, rip)` and `restore_context()`.

---

## USB HID Keyboard Driver

`UsbHidDriver` handles USB HID protocol, while the new `HidKeyboard` struct integrates into the `PeripheralManager` via the `PeripheralDevice` trait.

**Key features:**
- Full US QWERTY HID scancode → ASCII translation table
- Shift modifier support
- LED state write (`NumLock`, `CapsLock`, `ScrollLock`)
- Event queue with `push_event` / `poll_event`

---

## VESA Framebuffer Driver

`VesaDriver` (`src/drivers/vesa.rs`) now implements `PeripheralDevice` and exposes:

- `write_pixel_raw(x, y, color)` — Writes a 32-bit ARGB pixel directly to the MMIO framebuffer
- `fill_screen(color)` — Fills the entire screen efficiently
- PeripheralDevice `write()` protocol: interprets 8-byte packets as `(x_hi, x_lo, y_hi, y_lo, r, g, b, a)` pixel writes
- PeripheralDevice `read()` returns current mode info (width, height, bpp)
