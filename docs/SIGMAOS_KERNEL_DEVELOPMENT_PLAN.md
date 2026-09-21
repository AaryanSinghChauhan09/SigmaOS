# SigmaOS Kernel Subsystem (`Sigma-Kernel`) - Master Development Plan

## 1. Executive Summary & Vision

`Sigma-Kernel` is the native, memory-safe, hybrid monolithic/microkernel core of **SigmaOS**. Built in 100% pure Rust (`#![no_std]`), it combines the raw performance and rich subsystem ecosystem of Linux (6.8+ eBPF JIT, `io_uring` SQPOLL, EEVDF/BORE schedulers, Preempt-RT) with the modular elegance and security hardening of the BSD family (FreeBSD `kqueue`/`vnet`/Capsicum, OpenBSD W^X/KARL/`pledge`/`unveil`, DragonFly LWKT kthreads, and NetBSD Rump Kernel user-space driver sandboxing).

---

## 2. Inspirations from Linux & BSD Kernel Ecosystems

| Kernel Origin | Subsystem & Capability Absorbed | Target Subsystem / Module |
| :--- | :--- | :--- |
| **Linux Kernel 6.8+** | eBPF JIT compiler & XDP packet engine, `io_uring` SQPOLL zero-copy async I/O, EEVDF & BORE schedulers, Preempt-RT real-time locks, `cgroup v2`, SLUB slab allocator, `ftrace`, Linux Livepatching. | `src/kernel/` (`ebpf.rs`, `io_uring.rs`, `eevdf_sovereign.rs`, `bore.rs`, `livepatch.rs`) |
| **FreeBSD Kernel** | Scalable `kqueue`/`kevent` event notification engine, `vnet` virtualized network stack instances, Capsicum capability rights, Universal Memory Allocator (UMA), `bhyve` microvm hypervisor gateway. | `src/kernel/` (`kqueue.rs`, `bsd_kernel_parity.rs`, `hypervisor.rs`) |
| **OpenBSD Kernel** | W^X (Write-or-Execute) memory protection, KARL (Kernel Address Randomized Link), PINSYSCALL hardening, `pledge`/`unveil` syscall sandboxing. | `src/security/` (`pledge.rs`, `landlock.rs`, `kernel_hardening.rs`) |
| **DragonFly BSD Kernel** | Light Weight Kernel Threads (LWKT) per-CPU lockless scheduling, lock-free IPC ring buffers, Variant Symlinks. | `src/kernel/` (`sigma_kthread.rs`, `ipc.rs`) |
| **NetBSD Kernel** | Rump Kernels (sanitizing and executing device drivers in isolated user-space memory partitions for zero-crash kernel stability). | `src/kernel/` (`breakthroughs.rs`, `linux_bsd_innovations.rs`) |

---

## 3. 5-Layer Hybrid Kernel Architecture

```
┌────────────────────────────────────────────────────────────────────────┐
│ Layer 5: NetBSD Rump Isolation, Linux Livepatching & OpenBSD Hardening │
├────────────────────────────────────────────────────────────────────────┤
│ Layer 4: `io_uring` SQPOLL, FreeBSD `kqueue`, eBPF/XDP & VNET Fabric  │
├────────────────────────────────────────────────────────────────────────┤
│ Layer 3: EEVDF + BORE Scheduler, DragonFly LWKT & `cgroup v2` Engine   │
├────────────────────────────────────────────────────────────────────────┤
│ Layer 2: SLUB / UMA Memory Allocator, KSM, NUMA & W^X/KARL Protection  │
├────────────────────────────────────────────────────────────────────────┤
│ Layer 1: Hardware Abstraction Layer (HAL) & Multi-Arch Boot Core       │
└────────────────────────────────────────────────────────────────────────┘
```

### Layer 1: Hardware Abstraction Layer (HAL) & Multi-Arch Boot Core
- **14 CPU Target Architectures:** `X86_64`, `AArch64`, `Riscv64`, `LoongArch64`, `Ppc64Le`, `Mips64`, `S390x`, `Sparc64`, `Armv7`, `Riscv32`, `Sh4`, `Alpha`, `M68k`, `X86`.
- **Interrupt Controller Drivers:** APIC/IOAPIC, ARM GICv2/v3, RISC-V PLIC/CLINT, LoongArch ExtIOI, PowerPC XIVE.
- **Boot Protocol:** Multiboot2, EFI handover, and FDT DeviceTree hardware autoprobing.

### Layer 2: Memory Management & Allocator Core
- **SLUB / UMA Allocator:** Lockless per-CPU slab caches for kernel objects (`kmem_cache`).
- **KSM & NUMA:** Kernel Samepage Merging (KSM) de-duplication and NUMA-aware page placement algorithms.
- **W^X & KARL Hardening:** Enforcing strict Write-XOR-Execute memory permissions and relinking the kernel binary layout at every boot (KARL).

### Layer 3: Scheduler & Process Threading Engine
- **EEVDF + BORE Scheduler:** Earliest Eligible Virtual Deadline First (EEVDF) combined with Burst-Oriented Response Extension (BORE) for ultra-responsive desktop interactivity and high-throughput server workloads.
- **Preempt-RT Real-Time Core:** Low-latency preemption locks for audio/video DSP tasks.
- **DragonFly LWKT Threads:** Per-CPU Light Weight Kernel Threads operating without global spinlock contention.
- **Resource Control (`cgroup v2`):** Unified hierarchical accounting for CPU, Memory, I/O, and Process ID limits.

### Layer 4: Async I/O, Event & Network Subsystem
- **`io_uring` SQPOLL:** Zero-syscall submission/completion queue async I/O engine.
- **FreeBSD `kqueue`/`kevent`:** Scalable event multiplexing across sockets, files, signals, and process state changes.
- **eBPF JIT & XDP Engine:** In-kernel eBPF bytecode JIT compiler and Express Data Path (XDP) network packet processor.
- **FreeBSD VNET Virtualization:** Fully virtualized network stack instances isolated per container or jail.

### Layer 5: Security, Livepatching & Microkernel Isolation
- **NetBSD Rump Kernel Driver Isolation:** Fault-intolerant drivers executed in isolated userspace memory domains to prevent driver crashes from panicking the kernel.
- **Linux Livepatching:** Atomic function redirection via `ftrace` trampoline hooks, applying kernel security patches without rebooting.
- **OpenBSD `pledge`/`unveil` & Capsicum:** Granular process syscall filtering and file descriptor capability rights.

---

## 4. Implementation Roadmap

| Milestone | Target Phase | Objectives | Status |
| :--- | :--- | :--- | :--- |
| **Milestone 1** | Multi-Arch HAL | Implement Multi-Arch bootloader protocol, APIC/GIC/PLIC interrupt dispatchers, and paging in `src/hal/`. | Implemented |
| **Milestone 2** | Memory & Sched | Implement SLUB/UMA allocators, EEVDF + BORE scheduler, and `cgroup v2` controllers. | Implemented |
| **Milestone 3** | Async I/O & eBPF | Implement `io_uring` SQPOLL engine, FreeBSD `kqueue`, eBPF JIT compiler, and XDP packet fabric. | Implemented |
| **Milestone 4** | Hardening & VNET | Implement OpenBSD W^X/KARL/`pledge`/`unveil`, Capsicum rights, and FreeBSD VNET network virtualization. | Implemented |
| **Milestone 5** | Rump Isolation & Patch| Implement NetBSD Rump Kernel user-space driver sandboxing and Linux ftrace-based Livepatching. | Implemented |

---

## 5. Verification & Testing Strategy

1. **Unit Tests:** Standalone test suites in `src/hal/multi_arch.rs`, `src/kernel/module_loader.rs`, `src/kernel/missing_linux_kernel_components.rs`, and `src/security/landlock.rs`.
2. **QEMU Smoke & Integration Testing:** Boot testing across x86_64, AArch64, and RISC-V 64 QEMU virtual machine targets.
3. **Automated Verification:** Continuous validation via `./run_sigma_tests.sh`.
