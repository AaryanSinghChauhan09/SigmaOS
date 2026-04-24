# Σ SIGMAOS: THE SOVEREIGN SILICON ENTITY

[![SigmaOS CI](https://github.com/AaryanSinghChauhan09/SigmaOS/actions/workflows/ci.yml/badge.svg)](https://github.com/AaryanSinghChauhan09/SigmaOS/actions/workflows/ci.yml)

SigmaOS is an industrial-grade, zero-dependency operating system built on the **Sovereign Lattice**. It runs bare-metal on AArch64 (Raspberry Pi) and RISC-V silicon, bypassing POSIX and legacy abstractions to deliver hardware-native performance.

---

## 🏛️ Repository Architecture

| Path | Purpose |
|------|---------|
| `suites/S01_Genesis/` | Kernel entry, UEFI bootloader, HAL primitives |
| `suites/S03_Orchestrator/` | Decentralized persistence (CRDT), IPC, process management |
| `suites/S04_HAL/` | AArch64 & RISC-V exception vectors, MMIO drivers |
| `suites/S05_Memory/` | DMA coherent allocator, slab memory manager |
| `suites/S07_Scheduling/` | AI-native scheduler (CPU ↔ NPU dispatch) |
| `suites/S08_Security/` | Zero-trust capability system, Kani formal proofs |
| `web_ui/` | Zenith Web Dashboard — 7 live kernel observability panels |

---

## 🚀 Getting Started

### Prerequisites

| Tool | Version | Purpose |
|------|---------|---------|
| `gcc-aarch64-linux-gnu` | 11+ | AArch64 bare-metal compilation |
| `gcc-riscv64-linux-gnu` | 11+ | RISC-V 64 bare-metal compilation |
| `rustup` + nightly | 1.75+ | Orchestrator & formal verification |
| `node` | 18+ | Zenith UI & build shim synthesis |
| `qemu-system-aarch64` | optional | Bare-metal emulation |

---

### Unified Workflow (S-CLI)

The **Sigma Sovereign CLI (`s-cli`)** is the recommended tool for managing the Sovereign Lattice.

```bash
# 1. Compile Native CLI
g++ -std=c++20 s-cli.cpp -o s-cli

# 2. Build the kernel (default x86_64)
./s-cli build

# 3. Build for a specific architecture
./s-cli build --arch aarch64

# 4. Boot in QEMU
./s-cli run
```

### Build via Makefile (Legacy Wrapper)

```bash
make              # x86_64
make aarch64      # ARM64 / Raspberry Pi
make riscv64      # RISC-V boards
make run          # Boot in QEMU
make clean        # Remove artifacts
```

---

## 🔬 Formal Verification

Run the Kani model-checking proofs to verify IPC/DMA non-interference:

```bash
# Install Kani
cargo install --locked kani-verifier
cargo kani setup

# Run proofs
cargo kani --manifest-path suites/S08_Security/formal_proofs/Cargo.toml
```

---

## 💎 Sovereign Standards

- **Zero-Std Enforcement**: Strictly freestanding (`-ffreestanding`, `-nostdlib`)
- **Pure ASM Core**: Direct register-level hardware control on AArch64 & RISC-V
- **Unified Single Branch**: `main` is the hardened, immutable source of truth
- **CI-Enforced**: Every commit is built, linted, and formally verified automatically

---

## 🤝 Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for build setup, shard architecture, and PR guidelines.

**Contact**: [aaryansinghchauhan090305@gmail.com](mailto:aaryansinghchauhan090305@gmail.com) | [GitHub @AaryanSinghChauhan09](https://github.com/AaryanSinghChauhan09)

---

*Sovereignty is Absolute. The machine is Unified.*
