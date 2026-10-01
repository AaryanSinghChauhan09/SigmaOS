# SigmaOS AI Agent Roadmap: Missing Linux & BSD Subsystems (`AGENT.md`)

This document provides autonomous AI agents working on SigmaOS with a comprehensive architectural guide and future development roadmap to close all remaining feature, protocol, and driver gaps between SigmaOS and major Linux & BSD distributions (Arch Linux, Debian, Fedora, FreeBSD, OpenBSD, NetBSD, DragonFly BSD, Alpine, NixOS, openSUSE, Gentoo, Ubuntu, Illumos/Solaris).

---

## 1. Subsystem Parity Goals & Guidelines

SigmaOS aims for 100% architectural and operational parity with open-source OS ecosystems while adhering to strict sovereign design principles:
- **Language & Safety**: 100% Safe Rust in kernel and system space (unsafe permitted only for bare-metal hardware MMIO/register access).
- **Zero Third-Party Dependencies**: No external C/C++ libraries or non-Rust runtimes. All drivers, protocols, and parsers are implemented natively.
- **Tri-Agent Framework**:
  - ⚡ **Bolt**: Latency reduction, zero-copy memory buffers, and lock-free SPSC primitives.
  - 🎨 **Palette**: Zenith Desktop, Wayland protocols, accessibility, and Omarchy theme synchronization.
  - 🛡️ **Sentinel**: Security hardening, Landlock v5, OpenBSD pledge/unveil, and PQC cryptographic verification.

---

## 2. Missing Components & Development Roadmap

### Tier 1: Linux Kernel & Distro Innovations

| Component / Subsystem | Inspired By | Target Modules | Implementation Status & Goals |
| :--- | :--- | :--- | :--- |
| **Systemd 256/258 Parity** | Fedora / RHEL / Arch | `src/init/`, `src/distro/` | Complete userland integration for `systemd-homed` LUKS home directories, `systemd-vmspawn` microVM instantiation, and Varlink IPC protocol handlers. |
| **sched_ext BORE v2 Scheduler** | CachyOS / Linux 6.12 | `src/kernel/`, `src/distro/` | eBPF sched_ext CPU scheduler policy governor with dynamic burst-aware timeslices and P-Core/E-Core microarchitecture topology auto-pinning. |
| **ABRoot & APX Containers** | Vanilla OS | `src/distro/` | Immutable atomic A/B rootfs image transaction engine, cryptographic payload checksum verifier, and unprivileged APX subsystem container execution. |
| **Hermetic CAS Store & Flakes** | NixOS / GNU Guix | `src/sigpkg/`, `src/distro/` | Content-Addressable Store (CAS) closure validator, NAR archive integrity verifier, generation history management, and zero-copy hardlink deduplication. |
| **Stateless Config Architecture** | Clear Linux / NixOS | `src/system/`, `src/distro/` | Isolation of vendor defaults in `/usr/share/defaults` and dynamic user override resolution in `/etc` for instant factory resets. |
| **EAPI 8 Portage & USE Flags** | Gentoo | `src/distro/`, `src/package/` | Subslot rebuild trigger tracking, conditional dependency expression solver, and package-level vs. global USE flag resolution. |
| **Bcachefs Multi-Tiered Storage** | Linux Kernel 6.7+ | `src/storage/`, `src/distro/` | Automatic promotion/demotion tiering between Fast SSDs and Slow HDDs based on access frequency, with background scrub and self-healing. |

---

### Tier 2: BSD Subsystems & Security Innovations

| Component / Subsystem | Inspired By | Target Modules | Implementation Status & Goals |
| :--- | :--- | :--- | :--- |
| **FreeBSD 14/15 VNET & bhyve** | FreeBSD | `src/network/`, `src/virt/` | Per-jail independent network stack routing tables, epair virtual interfaces, and bhyve PCIe hardware passthrough with VirtIO device emulation. |
| **Pinned Syscalls & Fine IBT** | OpenBSD 7.6 / 7.8 | `src/security/`, `src/kernel/` | Enforcing registered executable code region boundaries for syscall execution (`pinsyscall`), Fine-Grained Indirect Branch Tracking (IBT), and capability locking. |
| **HAMMER2 Multi-Master PFS Sync** | DragonFly BSD | `src/distro/`, `src/storage/` | Clustering quorum consensus across HAMMER2 Pseudo Filesystems (PFS), automatic read-only failover upon network partition, and Emergency CoW deduplication. |
| **Rump Kernel & Veriexec** | NetBSD 10 | `src/distro/`, `src/security/` | Executing kernel drivers in userland sandboxes via rump hypercalls, in-kernel SHA-256/SHA-512 executable fingerprint auditing, and Veriexec enforcement. |
| **DTrace & Crossbow VNICs** | Illumos / Solaris | `src/observability/`, `src/net/` | Dynamic kernel/userland probe providers (FBT, SDT, Profile), aggregation metrics (Count, Sum, Avg), and virtual network interface (VNIC) etherstub routing. |
| **Softraid AES-XTS & bioctl** | OpenBSD / NetBSD | `src/crypto/`, `src/storage/` | Softraid full-disk AES-XTS / ChaCha20-Poly1305 encrypted RAID volume management and bioctl hardware/software storage controller management. |

---

## 3. Verification & CI Workflow Guidelines

Before submitting PRs, AI agents must execute:
```bash
# 1. Verify compilation and lib correctness
cargo check --lib

# 2. Run standalone test for distro next-gen innovations
mkdir -p build
rustc --test src/distro/sovereign_linux_bsd_distro_next_gen_innovations.rs --edition=2021 -o build/test_next_gen_distro
./build/test_next_gen_distro

# 3. Execute the full SigmaOS test suite runner
./run_sigma_tests.sh
```

---

*SigmaOS AI Agent Directive — Maintain 100% Test Pass Rates & Zero Compilation Warnings.*
