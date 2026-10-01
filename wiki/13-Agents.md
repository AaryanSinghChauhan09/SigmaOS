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

For detailed agent guidelines, see the component-specific files in the [Agents/](../Agents/) folder.
