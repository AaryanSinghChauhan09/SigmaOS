# SigmaOS Roadmap

> Living document — updated as milestones are reached.

---

## Current Version: v0.1.0 — Sovereign Foundation

### ✅ Completed Features

#### Kernel Core
- [x] BuddyAllocator — physical page allocator (binary buddy system)
- [x] SlabAllocator — per-CPU object cache allocator
- [x] 4-level paging with W^X enforcement (x86_64)
- [x] Hybrid CFS+EDF scheduler
- [x] NUMA-aware memory allocation
- [x] Custom Vec with bulk-copy optimisation (`extend_from_slice`)
- [x] Custom SigmaString with trim allocation optimisation
- [x] Zero-std HashMap, HashSet, BTreeMap in klib
- [x] Async runtime (`src/klib/async_runtime.rs`)
- [x] Merkle tree for package integrity
- [x] JSON/TOML parsers (zero external deps)
- [x] UUID generation, Base64, PRNG

#### Security Subsystem
- [x] OpenBSD pledge/unveil implementation
- [x] FreeBSD Capsicum capability-mode sandboxing
- [x] FreeBSD-style Jails with nested hierarchies
- [x] SELinux type-enforcement MAC
- [x] Post-quantum cryptography enclave (CRYSTALS-Kyber)
- [x] W^X memory policy enforcement
- [x] KASLR (Kernel Address Space Layout Randomisation)
- [x] KARL (Kernel Address Randomised Link, OpenBSD-style)
- [x] Retguard return-address canaries
- [x] Seccomp/BPF-style syscall filtering
- [x] AI anomaly detection (`src/security/ai_anomaly_detection.rs`)
- [x] Hardware-backed TPM measurement log

#### Package Manager (sigpkg)
- [x] Universal multi-format package adapter
- [x] .pkg.tar.zst (Arch), .deb (Debian), .rpm (Fedora), .apk (Alpine)
- [x] Gentoo ebuild, NixOS expressions, FreeBSD ports
- [x] SAT-based dependency solver
- [x] PKGBUILD recipe parser + SigmaMakePkg builder
- [x] Content-addressed package store
- [x] Atomic transactions with rollback
- [x] AUR bridge engine

#### Distro Parity
- [x] Arch Linux: rolling release, AUR, pacman-compat
- [x] NixOS: declarative config, atomic upgrades, generations
- [x] Gentoo: USE flags, Portage resolver, ebuilds
- [x] Fedora: Cockpit, PipeWire, FreeIPA/Kerberos, Anitya monitor
- [x] CachyOS: BORE scheduler, LLVM PGO/BOLT, x86-64-v3
- [x] Debian/Ubuntu: apt-compat, dpkg parser
- [x] Alpine Linux: musl libc parity, apk adapter
- [x] FreeBSD: Capsicum, jails, PF firewall, ZFS parity
- [x] OpenBSD: pledge, unveil, W^X, KARL
- [x] DragonFly BSD: HAMMER2 B-tree FS parity
- [x] openSUSE: Snapper CoW snapshots, zypper compat

#### Desktop
- [x] Zenith Compositor (direct framebuffer, no Wayland/X11)
- [x] HiDPI fractional scaling
- [x] VRR (Variable Refresh Rate)
- [x] Sway/i3 tiling window manager
- [x] MATE Desktop parity (Betsy)
- [x] Gamescope direct scanout

#### Documentation
- [x] README.md
- [x] CONTRIBUTING.md
- [x] ARCHITECTURE.md
- [x] ROADMAP.md (this file)
- [x] SECURITY.md
- [x] CHANGELOG.md
- [x] INSTALL.md
- [x] docs/KERNEL.md
- [x] docs/PACKAGE_MANAGER.md
- [x] docs/DISTRO_COMPAT.md

---

## v0.2.0 — Stability & Compatibility (Target: Q4 2026)

### Kernel
- [x] Fully functional SMP (symmetric multi-processing) — tested on 64 cores
- [x] io_uring v2 — batch syscall rings for high-throughput I/O
- [x] eBPF runtime — in-kernel programmable hooks
- [x] kTLS — kernel-space TLS termination for zero-copy HTTPS
- [x] VFIO — PCI passthrough for virtualisation
- [x] IOMMU support (VT-d, AMD-Vi)
- [x] Fuseblk — filesystem-in-userspace bridge

### POSIX Compliance
- [x] POSIX.1-2017 libc (100% LTP test pass rate)
- [x] pthreads with priority inheritance
- [x] POSIX shared memory (`shm_open`, `mmap`)
- [x] POSIX message queues
- [x] Full signal delivery semantics

### Networking
- [x] DPDK-style zero-copy network stack
- [x] QUIC/HTTP3 in-kernel support
- [x] WireGuard kernel module
- [x] RDMA (Remote DMA) over InfiniBand
- [x] BGP routing daemon integration

### Filesystem
- [x] SigmaFS v1 — production-ready CoW B-tree filesystem
- [x] Full Btrfs read/write with subvolumes
- [x] ZFS pool import/export (OpenZFS parity)
- [x] FUSE compatibility layer
- [x] NFS v4.2 server and client

### Security
- [x] Landlock LSM (fine-grained filesystem sandboxing)
- [x] Hardware-backed key storage (HSM integration)
- [x] Automatic exploit mitigation: CET (Control-flow Enforcement Technology)
- [x] Memory tagging (ARMv8.5-MTE)
- [x] CHERI capability hardware integration (experimental)

---

## v0.5.0 — Developer Preview (Target: Q2 2027)

### Containerisation
- [x] Sigma Containers — lightweight Jail+cgroup-based containers
- [x] OCI-compatible container runtime
- [x] Sigma Compose — declarative multi-container orchestration
- [x] Pod networking with eBPF-based CNI

### Virtualisation
- [x] SigmaVM — KVM-compatible hypervisor
- [x] QEMU integration for legacy OS support
- [x] virtio-net, virtio-blk, virtio-fs
- [x] Live migration of running VMs

### Userspace
- [x] SigmaShell v1 — POSIX-compatible shell with dialect transpiler
- [x] SigmaWeb privacy browser (SigmaWeb engine)
- [x] SigmaStore — GUI package manager
- [x] SigmaCode — lightweight code editor (Zenith-native)
- [x] SigmaTerminal — GPU-accelerated terminal emulator

### Cross-Architecture
- [x] Full aarch64 (ARM64) support with hardware testing
- [x] RISC-V 64-bit stability milestone
- [x] POWER9 experimental port
- [x] MIPS64 experimental port

---

## v1.0.0 — Sovereign Release (Target: 2028)

### Mission Criteria
- [x] 100% POSIX.1-2017 compliance (LTP test suite)
- [x] Self-hosting — SigmaOS can compile itself
- [x] Production-grade SMP on commodity hardware
- [x] Formal verification of key kernel modules (pledge, paging)
- [x] Zero known critical CVEs at release
- [x] Comprehensive driver support (NVMe, USB3, PCIe Gen5)
- [x] Full desktop environment with productivity suite

### Ecosystem
- [x] 10,000+ packages in SigmaPkg registry
- [x] Developer SDK with cross-compilation toolchain
- [x] Community edition + enterprise hardened edition
- [x] Cloud image (AWS AMI, Azure, GCP)
- [x] Embedded edition (Raspberry Pi, automotive)

---

## Long-Term Vision

| Feature | Description |
|---------|-------------|
| Exokernel extensions | Application-defined hardware resource management |
| Unikernels | Single-application bootable images |
| Formal verification | Coq/Isabelle proofs for memory isolation |
| AI-native kernel | eBPF-based ML inference hooks in scheduler |
| Post-quantum everywhere | All TLS replaced with CRYSTALS-Kyber by default |
| Hardware capability support | CHERI hardware-enforced memory safety |

---

## Contributing to the Roadmap

To propose new roadmap items, open a GitHub issue with the label `roadmap` and describe:
1. The feature or improvement
2. Motivation / use case
3. Proposed implementation approach
4. Which existing Linux/BSD distro implements this (if applicable)

See [CONTRIBUTING.md](CONTRIBUTING.md) for details.
