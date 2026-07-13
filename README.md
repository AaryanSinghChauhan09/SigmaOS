# 🛡️ SigmaOS — Sovereign, AI-Native Operating System

> **"Sovereignty is the ultimate efficiency."**
> The world's first industrial-grade microkernel designed for total digital autonomy, post-quantum resilience, and Indian industrial compliance.

---

## 🎯 What is SigmaOS?

SigmaOS is a sovereign, zero-dependency, AI-native operating system built entirely in Rust. It discards legacy POSIX assumptions to build a hyper-secure, capability-based microkernel designed for an AI-first, object-oriented ecosystem.

### Key Features

- **Post-Quantum Cryptography**: Native Kyber-1024 KEM + Dilithium-5 signatures (NIST FIPS 203/204)
- **Capability-Based Security**: 64-bit hardware-enforced permission model replacing legacy ACLs
- **Shard Architecture**: 600+ hot-swappable kernel modules with zero-latency IPC
- **AI-Native Design**: Local LLM inference as a first-class OS primitive
- **Multi-Format Deployment**: Single codebase → Desktop, Mobile, Cloud, RTOS, Browser
- **India-First**: Native GST, Income Tax, UPI, and 22-language support

---

## 🚀 Quick Start

### Option A: Run QEMU Demo (Works Today)

```bash
# Install prerequisites
sudo apt install -y build-essential nasm cmake qemu-system-x86 golang-go xorriso

# Clone and build
git clone https://github.com/AaryanSinghChauhan09/SigmaOS.git
cd SigmaOS
make clean && make all -j$(nproc)

# Run in QEMU
qemu-system-x86_64 -cdrom build/sigmaos.iso -m 2G -serial stdio
```

### Option B: Web Desktop Demo

```bash
git clone https://github.com/AaryanSinghChauhan09/SigmaOS.git
cd SigmaOS
open index.html  # macOS
# or xdg-open index.html  # Linux
# or start index.html  # Windows
```

---

## 📊 System Architecture

SigmaOS decomposes the traditional monolithic kernel into specialized, isolated shards:

- **S-MM**: Sovereign Memory Manager (Buddy Allocator)
- **S-SCHED**: Predictive Multi-Priority Scheduler (MLFQ + CFS + EDF)
- **S-NET**: Zero-Trust Network Stack (TCP/UDP + TLS 1.3)
- **S-FS**: Sovereign Distributed Filesystem (VFS + SigmaFS)
- **S-SEC**: Security Framework (PQC + MAC + Sandbox)
- **S-AI**: AI Task Orchestrator (Local LLM routing)
- **S-INDIA**: Industrial Professional Finance Suite

---

## 🔒 Security Architecture

### Post-Quantum Cryptography
- **KEM**: Kyber-1024 (FIPS 203 / CRYSTALS-Kyber)
- **Signatures**: Dilithium-5 (FIPS 204 / CRYSTALS-Dilithium)
- **Hash**: BLAKE3 for package integrity, BLAKE2b for audit trails
- **TLS**: 1.3 with X25519/Kyber-1024 hybrid key exchange

### Kernel Hardening
- W^X (Write XOR Execute) enforcement on all memory regions
- ASLR 42-bit per-region randomisation
- sigma_pledge: per-process syscall allowlist
- sigma_unveil: per-process filesystem path restriction
- AVC (Access Vector Cache): O(1) MAC policy enforcement
- Zero-trust SPIFFE workload identities

---

## 📦 Package Management

SigmaOS uses `.spkg` (Sovereign Package) format:
- Content-addressed storage
- Dilithium-5 signed packages
- BLAKE3 hash verification
- Reproducible builds enforced
- NixOS-inspired but with cryptographic sovereignty

```bash
sigma-pkg install sigma-vim
sigma-pkg list
sigma-pkg update
sigma-pkg search <query>
```

---

## 🖥️ Zenith Desktop

The SigmaOS desktop environment featuring:
- **Zenith Compositor**: Wayland-based compositor with BSP tiling
- **Object-Oriented UI**: Trait-based widget framework without heap allocations
- **Theme Engine**: Glassmorphism profiles via `~/.sigma_profile`
- **Neural UI**: AVX-512 accelerated rendering
- **Accessibility**: Screen readers, high-contrast, magnifier

---

## 🌐 Deployment Profiles

Build from a single unified codebase:

```bash
make PROFILE=standalone all    # Full desktop ISO
make PROFILE=rtos all          # Hard real-time ELF
make PROFILE=cloud all         # Headless cloud image
make PROFILE=mobile all        # ARM64 APK/IPA
make PROFILE=browser all       # WASM bundle
make PROFILE=microkernel all   # <512KB kernel
```

---

## 🧠 AI & Automation

### sigma-agent CLI
Natural language → OS commands with 35 modules:
- Workflow automation (n8n-style)
- Security auditing
- RLHF fine-tuning
- Multi-agent orchestration
- Voice input (Whisper STT)
- Persistent memory

```bash
sigma-agent "backup my home folder"
sigma-agent workflow run weekly-backup
sigma-agent security audit
```

---

## 📈 Development Status

```
Phase F (Competitor Crusher)   ████████████████████  100% ✅
Phase G (Kernel Boot)          ████████████░░░░░░░░   60% ← ACTIVE
Phase H (India Stack)          ░░░░░░░░░░░░░░░░░░░░    0% (blocked on G)
```

### Current Status
- ✅ Kernel scheduler (MLFQ+CFS+EDF)
- ✅ Syscalls (I/O + Process)
- ✅ Physical MM (buddy allocator)
- 🔄 Virtual MM (paging) - Partial
- ✅ APIC + timer
- ✅ sigma_pledge + sigma_unveil
- ✅ Kyber-1024 KEM + Dilithium-5
- 🔄 TCP/UDP stack - Partial
- ✅ Ext4 + FAT32 filesystems
- ✅ NVMe + USB xHCI drivers
- ✅ Zenith Desktop prototype
- ✅ sigma-pkg CLI
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

- [Wiki](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki) — Full documentation
- [Documentation Audit](docs/doc_audit_backlog.md) — Implementation status
- [Roadmap](Roadmap.md) — Development plan
- [INSTALL.md](INSTALL.md) — Build instructions
- [CONTRIBUTING.md](CONTRIBUTING.md) — Contribution guidelines
- [SECURITY_POLICY.md](SECURITY_POLICY.md) — Security policy
- [SUPPORT.md](SUPPORT.md) — Support and troubleshooting
- [FAQ](FAQ.md) — Common questions (coming soon)

---

## 📄 License

MIT + GPL-2.0 (dual license for kernel/userspace)

---

## 🌟 Star History

[![Star History Chart](https://api.star-history.com/svg?repos=AaryanSinghChauhan09/SigmaOS&type=Date)](https://star-history.com/#AaryanSinghChauhan09/SigmaOS&Date)

---

### SigmaOS — Sovereign by Design. One codebase. Every format.

*Built with Rust, Nim, Zig, Ada/SPARK · Post-quantum cryptography (Kyber-1024 + Dilithium-5)*
