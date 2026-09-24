# SigmaOS Kernel Documentation

## Overview

The SigmaOS kernel (`src/kernel/`) is a microkernel-based operating system kernel written in Rust with `#![no_std]` at its core. It provides:

- **Capability-based security model** — every resource access requires an unforgeable capability token (inspired by seL4 and FreeBSD Capsicum)
- **BORE/EEVDF hybrid scheduler** — CachyOS-inspired scheduler with burst-oriented response enhancement and FreeBSD ULE interactivity scoring
- **Zero-copy IPC channels** — message passing without unnecessary memory copies, plus Android Binder handle translation
- **Memory isolation** — hardware-enforced process isolation via MMU/page tables, ASLR, KASLR, and KPTI
- **Cgroup v2 integration** — hierarchical resource control groups with CPU/memory quotas and IO weighting

---

## 🔍 Kernel Subsystem Gap Analysis & Strategic Roadmap

### Current Kernel Capabilities & Gaps
SigmaOS combines seL4 capability isolation, CachyOS BORE+EEVDF CPU scheduling, Linux cgroups v2, and OpenBSD zero-trust sandboxing (`pledge`/`unveil`). However, achieving enterprise-grade kernel maturity comparable to production Linux and BSD kernels requires addressing key gaps:

1. **Control Flow Integrity (CFI) & Shadow Stacks**: HardenedBSD and Linux CFI/IBT mitigate indirect jump hijacking. SigmaOS requires hardware CET shadow stack integration and Forward-Edge CFI checks.
2. **Zero-Downtime Livepatching**: Linux `kpatch`/`Ksplice` and FreeBSD `kld` dynamic patching support function redirection without rebooting. SigmaOS requires a live symbol redirection framework (`LinuxKernelLivepatchEngine`).
3. **Multi-ISA Hardware HAL Expansion**: While x86_64 long mode is well-supported, ARM64 (aarch64 MMU/GICv3) and RISC-V 64 (Sv39/Sv48 MMU & PLIC) HAL shards require full hardware initialization.
4. **eBPF Kernel Verifier Hardening**: Linux eBPF provides JIT compilation and static verification. SigmaOS requires strict register type tracking and pointer bounds checks in the eBPF runtime (`EbpfRuntime` / `SovereignEbpfJitCompiler`).
5. **Formal Verification Proofs**: Inspired by seL4, capability grant/revoke state transitions (`CapabilityDerivationTree`) require automated formal proofs for mathematically proven non-interference.
6. **Hard Real-Time Latency Guarantees**: Integrating `SCHED_DEADLINE` and lockless RCU read-side critical sections (`LinuxRcuSynchronizationEngine`) to guarantee <10µs maximum scheduling latency for robotics and edge computing.

---

## 🔍 Universal Kernel Format Engine (Linux & BSD Parity)

The SigmaOS Universal Kernel Format Engine (`src/kernel/universal_kernel_format.rs`) provides multi-OS format detection, header parsing, section mapping, initramfs extraction, and transpilation across 20+ kernel formats:

- **Linux Formats**: bzImage, compressed vmlinuz (Gzip, Zstd, XZ, LZ4, LZO), U-Boot uImage, FIT Image (Flattened Image Tree), Unified Kernel Image (UKI PE/COFF), Android boot.img
- **BSD Formats**: FreeBSD dynamic ELF kernel & KLD modules, NetBSD Multiboot/ELF kernel & rump payload, OpenBSD bsd/bsd.rd rescue kernel, DragonFly BSD Virtual Kernel (vkernel)
- **Hypervisor & Multi-OS**: Xen PV/PVH guest kernel, Apple Darwin Mach-O kernelcache, Windows NT Executive (`ntoskrnl.exe`), Haiku ELF kernel, Redox microkernel, Solaris/Illumos unix ELF kernel

---

## 🔍 Kernel Ring Buffer (`dmesg`) Logging Subsystem Gap Analysis & Strategic Roadmap

### Current Capabilities & Logging Gaps
SigmaOS kernel logging provides circular ring buffer logging (`DmesgLog` / `KernelDmesgEntry`), pre-populated boot logs, and security sysctl restrictions (`kernel.dmesg_restrict = 1`). Achieving full parity with mature Linux (`dmesg`, `klogctl`, syslog facilities) and BSD (`dmesg`, `kern.bootctl`) logging frameworks requires addressing key gaps:

1. **Lock-Free Atomic Ring Buffer Overhead**: Expanding the kernel circular buffer into an atomic lock-free power-of-two ring buffer to prevent printk lock contention during high-frequency IRQ storms.
2. **`klogctl` Syscall Parity**: Implementing Linux `klogctl` system call operations (`SYSLOG_ACTION_READ`, `SYSLOG_ACTION_READ_ALL`, `SYSLOG_ACTION_CLEAR`, `SYSLOG_ACTION_CONSOLE_OFF`, `SYSLOG_ACTION_SIZE_BUFFER`).
3. **`dmesg` CLI Feature Parity**: Supporting `-w` (live follow stream), `-c` (read and clear), `-k`/`-u` (facility filter for kernel vs userland), `-H` (human-readable delta timestamps), and `-L` (colorized log levels).
4. **Syslog Facility & Severity Matrix**: Encoding standard syslog facilities (kern, user, mail, daemon, auth, syslog, lpr) and severity levels (0-Emergency to 7-Debug) in every `dmesg` message entry.
5. **Persistent Panic Logs & NVRAM Dump**: Preserving post-mortem kernel panic crash logs across soft reboots and hardware Machine Check Exceptions (MCE) via non-volatile RAM (NVRAM) or Pstore buffer dumps.

---

### 📊 `dmesg` Logging Gap Dashboard

| Logging Feature | Current State (SigmaOS) | Target State (Linux `dmesg`/`klogctl` & BSD) |
|---|---|---|
| **Ring Buffer Storage** | `DmesgLog` / `KernelDmesgEntry` | Atomic lock-free power-of-two circular ring buffer |
| **Syscall Interface** | Custom logger API | `klogctl` / `syslog(2)` POSIX syscall parity |
| **Security Controls** | `kernel.dmesg_restrict` rule | Capability-bounded `kernel.dmesg_restrict` sysctl enforcement |
| **CLI Options** | Basic log list output | `dmesg -w` follow, `-c` clear, `-k`/`-u` facility filters, `-L` colors |
| **Crash Persistence** | In-memory log buffer | NVRAM / Pstore panic dump preservation across reboots |

---

### 🚀 3-Phase `dmesg` Logging Development Roadmap

#### Phase 1: Circular Ring Buffer & Security Restriction (0–6 Months)
- **Lock-Free Ring Buffer**: Build atomic circular ring buffer storing `[  0.000000]` boot-relative timestamped log entries.
- **`klogctl` Syscall Parity**: Implement `syslog(2)` / `klogctl` syscall operations for reading, clearing, and sizing the log buffer.
- **`kernel.dmesg_restrict` Enforcement**: Block unprivileged userspace read access to `dmesg` unless `CAP_SYS_ADMIN` is held.

#### Phase 2: CLI Options Parity & Facility Filtering (6–12 Months)
- **Follow & Clear Modes**: Implement `dmesg -w` (live log stream follow) and `dmesg -c` (read-and-clear).
- **Facility & Severity Filter**: Support `-k` (kernel messages), `-u` (userland messages), and severity level filtering.
- **Colorized Formatting**: Add `dmesg -L` colorized log level output (red for emergency/panic, yellow for warning, green for info).

#### Phase 3: Persistent Panic Logs & NVRAM Dump (12–18 Months)
- **NVRAM Panic Dump**: Automatically flush `dmesg` ring buffer to NVRAM / EFI pstore upon kernel panic or MCE hardware fault.
- **PII Scrubbing**: Sanitize sensitive memory addresses and keys from public `dmesg` bug report outputs.
- **eBPF Log Subscribers**: Allow unprivileged eBPF probes to subscribe to specific kernel log event channels.

---

---

## 🔍 Kernel Subsystem Gap Analysis & Strategic Roadmap

### Current Kernel Capabilities & Gaps
SigmaOS combines seL4 capability isolation, CachyOS BORE+EEVDF CPU scheduling, and OpenBSD zero-trust sandboxing. However, achieving enterprise-grade kernel maturity comparable to production Linux and BSD kernels requires addressing key gaps:

1. **Control Flow Integrity (CFI) & Shadow Stacks**: HardenedBSD and Linux CFI/IBT mitigate indirect jump hijacking. SigmaOS requires hardware CET shadow stack integration and Forward-Edge CFI checks.
2. **Zero-Downtime Livepatching**: Linux `kpatch`/`Ksplice` and FreeBSD `kld` dynamic patching support function redirection without rebooting. SigmaOS requires a live symbol redirection framework.
3. **Multi-ISA Hardware HAL Expansion**: While x86_64 long mode is well-supported, ARM64 (aarch64 MMU/GICv3) and RISC-V 64 (Sv39/Sv48 MMU & PLIC) HAL shards require full hardware initialization.
4. **eBPF Kernel Verifier Hardening**: Linux eBPF provides JIT compilation and static verification. SigmaOS requires strict register type tracking and pointer bounds checks in the eBPF runtime.
5. **Formal Verification Proofs**: Inspired by seL4, capability grant/revoke state transitions require automated formal proofs for mathematically proven non-interference.
6. **Hard Real-Time Latency Guarantees**: Integrating `SCHED_DEADLINE` and lockless RCU read-side critical sections to guarantee <10µs maximum scheduling latency for robotics and edge computing.

---

## 🔍 Kernel Ring Buffer (`dmesg`) Logging Subsystem Gap Analysis & Strategic Roadmap

### Current Capabilities & Logging Gaps
SigmaOS kernel logging (`src/logging/logger.rs`, `src/security/kali_stack.rs`, `src/tools/sovereign_commands.rs`) provides circular ring buffer logging (`DmesgLog` / `KernelDmesgEntry`), pre-populated boot logs, and security sysctl restrictions (`kernel.dmesg_restrict = 1`). Achieving full parity with mature Linux (`dmesg`, `klogctl`, syslog facilities) and BSD (`dmesg`, `kern.bootctl`) logging frameworks requires addressing key gaps:

1. **Lock-Free Atomic Ring Buffer Overhead**: Expanding the kernel circular buffer into an atomic lock-free power-of-two ring buffer to prevent printk lock contention during high-frequency IRQ storms.
2. **`klogctl` Syscall Parity**: Implementing Linux `klogctl` system call operations (`SYSLOG_ACTION_READ`, `SYSLOG_ACTION_READ_ALL`, `SYSLOG_ACTION_CLEAR`, `SYSLOG_ACTION_CONSOLE_OFF`, `SYSLOG_ACTION_SIZE_BUFFER`).
3. **`dmesg` CLI Feature Parity**: Supporting `-w` (live follow stream), `-c` (read and clear), `-k`/`-u` (facility filter for kernel vs userland), `-H` (human-readable delta timestamps), and `-L` (colorized log levels).
4. **Syslog Facility & Severity Matrix**: Encoding standard syslog facilities (kern, user, mail, daemon, auth, syslog, lpr) and severity levels (0-Emergency to 7-Debug) in every `dmesg` message entry.
5. **Persistent Panic Logs & NVRAM Dump**: Preserving post-mortem kernel panic crash logs across soft reboots and hardware Machine Check Exceptions (MCE) via non-volatile RAM (NVRAM) or Pstore buffer dumps.

---

### 📊 `dmesg` Logging Gap Dashboard

| Logging Feature | Current State (SigmaOS) | Target State (Linux `dmesg`/`klogctl` & BSD) |
|---|---|---|
| **Ring Buffer Storage** | `DmesgLog` / `KernelDmesgEntry` | Atomic lock-free power-of-two circular ring buffer |
| **Syscall Interface** | Custom logger API | `klogctl` / `syslog(2)` POSIX syscall parity |
| **Security Controls** | `kernel.dmesg_restrict` rule | Capability-bounded `kernel.dmesg_restrict` sysctl enforcement |
| **CLI Options** | Basic log list output | `dmesg -w` follow, `-c` clear, `-k`/`-u` facility filters, `-L` colors |
| **Crash Persistence** | In-memory log buffer | NVRAM / Pstore panic dump preservation across reboots |

---

### 🚀 3-Phase `dmesg` Logging Development Roadmap

#### Phase 1: Circular Ring Buffer & Security Restriction (0–6 Months)
- **Lock-Free Ring Buffer**: Build atomic circular ring buffer storing `[  0.000000]` boot-relative timestamped log entries.
- **`klogctl` Syscall Parity**: Implement `syslog(2)` / `klogctl` syscall operations for reading, clearing, and sizing the log buffer.
- **`kernel.dmesg_restrict` Enforcement**: Block unprivileged userspace read access to `dmesg` unless `CAP_SYS_ADMIN` is held.

#### Phase 2: CLI Options Parity & Facility Filtering (6–12 Months)
- **Follow & Clear Modes**: Implement `dmesg -w` (live log stream follow) and `dmesg -c` (read-and-clear).
- **Facility & Severity Filter**: Support `-k` (kernel messages), `-u` (userland messages), and severity level filtering.
- **Colorized Formatting**: Add `dmesg -L` colorized log level output (red for emergency/panic, yellow for warning, green for info).

#### Phase 3: Persistent Panic Logs & NVRAM Dump (12–18 Months)
- **NVRAM Panic Dump**: Automatically flush `dmesg` ring buffer to NVRAM / EFI pstore upon kernel panic or MCE hardware fault.
- **PII Scrubbing**: Sanitize sensitive memory addresses and keys from public `dmesg` bug report outputs.
- **eBPF Log Subscribers**: Allow unprivileged eBPF probes to subscribe to specific kernel log event channels.

---

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

### Process Scheduler (`src/kernel/scheduler.rs`, `src/kernel/bore.rs`)

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

### Capability System (`src/kernel/linux_bsd_innovations.rs`)

Every kernel resource (file, device, process) is accessed via unforgeable capability tokens:

| Capability Type | Description |
|----------------|-------------|
| `MemoryCap` | Access to a physical memory frame |
| `EndpointCap` | Send/receive on IPC channel |
| `ThreadCap` | Control a kernel thread |
| `DeviceCap` | Access hardware device MMIO |
| `IrqCap` | Register an interrupt handler |
| `FrameCap` | Map a physical frame |

### Interrupt Handling (`src/kernel/irq/`)

- **APIC/xAPIC** — Advanced Programmable Interrupt Controller
- **MSI/MSI-X** — Message-Signaled Interrupts for PCIe devices
- **Interrupt coalescing** — batch processing for high-rate interrupts
- **Deferred processing** — top-half/bottom-half split (`BottomHalfKernelThread`)

### System Calls (`src/kernel/syscall/`)

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

SigmaOS implements MAC at the kernel level via LSM (Linux Security Module) compatible hooks and OpenBSD pledge/unveil restrictions:

- **Inode access hooks** — enforce file access policy (`LinuxLandlockLsmRuleEngine`)
- **Process creation hooks** — validate new process context
- **Network socket hooks** — enforce network policy (`BsdPfStateTable`)
- **Capability mode** — FreeBSD Capsicum capability mode enforcement (`FreeBsdCapsicumEngine`)

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

1. Add syscall number to `src/kernel/syscall/numbers.rs`
2. Implement handler in `src/kernel/syscall/handlers.rs`
3. Add to dispatch table in `src/kernel/syscall/dispatch.rs`
4. Write tests in `tests/`
5. Document in `docs/api-reference.md`

### Adding a Kernel Module

1. Create `src/kernel/my_module.rs`
2. Add `pub mod my_module;` to `src/kernel/mod.rs`
3. Implement the `KernelModule` trait
4. Register in `src/kernel/main.rs`

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
- [FreeBSD Capsicum Framework](https://www.freebsd.org/cgi/man.cgi?query=capsicum&sektion=4)
- [OpenBSD pledge(2) & unveil(2)](https://man.openbsd.org/pledge.2)
- [x86_64 Architecture Manual](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html)
