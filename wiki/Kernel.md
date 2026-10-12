# Kernel Architecture & Subsystems

The **SigmaOS Kernel** is a microkernel-inspired, memory-safe, high-throughput operating system foundation built in native **Rust** (`no_std`) with boot and hardware initialization modules in **Zig**. It absorbs the best architectural concepts from Linux (CFS, EEVDF, eBPF, io_uring), FreeBSD (ULE scheduler, kqueue, GEOM), and OpenBSD (W^X, KARL ASLR).

---

## 1. Subsystem Architecture

### 1.1 Bare-Metal Bootstrap & Assembly Entry (`_start`)
- **Location**: `src/arch/x86_64/`, `src/arch/arm64/`, `src/arch/riscv64/`
- **Entry Protocol**: Implemented in clean Rust and Zig assembly stubs. Configures early Global Descriptor Table (GDT), Interrupt Descriptor Table (IDT), long mode (64-bit), and 4-level/5-level page tables before leaping into Rust `kernel_main`.
- **Zero BIOS/Legacy Dependence**: Supports modern UEFI PE/COFF Handover and Multiboot2 entry points with verifiable memory map handoff.

### 1.2 ELF Binary Loader
- **Location**: `src/loader/elf/`
- **Specification**: Complete 64-bit ELF executable and shared object loader. Parses `Elf64_Ehdr`, `Elf64_Phdr` (`PT_LOAD`, `PT_DYNAMIC`, `PT_INTERP`, `PT_TLS`), sets page protection flags (`PROT_READ`, `PROT_WRITE`, `PROT_EXEC`), and resolves dynamic relocations (`R_X86_64_RELATIVE`, `R_X86_64_GLOB_DAT`, `R_X86_64_JUMP_SLOT`).

### 1.3 Multi-Paradigm CPU Scheduling
- **Location**: `src/scheduler/`
- **Linux EEVDF (Earliest Eligible Virtual Deadline First)**: Default scheduler for low-latency interactive responsiveness. Tracks virtual runtime and eligible deadlines, eliminating legacy heuristics.
- **Linux CFS (Completely Fair Scheduler)**: Red-black tree virtual runtime fairness for throughput workloads.
- **FreeBSD ULE Scheduler**: Multi-queue interactive score calculation (`score = (run_ticks * MAX_SCORE) / sleep_ticks`) with CPU affinity caches for SMP scalability.

### 1.4 Transparent Huge Pages (THP) & `khugepaged` Defrag
- **Location**: `src/mm/khugepaged_defrag.rs`, `src/memory/`
- **Specification**: Autonomous background memory promotion daemon. Scans 4KB base pages within 2MB aligned virtual addresses, checks compaction feasibility, allocates contiguous 2MB physical frames, copies page contents, and atomic-swaps page directory entries (`PDE`) with zero userland disruption.

### 1.5 Signals & Asynchronous Interruption
- **Location**: `src/signal/`, `src/kernel/signal.rs`
- **Specification**: Complete POSIX and real-time signal subsystem (`SIGKILL`, `SIGSEGV`, `SIGALRM`, `SIGRTMIN`..`SIGRTMAX`). Features thread-level signal masks, pending queues, interruptible system call restart (`SA_RESTART`), and sigaltstack handling.

### 1.6 Kernel Sanitizers & Diagnostics
- **Location**: `src/kernel/kasan.rs`, `src/kernel/kmsan.rs`, `src/kernel/crash_dump.rs`, `src/kernel/serial_console.rs`
- **KASAN (Kernel Address Sanitizer)**: Shadow memory tracking (1 byte shadow for every 8 bytes memory). Catches use-after-free and out-of-bounds access.
- **KMSAN (Kernel Memory Sanitizer)**: Shadow and origin tracking for uninitialized memory reads.
- **Crash Dump & Serial Console**: Deterministic panic handler capturing CPU registers, stack traces, and physical memory dump via 16550 UART serial console.

---

## 2. Linux & BSD Distro Parity

| Feature | Linux 6.x | FreeBSD 14 | OpenBSD 7.x | SigmaOS Native Implementation |
|---|---|---|---|---|
| **Scheduler** | EEVDF / CFS | ULE | Multi-queue BSD | Unified EEVDF + ULE runtime selector |
| **THP Compaction** | `khugepaged` | Superpages | None | Native `khugepaged_defrag.rs` daemon |
| **Address Sanitizer** | KASAN | None | None | In-kernel `kasan.rs` shadow mapping |
| **Memory Sanitizer** | KMSAN | None | None | In-kernel `kmsan.rs` origin tracker |
| **Crash Logging** | kexec / kdump | textdump | ddb | Zero-allocation `crash_dump.rs` |

---

## ## AI Agent Maintenance Instructions for Future Releases

1. **Language Invariant**: The kernel core must remain **100% Rust (`no_std`)** or **Zig**. No C, C++, or assembly pre-binaries are permitted.
2. **Memory Safety**: Every `unsafe` block must be strictly justified with a `// SAFETY:` doc comment detailing memory layout, alignment, and lifetime soundness.
3. **Zero Predefined Dependencies**: Do not import external crates that pull in OS or libc dependencies (e.g., `spin`, `lazy_static`). Use standard core atomic primitives (`core::sync::atomic::{AtomicBool, AtomicU64, AtomicPtr}`).
4. **Validation Test Gate**: Any modification to scheduling, memory management, or syscall dispatching must pass:
   ```bash
   cargo check --lib
   cargo test --lib
   ```
5. **Deterministic Performance**: Hot paths must not allocate memory dynamically on the kernel heap. Use pre-allocated circular buffers and static slab allocators.
