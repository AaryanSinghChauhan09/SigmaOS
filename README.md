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

**Networking & Storage:**
- 🔄 TCP/UDP stack - Partial
- ✅ Ext4 + FAT32 filesystems
- ✅ NVMe + USB xHCI drivers

**Desktop & Productivity:**
- ✅ Zenith Desktop prototype
- ✅ Screen recorder with GPU acceleration
- ✅ Screenshot tool with annotation
- ✅ Calendar + task manager
- ✅ Email client with IMAP/SMTP
- ✅ Note-taking app with Markdown
- ✅ Code editor with LSP support
- ✅ Integrated terminal
- ✅ Clipboard manager with history
- ✅ Task manager

**Security:**
- ✅ Encrypted file vault
- ✅ Password manager with biometric unlock
- ✅ Intrusion detection system
- ✅ Secure VPN client
- ✅ Capability-based security framework

**System Tools:**
- ✅ File manager
- ✅ Archive manager
- ✅ Disk usage analyzer
- ✅ System monitor
- ✅ Process manager
- ✅ Virtual machine manager (QEMU/KVM)
- ✅ Container manager (Docker/Podman)

**Package Management:**
- ✅ sigma-pkg CLI
- ✅ Universal package manager
- ✅ Rollback package snapshots

**Networking:**
- ✅ Cloud sync engine
- ✅ Built-in torrent client
- ✅ Network traffic analyzer

**AI & Automation:**
- ✅ AI orchestrator for system optimization

**Customization:**
- ✅ Unified control center
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

- [Future Development & Distro-Parity Roadmap](FUTURE-DEVELOPMENT-ROADMAP.md) — Strategic roadmap detailing gaps & improvements vs mainstream Linux distros
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

## 📄 License

Dual-licensed under MIT and GPL-2.0. See the `LICENSE` file for details.
