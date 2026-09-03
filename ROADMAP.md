# SigmaOS Roadmap

> Living document — updated as milestones are reached.

***

## Current Version: v0.1.0 — Sovereign Foundation

### ✅ Completed Features

#### Kernel Core

*   \[x] BuddyAllocator — physical page allocator (binary buddy system)
*   \[x] SlabAllocator — per-CPU object cache allocator
*   \[x] 4-level paging with W^X enforcement (x86\_64)
*   \[x] Hybrid CFS+EDF scheduler
*   \[x] NUMA-aware memory allocation
*   \[x] Custom Vec with bulk-copy optimisation (`extend_from_slice`)
*   \[x] Custom SigmaString with trim allocation optimisation
*   \[x] Zero-std HashMap, HashSet, BTreeMap in klib
*   \[x] Async runtime (`src/klib/async_runtime.rs`)
*   \[x] Merkle tree for package integrity
*   \[x] JSON/TOML parsers (zero external deps)
*   \[x] UUID generation, Base64, PRNG

#### Security Subsystem

*   \[x] OpenBSD pledge/unveil implementation
*   \[x] FreeBSD Capsicum capability-mode sandboxing
*   \[x] FreeBSD-style Jails with nested hierarchies
*   \[x] SELinux type-enforcement MAC
*   \[x] Post-quantum cryptography enclave (CRYSTALS-Kyber)
*   \[x] W^X memory policy enforcement
*   \[x] KASLR (Kernel Address Space Layout Randomisation)
*   \[x] KARL (Kernel Address Randomised Link, OpenBSD-style)
*   \[x] Retguard return-address canaries
*   \[x] Seccomp/BPF-style syscall filtering
*   \[x] AI anomaly detection (`src/security/ai_anomaly_detection.rs`)
*   \[x] Hardware-backed TPM measurement log

#### Package Manager (sigpkg)

*   \[x] Universal multi-format package adapter
*   \[x] .pkg.tar.zst (Arch), .deb (Debian), .rpm (Fedora), .apk (Alpine)
*   \[x] Gentoo ebuild, NixOS expressions, FreeBSD ports
*   \[x] SAT-based dependency solver
*   \[x] PKGBUILD recipe parser + SigmaMakePkg builder
*   \[x] Content-addressed package store
*   \[x] Atomic transactions with rollback
*   \[x] AUR bridge engine

#### Distro Parity

*   \[x] Arch Linux: rolling release, AUR, pacman-compat
*   \[x] NixOS: declarative config, atomic upgrades, generations
*   \[x] Gentoo: USE flags, Portage resolver, ebuilds
*   \[x] Fedora: Cockpit, PipeWire, FreeIPA/Kerberos, Anitya monitor
*   \[x] CachyOS: BORE scheduler, LLVM PGO/BOLT, x86-64-v3
*   \[x] Debian/Ubuntu: apt-compat, dpkg parser
*   \[x] Alpine Linux: musl libc parity, apk adapter
*   \[x] FreeBSD: Capsicum, jails, PF firewall, ZFS parity
*   \[x] OpenBSD: pledge, unveil, W^X, KARL
*   \[x] DragonFly BSD: HAMMER2 B-tree FS parity
*   \[x] openSUSE: Snapper CoW snapshots, zypper compat

#### Desktop

*   \[x] Zenith Compositor (direct framebuffer, no Wayland/X11)
*   \[x] HiDPI fractional scaling
*   \[x] VRR (Variable Refresh Rate)
*   \[x] Sway/i3 tiling window manager
*   \[x] MATE Desktop parity (Betsy)
*   \[x] Gamescope direct scanout

#### Documentation

*   \[x] README.md
*   \[x] CONTRIBUTING.md
*   \[x] ARCHITECTURE.md
*   \[x] ROADMAP.md (this file)
*   \[x] SECURITY.md
*   \[x] CHANGELOG.md
*   \[x] INSTALL.md
*   \[x] docs/KERNEL.md
*   \[x] docs/PACKAGE\_MANAGER.md
*   \[x] docs/DISTRO\_COMPAT.md

***

## v0.2.0 — Stability & Compatibility (Target: Q4 2026)

### Kernel

*   \[ ] Fully functional SMP (symmetric multi-processing) — tested on 64 cores
*   \[ ] io\_uring v2 — batch syscall rings for high-throughput I/O
*   \[ ] eBPF runtime — in-kernel programmable hooks
*   \[ ] kTLS — kernel-space TLS termination for zero-copy HTTPS
*   \[ ] VFIO — PCI passthrough for virtualisation
*   \[ ] IOMMU support (VT-d, AMD-Vi)
*   \[ ] Fuseblk — filesystem-in-userspace bridge

### POSIX Compliance

*   \[ ] POSIX.1-2017 libc (100% LTP test pass rate)
*   \[ ] pthreads with priority inheritance
*   \[ ] POSIX shared memory (`shm_open`, `mmap`)
*   \[ ] POSIX message queues
*   \[ ] Full signal delivery semantics

### Networking

*   \[ ] DPDK-style zero-copy network stack
*   \[ ] QUIC/HTTP3 in-kernel support
*   \[ ] WireGuard kernel module
*   \[ ] RDMA (Remote DMA) over InfiniBand
*   \[ ] BGP routing daemon integration

### Filesystem

*   \[ ] SigmaFS v1 — production-ready CoW B-tree filesystem
*   \[ ] Full Btrfs read/write with subvolumes
*   \[ ] ZFS pool import/export (OpenZFS parity)
*   \[ ] FUSE compatibility layer
*   \[ ] NFS v4.2 server and client

### Security

*   \[ ] Landlock LSM (fine-grained filesystem sandboxing)
*   \[ ] Hardware-backed key storage (HSM integration)
*   \[ ] Automatic exploit mitigation: CET (Control-flow Enforcement Technology)
*   \[ ] Memory tagging (ARMv8.5-MTE)
*   \[ ] CHERI capability hardware integration (experimental)

***

## v0.5.0 — Developer Preview (Target: Q2 2027)

### Containerisation

*   \[ ] Sigma Containers — lightweight Jail+cgroup-based containers
*   \[ ] OCI-compatible container runtime
*   \[ ] Sigma Compose — declarative multi-container orchestration
*   \[ ] Pod networking with eBPF-based CNI

### Virtualisation

*   \[ ] SigmaVM — KVM-compatible hypervisor
*   \[ ] QEMU integration for legacy OS support
*   \[ ] virtio-net, virtio-blk, virtio-fs
*   \[ ] Live migration of running VMs

### Userspace

*   \[ ] SigmaShell v1 — POSIX-compatible shell with dialect transpiler
*   \[ ] SigmaWeb privacy browser (SigmaWeb engine)
*   \[ ] SigmaStore — GUI package manager
*   \[ ] SigmaCode — lightweight code editor (Zenith-native)
*   \[ ] SigmaTerminal — GPU-accelerated terminal emulator

### Cross-Architecture

*   \[ ] Full aarch64 (ARM64) support with hardware testing
*   \[ ] RISC-V 64-bit stability milestone
*   \[ ] POWER9 experimental port
*   \[ ] MIPS64 experimental port

***

## v1.0.0 — Sovereign Release (Target: 2028)

### Mission Criteria

*   \[ ] 100% POSIX.1-2017 compliance (LTP test suite)
*   \[ ] Self-hosting — SigmaOS can compile itself
*   \[ ] Production-grade SMP on commodity hardware
*   \[ ] Formal verification of key kernel modules (pledge, paging)
*   \[ ] Zero known critical CVEs at release
*   \[ ] Comprehensive driver support (NVMe, USB3, PCIe Gen5)
*   \[ ] Full desktop environment with productivity suite

### Ecosystem

*   \[ ] 10,000+ packages in SigmaPkg registry
*   \[ ] Developer SDK with cross-compilation toolchain
*   \[ ] Community edition + enterprise hardened edition
*   \[ ] Cloud image (AWS AMI, Azure, GCP)
*   \[ ] Embedded edition (Raspberry Pi, automotive)

***

## Long-Term Vision

| Feature | Description |
|---------|-------------|
| Exokernel extensions | Application-defined hardware resource management |
| Unikernels | Single-application bootable images |
| Formal verification | Coq/Isabelle proofs for memory isolation |
| AI-native kernel | eBPF-based ML inference hooks in scheduler |
| Post-quantum everywhere | All TLS replaced with CRYSTALS-Kyber by default |
| Hardware capability support | CHERI hardware-enforced memory safety |

***

## Contributing to the Roadmap

To propose new roadmap items, open a GitHub issue with the label `roadmap` and describe:

1.  The feature or improvement
2.  Motivation / use case
3.  Proposed implementation approach
4.  Which existing Linux/BSD distro implements this (if applicable)

See [CONTRIBUTING.md](CONTRIBUTING) for details.
