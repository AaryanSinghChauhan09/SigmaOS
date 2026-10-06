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


## AI Agent Maintenance Instructions

- **Bolt ⚡**: Ensure documentation of any new zero-allocation optimizations or performance improvements are added concisely without marketing fluff.
- **Palette 🎨**: Maintain Arch Linux wiki style: clear, factual, one page per topic, using appropriate markdown formatting and tables where necessary.
- **Sentinel 🛡️**: Verify that no hardcoded credentials or unvetted cryptographic algorithms are documented as production-ready. Ensure security limitations are accurately stated.
- **General**: Keep pages up-to-date with current repository capabilities. Remove redundant files when consolidating information.

## ADR-004: Linux 6.12+ EEVDF & `sched_ext` Pluggable BPF Task Scheduling
- **Status:** Accepted
- **Context:** Standard POSIX schedulers suffer from context-switch latency spikes (> 500ns) and lock contention under high thread concurrency.
- **Decision:** Adopt an Earliest Eligible Virtual Deadline First (EEVDF) scheduler combined with `sched_ext` (SCX) pluggable BPF scheduler infrastructure. Enforce sub-80ns context switching by leveraging 64-bit TSS stack switching (`RSP0`) and CPU cache affinity bounds.

## ADR-005: Linux eBPF/XDP Zero-Copy Network Redirection & Socket Bypass
- **Status:** Accepted
- **Context:** BSD socket layer overhead and kernel-to-userland data copying limit networking throughput under 100GbE / Wi-Fi 7 loads.
- **Decision:** Implement eBPF eXpress Data Path (XDP) driver-level packet filtering and `sockmap` / `sk_msg` socket redirection. Bypass the network stack for local IPC, achieving sub-microsecond packet processing.

## ADR-006: FreeBSD VNET & Zero-Copy Network Stack Virtualization
- **Status:** Accepted
- **Context:** Multi-tenant container networking requires complete network stack isolation without virtualization overhead.
- **Decision:** Adopt FreeBSD VNET virtualized network stack architecture. Provide isolated routing tables, interface bounds, and firewall state tables per container instance with zero-copy packet passing.

## ADR-007: OpenBSD MAP_STACK & Retguard Zero-Overhead Security Mitigation
- **Status:** Accepted
- **Context:** Traditional security checks introduce significant runtime CPU penalties during system call execution.
- **Decision:** Implement OpenBSD `MAP_STACK` region enforcement and Retguard return-address XOR canary verification using hardware FNV-1a hashing, providing zero-overhead exploitation protection.

## ADR-008: DragonFly BSD HAMMER2 Lock-Free B-Tree CoW Storage Architecture
- **Status:** Accepted
- **Context:** File system lock contention during high-frequency parallel write workloads degrades disk I/O performance.
- **Decision:** Adopt DragonFly BSD HAMMER2 multi-grained Copy-on-Write (CoW) B-tree storage architecture. Utilize lock-free extent trees, in-memory Merkle trees, and automatic block deduplication.

## ADR-009: NetBSD Rump Kernel Userland Driver Isolation Architecture
- **Status:** Accepted
- **Context:** Device driver crashes in Ring 0 compromise kernel stability and require hard reboot cycles.
- **Decision:** Implement NetBSD Rump kernel architecture to run device drivers in memory-isolated userland processes. Hypercalls bridge userland drivers to kernel subsystems with zero allocation fast-paths.

## ADR-010: Linux `io_uring` Asynchronous System Call Submission Architecture
- **Status:** Accepted
- **Context:** Synchronous system calls cause frequent userland-to-kernel mode transitions and CPU pipeline flushes.
- **Decision:** Implement lock-free lockless Submission Queue (SQ) and Completion Queue (CQ) ring buffers (`io_uring`). Batch I/O, network, and process operations into single kernel entry passes.

## ADR-011: Solaris / Illumos DTrace Zero-Cost Dynamic Instrumentation
- **Status:** Accepted
- **Context:** Production performance profiling tools often introduce observer effect overhead that distorts benchmark telemetry.
- **Decision:** Integrate Solaris DTrace dynamic tracing provider engine. When probes are disabled, NOP instruction patching guarantees absolute zero performance overhead.

## ADR-012: Linux THP & Lockless Page Cache Memory Engine
- **Status:** Accepted
- **Context:** Page table walking and TLB misses for large-memory workloads reduce execution efficiency.
- **Decision:** Implement Transparent Huge Pages (2MiB / 1GiB page frames), lockless RCU-protected page cache lookup maps, and zRAM compressed swap pools.
