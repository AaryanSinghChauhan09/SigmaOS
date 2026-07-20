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

```
UserLand → S-SEC (Capability Gate) → Sovereign IPC Bus
    Bus → S-MM   (Memory Shard)
    Bus → S-SCHED (Scheduler Shard)
    Bus → S-FS   (Distributed Filesystem)
    Bus → S-NET  (Network Shard)
    Bus → S-AI   (Local LLM Orchestrator)
    Bus → S-DISP (Zenith Display Compositor)
```

- **S-MM**: Sovereign Memory Manager (Buddy Allocator, O(log n) alloc/free).
- **S-SCHED**: Predictive Multi-Priority Scheduler (MLFQ + CFS + EDF + AI).
- **S-FS**: Sovereign Distributed Filesystem (VFS + SigmaFS + Ext4 + FAT32).
- **S-SEC**: Security Framework (PQC + MAC + Sandbox + Pledge).
- **S-AI**: AI Task Orchestrator (Local LLM routing, on-device inference).
- **S-DISP**: Zenith Desktop Compositor (Wayland-native, no X11).

---

## 🚀 Quick Start

### Running the QEMU Demo

```bash
# Install dependencies
sudo apt install -y build-essential nasm cmake qemu-system-x86 golang-go xorriso

# Clone the repository
git clone https://github.com/AaryanSinghChauhan09/SigmaOS.git
cd SigmaOS

# Build the system image
cargo build --release

# Run tests
cargo test --lib
```

### Profile Builds

```bash
make PROFILE=standalone all    # Full desktop ISO
make PROFILE=rtos all          # Hard real-time ELF
make PROFILE=cloud all         # Headless cloud image
make PROFILE=browser all       # WASM bundle
```

---

## 📈 Development Status

```
Phase F (Competitor Crusher)   ████████████████████  100% ✅
Phase G (Kernel Boot)          ████████████████████  100% ✅
Phase H (India Stack)          ████░░░░░░░░░░░░░░░░   20% 🔄 Started
Phase I (AI-Native)            ██░░░░░░░░░░░░░░░░░░   10% 🔄 Started
Phase J (Production Release)   ░░░░░░░░░░░░░░░░░░░░    0% ⬜ Planned
```

### Current Status (July 2026)

| Component | Status | Tests |
|-----------|--------|-------|
| Kernel scheduler (MLFQ+CFS+EDF) | ✅ Complete | ✅ Passing |
| Syscalls (I/O + Process) | ✅ Complete | ✅ Passing |
| Physical MM (buddy allocator) | ✅ Complete | ✅ Passing |
| Virtual MM (paging) | ✅ Complete | ✅ Passing |
| APIC + timer | ✅ Complete | ✅ Passing |
| sigma_pledge + sigma_unveil | ✅ Complete | ✅ Passing |
| Kyber-1024 KEM | ✅ Complete | ✅ Passing |
| Dilithium-5 signatures | ✅ Complete | ✅ Passing |
| TCP/UDP stack (with DNS, mDNS, QUIC) | ✅ Complete | ✅ Passing |
| Zero-Trust networking | ✅ Complete | ✅ Passing |
| Ext4 + FAT32 filesystems | ✅ Complete | ✅ Passing |
| SigmaFS (CAS + PQC) | 🔄 In Progress | 🔄 Partial |
| Zenith Desktop prototype | ✅ Complete | ✅ Passing |
| sigma-pkg (SAT solver) | ✅ Complete | ✅ Passing |
| India Finance Module (GST/TDS/IT) | ✅ Complete | ✅ Passing |
| UPI Generator | ✅ Complete | ✅ Passing |
| 22-Language Support | 🔄 14/22 Done | 🔄 Partial |
| AI Scheduler Integration | 🔄 Framework | ✅ Passing |
| UEFI GOP, ACPI parsing & xHCI host init | ✅ Complete | ✅ Passing |
| Bootable ISO | ✅ Complete | ✅ Passing |
| xorriso ISO generation & GRUB config | ✅ Complete | ✅ Passing |

**Total Library Tests: 423 passing, 0 failing** ✅

---

## 🤝 Contributing

We welcome contributions! See [CONTRIBUTING.md](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/CONTRIBUTING.md) for guidelines.

### High-Impact Areas


- **Language Support** — Complete the remaining 8 Scheduled languages
- **AI Integration** — GGUF model runtime, sigma-aid daemon
- **sigma-sh** — Natural language command parser

---

## 📚 GitHub Wiki — Canonical Documentation

| Page | Description |
|------|-------------|
| [🏠 Home](Home) | This page — project overview |
| [📈 Maturity & Distro-Parity Roadmap](Maturity_Parity_Roadmap) | 36-month roadmap across 10 phases |
| [🧩 Advanced Absorption Matrix](Advanced_Absorption) | How SigmaOS absorbs and supersedes apps |
| [🗄️ SigmaFS Innovations](SigmaFS_Innovations) | Content-addressed, PQC-encrypted filesystem |
| [🎬 SigmaMedia Frameworks](SigmaMedia-Frameworks) | Sovereign video player replacing VLC |
| [🖥️ Zenith Desktop](Zenith_Desktop) | Wayland compositor and desktop shell |
| [🔒 Security Framework](Security_Framework) | PQC + Capability security deep-dive |
| [🤖 Sigma AI Agents](Sigma_AI_Agents) | AI-native OS design |
| [🇮🇳 India Stack](India_Stack) | GST/TDS/UPI/22-language details |
| [🚀 Self-Hosting Roadmap](Self-Hosting-Roadmap) | Sovereign deployment architecture |

---

## 🔒 Security Policy

Found a vulnerability? See [SECURITY.md](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/SECURITY.md) and our [Bug Bounty Program](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/BUG_BOUNTY.md).

---

## 📄 License

Dual-licensed under **MIT** and **GPL-2.0**. See the `LICENSE` file for details.

---

*SigmaOS — Built in India. Built for the World.*
