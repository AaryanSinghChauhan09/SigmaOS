# 🛡️ SigmaOS — Sovereign, AI-Native Operating System

> **"Sovereignty is the ultimate efficiency."**  
> The world's first industrial-grade microkernel designed for total digital autonomy, post-quantum resilience, and Indian industrial compliance.

---

## 🎯 Overview

SigmaOS is a sovereign, zero-dependency, AI-native operating system built entirely in Rust. It discards legacy POSIX assumptions to build a hyper-secure, capability-based microkernel designed for an AI-first, object-oriented ecosystem.

### Core Pillars

- **Post-Quantum Cryptography**: Native Kyber-1024 KEM + Dilithium-5 signatures (NIST FIPS 203/204).
- **Capability-Based Security**: 64-bit hardware-enforced permission model replacing legacy ACLs.
- **Shard Architecture**: 600+ hot-swappable kernel modules with zero-latency IPC.
- **AI-Native Design**: Local LLM inference as a first-class OS primitive.
- **India-First**: Native GST, Income Tax, UPI, and 22-language support.


---

## 📊 System Architecture

SigmaOS decomposes the traditional monolithic kernel into specialized, isolated shards. The interaction between these shards is governed by a capability-enforced transaction bus.

```mermaid
graph TD
    UserLand[Userland Applications] -->|Syscall Capability Gate| KernelGate[S-SEC Security Shard]
    KernelGate -->|Validated Message| Bus[Sovereign IPC Bus]
    Bus --> S-MM[S-MM: Memory Shard]
    Bus --> S-SCHED[S-SCHED: Scheduler Shard]
    Bus --> S-FS[S-FS: Distributed Filesystem]
    Bus --> S-NET[S-NET: Network Shard]
    Bus --> S-AI[S-AI: Local LLM Orchestrator]
```

- **S-MM**: Sovereign Memory Manager (Buddy Allocator).
- **S-SCHED**: Predictive Multi-Priority Scheduler (MLFQ + CFS + EDF).
- **S-FS**: Sovereign Distributed Filesystem (VFS + SigmaFS).
- **S-SEC**: Security Framework (PQC + MAC + Sandbox).
- **S-AI**: AI Task Orchestrator (Local LLM routing).


---

## 🚀 Quick Start

### Running the QEMU Demo (Works Today)

Ensure you have the required compiler toolchain and emulation packages:

```bash

# Install dependencies

sudo apt install -y build-essential nasm cmake qemu-system-x86 golang-go xorriso

# Clone the repository

git clone https://github.com/AaryanSinghChauhan09/SigmaOS.git
cd SigmaOS

# Build the system image

make clean && make all -j$(nproc)

# Run in QEMU

qemu-system-x86_64 -cdrom build/sigmaos.iso -m 2G -serial stdio
```

### Profile Builds

SigmaOS supports declarative compilation profiles specified at build-time:

```bash
make PROFILE=standalone all    # Full desktop ISO
make PROFILE=rtos all          # Hard real-time ELF
make PROFILE=cloud all         # Headless cloud image
make PROFILE=browser all       # WASM bundle
```

---

## 🔒 Security & Sandboxing

SigmaOS features a capability-native access control system. Programs are executed with explicit privilege tokens (capabilities) rather than generic user IDs.

```rust
// Capability delegation example
let token = CapabilityToken::new()
    .allow_network("tcp", 80)
    .allow_read("/var/www");
```

For a detailed review of all security policies, see the canonical [Security Framework](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki) page on the Wiki.

---

## 📚 Canonical Documentation (GitHub Wiki)

```text
Phase F (Competitor Crusher)   ████████████████████  100% ✅
Phase G (Kernel Boot)          ████████████░░░░░░░░   60% ← ACTIVE
Phase H (India Stack)          ░░░░░░░░░░░░░░░░░░░░    0% (blocked on G)
```

### Current Status

**Kernel Core:**
- ✅ Kernel scheduler (MLFQ+CFS+EDF)
- ✅ Syscalls (I/O + Process)
- ✅ Physical MM (buddy allocator)
- 🔄 Virtual MM (paging) - Partial
- ✅ APIC + timer
- ✅ sigma_pledge + sigma_unveil
- ✅ Kyber-1024 KEM + Dilithium-5
- ✅ Kernel Evolution Architecture (OOP-based trait hierarchy)
- ✅ Linux Driver Absorption Engine
- ✅ 5 Abstract Base Traits (DeviceDriver, NetworkStack, FileSystem, MemoryManager, Scheduler)

**Networking & Storage:**
- 🔄 TCP/UDP stack - Partial
- ✅ Ext4 + FAT32 filesystems
- ✅ NVMe + USB xHCI drivers

**Desktop & Productivity:**
- ✅ Zenith Desktop prototype
- 🔄 Screen recorder with GPU acceleration
- 🔄 Screenshot tool with annotation
- 🔄 Calendar + task manager
- 🔄 Email client with IMAP/SMTP
- 🔄 Note-taking app with Markdown
- 🔄 Code editor with LSP support
- ✅ Integrated terminal
- ✅ Clipboard manager with history
- 🔄 Task manager

**Security:**
- ✅ Encrypted file vault
- 🔄 Password manager with biometric unlock
- ✅ Intrusion detection system
- 🔄 Secure VPN client
- ✅ Capability-based security framework

**System Tools:**
- ✅ File manager
- ✅ Archive manager
- ✅ Disk usage analyzer
- ✅ System monitor
- ✅ Process manager
- 🔄 Virtual machine manager (QEMU/KVM)
- 🔄 Container manager (Docker/Podman)

**Package Management:**
- ✅ sigma-pkg CLI
- 🔄 Universal package manager
- 🔄 Rollback package snapshots

**Networking:**
- 🔄 Cloud sync engine
- 🔄 Built-in torrent client
- 🔄 Network traffic analyzer

**AI & Automation:**
- 🔄 AI orchestrator for system optimization

**Customization:**
- 🔄 Unified control center
- ✅ Declarative theming engine

**Boot & Deployment:**
- ⬜ Bootable ISO (Phase G)


---

## 🤝 Contributing

We welcome contributions! See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

### High-Impact Areas

- Round-robin scheduler implementation
- Buddy allocator completion
- sigma-sh REPL
- USB HID keyboard driver
- VESA framebuffer driver
- Package recipes


---

## 📚 Documentation

### Repository Documentation

- [36-Month Master Strategic Roadmap & Gap-Closing Blueprint](SigmaOS_Gap_Closing_Roadmap.md) — Multi-phase strategic blueprint detailing deep architectural gap-closures between Fedora Linux and SigmaOS
- [Future Development & Distro-Parity Roadmap](FUTURE-DEVELOPMENT-ROADMAP.md) — Strategic roadmap detailing gaps & improvements vs mainstream Linux distros
- [Legacy Compatibility & Subsystem Parity Blueprint](LEGACY_COMPATIBILITY_BLUEPRINT.md) — Architectural design and implementation of legacy adapters, bridges, and workload optimizers
- [Documentation Audit](docs/doc_audit_backlog.md) — Implementation status
- [Roadmap](Roadmap.md) — Development plan
- [INSTALL.md](INSTALL.md) — Build instructions
- [CONTRIBUTING.md](CONTRIBUTING.md) — Contribution guidelines
- [SECURITY_POLICY.md](SECURITY_POLICY.md) — Security policy
- [SUPPORT.md](SUPPORT.md) — Support and troubleshooting
- [FAQ](FAQ.md) — Common questions (coming soon)


### GitHub Wiki (Canonical Documentation)

Detailed conceptual documentation is managed exclusively in the GitHub Wiki:

- **Master Roadmap**: [Maturity & Distro-Parity Roadmap](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki/Maturity_Parity_Roadmap)
- **Kernel Evolution**: [Kernel Evolution Architecture](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki/Kernel_Evolution_Architecture)
- **Driver Ecosystem**: [Driver Ecosystem](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki/Driver_Ecosystem)
- **Strategic Planning**: [Gap Filling Strategic Plan](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki/GAP_FILLING_STRATEGIC_PLAN)
- **Advanced Core Architecture**: [Advanced Absorption Matrix](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki/Advanced_Absorption)
- **Filesystem Design**: [SigmaFS Innovations](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki/SigmaFS_Innovations)
- **Interactive UI Compositor**: [SigmaMedia Frameworks](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki/SigmaMedia_Frameworks)
- **Local AI Daemon**: [Sigma AI Agents](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki/Sigma_AI_Agents)
- **Linux Distro Absorption**: [Strategic Distro Absorption Specification](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki/LINUX_DISTRO_ABSORPTION_SPEC)
- **S-Boot Firmware**: [Sovereign BIOS & UEFI Firmware Specification](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki/BIOS_FIRMWARE_SPEC)
- **Zenith Compositor**: [Wayland Zenith UI Specification](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki/WAYLAND_ZENITH_SPEC)
- **Portable Apps**: [Portable Application Format Specification](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki/PORTABLE_APP_FORMAT_PLAN)
- **Custom Personalization**: [Custom Personalization & Theme Specification](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki/CUSTOM_PERSONALIZATION_SPEC)
- **Kernel Performance**: [Kernel Performance Optimization Specification](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki/KERNEL_PERFORMANCE_PLAN)
- **Zig Driver Integration**: [Zig Language Driver Integration Specification](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki/ZIG_INTEGRATION_PLAN)
- **Nim Driver Integration**: [Nim Language Driver Integration Specification](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki/NIM_INTEGRATION_PLAN)


---

## 🏛️ Design Specification & Architecture Layers (Zenith Release Microkernel)

Dual-licensed under MIT and GPL-2.0. See the `LICENSE` file for details.
||||||| 984d1301f
Dual-licensed under MIT and GPL-2.0. See the `LICENSE` file for details.
# SigmaOS Zenith (v15.2 - Release Microkernel)

The Sovereign Industrial Microkernel.

This branch represents the core modular microkernel layout of SigmaOS, structured to align with established Linux distribution layouts for robustness, isolation, and silicon-direct execution.

---

## 🏛️ Design Specification & Architecture Layers

SigmaOS is organized into isolated functional layers to guarantee complete safety and hardware-isolation boundary conditions:

### 1. Kernel Layer (`/kernel/`)
- **Process Scheduler**: Multi-level Feedback Queue (MLFQ) and Round-Robin scheduler handling task priorities and time-slice yields.
- **Memory Management**: Physical Page Frame Allocator (PMM) and Virtual Memory Paging (VMM) supporting 4-level paging tables.
- **Hardware Drivers**: Low-level abstractions for COM1 serial logs, PS/2 keyboards, standard VGA text mode, and ATA disk sector operations.

### 2. Standard Libraries (`/lib/`)
- **Sovereign Libc**: Independent, zero-dependency C11 standard library implementation providing `sigma_printf`, memory manipulators (`memcpy`, `memset`), string utilities, and attestation helpers (`crc32`).

### 3. Init System (`/init/`)
- **PID 1 Bootstrap**: Orchestrates clean startup sequences using Runlevels (1 to 5) to boot vital telemetry, load the virtual file system, initialize the TCP/IP stack, and spawn the user shell in order.

### 4. Virtual File System (`/fs/`)
- **VFS Interface**: Standardizes operations like `open`, `close`, `read`, and `write` via file descriptor tables and inode indexing.
- **Ext4/FAT32 Drivers**: Handles block storage, reads superblock states, and walks clusters.

### 5. Networking Stack (`/net/`)
- **Loopback NIC**: Direct virtual hardware interface loopback (`lo` at `127.0.0.1`).
- **TCP/IP Suite**: Custom TCP 3-way handshake state machine and UDP port binding.
- **DNS Lookup**: Local resolver mapping domain endpoints to IPv4 destinations.

### 6. Userland utilities (`/usr/`)
- **sh Shell**: Interactive CLI command execution environment mapping user inputs to system calls.

---

## 🛠️ Build, Test, & Execution Instructions

### Dependencies
- Make, NASM assembler, GCC, QEMU

### 1. Compile all Modular Subsystems
```bash
make clean
make all
```

### 2. Running the Emulator
```bash
qemu-system-x86_64 -cdrom build/sigmaos.iso -serial stdio -m 2G
```

### 3. Running Unit Tests
```bash
npm run test
```
All unit tests in `/tests` must return green states before submitting patches.
Dual-licensed under MIT and GPL-2.0. See the `LICENSE` file for details.
||||||| 984d1301f
Dual-licensed under MIT and GPL-2.0. See the `LICENSE` file for details.
This section represents the core modular microkernel layout of SigmaOS, structured to align with established Linux distribution layouts for robustness, isolation, and silicon-direct execution.

SigmaOS is organized into isolated functional layers to guarantee complete safety and safety-critical isolation boundary conditions:

## 🏛️ Design Specification & Architecture Layers

SigmaOS is organized into isolated functional layers to guarantee complete safety and hardware-isolation boundary conditions:

### 1. Kernel Layer (`/kernel/`)
- **Process Scheduler**: Multi-level Feedback Queue (MLFQ) and Round-Robin scheduler handling task priorities and time-slice yields.
- **Memory Management**: Physical Page Frame Allocator (PMM) and Virtual Memory Paging (VMM) supporting 4-level paging tables.
- **Hardware Drivers**: Low-level abstractions for COM1 serial logs, PS/2 keyboards, standard VGA text mode, and ATA disk sector operations.

### 2. Standard Libraries (`/lib/`)
- **Sovereign Libc**: Independent, zero-dependency C11 standard library implementation providing `sigma_printf`, memory manipulators (`memcpy`, `memset`), string utilities, and attestation helpers (`crc32`).

### 3. Init System (`/init/`)
- **PID 1 Bootstrap**: Orchestrates clean startup sequences using Runlevels (1 to 5) to boot vital telemetry, load the virtual file system, initialize the TCP/IP stack, and spawn the user shell in order.

### 4. Virtual File System (`/fs/`)
- **VFS Interface**: Standardizes operations like `open`, `close`, `read`, and `write` via file descriptor tables and inode indexing.
- **Ext4/FAT32 Drivers**: Handles block storage, reads superblock states, and walks clusters.

### 5. Networking Stack (`/net/`)
- **Loopback NIC**: Direct virtual hardware interface loopback (`lo` at `127.0.0.1`).
- **TCP/IP Suite**: Custom TCP 3-way handshake state machine and UDP port binding.
- **DNS Lookup**: Local resolver mapping domain endpoints to IPv4 destinations.

### 6. Userland utilities (`/usr/`)
- **sh Shell**: Interactive CLI command execution environment mapping user inputs to system calls.

---

## 🛠️ Build, Test, & Execution Instructions

### Dependencies
- Make, NASM assembler, GCC, QEMU

### 1. Compile all Modular Subsystems
```bash
make clean
make all
```

### 2. Running the Emulator
```bash
qemu-system-x86_64 -cdrom build/sigmaos.iso -serial stdio -m 2G
```

### 3. Running Unit Tests
```bash
npm run test
```
All unit tests in `/tests` must return green states before submitting patches.


---

## 📄 License

Dual-licensed under MIT and GPL-2.0. See the `LICENSE` file for details.
