<div align="center">

# Σ SigmaOS Zenith

### *The Sovereign Industrial Microkernel — Silicon-Direct, Zero-Dependency, Absolute.*

[![Build](https://img.shields.io/github/actions/workflow/status/AaryanSinghChauhan09/SigmaOS/ci.yml?branch=main&label=CI%20Build&style=for-the-badge&logo=github-actions&logoColor=white)](https://github.com/AaryanSinghChauhan09/SigmaOS/actions)
[![License](https://img.shields.io/badge/License-Sovereign%20OSL-blueviolet?style=for-the-badge)](LICENSE)
[![Version](https://img.shields.io/badge/Version-v15.2%20ZENITH-00d4ff?style=for-the-badge)](CHANGELOG.md)
[![Wiki](https://img.shields.io/badge/Wiki-Sovereign%20Docs-orange?style=for-the-badge&logo=github)](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki)
[![Stars](https://img.shields.io/github/stars/AaryanSinghChauhan09/SigmaOS?style=for-the-badge&logo=github)](https://github.com/AaryanSinghChauhan09/SigmaOS/stargazers)
[![Branches](https://img.shields.io/badge/Branches-12%20Targets-success?style=for-the-badge)](https://github.com/AaryanSinghChauhan09/SigmaOS/branches)

</div>

---

## 🏛️ What is SigmaOS?

**SigmaOS** is an industrial-grade, sovereign microkernel operating system built on a **600-shard C++ singleton lattice** — targeting **x86_64**, **ARM64**, and **RISC-V** without any monolithic Linux or Windows dependency.

Every subsystem — scheduler, allocator, IPC, filesystem, GPU layer, networking stack, and desktop compositor — is implemented as a discrete, formally-isolated **shard**. There are no third-party kernel modules, no bloated runtimes, no vendor lock-in.

> **Σ SigmaOS** — Absolute Sovereignty. Singularity Achieved.

---

## ⚡ Core Capabilities

| Capability | Description |
| :--- | :--- |
| **Shard-Aware CFS Scheduler** | NUMA-balanced with inline assembly context switches and O(1) priority dispatch |
| **O(1) Slab Allocator** | Lockless, fragmentation-free, power-of-2 bucket design |
| **Lock-Free SPSC IPC** | Zero-copy ring buffer for inter-shard messaging |
| **S-HAL** | Platform-agnostic hardware abstraction for x86_64, ARM64, RISC-V |
| **SovereignVulkanLayer** | Direct SPIR-V GPU shader routing without the Vulkan SDK overhead |
| **Modular C Syscall Dispatcher** | 256-slot registry with O(1) runtime handler registration |
| **Post-Quantum Cryptography** | Dilithium-5 attested boot chain and IPC message signing |
| **Zenith Desktop UI** | Glassmorphic hardware-composited desktop with a Vulkan compositor |

---

## 🏗️ Architecture Layers

```
┌──────────────────────────────────────────────────────┐
│  Ring-3 (Userland)                                   │
│  sigma-sh | sigma-forensics | Zenith Desktop UI      │
└──────────────────────┬───────────────────────────────┘
                       │  syscall / SYSRET
┌──────────────────────▼───────────────────────────────┐
│  SyscallDispatcher  (256-slot O(1) C table)          │
└──────────────────────┬───────────────────────────────┘
                       │
┌──────────────────────▼───────────────────────────────┐
│  Ring-0 (Sovereign Kernel Lattice)                   │
│  CFS Scheduler | Slab Allocator | VFS | SPSC IPC     │
│  PQC Engine    | Container Runtime | NetStack         │
└──────────────────────┬───────────────────────────────┘
                       │
┌──────────────────────▼───────────────────────────────┐
│  S-HAL (Sovereign Hardware Abstraction Layer)        │
│  x86_64 APIC | ARM64 GIC | RISC-V PLIC/CLINT        │
└──────────────────────┬───────────────────────────────┘
                       │
              ⚙️ Physical Hardware
```

---

## 🌿 Branch Architecture (12 Targets)

| Branch | Target Archetype | Optimization Focus |
| :--- | :--- | :--- |
| `main` | Stable Production | Standard balanced shard scheduler |
| `release/standalone` | Bare-Metal Desktop | Direct CPU-bound execution, local peripherals |
| `release/rtos` | Real-Time Embedded | Deterministic scheduling, zero-latency interrupts |
| `release/mobile` | Energy-Aware Mobile | DVFS, touch UI, ARM64 HAL tuning |
| `release/microkernel` | Ultra-Minimal | 120-shard hyper-secure critical node config |
| `release/dual-boot` | Co-operative Boot | GRUB chain-load, rollback snapshot integration |
| `release/distributed` | Cluster-Native | RPC shard sync, SovereignCloudFS |
| `release/cloud` | Headless Virtualization | Hypervisor hosting, multi-tenant memory pages |
| `release/browser` | WebAssembly Runtime | WASM core, sandboxed browser apps |
| `release/app` | App-Store Sandbox | Static containers, locked filesystem profiles |
| `performance-optimized` | SIMD-Tuned | AVX-512 / ARM Neon, O(1) slab, max PQC throughput |
| `gh-pages` | Static Web | Interactive demo, docs, live installer guides |

---

## 🛠️ Quickstart

### Prerequisites
- `gcc` / `clang` (cross-compiled x86_64-elf toolchain)
- `nasm` (assembler)
- `qemu-system-x86_64` (emulator)
- `make`, `xorriso`, `grub-pc-bin`
- `node` + `npm` (for UI tests)

### 1. Clone & Setup
```bash
git clone https://github.com/AaryanSinghChauhan09/SigmaOS.git
cd SigmaOS
chmod +x scripts/setup.sh && ./scripts/setup.sh
```

### 2. Build
```bash
make clean && make all
```

### 3. Run in QEMU
```bash
qemu-system-x86_64 -cdrom build/sigmaos.iso -serial stdio -m 2G
```

### 4. Run Tests
```bash
npm install && npm run test
```
All 82 tests in `/tests` must pass green before submitting patches.

---

## 📚 Documentation

| Resource | Link |
| :--- | :--- |
| 📖 **Full Wiki** | [SigmaOS Wiki](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki) |
| 🏛️ **Architecture** | [Architecture Overview](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki/Architecture-Overview) |
| 🚀 **Getting Started** | [Getting Started Guide](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki/Getting-Started) |
| 🛡️ **Security** | [Security Framework](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki/Sovereign-Security-Framework) |
| 🌿 **Branch Guide** | [Branch Guide](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki/Branch-Guide) |
| 📋 **Contributing** | [CONTRIBUTING.md](CONTRIBUTING.md) |
| 📜 **Changelog** | [CHANGELOG.md](CHANGELOG.md) |
| 🗺️ **Roadmap** | [ROADMAP.md](roadmap.md) |

---

## 🤝 Contributing

We welcome contributions! Please read [CONTRIBUTING.md](CONTRIBUTING.md) before submitting pull requests.

**Quick rules:**
- All kernel code must be **zero-dependency** (no stdlib includes in Ring-0 shards)
- Use `strncpy` and bounded string operations — never unbounded
- All PRs must pass `npm run test` (82 green tests required)
- Follow commit conventions: `type(scope): message` (e.g., `fix(scheduler): correct NUMA pinning`)

---

## 📄 License

SigmaOS is released under the **Sovereign Open Source License**. See [LICENSE](LICENSE) for details.

---

<div align="center">

**© 2026 SigmaOS Sovereign Project** | v15.2 [ZENITH-SINGULARITY]

*Sovereignty is the ultimate efficiency.*

</div>
