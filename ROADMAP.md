# SigmaOS Roadmap

## Vision

SigmaOS aims to be a production-quality, research-grade operating system that:

*   Compiles fully in Rust with minimal unsafe code
*   Runs on bare metal (x86\_64, AArch64, RISC-V)
*   Achieves POSIX compliance for broad software compatibility
*   Provides state-of-the-art security inspired by OpenBSD
*   Supports modern hardware and hypervisors

***

## Phase 1 — Foundation (Current)

*   \[x] Custom bootloader (Stage 1 + Stage 2)
*   \[x] Buddy memory allocator
*   \[x] MLFQ process scheduler
*   \[x] Basic VFS layer
*   \[x] x86\_64 interrupt handling
*   \[x] Serial/console output
*   \[x] SigmaPkg package manager (alpha)
*   \[x] Linux/BSD driver compatibility layer
*   \[x] Windows compatibility layer (NDIS/WDM subset)
*   \[x] Basic security subsystem (ASLR, capabilities)
*   \[x] CodeQL scanning integration

## Phase 2 — Stabilization (Near-term)

*   \[ ] SMP (multi-core) scheduler
*   \[ ] Preemptive kernel threads
*   \[ ] USB support (xHCI)
*   \[ ] PCIe enumeration and hot-plug
*   \[ ] NVMe storage driver
*   \[ ] Full ext4 read/write support
*   \[ ] Network stack: TCP/UDP IPv4/IPv6 complete
*   \[ ] POSIX compliance test suite (LTP)
*   \[ ] Fully working SigmaShell
*   \[ ] SigmaPkg binary repository

## Phase 3 — Features (Mid-term)

*   \[ ] AArch64 (ARM64) port
*   \[ ] Containerization (Sigma Jails, inspired by FreeBSD)
*   \[ ] GPU drivers (virtio-gpu, basic DRM)
*   \[ ] Wayland compositor (SigmaComp)
*   \[ ] io\_uring-style async I/O
*   \[ ] eBPF-inspired kernel programmability
*   \[ ] ZFS-compatible checksumming in SigmaFS
*   \[ ] Distributed 9P filesystem support

## Phase 4 — Production (Long-term)

*   \[ ] RISC-V 64-bit port
*   \[ ] Formal verification of security-critical paths
*   \[ ] Hypervisor mode (KVM-compatible)
*   \[ ] Hardware Security Module (HSM) integration
*   \[ ] Certified POSIX compliance
*   \[ ] Enterprise support and LTS releases

***

## Feature Tracking

Track individual features via [GitHub Issues](https://github.com/AaryanSinghChauhan09/SigmaOS/issues)
with the appropriate milestone label.
