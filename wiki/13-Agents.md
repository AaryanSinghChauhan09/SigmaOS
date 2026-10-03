# AI Agent Guidelines & Operating Rules

SigmaOS provides comprehensive guidelines for AI agents working on various components to enable autonomous development, continuous improvement, and multi-distro parity.

## Primary Task Guidelines & Rules

1. **Universal Multi-Distro PR Package Management**:
   - Every foreign package format (.deb, .pkg.tar.zst, .rpm, .apk, .ebuild, .xbps, .pkg, .nix, .flatpak, .snap, .appimage, etc.) MUST be transpiled and handled natively by `sigma-pkg` in Pull Request (PR) format.
   - Foreign dependencies MUST be mapped to canonical `sovereign-*` system package names (e.g. `glibc`/`musl` -> `sovereign-libc`, `openssl-devel`/`libssl-dev` -> `sovereign-openssl`).
   - Every PR package import MUST generate SLSA Provenance v1.0 attestations, CycloneDX/SPDX SBOM metadata, and unified diff manifest summaries.

2. **Zero-Dependency Bare-Metal Architecture**:
   - Kernel and system code MUST be written in `#![no_std]` safe Rust with zero external third-party crate dependencies.
   - All data structures and system abstractions MUST rely on custom `klib` primitives or `alloc::` primitives.

3. **Multi-Distro Subsystem Parity**:
   - SigmaOS absorbs innovations from Linux (CFS/EEVDF scheduler, io_uring, eBPF, cgroups v2, OverlayFS, PipeFS, Landlock) and BSD (FreeBSD Capsicum, Jails, GEOM, RCTL, OpenBSD pledge, unveil, PF, CARP, NetBSD Rump).
   - Component improvements MUST maintain 100% test passing status across all standalone unit tests (`./run_sigma_tests.sh`).

## Tri-Agent Framework

SigmaOS employs a three-agent autonomous continuous development framework:

### ⚡ Bolt: Performance Agent
Identifies and implements focused, measurable performance improvements to make SigmaOS faster, lighter, and more memory-efficient.

### 🎨 Palette: UX & Accessibility Agent
Enhances Zenith Desktop, Web UI, and CLI user interfaces with accessible, intuitive, and delightful user interactions.

### 🛡️ Sentinel: Security Agent
Protects SigmaOS kernel and userland from security vulnerabilities, privilege escalation, memory unsafety, and data leaks.

## Component-Specific Agent Guidelines

Located in the `Agents/` folder, each major subsystem has dedicated agent documentation:

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

## Open Source Inspiration

Each component agent document includes:
- Linux kernel references and implementation patterns
- FreeBSD and OpenBSD design inspirations
- Specific improvement opportunities
- Performance optimization strategies
- Security hardening guidelines

## Continuous Improvement

These agent guidelines enable SigmaOS to continuously improve by:
- Learning from open source competitors (Linux, FreeBSD, OpenBSD)
- Implementing best practices from mature operating systems
- Maintaining zero-dependency philosophy
- Ensuring security and performance excellence

For detailed agent guidelines, see the component-specific files in the [Agents/](../Agents/) folder.
