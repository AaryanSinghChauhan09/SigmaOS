> Imported repository document from [`README.md`](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/README.md). For current component status and roadmap, use the canonical component page linked from [Home](Home.md).

---

## Star History

<a href="https://www.star-history.com/?repos=aaryansinghchauhan09%2Fsigmaos&type=date&legend=top-left">
 <picture>
   <source media="(prefers-color-scheme: dark)" srcset="https://api.star-history.com/chart?repos=aaryansinghchauhan09/sigmaos&type=date&theme=dark&legend=top-left" />
   <source media="(prefers-color-scheme: light)" srcset="https://api.star-history.com/chart?repos=aaryansinghchauhan09/sigmaos&type=date&legend=top-left" />
   <img alt="Star History Chart" src="https://api.star-history.com/chart?repos=aaryansinghchauhan09/sigmaos&type=date&legend=top-left" />
 </picture>
</a>

# 🚀 SigmaOS — Sovereign AI-Native Operating System

<p align="center">
  <strong>An experimental Rust operating-system project under active development</strong><br>
  <em>Learning from Linux, Arch Linux, Linux Mint, Omarchy, Redox OS, and OSDev projects</em>
</p>

<p align="center">
  <a href="#architecture">Architecture</a> •
  <a href="#source-areas">Source areas</a> •
  <a href="#quick-start">Quick Start</a> •
  <a href="#source-areas">Source Areas</a> •
  <a href="#development-priorities">Development Priorities</a> •
  <a href="#contributing">Contributing</a>
</p>

---

## 📖 Overview

SigmaOS is a Rust operating-system codebase containing kernel, userspace, desktop, packaging, and tooling experiments. Many subsystems have unit-tested models, but passing those tests does not establish that the complete system boots or is suitable for installation.

> **Project status:** SigmaOS is not currently documented as a supported general-purpose desktop installation. The ISO builder and boot path need end-to-end validation. Use the commands below to build and test the development code; do not treat a generated ISO or a listed subsystem as proof of a usable OS.

The project focuses on small, reviewable subsystem boundaries, explicit capability status, and repeatable tests. See [project status](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/docs/PROJECT_STATUS.md), [testing and release-readiness guidance](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/wiki/Testing.md), and [reference-project adaptation notes](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/docs/INSPIRATION_AND_REFERENCE_PROJECTS.md).

---

## 🏗️ Architecture

```
SigmaOS/
├── src/          # Rust system, userspace, desktop, storage, and tool modules
├── tests/        # Rust, Python, and integration test sources
├── scripts/      # Build, smoke-test, and maintenance scripts
├── docs/         # Project policy, status, design, and roadmap documents
├── wiki/         # Versioned mirror of the GitHub Wiki
├── Cargo.toml    # Rust package and build configuration
└── Makefile      # Common development commands
```

---

## ✨ Source areas

### Kernel Subsystems

The source tree contains modules for memory management, scheduling, VFS/filesystems, syscall dispatch, interrupts, device drivers, panic handling, and ELF loading. Review [capability status](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/FEATURE_STATUS.toml) and the relevant component page for validation. “Code present” and “unit tests pass” are not the same as kernel integration or hardware support.

### Hardware Drivers

Driver modules include serial, RTC, display, input, storage, USB, audio, and network code. Supported devices and operations must be confirmed in the [feature inventory](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/FEATURE_STATUS.toml) and the [Drivers component page](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/wiki/Hardware-Drivers.md); a driver name alone does not imply tested hardware support.

### User Space

The repository includes shell, core utility, package, and desktop code. Their runnable status and limitations are tracked per component; this README does not promise a ready-to-use userland or desktop session.

---

## 🚀 Quick Start

### Prerequisites

```bash
# Install Rust toolchain
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Optional: install QEMU for boot work once a validated boot image is available
# sudo apt install qemu-system-x86
```

### Build & Test

```bash
# Clone the repository
git clone https://github.com/AaryanSinghChauhan09/SigmaOS.git
cd SigmaOS

# Verify the library compiles
cargo check --lib

# Run library tests
cargo test --lib

# Run repository standalone suites
./run_sigma_tests.sh
```

### Development Commands

```bash
make check    # Static analysis via cargo check
make test     # Execute native test suites
make format   # Verify code formatting
make clean    # Remove build artifacts
make help     # Show all available targets
```

---

## 🔧 Kernel subsystem status

Subsystems are developed in `src/` and exported through the library. Their integration paths are uneven; use the [component index](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/wiki/Home.md) to find implementation links and the checks needed before relying on a subsystem. The project does not yet claim a complete firmware-to-desktop boot path.

---

## 📋 Development priorities

Use the [component roadmaps](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/wiki/Home.md) and [capability matrix](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/CAPABILITY_MATRIX.toml) as the sources for current work. Near-term priorities are to validate the boot-to-shell path, make the installer and update path recoverable, document supported hardware with test evidence, and connect subsystem models to runtime paths. Roadmap items remain proposals until their completion criteria pass.

---

## 🧪 Testing

```bash
# Run library tests
cargo test --lib

# Run repository standalone suites
./run_sigma_tests.sh
```

---

## 🤝 Contributing

Contributions are welcome! Please follow these guidelines:

Follow [CONTRIBUTING.md](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/CONTRIBUTING.md) for the reviewable change workflow, safety expectations, verification commands, and pull-request checklist.

---

## 📚 Documentation

| Document | Description |
|----------|-------------|
| [docs/PRODUCT_VISION.md](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/docs/PRODUCT_VISION.md) | Product vision and manifesto |
| [docs/PROJECT_STATUS.md](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/docs/PROJECT_STATUS.md) | Verified checks, release blockers, and next milestones |
| [FEATURE_STATUS.toml](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/FEATURE_STATUS.toml) | Component capability inventory |
| [FUTURE-DEVELOPMENT-ROADMAP.md](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/FUTURE-DEVELOPMENT-ROADMAP.md) | Development roadmap |
| [docs/OPEN_SOURCE_OS_COMPARATIVE_GAP_ANALYSIS.md](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/docs/OPEN_SOURCE_OS_COMPARATIVE_GAP_ANALYSIS.md) | Comparative design gaps and areas to investigate |
| [WHAT_IS_WORKING_AND_NOT_WORKING.md](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/WHAT_IS_WORKING_AND_NOT_WORKING.md) | Component status tracker |

---

## 🏆 Inspiration & References

See [Inspiration and Reference Projects](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/docs/INSPIRATION_AND_REFERENCE_PROJECTS.md) for concrete adaptation guidance, validation criteria, and a prioritized improvement agenda. These projects are design references; their feature names alone do not establish SigmaOS parity.

SigmaOS draws inspiration from these outstanding projects:

| Project | Inspiration |
|---------|-------------|
| [Linux Kernel](https://github.com/torvalds/linux) | Kernel architecture, VFS, scheduler design, driver model |
| [Omarchy](https://github.com/omacom/omarchy) | Keyboard-first workflows, theme discovery, command palette |
| [os-tutorial](https://github.com/cfenollosa/os-tutorial) | Bootloader patterns, interrupt handling, educational approach |
| [Linux Mint](https://github.com/linuxmint) | Onboarding, update guidance, troubleshooting and recovery |
| [Arch Linux and ArchWiki](https://wiki.archlinux.org/) | Transparent configuration, task-oriented procedures, troubleshooting |
| [Redox OS](https://www.redox-os.org/) | Rust-based microkernel design patterns |
| [xv6](https://github.com/mit-pdos/xv6-riscv) | Clean educational OS design |

---

## 📄 License

SigmaOS is dual-licensed under [MIT](LICENSE) or [GPL-2.0](LICENSE-GPL).

---

<p align="center">
  <strong>Built with 🦀 Rust | Designed for the Future</strong>
</p>

## 🤖 AI Agent Guidelines

SigmaOS provides comprehensive guidelines for AI agents working on various components. Each major subsystem has dedicated agent documentation for autonomous development and improvement.

### Component Agent Documentation

Located in `Agents/`, each subsystem has specialized guidelines:

- **KERNEL_AGENTS.md** - Core kernel subsystems (scheduling, interrupts, syscalls)
- **MEMORY_AGENTS.md** - Memory management (buddy allocator, slab allocator, paging)
- **FILESYSTEM_AGENTS.md** - Virtual filesystem layer and implementations
- **NETWORK_AGENTS.md** - Networking stack (TCP/IP, WireGuard, packet filtering)
- **SECURITY_AGENTS.md** - Security framework (capabilities, seccomp, sandboxing)
- **DESKTOP_AGENTS.md** - Zenith desktop environment and window management
- **PACKAGE_AGENTS.md** - Universal package manager SigmaPkg
- **DISTRO_AGENTS.md** - Linux/BSD distro compatibility and gateway
- **AUDIO_AGENTS.md** - Audio subsystem and sound management
- **BLUETOOTH_AGENTS.md** - Bluetooth stack and GATT client
- **DRIVERS_AGENTS.md** - Hardware drivers (PCIe, NVMe, USB, WiFi)
- **CRYPTO_AGENTS.md** - Cryptographic operations and encryption
- **IPC_AGENTS.md** - Inter-process communication mechanisms
- **ARCH_AGENTS.md** - Architecture portability (x86_64, ARM64, RISC-V)

### Open Source Inspiration

Each component agent document includes:
- Linux kernel references and implementation patterns
- FreeBSD and OpenBSD design inspirations
- Specific improvement opportunities
- Performance optimization strategies
- Security hardening guidelines

### Continuous Improvement

These agent guidelines enable SigmaOS to continuously improve by:
- Learning from open source competitors (Linux, FreeBSD, OpenBSD)
- Implementing best practices from mature operating systems
- Maintaining zero-dependency philosophy
- Ensuring security and performance excellence

For detailed agent guidelines, see [AGENTS.md](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/AGENTS.md) and the component-specific files in [Agents/](Agents/).

---


## 🗺️ Future Development

Use the [component roadmaps](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki) for scoped work and acceptance checks, and the [repository roadmap](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/FUTURE-DEVELOPMENT-ROADMAP.md) for cross-component sequencing. A proposal becomes completed work only after its code path and stated checks are verified.

---
