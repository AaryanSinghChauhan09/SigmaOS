# SigmaOS Architecture Overview

## Vision

SigmaOS is a sovereign, zero-dependency operating system written entirely in Rust. It aims to combine the best of Linux distributions, BSDs, and modern research OSes into a single cohesive, secure, and performant platform.

***

## Architecture Diagram

    ┌─────────────────────────────────────────────────────────┐
    │                   USER APPLICATIONS                     │
    │  (SigmaShell · SigmaTools · Zenith Desktop · sigpkg)    │
    ├─────────────────────────────────────────────────────────┤
    │               SYSTEM SERVICES LAYER                     │
    │  (Init/Runit · SSH Daemon · Network Manager · AI Daemon)│
    ├────────────┬──────────────┬──────────────┬──────────────┤
    │  PACKAGE   │   SECURITY   │  CONTAINER   │  AI / ML     │
    │  MANAGER   │   SUBSYSTEM  │  RUNTIME     │  ENGINE      │
    │  (sigpkg)  │(pledge/unveil│(sigmakube)   │(orchestrator)│
    ├────────────┴──────────────┴──────────────┴──────────────┤
    │                    KERNEL CORE                          │
    │  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌────────┐  │
    │  │Scheduler │  │ Memory   │  │ VFS /    │  │ IPC /  │  │
    │  │(BORE MLFQ│  │ Manager  │  │Filesystem│  │Signals │  │
    │  │+Thermal) │  │(zone/slab│  │(ext4/fat)│  │(POSIX) │  │
    │  └──────────┘  └──────────┘  └──────────┘  └────────┘  │
    ├─────────────────────────────────────────────────────────┤
    │              HARDWARE ABSTRACTION LAYER (HAL)           │
    │  (x86_64 · AArch64 · RISC-V · DRM/KMS · USB xHCI)      │
    ├─────────────────────────────────────────────────────────┤
    │                   HARDWARE                              │
    │  (CPU · RAM · NVMe · GPU · Network · USB · TPM)         │
    └─────────────────────────────────────────────────────────┘

***

## Core Design Principles

### 1. Zero External Dependencies

All critical subsystems are implemented natively in Rust without pulling in external C libraries. This includes:

*   Custom string, vec, hashmap, btreemap implementations (`src/klib/`)
*   Custom memory allocators (buddy + slab)
*   Post-quantum cryptography (Kyber, Dilithium) — no OpenSSL dependency
*   Native TLS 1.3 implementation

### 2. Security by Default

Every component applies defense-in-depth:

*   `pledge()` and `unveil()` sandbox every process (OpenBSD-inspired)
*   SELinux-style Mandatory Access Control
*   Post-quantum resistant cryptography throughout
*   TPM 2.0 attestation for secure boot
*   W^X (Write XOR Execute) memory policy enforced kernel-wide

### 3. Multi-Distro Compatibility

SigmaOS can run packages from virtually any Linux distribution through compatibility layers and universal adapters:

*   Arch Linux (ALPM/AUR) via `src/sigpkg/`
*   Debian/Ubuntu (.deb/apt) via universal adapter
*   Fedora/RHEL (.rpm/dnf) via compatibility layer
*   Gentoo (Portage USE flags) via `src/sigpkg/spec.rs`
*   Flatpak, AppImage, Snap natively supported

### 4. AI-Native Design

The OS integrates AI at the kernel level:

*   AI-driven process scheduling hints
*   Local LLM inference for system automation
*   Multi-agent orchestration for complex tasks
*   Predictive memory prefetching using ML models

### 5. Modular & Composable

Every subsystem is an independently loadable kernel module:

*   Hot-swappable drivers
*   Pluggable schedulers (MLFQ, BORE, PDS, BMQ)
*   Composable security policies
*   Declarative system configuration (NixOS-inspired)

***

## Kernel Subsystems

### Scheduler

*   **Primary:** MLFQ (Multi-Level Feedback Queue) — `src/kernel/sched/sigma_mlfq.rs`
*   **Enhanced:** BORE (Burst-Oriented Response Enhancer) — CachyOS-inspired
*   **Thermal:** Temperature-aware CPU frequency scaling — `src/kernel/sched/sigma_thermal_sched.rs`
*   **Real-time:** SCHED\_FIFO and SCHED\_RR policies for RT workloads

### Memory Manager

*   **Physical:** Buddy allocator (`src/buddy.rs`) — power-of-2 page allocation
*   **Object:** Slab/SLUB allocator (`src/slab.rs`) — per-type object caches
*   **Virtual:** Zone-based allocator (`src/memory/zone.rs`) — DMA/Normal/High
*   **Swap:** kswapd daemon (`src/memory/kswapd.rs`) — background page reclamation
*   **cgroups v2:** Resource group management (`src/memory/cgroups.rs`)

### Filesystem

*   **VFS:** Virtual filesystem abstraction (`src/filesystem/`)
*   **Formats:** ext4, FAT32, tmpfs, overlayfs
*   **Integrity:** Block-level checksums (ZFS-inspired)
*   **Snapshots:** Atomic filesystem snapshots for rollbacks

### Networking

*   **Stack:** Full TCP/UDP/IP implementation (`src/network/`)
*   **Security:** PQC-TLS 1.3 (`src/crypto/`)
*   **Wireless:** WPA3/802.11ax support (`src/wireless/`)
*   **Bluetooth:** BLE 5.0 support (`src/bluetooth/`)

***

## Boot Sequence

    Power On
      └── UEFI Firmware
            └── SigmaBoot (UEFI bootloader)
                  ├── TPM 2.0 attestation
                  ├── Secure boot verification
                  └── Kernel load
                        └── HAL init (CPU, memory map)
                              └── Kernel core init
                                    ├── Memory manager
                                    ├── Scheduler
                                    ├── VFS mount
                                    └── PID 1 (Init/Runit)
                                          └── Services
                                                └── Desktop / Shell

***

## Source Tree Structure

    src/
    ├── ai/          — AI orchestrator, daemon, inference routing
    ├── audio/       — ALSA-like audio subsystem
    ├── boot/        — UEFI bootloader (uefi.rs)
    ├── compatibility/ — Multi-distro compatibility layers
    ├── container/   — Container runtime (sigmakube)
    ├── crypto/      — Post-quantum cryptography
    ├── desktop/     — Zenith desktop environment
    ├── driver/      — Universal driver framework
    ├── filesystem/  — VFS and filesystem implementations
    ├── graphics/    — GPU-accelerated rendering
    ├── kernel/      — Core kernel (scheduler, proc, HAL)
    ├── klib/        — Zero-dependency standard library replacement
    ├── loader/      — ELF binary loader and relocator
    ├── memory/      — Physical/virtual memory management
    ├── ml/          — Machine learning inference/training
    ├── network/     — TCP/IP, wireless, analyzer
    ├── package/     — Universal package management
    ├── security/    — pledge, unveil, MAC, audit, PQC
    ├── shell/       — Interactive kernel shell (REPL)
    ├── sigpkg/      — Native SigmaOS package manager
    └── tools/       — Build tools, debugger, diagnostics

***

*Last updated: 2026-08-23 | SigmaOS Architecture Team*
