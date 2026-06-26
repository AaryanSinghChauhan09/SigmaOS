# SigmaOS Wiki

> **SigmaOS Zenith** — The Sovereign Industrial Microkernel. A zero-dependency, browser-first operating system built for silicon sovereignty across every computing profile.

---

## What is SigmaOS?

SigmaOS is a radical rethinking of what an operating system can be. It boots straight to Chromium in under 3 seconds, turns the browser itself into the OS shell, and gives web apps access to raw Unix primitives — `pipe`, `spawn`, `mmap`, `/dev` files — that were previously locked away behind native APIs.

Built on a minimal Linux base (Buildroot + systemd), SigmaOS replaces the traditional desktop stack with a browser-first, capability-gated platform where every process runs inside a bubblewrap sandbox. The same sovereign microkernel core also runs bare-metal on x86_64, ARM64, and RISC-V without any host libc dependency.

---

## Deployment Profiles

SigmaOS ships as **eight purpose-built release profiles** from a single shared kernel core. Choose the one that matches your use case:

| Profile | Branch | Target | Unique Capability |
|---|---|---|---|
| **Standalone** | `release/standalone` | Developer laptops / workstations | Full Zenith DE, SigmaCode IDE, one-command install |
| **Browser** | `release/browser` | Consumer laptops, thin clients | `navigator.sigmaos.*` API, zero-install packages |
| **Microkernel** | `release/microkernel` | Servers, research, hypervisors | Minimal Ring-0 binary, no GUI overhead |
| **Mobile** | `release/mobile` | ARM64 / RISC-V tablets | Adaptive P/C-state scheduling, touch UI |
| **RTOS** | `release/rtos` | Industrial control, robotics | `SCHED_SOVEREIGN` hard real-time class |
| **Dual-Boot** | `release/dual-boot` | Users keeping Windows/Linux | Multiboot2, GRUB chain-load |
| **Cloud** | `release/cloud` | AWS / Azure / GCP VMs | Immutable root, A/B partition rollback |
| **Distributed** | `release/distributed` | Multi-node clusters | ZeroNet mesh, CRDT sync, container orchestration |

→ Full details: [Branch Guide](Branch-Guide)

---

## How is it different from ChromeOS?

| Feature | ChromeOS | SigmaOS |
|---|---|---|
| Shell environment | Chrome browser + Linux VM | Chrome as the OS shell itself |
| Unix access for web apps | Crostini (heavy VM) | Direct via native messaging daemons |
| Package management | `.deb` via Crostini | Zero-install via `navigator.sigmaos.pkg.ensure()` |
| Process spawn | Not available to PWAs | `navigator.sigmaos.process.spawn()` |
| Capability system | Origin-based permissions | Explicit per-capability grants + kernel pledge/unveil |
| Window management | Browser tabs | Native WebKit frameless floating windows |
| Boot time | ~10–15 seconds | Under 3 seconds |
| Kernel | Linux (full) | Custom sovereign microkernel (freestanding, no glibc) |
| Real-time scheduling | None | `SCHED_SOVEREIGN` hard real-time class (`release/rtos`) |
| Multi-arch | x86_64 only | x86_64 + ARM64 + RISC-V |
| Security model | Chrome sandbox | pledge + unveil + bwrap + zero-trust SPIFFE workload IDs |
| Post-quantum crypto | None | Kyber-1024 (key exchange) + Dilithium3 (signatures) |

---

## Quick Start

```bash
# Clone
git clone https://github.com/AaryanSinghChauhan09/SigmaOS.git
cd SigmaOS

# Build the standalone desktop ISO (requires Ubuntu 22.04)
make clean && make iso

# Boot in QEMU
qemu-system-x86_64 -cdrom build/sigmaos.iso -serial stdio -m 2G

# Multi-arch (ARM64 IoT build)
cmake -B build -DCMAKE_TOOLCHAIN_FILE=profiles/iot-minimal.cmake
make -C build -j$(nproc)

# RTOS build (hard real-time, no GUI)
make SIGMA_USE_ZENITH_DE=0 SIGMA_SCHED_REALTIME=1

# Cloud-immutable build
cmake -B build -DSIGMA_PROFILE=cloud-x86 -B build
```

---

## Wiki Pages

### Getting Started
- [Building from Source](Building-from-Source) — Step-by-step build guide for Ubuntu 22.04, all profiles
- [Branch Guide](Branch-Guide) — All release branches explained: standalone, browser, RTOS, cloud, distributed and more
- [Contributor Roadmap](Contributor-Roadmap) — Honest gap analysis, phase plan, and how to contribute

### Architecture
- [Architecture Overview](Architecture-Overview) — Four-layer diagram: kernel → daemons → extension → web-shell
- [Kernel Architecture](Kernel) — MLFQ scheduler, VMM, VFS, TCP/IP, PID 1 event loop
- [Hardware Abstraction Layer](HAL) — HAL API across x86_64, ARM64, RISC-V; boot sequence
- [Networking Stack](Networking) — TCP state machine, socket API, conntrack, firewall

### Security
- [Security Model](Security-Model) — Capability system, bwrap sandboxing, zero-trust workload IDs, Kyber-1024 attestation
- [sigma_pledge](Security-Model#sigma-pledge) — Per-process syscall restriction (OpenBSD-inspired)
- [sigma_unveil](Security-Model#sigma-unveil) — Per-process filesystem restriction (OpenBSD-inspired)

### API Reference
- [navigator.sigmaos API](API-Reference) — Full reference for all `navigator.sigmaos.*` APIs
- [Syscall Dispatcher](Syscall-Dispatcher) — Kernel syscall table and O(1) dispatch mechanism
- [Driver Development](Driver-Development) — Unified Driver API for Wi-Fi, USB, NVMe, IoT peripherals

### Application Development
- [Writing Your First SigmaOS App](Your-First-App) — Hello world PWA tutorial with capability enforcement
- [App Manifest Format](App-Manifest) — Schema reference, capability strings, validation CLI

### Desktop & Profiles
- [Zenith Desktop](Zenith-Desktop) — Silicon attestation, shard matrix, Neural UI Engine, reactive events
- [Release Profiles](Release-Profiles) — Profession-specific shard bundles (AI Researcher, Cybersecurity, Gaming, etc.)
- [Performance Architecture](Performance-Architecture) — NUMA pinning, lockless allocators, AVX-512, PGO builds

---

## Feature Matrix

The CI pipeline (`scripts/ci_branch_check.sh`) validates each branch against the feature matrix:

| Subsystem | main | standalone | browser | microkernel | rtos | cloud | distributed |
|---|---|---|---|---|---|---|---|
| MLFQ Scheduler | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| SCHED_SOVEREIGN (RT) | ~ | ~ | — | — | ✓ | — | — |
| VMM (4-level paging) | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| TCP/IP stack | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| Zenith Desktop | ~ | ✓ | ✓ | — | — | ~ | — |
| Auto-tiling WM | ~ | ✓ | ~ | — | — | — | — |
| bwrap sandboxing | ✓ | ✓ | ✓ | — | — | ✓ | ✓ |
| pledge / unveil | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| Immutable root | — | — | — | — | — | ✓ | ✓ |
| A/B partition rollback | — | — | — | — | — | ✓ | ✓ |
| Container orchestrator | ~ | ✓ | ✓ | ~ | — | ✓ | ✓ |
| Distributed VFS / CRDT | — | — | — | — | — | ~ | ✓ |
| Post-quantum crypto | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| navigator.sigmaos API | ✓ | ✓ | ✓ | — | — | ~ | ~ |

`✓` present · `~` partial · `—` not required for this profile

---

## Engineering Roadmap

```
Phase 1 — Boot & HAL (complete)
  ✓ Freestanding x86_64 kernel (no glibc)
  ✓ Multi-arch HAL (x86_64, ARM64, RISC-V stubs)
  ✓ PID 1 signalfd event loop (no more 5-iteration bug)
  ✓ IDT with 32 ISR stubs, Ring 3 transition

Phase 2 — Services (in progress)
  ✓ Slab allocator spec
  ✓ Lock-free SPSC ring buffers
  ~ Syscall table (partial)
  □ Ext4 JBD2 ordered journaling
  □ NVMe / e1000 real drivers

Phase 3 — Hardware (planned)
  □ Linux DRM/KMS compatibility shim
  □ USB 3.0 controller
  □ Native IPv4/IPv6 + VPN stack

Phase 4 — Desktop & Tooling (planned)
  □ Zenith native C++ compositor (replacing JS prototype)
  □ Sigma Shell scripting pipelines
  □ Guided graphical installer (Calamares equivalent)

Phase 5 — Specialized Profiles & Immutable Distribution (planned)
  □ Signed .spkg registry with BLAKE2b + Dilithium3
  □ Karma-gated staged rollout (Fedora Bodhi-inspired)
  □ Profession profile bundles (AI, Security, Gaming, Education)
```

---

## Current Version

**v15.2 — Zenith Release Microkernel**

See the [Changelog](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/CHANGELOG.md) for what's new.
