# SigmaOS Kernel Architecture

## Overview

SigmaOS is built on a **capability-based microkernel** architecture written entirely in `no_std` Rust. Unlike monolithic kernels (Linux) or hybrid kernels (macOS XNU), SigmaOS keeps the kernel minimal — only IPC, scheduling, memory management, and capability enforcement run in ring 0. All device drivers, filesystems, and protocol stacks run in isolated **S-SHARD** userspace containers.

```
┌────────────────────────────────────────────────────────┐
│                    User Applications                    │
├────────────────────────────────────────────────────────┤
│           S-SHARD Isolated Service Containers           │
│  ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────┐  │
│  │ Drivers  │ │   VFS    │ │  NetStk  │ │  SigPkg  │  │
│  └──────────┘ └──────────┘ └──────────┘ └──────────┘  │
├────────────────────────────────────────────────────────┤
│                  Sigma Microkernel                       │
│  ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────┐  │
│  │   IPC    │ │  Sched   │ │  VMM     │ │  CapDB   │  │
│  └──────────┘ └──────────┘ └──────────┘ └──────────┘  │
├────────────────────────────────────────────────────────┤
│                    Hardware (x86_64 / AArch64)          │
└────────────────────────────────────────────────────────┘
```

## Comparison with Other Kernels

| Feature | SigmaOS | Linux | FreeBSD | Fuchsia |
|---------|---------|-------|---------|---------|
| Architecture | Microkernel | Monolithic | Monolithic+Modules | Microkernel |
| Language | Rust (no_std) | C | C | Rust+C++ |
| Driver model | S-SHARD (userspace) | In-kernel / DKMS | KLD modules | Fuchsia DDK |
| IPC | Capability tokens | sockets/pipes | sockets/pipes | Zircon channels |
| Security model | Capabilities + post-quantum | DAC/MAC (SELinux) | MAC (TrustedBSD) | Capabilities |
| Memory safety | Rust ownership | Manual C | Manual C | Rust + C++ RAII |

## S-SHARD Isolation Model

**S-SHARD** (Sigma-Sovereign Hardware-Attributed Resource Domain) is SigmaOS's equivalent of microkernel servers. Each S-SHARD:

- Runs in ring 3 with its own virtual address space
- Communicates only through typed capability tokens
- Cannot access kernel memory or other SHARDs without explicit capability grant
- Uses IOMMU to prevent DMA attacks from hardware
- Terminated and restarted automatically on fault (like Fuchsia components)

This is inspired by:
- **L4 microkernel** server model (Picokernel IPC)
- **Fuchsia** component isolation
- **MINIX 3** driver isolation and self-healing

## Kernel Entry Points

### Syscall Interface

SigmaOS uses a **capability-gated syscall table**. Every syscall requires a valid capability token:

```rust
// src/kernel/syscall.rs
pub enum SigmaSyscall {
    // Process management
    SpawnProcess { cap: CapToken, binary: &'static [u8] },
    KillProcess  { cap: CapToken, pid: ProcessId },
    WaitProcess  { cap: CapToken, pid: ProcessId },

    // Memory management
    MapMemory    { cap: CapToken, addr: usize, size: usize, flags: MapFlags },
    UnmapMemory  { cap: CapToken, addr: usize, size: usize },

    // IPC
    SendMessage  { cap: CapToken, endpoint: EndpointId, msg: &'static [u8] },
    RecvMessage  { cap: CapToken, endpoint: EndpointId },

    // Capability management
    GrantCap     { cap: CapToken, target_pid: ProcessId, right: CapRight },
    RevokeCap    { cap: CapToken, right: CapRight },
}
```

### Interrupt Handling

The kernel uses a two-level interrupt model inspired by **FreeBSD's interrupt threads** and **Linux's softirq**:

1. **Top half** — minimal ISR, records event in ring buffer, ACKs PIC/APIC
2. **Bottom half** — deferred processing in kernel interrupt thread

```
Hardware IRQ → IDT Handler (top half, <1μs) → Interrupt Ring Buffer
                                                        ↓
                                          Interrupt Thread (bottom half)
                                                        ↓
                                           Driver S-SHARD notification
```

## Memory Layout

```
0xFFFFFFFF_FFFFFFFF ┬─ Kernel space (PML4 entry 511)
                    │   ├── Kernel code/data (mapped 1:1 with physical)
                    │   ├── Kernel heap (slab + buddy allocator)
                    │   ├── MMIO regions (per-device)
                    │   └── Interrupt stacks (per-CPU)
0xFFFF8000_00000000 ┘
0x00007FFF_FFFFFFFF ┬─ User space
                    │   ├── S-SHARD 0 (Drivers)     0x0000_4000_0000
                    │   ├── S-SHARD 1 (VFS)          0x0000_8000_0000
                    │   ├── S-SHARD 2 (Network)       0x0000_C000_0000
                    │   ├── User app heap             grows up
                    │   └── User stack                grows down
0x0000000000000000  ┘
```

## Boot Sequence

1. **UEFI/BIOS** → Bootloader (SigmaBoot)
2. **SigmaBoot** → Loads kernel ELF, sets up page tables, enters 64-bit long mode
3. **Kernel init** (`kernel_main`) →
   - Initializes GDT, IDT, TSS
   - Sets up physical memory map from UEFI memory descriptor
   - Initializes buddy allocator for physical frames
   - Initializes slab allocator for kernel objects
   - Sets up virtual memory (PML4 page tables)
   - Starts AP cores (SMP)
   - Spawns first S-SHARD (init server)
4. **Init server** → Starts driver SHARDs, VFS SHARD, network SHARD
5. **Login manager** → First userspace process

## IPC Design

SigmaOS IPC is inspired by **seL4's synchronous IPC** and **Mach ports**:

- **Synchronous fast path**: register-based message passing for small messages (<64 bytes) — zero kernel memory allocation
- **Async channels**: shared ring buffer with capability-protected endpoints for large data
- **Shared memory**: explicitly granted via capability, IOMMU-protected

```rust
// Fast path IPC — fits in registers, no allocation
pub struct FastMessage {
    pub label: u64,       // message type discriminant
    pub words: [u64; 7],  // 7 × 64-bit payload words
    pub cap_count: u8,     // number of capability grants
    pub caps: [CapToken; 3], // inline capability transfer
}
```

## Scheduling

See [SCHEDULER.md](SCHEDULER.md) for full details.

- **Default**: Weighted Round-Robin with priority inheritance
- **Real-time**: Earliest Deadline First (EDF) for RT processes
- **SMP**: Per-CPU run queues with work stealing (inspired by FreeBSD ULE and Linux CFS)

## Capability Database (CapDB)

The kernel maintains a per-process **capability space** (CSpace) — a radix-tree indexed by capability slot numbers. Inspired by **seL4 CSpace** and **Fuchsia handles**:

```
Process CSpace:
  Slot 0 → Endpoint capability (send to network SHARD)
  Slot 1 → Memory capability (mapped region 0x8000_0000, rw)
  Slot 2 → IRQ capability (IRQ line 10)
  Slot 3 → Derived from Slot 0 (send-only, no grant)
```

Rights supported: `Read`, `Write`, `Execute`, `Grant`, `Revoke`, `Seal`

## References

- [seL4 Reference Manual](https://sel4.systems/Info/Docs/seL4-manual-latest.pdf)
- [Fuchsia Zircon Kernel Concepts](https://fuchsia.dev/fuchsia-src/concepts/kernel)
- [FreeBSD Architecture Handbook](https://docs.freebsd.org/en/books/arch-handbook/)
- [L4 Picokernel Interface](https://os.inf.tu-dresden.de/L4/overview.html)
