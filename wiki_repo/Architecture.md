# SigmaOS Architecture

## Vision
A sovereign, bare-metal OS that is dramatically safer (memory & capability security), equal-or-better performance (latency, throughput, boot time), developer-friendly (tooling, reproducibility), interoperable (run or host Linux workloads, container/VM compatibility), extensible (sandboxed drivers & services), and certifiable (secure boot, attestation).

## Primary KPIs
- Syscall latency / context switch cost (microbenchmarks)
- Single-thread & multi-threaded throughput (fileserver, network)
- Boot time (cold boot to login/service)
- Mean Time Between Failures (driver+kernel crashes per 1000 hours)
- Vulnerabilities found (CVE-equivalent count)
- Memory overhead for comparable workloads
- Power efficiency (mobile/edge targets)
- Time-to-driver (days to get a new NIC/GPU working via vendor-supplied spec)
- Adoption metrics: number of production deployments, packaged apps, contributors

## High-Level Architecture Recommendation
### Capability-Based Hybrid Microkernel
- **Kernel Responsibilities**: Scheduling, low-level IPC, address-space management, interrupts, minimal drivers for boot & storage if necessary
- **Userspace**: Most drivers, filesystems, network stack, window compositor, etc., run in isolated processes with capability tokens
- **Hybrid Compromise**: Keep very hot paths (timer tick, fast-path IO queue handling) in the kernel; implement everything else in userland
- **Language**: Rust for kernel + drivers, Nim/Rust for tooling & UI, JS/TypeScript for web-based higher-level UX as needed

## Memory Safety & Correctness
- Use Rust with no_std where appropriate (kernel), strictly audited unsafe blocks
- Adopt formal verification for core primitives (bootloader, capability manager, scheduler critical sections)
- Leverage hardware features: PAE/SME/MTE (ARMv8-MTE), Intel CET, SMEP/SMAP, page table features

## Driver Strategy
- **Primary Driver Runtime**: WASM with a rich host ABI or a vetted Rust driver ABI
- **Driver Lifecycles**: Load/unload, hot-restart, isolated crash recovery
- **Standardized Driver Host API**: DMA, interrupts, memory mapping, io_uring-like submission queues with capability tokens
- **Vendor Strategy**: SDK, certification harnesses, Linux-hosted adapter

## Userspace & Compatibility
- **Native API**: Modern, async-first syscall interface with io_uring-style completion queues
- **POSIX Compatibility**: Small POSIX shim layer
- **Linux Compatibility**: Linux-ABI shim or KVM for drivers/apps
- **Container Support**: Native secure container runtime for OCI images

## Scheduler & Performance
- NUMA-aware hierarchical scheduler with core isolation
- Lock-free/RCU for read-heavy paths
- Async-first design with io_uring-like batching
- PGO, LTO, SIMD for performance

## Security
- Fine-grained capabilities, no global root
- Secure boot, measured boot, TPM attestation
- Mandatory code signing, sandboxing, CFI, DEP, ASLR, PAC

## Observability & Tooling
- Low-overhead tracing, flamegraphs
- eBPF-like introspection
- Reproducible cross-compilers, VS Code debugger integration
- Fuzzing, HIL tests, formal verification

## Roadmap Milestones
- **Phase 0 (0-3 months)**: Architecture RFCs, dev env, bootloader, minimal kernel
- **Phase 1 (3-9 months)**: Kernel v0 with memory management, IPC, async syscalls, basic userspace, Linux-compat prototype
- **Phase 2 (9-18 months)**: Stable driver model, NIC/block drivers, scheduler tuning, secure boot
- **Phase 3 (18-36 months)**: Full userspace, production filesystems, GPU stack, formal verification
- **Phase 4 (36+ months)**: Certifications, vendor partnerships, migration tools
