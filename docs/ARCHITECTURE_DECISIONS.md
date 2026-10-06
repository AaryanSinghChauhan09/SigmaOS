# Architecture Decision Records (ADRs)

## ADR-001: Sovereign Zero-Dependency Philosophy
- **Status:** Accepted
- **Context:** SigmaOS aims to be a self-sufficient, high-performance operating system surpassing legacy open-source projects.
- **Decision:** Implement all core operating system capabilities natively in Rust under `#![no_std]` without external third-party crate dependencies.

## ADR-002: Universal Package Manager Interop
- **Status:** Accepted
- **Context:** Applications across various Linux distributions and BSD variants use diverse package formats (.deb, .rpm, .apk, pkg, etc.).
- **Decision:** Provide native parsing, DPLL SAT dependency resolution, scriptlet sandboxing, and format translation for 30+ package formats into canonical `SigmaPkg`.

## ADR-003: Multi-Core SMP and Modern Kernel Subsystems
- **Status:** Accepted
- **Context:** Modern hardware requires efficient multi-core processing, async I/O, and low-latency IPC.
- **Decision:** Integrate LAPIC/IPI/MADT SMP, io_uring, kqueue, cgroups v2, OverlayFS, and PQC VPN firewall into the core kernel architecture.

## ADR-004: Native Universal Device Support with Micro-Footprint Optimization
- **Status:** Accepted
- **Context:** Supporting diverse hardware across x86-64, AArch64, and RISC-V without bloat requires zero-dependency driver abstractions.
- **Decision:** Deploy zero-allocation hardware driver abstractions, PCI/USB modalias matching, and modular driver tiers.

## ADR-005: Supreme Performance Multi-Queue CPU Scheduling & Lockless IPC Architecture
- **Status:** Accepted
- **Context:** Achieving sub-80ns context switching, zero-allocation fast paths, and sub-5µs real-time preemption latency across heterogeneous multi-core topologies requires advanced scheduling and IPC primitives.
- **Decision:** Absorb Linux 6.6+ EEVDF (Earliest Eligible Virtual Deadline First) scheduling, CachyOS BORE (Burst-Oriented Response Enhancer), FreeBSD ULE interactivity scoring, and lockless atomic ring buffer IPC channels (`src/process/sovereign_scheduler_governor.rs`, `src/ipc/sovereign_async_procedure_call.rs`).

## ADR-006: Asynchronous I/O Multiplexing & Zero-Copy Network Fabric
- **Status:** Accepted
- **Context:** High-throughput network packet processing and storage I/O suffer from POSIX syscall overhead and memory copying bottlenecks.
- **Decision:** Synthesize Linux `io_uring` SQ/CQ submission/completion rings, Linux eBPF/XDP zero-copy packet redirection, FreeBSD `kqueue`/`kevent` event filtering, and OpenBSD `poll`/`select` fallback into a unified non-blocking I/O multiplexing subsystem (`src/network/sovereign_async_io.rs`).

## ADR-007: Sovereign Memory Subsystem & Zero-Allocation Page Cache Engine
- **Status:** Accepted
- **Context:** Memory fragmentation, lock contention in page allocators, and cache false sharing degrade overall system throughput.
- **Decision:** Implement a SLUB-inspired slab object allocator (`src/memory/slab_allocator.rs`), THP (Transparent Huge Pages), RCU (Read-Copy-Update) wait-free read paths (`src/kernel/rcu.rs`), and 64-byte CPU cache-line aligned atomic ring buffers with zero dynamic heap allocations on hot paths.

## ADR-008: Post-Quantum Cryptographic Security & Sandbox Isolation Architecture
- **Status:** Accepted
- **Context:** Enterprise security requires defense-in-depth and quantum-resistant cryptographic protection without sacrificing execution speed.
- **Decision:** Integrate Kyber-1024 KEM and Dilithium-5 signatures (`src/crypto/`), OpenBSD `pledge(2)`/`unveil(2)` path sandboxing (`src/security/pledge.rs`), FreeBSD Capsicum descriptor capability rights (`src/security/capsicum.rs`), and Linux Landlock LSM policies into a fail-closed security architecture.
