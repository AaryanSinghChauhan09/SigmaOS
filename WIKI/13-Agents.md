# AI Agent Guidelines

SigmaOS provides comprehensive guidelines for AI agents working on various components to enable autonomous development and continuous improvement.

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

For detailed agent guidelines, see the component-specific files in the [Agents directory](https://github.com/AaryanSinghChauhan09/SigmaOS/tree/main/Agents).

## AI maintenance workflow

Use the repository implementation and its current checks as the source of truth. Treat roadmap entries and feature names as plans until the code path is present, wired into the running system, and verified. Do not describe a model, stub, or standalone compiler as an operating-system feature that is enforced at runtime.

| Topic | Canonical implementation | Maintainer guide |
| --- | --- | --- |
| Boot and installation | `src/boot/`, `src/installer/` | `Agents/ARCH_AGENTS.md` |
| Kernel and system calls | `src/kernel/`, `src/syscall/` | `Agents/KERNEL_AGENTS.md` |
| Memory | `src/memory/` | `Agents/MEMORY_AGENTS.md` |
| Filesystems | `src/vfs/`, `src/filesystem/` | `Agents/FILESYSTEM_AGENTS.md` |
| Networking | `src/net/`, `src/network/`, `src/networking/` | `Agents/NETWORK_AGENTS.md` |
| Security and cryptography | `src/security/`, `src/auth/` | `Agents/SECURITY_AGENTS.md`, `Agents/CRYPTO_AGENTS.md` |
| Desktop and accessibility | `src/desktop/`, `src/accessibility/` | `Agents/DESKTOP_AGENTS.md` |
| Packages and distro compatibility | `src/sigpkg/`, `src/userland/pkg/`, `src/distro/`, `src/compatibility/` | `Agents/PACKAGE_AGENTS.md`, `Agents/DISTRO_AGENTS.md` |
| Drivers, audio, Bluetooth, IPC | `src/drivers/`, `src/usb/`, `src/audio/`, `src/bluetooth/`, `src/ipc/` | Matching `Agents/*_AGENTS.md` files |

For each change, read the matching component guide, trace the call path to its integration point, preserve safe Rust and secure defaults, and avoid new dependencies unless the repository owner has approved them. Update the single canonical topic page and its repository mirrors together. Keep titles and headings in sentence case, start sections at level 2, use concise factual prose, and put related links in a final “See also” section. Keep roadmap entries in historical oldest-first order; never delete a design note until the feature is verified and its content has been transferred to the canonical wiki topic.

Before publishing, run `cargo fmt --check`, `cargo check --lib`, `cargo test --lib`, and `./run_sigma_tests.sh`. Run `pytest tests/` when pytest is available. Report unavailable checks and warnings; do not mark features complete solely because a test or name exists.
