# SigmaOS Kernel Documentation

## Overview

The SigmaOS kernel (`src/kernel/`) is a microkernel-based operating system kernel written in Rust with `#![no_std]` at its core. It provides:

- **Capability-based security model** — every resource access requires an unforgeable capability token
- **BORE/EEVDF hybrid scheduler** — CachyOS-inspired scheduler with burst-oriented response enhancement
- **Zero-copy IPC channels** — message passing without unnecessary memory copies
- **Memory isolation** — hardware-enforced process isolation via MMU/page tables
- **Cgroup v2 integration** — hierarchical resource control groups

## Architecture

```
┌─────────────────────────────────────────────────────┐
│                   User Space                        │
│  Applications │ Shell │ Package Mgr │ Desktop       │
└──────────────────────┬──────────────────────────────┘
                       │ Syscall Interface
┌──────────────────────▼──────────────────────────────┐
│                  Microkernel                        │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐ │
│  │  Scheduler  │  │  IPC/Caps   │  │    MMU      │ │
│  └─────────────┘  └─────────────┘  └─────────────┘ │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐ │
│  │   Cgroups   │  │  Interrupts │  │  Syscalls   │ │
│  └─────────────┘  └─────────────┘  └─────────────┘ │
└──────────────────────┬──────────────────────────────┘
                       │ Hardware Abstraction Layer
┌──────────────────────▼──────────────────────────────┐
│               Hardware (x86_64/aarch64/riscv64)     │
└─────────────────────────────────────────────────────┘
```

## Key Components

### Process Scheduler (`src/kernel/scheduler.rs`)

SigmaOS uses a hybrid BORE (Burst-Oriented Response Enhancer) + EEVDF (Earliest Eligible Virtual Deadline First) scheduler inspired by CachyOS:

- **EEVDF** provides fair CPU time distribution with deadline-aware scheduling
- **BORE extension** enhances responsiveness for interactive workloads
- **Real-time support** — configurable SCHED_FIFO/SCHED_RR priority classes
- **CPU affinity** — pin threads to specific cores

```rust
// Example: Setting scheduler policy
use sigma::kernel::scheduler::{SchedulerPolicy, ProcessPriority};

let policy = SchedulerPolicy::new()
    .with_algorithm(SchedAlgorithm::EevdfBore)
    .with_priority(ProcessPriority::Interactive)
    .with_time_slice_us(1000); // 1ms time slice
```

### Memory Management (`src/kernel/memory.rs`, `src/memory/`)

- **4-level page tables** (PML4 → PDPT → PD → PT) on x86_64
- **ASLR** — Address Space Layout Randomization enabled by default
- **KASLR** — Kernel ASLR for kernel text/data regions
- **Huge pages** — 2MB/1GB transparent huge page support
- **NUMA awareness** — node-local allocation preference
- **Slab allocator** — O(1) fixed-size object allocation

```rust
// Physical memory allocation
let frame = FRAME_ALLOCATOR.allocate_frame()?;

// Virtual memory mapping
let page = Page::containing_address(virt_addr);
let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE | PageTableFlags::NO_EXECUTE;
mapper.map_to(page, frame, flags, &mut frame_allocator)?;
```

### Inter-Process Communication (`src/kernel/ipc.rs`)

SigmaOS IPC is built on **capability-based channels**:

- **Synchronous calls** — blocking request/reply (like seL4 IPC)
- **Asynchronous notifications** — non-blocking event delivery
- **Shared memory regions** — zero-copy large data transfer
- **Capability delegation** — pass capabilities through IPC

```rust
// Creating an IPC endpoint
let endpoint = IpcEndpoint::create(KERNEL_CAPABILITY)?;

// Sending a message
endpoint.send(IpcMessage {
    label: MSG_READ,
    data: [0u64; 4],
    caps: [capability],
})?;

// Receiving a message
let msg = endpoint.recv()?;
```

### Capability System (`src/kernel/capabilities.rs`)

Every kernel resource (file, device, process) is accessed via unforgeable capability tokens:

| Capability Type | Description |
|----------------|-------------|
| `MemoryCap` | Access to a physical memory frame |
| `EndpointCap` | Send/receive on IPC channel |
| `ThreadCap` | Control a kernel thread |
| `DeviceCap` | Access hardware device MMIO |
| `IrqCap` | Register an interrupt handler |
| `FrameCap` | Map a physical frame |

### Interrupt Handling (`src/kernel/interrupts.rs`)

- **APIC/xAPIC** — Advanced Programmable Interrupt Controller
- **MSI/MSI-X** — Message-Signaled Interrupts for PCIe devices
- **Interrupt coalescing** — batch processing for high-rate interrupts
- **Deferred processing** — top-half/bottom-half split

### System Calls (`src/syscall/`)

SigmaOS uses a **fast syscall interface** (SYSCALL/SYSRET on x86_64):

| Syscall | Number | Description |
|---------|--------|-------------|
| `sigma_ipc_send` | 0 | Send IPC message |
| `sigma_ipc_recv` | 1 | Receive IPC message |
| `sigma_cap_invoke` | 2 | Invoke a capability |
| `sigma_thread_create` | 3 | Create new thread |
| `sigma_memory_map` | 4 | Map memory region |
| `sigma_yield` | 5 | Yield CPU timeslice |
| `sigma_exit` | 6 | Terminate current thread |
| `sigma_debug` | 7 | Debug output (debug builds) |

## Boot Process

```
1. UEFI Firmware
   └── 2. SigmaOS Bootloader (sigma-boot)
       ├── Load kernel ELF
       ├── Set up initial page tables
       ├── Switch to long mode (x86_64)
       └── 3. Kernel Entry Point (start64)
           ├── Initialize BSS segment
           ├── Set up GDT/IDT
           ├── Initialize APIC
           ├── Start memory manager
           ├── Start scheduler
           └── 4. Init Process (PID 1: sigma-init)
               ├── Mount root filesystem
               ├── Start system services
               └── Launch user session
```

## Kernel Parameters

Kernel parameters can be set at boot via GRUB/UEFI boot args or `sigma.toml`:

| Parameter | Default | Description |
|-----------|---------|-------------|
| `sigma.heap_size` | `64M` | Kernel heap size |
| `sigma.max_procs` | `65536` | Max concurrent processes |
| `sigma.scheduler` | `eevdf-bore` | Scheduler algorithm |
| `sigma.kaslr` | `true` | Enable KASLR |
| `sigma.debug` | `false` | Enable kernel debug output |
| `sigma.loglevel` | `4` | Log verbosity (0-7) |

## Security Features

### Mandatory Access Control (MAC)

SigmaOS implements MAC at the kernel level via LSM (Linux Security Module) compatible hooks:

- **Inode access hooks** — enforce file access policy
- **Process creation hooks** — validate new process context
- **Network socket hooks** — enforce network policy
- **IPC hooks** — control inter-process communication

### Exploit Mitigations

| Mitigation | Status | Description |
|------------|--------|-------------|
| KASLR | ✅ | Kernel address space randomization |
| SMEP | ✅ | Supervisor mode execution prevention |
| SMAP | ✅ | Supervisor mode access prevention |
| KPTI | ✅ | Kernel page-table isolation (Meltdown) |
| Stack canaries | ✅ | Stack overflow detection |
| RELRO | ✅ | Read-only GOT after relocation |
| PIE | ✅ | Position-independent executable kernel |
| CFI | ⬜ | Control-flow integrity (planned) |
| Shadow stacks | ⬜ | CET shadow stack support (planned) |

## Kernel Development

See [Kernel Development](../docs/kernel.md) and [CONTRIBUTING.md](../CONTRIBUTING.md) for guidelines.

### Adding a Syscall

1. Add syscall number to `src/syscall/numbers.rs`
2. Implement handler in `src/syscall/handlers.rs`
3. Add to dispatch table in `src/syscall/dispatcher.rs`
4. Write tests in `tests/`
5. Document in `docs/api-reference.md`

### Adding a Kernel Module

1. Create `src/kernel/my_module.rs`
2. Add `pub mod my_module;` to `src/kernel/mod.rs`
3. Implement the `KernelModule` trait
4. Register in `src/kernel/init.rs`

## Testing

```bash
# Run kernel unit tests
cargo test --package sigma-kernel

# Run kernel integration tests
bash run_sigma_tests.sh

# Run with QEMU
make run-tests
```

## References

- [seL4 Microkernel Formal Verification](https://sel4.systems/)
- [EEVDF Scheduler Paper](https://citeseerx.ist.psu.edu/document?repid=rep1&type=pdf&doi=805acf7726282723e7deff18527a37f5082e3c7f)
- [CachyOS BORE Scheduler](https://github.com/cachyos/kernel-patches)
- [x86_64 Architecture Manual](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html)
