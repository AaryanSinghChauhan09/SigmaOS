# SigmaOS Architecture Overview

> Quick navigation: [Kernel](Kernel) · [HAL](HAL) · [Networking](Networking) · [Security-Model](Security-Model) · [System-Daemons](System-Daemons)

---

## System Layers

```
┌─────────────────────────────────────────────────────────────────┐
│  USER SPACE (Ring 3 / EL0)                                      │
│  PWAs · Zenith Desktop · profession apps · sigma-ai LLM         │
├─────────────────────────────────────────────────────────────────┤
│  BROWSER SHELL (browser profile)                                │
│  Custom Chromium + navigator.sigmaos.* API                      │
├─────────────────────────────────────────────────────────────────┤
│  SYSTEM DAEMONS (Ring 3, pledge-restricted)                     │
│  sigmad-health · sigmad-pkg · sigmad-netd · sigmad-vault        │
├─────────────────────────────────────────────────────────────────┤
│  SYSCALL INTERFACE                                              │
│  sigma_pledge · sigma_unveil · seccomp-BPF · AVC MAC            │
├─────────────────────────────────────────────────────────────────┤
│  KERNEL (Ring 0 — freestanding, no glibc)                       │
│  Scheduler · Memory · Security · Network · Filesystem           │
│  IPC · IRQ/APIC · cgroups · namespaces · eBPF                   │
├─────────────────────────────────────────────────────────────────┤
│  SOVEREIGN HAL — x86_64 · ARM64 · RISC-V                        │
├─────────────────────────────────────────────────────────────────┤
│  HARDWARE — CPU · NVMe · GPU · NIC · USB · TPM2 · UEFI          │
└─────────────────────────────────────────────────────────────────┘
```

---

## Core Subsystems

### Scheduler
- MLFQ (4 queues + aging), CFS clone (vruntime/red-black tree)
- EDF for RTOS profile, NUMA-aware placement
- sigma-ai predictive pre-warming (Phase H)

### Memory
- Buddy allocator (2^n frames), Slab allocator (kmalloc)
- 4-level paging (x86_64 PML4), ASLR 42-bit per VMA, W^X

### Security
- sigma_pledge / sigma_unveil (OpenBSD-inspired)
- AVC O(1) MAC, Zero-trust SPIFFE, TPM2 attestation
- PQC: Kyber-1024 KEM + Dilithium-5 signatures

### Networking
- TCP/IP · TLS 1.3+Kyber · DNS/DoH · DHCP · WPA3 · Firewall

### Filesystem
- VFS → SigmaFS (native CoW) / Ext4 / FAT32 / Tmpfs / dm-verity

### HAL
- PCI/PCIe MSI-X, ACPI (MADT/SRAT/DSDT), UEFI runtime

---

## Shard System

600+ atomic capability modules (`suites/S001–S500+`) — each independently testable and deployable. Shards are merged into profiles at build time via CMake feature flags.

---

## 8 Deployment Profiles

| Profile | Branch | Use Case |
|---------|--------|---------|
| Standalone | `release/standalone` | Developer laptops, workstations |
| Browser | `release/browser` | Chromebook-style thin clients |
| Microkernel | `release/microkernel` | Servers, hypervisors, research |
| Mobile | `release/mobile` | Raspberry Pi, ARM64 tablets |
| RTOS | `release/rtos` | Industrial control, robotics |
| Dual-Boot | `release/dual-boot` | Alongside Windows/Linux |
| Cloud | `release/cloud` | AWS/Azure VMs, BharatCloud |
| Distributed | `release/distributed` | Multi-node clusters |

---

## Key Directories

| Path | Purpose |
|------|---------|
| `kernel/` | Microkernel core |
| `arch/` | x86_64, arm64, riscv64 code |
| `drivers/` | SDF hardware drivers |
| `hal/` | Hardware abstraction |
| `fs/` | Filesystems |
| `net/` | Network stack |
| `security/` | Security subsystems |
| `crypto/` | PQC primitives (Kyber, Dilithium) |
| `suites/` | 600+ capability shards |
| `include/` | All header files |
| `docs/` | Extended documentation |
| `wiki_repo/` | This wiki |

---

*Full spec: [ARCHITECTURE.md](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/ARCHITECTURE.md) · [Kernel internals](Kernel) · [Development Roadmap](Development-Roadmap)*
