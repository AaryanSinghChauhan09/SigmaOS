# Branch Guide

SigmaOS uses a structured multi-branch model. Each branch targets a specific deployment profile or development area. This page maps every active branch to its purpose, audience, and key differences from `main`.

---

## Development Branches

| Branch | Purpose | Key Additions |
|---|---|---|
| `main` | Stable trunk — everything merged here | Full feature set |
| `kernel-exp` | Kernel subsystem experiments | Scheduler, VMM, IPC research code |
| `drivers-dev` | Hardware driver development | Unified Driver API, USB, NVMe, Wi-Fi shims |
| `performance-optimized` | Latency / throughput tuning | NUMA pinning, lockless allocators, AVX-512 paths |
| `fs-dev` | Filesystem research | Ext4 journaling, CoW layer, SovereignFS prototype |
| `tools-dev` | Userland toolchain | `sigma-cc`, `sigma-pkg`, CLI utilities |
| `docs-update` | Documentation sync | Architecture diagrams, API reference updates |
| `prepare-sigmaos-launch` | Launch readiness | Contributor roadmap, gap analysis, launch checklist |
| `gh-pages` | GitHub Pages website | Landing page, app store HTML |

---

## Release Branches

Each `release/*` branch is a purpose-built variant of SigmaOS. They all share the same Ring-0 microkernel core but differ in enabled features, target hardware, and build flags.

### `release/standalone`

The reference desktop build. Bundles the Vite frontend, Electron shell, and full Zenith GUI into a self-contained package requiring no host setup.

- **Target**: Developer workstations, laptops
- **Boot time**: ~3 seconds to Chromium shell
- **Key features**: Full Zenith DE, SigmaCode IDE, SigmaTerm, SigmaNotes
- **Build**: `cmake -DCMAKE_TOOLCHAIN_FILE=profiles/workstation.cmake -B build`

---

### `release/microkernel`

The minimal kernel-only build. No GUI, no daemons — just the freestanding Ring-0 binary. Used as the base for all other profiles and for academic/research use.

- **Target**: Bare-metal servers, hypervisor guests, research
- **Key features**: MLFQ scheduler, VMM, VFS, TCP/IP stack, PID 1 event loop
- **Build**: `make SIGMA_USE_ZENITH_DE=0 SIGMA_USE_AI_ENGINE=0`

---

### `release/browser`

SigmaOS with Chromium as the OS shell — the closest analog to ChromeOS but with Unix primitives exposed to web apps via `navigator.sigmaos.*`.

- **Target**: Consumer laptops, thin clients
- **Key features**: Full `navigator.sigmaos` API, native messaging host, bwrap sandboxing, zero-install packages
- **Build**: `cmake -DSIGMA_PROFILE=standalone -DSIGMA_USE_ZENITH_DE=ON -B build`

---

### `release/mobile`

Tailored for ARM64 and RISC-V architectures. Optimized for low-power operation with adaptive P/C-state scheduling and touch-friendly UI scaling.

- **Target**: ARM laptops, tablets, Raspberry Pi
- **Toolchain**: `aarch64-linux-gnu-gcc` / `riscv64-linux-gnu-gcc`
- **Key features**: Hardware-adaptive scheduler, low-power idle states, responsive Zenith UI
- **Build**: `cmake -DCMAKE_TOOLCHAIN_FILE=profiles/iot-minimal.cmake -B build`

---

### `release/rtos`

Hard real-time extensions. Tasks with priority > 80 are promoted to `SCHED_SOVEREIGN` — a deterministic scheduling class with strict execution deadlines.

- **Target**: Industrial control systems, robotics, avionics simulators
- **Key features**:
  - `SCHED_SOVEREIGN` hard real-time class
  - Priority inheritance via `SovereignMutex` (prevents priority inversion)
  - Lock-free SPSC ring buffers for sub-microsecond IPC
  - Zero-copy memory segments for inter-task messaging
- **Build**: `make SIGMA_USE_ZENITH_DE=0 SIGMA_SCHED_REALTIME=1`

---

### `release/dual-boot`

Coexistence with Windows and Linux. Implements the Multiboot2 specification so standard GRUB installations can chain-load SigmaOS without repartitioning.

- **Target**: Users who want SigmaOS alongside an existing OS
- **Key features**:
  - Multiboot2 header in `arch/boot/multiboot_header.asm`
  - ELF64 output at load address `0x100000` (see `linker.ld`)
  - GRUB configuration generator in `Makefile` (`grub-mkrescue`)
  - Shared `/home` partition support via VFS mount abstraction
- **Setup**: See [Building from Source](Building-from-Source) for GRUB chain-load instructions

---

### `release/cloud`

CoreOS-style immutable cloud image. Root filesystem is read-only. Updates happen via A/B partition swap with attestation validation before commit.

- **Target**: AWS, Azure, GCP bare-metal and VM instances
- **Key features**:
  - Immutable root (handled by `SovereignImmutableHostEngine`)
  - A/B partition slots — instant rollback if boot attestation fails
  - Declarative system configuration (NixOS-inspired)
  - Hardened `sigmad-fleet` telemetry daemon for enterprise monitoring
- **Build**: `cmake -DSIGMA_PROFILE=cloud-x86 -B build`

---

### `release/distributed`

Extends the cloud profile with decentralized coordination. Virtual filesystems sync across nodes via secure sockets. Supports ZeroNet mesh networking and P2P shard replication.

- **Target**: Multi-node clusters, sovereign cloud data centers
- **Key features**:
  - Decoupled VFS synced via encrypted sockets (BLAKE2b integrity)
  - `sigma-cluster` daemon for distributed state coordination
  - Sovereign Container Orchestrator (Kubernetes-equivalent)
  - CRDT-based offline-first sync for eventually consistent deployments
- **Build**: `cmake -DSIGMA_PROFILE=cloud-x86 -DSIGMA_USE_CLUSTER=ON -B build`

---

## Choosing a Branch

```
Are you building for a laptop/desktop?
  └─ Yes → release/standalone  (or main)

Are you targeting ARM / low-power hardware?
  └─ Yes → release/mobile  (or profiles/iot-minimal.cmake)

Do you need hard real-time guarantees?
  └─ Yes → release/rtos

Do you need to coexist with Windows/Linux?
  └─ Yes → release/dual-boot

Are you deploying to cloud VMs?
  └─ Immutable single-node  → release/cloud
  └─ Multi-node cluster     → release/distributed

Are you doing kernel research?
  └─ release/microkernel  +  kernel-exp

Are you developing drivers?
  └─ drivers-dev

Are you working on performance?
  └─ performance-optimized
```

---

*See also: [Building from Source](Building-from-Source) · [Architecture Overview](Architecture-Overview) · [Contributor Roadmap](Contributor-Roadmap)*
