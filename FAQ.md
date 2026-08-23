# ❓ Frequently Asked Questions

## General

### What is SigmaOS?
SigmaOS is a next-generation operating system built from scratch in **Rust** and **C**. It features a custom kernel, AI-native architecture, post-quantum cryptography, and extensive Linux/BSD compatibility.

### Is SigmaOS based on Linux?
No. SigmaOS has its **own custom kernel** written in Rust/C. However, it is heavily inspired by Linux and aims for high compatibility with the Linux ecosystem (AUR, Flatpak, AppImage, etc.).

### Can I run Linux apps on SigmaOS?
Yes! SigmaOS supports:
- **Flatpak** packages (most Linux GUI apps)
- **AppImage** (portable Linux apps)
- **AUR packages** (Arch User Repository, in progress)
- **sigma-pkg** native packages
- Future: `.deb`, `.rpm`, Snap support

### What architectures are supported?
| Architecture | Status |
|-------------|--------|
| x86_64 (Intel/AMD 64-bit) | ✅ Primary |
| ARM64/AArch64 | ✅ Active |
| RISC-V 64 | 🔬 Experimental |

### What is S-AI?
S-AI is SigmaOS's built-in **AI orchestration system**. It provides:
- Local LLM inference (llama.cpp, Ollama)
- Multi-agent task decomposition
- Sigma Copilot (CLI/GUI assistant)
- AI-driven power management
- Predictive memory prefetching

All processing is local — no data sent to external servers without explicit consent.

---

## Technical

### What filesystem does SigmaOS use by default?
**Btrfs** is the default root filesystem, providing:
- Automatic snapshots and rollback
- Transparent compression (zstd)
- Copy-on-write clones
- Built-in RAID support

### How does the scheduler work?
SigmaOS uses a **hybrid scheduler** combining:
- **EEVDF** (Earliest Eligible Virtual Deadline First) — Linux 6.6 algorithm
- **BORE** (Burst-Oriented Response Enhancer) — CachyOS innovation
- **MLFQ** (Multi-Level Feedback Queue) — for workload classification
- **NUMA-aware work stealing** — for multi-socket systems

### What security features are included?
- Post-quantum cryptography (Kyber-1024 + Dilithium-5)
- SELinux mandatory access control
- AppArmor application sandboxing
- pledge()/unveil() OpenBSD-style capability restrictions
- eBPF-based firewall
- TPM 2.0 secure boot chain
- Zero-trust networking

### What is the memory architecture?
- **x86_64**: 4-level page tables (PML4), huge pages (2MB/1GB)
- **Buddy allocator**: Physical memory zones
- **Slab allocator**: Object caches for kernel structures
- **kswapd**: Background memory reclamation
- **KSM**: Same-page merging for VMs/containers
- **CoW**: Fork-on-write for efficient process creation

### Does SigmaOS support containers?
Yes! OCI-compatible containers with:
- Linux namespaces (PID, net, mnt, UTS, IPC, user)
- cgroups v2 resource limits
- OverlayFS for layer storage
- Seccomp-BPF syscall filtering
- Landlock LSM filesystem restriction

---

## Installation

### System Requirements

| | Minimum | Recommended |
|--|---------|-------------|
| CPU | x86_64 dual-core | x86_64/ARM64 quad-core |
| RAM | 4 GB | 16 GB |
| Storage | 20 GB SSD | 100+ GB NVMe |
| GPU | Any | Vulkan-compatible |
| Firmware | UEFI | UEFI with Secure Boot |

### How do I install SigmaOS?
Currently available methods:
1. **Build from source** (developers): `cargo build && make iso`
2. **Dev container**: Docker-based development environment
3. **Live ISO** (coming soon): Boot-and-install experience

### Can I run SigmaOS in a VM?
Yes! QEMU/KVM is the recommended VM environment.
```bash
qemu-system-x86_64 -enable-kvm -m 4G -smp 4 sigmaos.iso
```

---

## Development

### How do I contribute?
See the [[Contributing]] wiki page for a detailed guide.

### Where is the roadmap?
See the [[Roadmap]] wiki page.

### How do I report bugs?
Open a [GitHub Issue](https://github.com/AaryanSinghChauhan09/SigmaOS/issues) with:
- SigmaOS version
- Steps to reproduce
- Expected vs actual behavior
- System information

### How do I report security vulnerabilities?
Use [GitHub Security Advisories](https://github.com/AaryanSinghChauhan09/SigmaOS/security/advisories) for responsible disclosure.
