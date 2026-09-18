# Welcome to the SigmaOS Wiki

**Your source for SigmaOS documentation on the web.**

SigmaOS is a secure, fast, opinionated Rust desktop operating system with atomic updates, capability-based applications, and a curated Zenith workflow.

Visit the [Table of contents](Table-of-contents) for a listing of article categories.

---

## The Distribution

- **[About SigmaOS](About-SigmaOS)**
  Overview of SigmaOS describing what to expect from a SigmaOS system.

- **[Frequently Asked Questions](Frequently-Asked-Questions)**
  Notable questions and facts about the distribution.

- **[SigmaOS Compared to Other Distributions](SigmaOS-Compared-to-Other-Distributions)**
  Summarizes the similarities and differences between SigmaOS and other distributions.

- **[Installation Guide](Installation-Guide)**
  Guide through the process of installing SigmaOS.

- **[General Recommendations](General-Recommendations)**
  Annotated index of post-installation tutorials and other popular articles.

- **[List of Applications](List-of-Applications)**
  Categorized presentation of common applications and packages.

- **[Category:Installation](Category-Installation)**
  Installation-related articles and guides.

---

## The Desktop Environment

- **[Zenith Compositor](Zenith-Compositor)**
  Overview of the Zenith desktop environment and compositor system.

- **[Zenith Desktop & Workflow](Zenith-Desktop-and-Omarchy-Workflow)**
  Detailed guide to the Zenith desktop workflow and Omarchy integration.

- **[Bolt ⚡ Fast Launcher](Zenith-Desktop-and-Omarchy-Workflow#bolt-fast-path-launcher-and-agentic-steering)**
  Fast path launcher and agentic steering system.

- **[Palette 🎨 UI Themes](Zenith-Desktop-and-Omarchy-Workflow#palette-dynamic-theme--color-engine)**
  Dynamic theme and color engine for Zenith.

- **[Sentinel 🛡️ Exec Guard](Zenith-Desktop-and-Omarchy-Workflow#sentinel-sandboxing--exec-guard)**
  Capability-based sandboxing and execution guard.

- **[Category:Desktop-Environment](Category-Desktop-Environment)**
  Desktop environment articles and guides.

---

## Package Management

- **[SigmaPkg - Universal Package Manager](SigmaPkg)**
  Universal package management system with multi-format support.

- **[Universal Package Engine (sigpkg)](Package-Management-and-Sigpkg)**
  Overview of the SigmaPkg package management system.

- **[Arch PKGBUILD & AUR Helper](ARCH_LINUX_PARITY_FEATURES)**
  Integration with Arch Linux package formats and AUR.

- **[Multi-Distro Adapters](UNIVERSAL_PACKAGE_SYSTEM_IMPLEMENTATION_PLAN)**
  Support for multiple Linux and BSD package formats.

- **[Content-Addressed Storage](Content-Addressed-Storage)**
  Immutable, cryptographically-verified package storage.

- **[Category:Package-Management](Category-Package-Management)**
  Package management articles and guides.

---

## System Administration

- **[Services & Supervision](System-Administration-and-Services)**
  Service management and supervision system.

- **[Privilege Delegation (doas)](SECURITY)**
  Secure privilege escalation and delegation.

- **[Networking & WireGuard](networking)**
  Network configuration and WireGuard VPN integration.

- **[Logging & Journalctl](syslog)**
  System logging and journal management.

---

## Security & Hardening

- **[Security Sandbox & Isolation](Security-Sandbox-Isolation)**
  Comprehensive security sandboxing with Landlock, Capsicum, and pledge/unveil.

- **[Security & Sandboxing](Security-Sandboxing-and-Hardening)**
  Comprehensive security model and sandboxing capabilities.

- **[Zorin Exec Guard](Security-Sandboxing-and-Hardening#zorin-exec-guard)**
  Default-deny capability permission model.

- **[Hardware Enclaves & PQC](Security-Sandboxing-and-Hardening#hardware-enclaves--post-quantum-cryptography)**
  Hardware enclaves and post-quantum cryptography support.

- **[Memory Safety Guarantees](SECURITY)**
  Rust-enforced memory safety and kernel hardening features.

- **[Category:Security](Category:Security)**
  Security and hardening articles.


## Performance & Kernel

- **[Demand Paging & Swap](Demand-Paging-and-Swap)**
  Memory management with demand paging and swap support.

- **[Kernel Syscall Enforcement](Kernel-Syscall-Enforcement)**
  Kernel-space syscall restriction and security enforcement.

- **[eBPF JIT Compilation](eBPF-JIT-Compilation)**
  eBPF JIT compiler for high-performance packet filtering.

- **[Performance & BORE Scheduler](Performance-Tuning-and-Kernel)**
  Performance tuning and BORE scheduler configuration.


- **[Dynamic Kernel Module Loading](Dynamic-Kernel-Module-Loading)**
  Runtime kernel module loading and unloading.

- **[Interrupt Balancing & MSI-X](Interrupt-Balancing-and-MSI-X)**
  Advanced interrupt balancing and MSI-X support.

- **[Cgroups v2 Memory Controller](Cgroups-v2-Memory-Controller)**
  Fine-grained memory resource management and isolation.


- **[ZFS Integration with ARC](ZFS-Integration-with-ARC)**
  Advanced storage management with data integrity and compression.

- **[Btrfs Subvolumes & Send/Receive](Btrfs-Subvolumes-and-Send-Receive)**

- **[File Management](File-Management)**
  VFS, file permissions, file locking, directory operations.

- **[System Security](System-Security)**
  Access control, auditing, MAC framework, security policies.

- **[Btrfs Subvolumes & Send/Receive](Btrfs-Subvolumes-and-Send-Receive)**

- **[Process Management](Process-Management)**
  Process scheduling, cgroups, resource limits, namespaces.

- **[Memory Management](Memory-Management)**
  Virtual memory, demand paging, slab allocator, memory compaction.

- **[Btrfs Subvolumes & Send/Receive](Btrfs-Subvolumes-and-Send-Receive)**

- **[Security Hardening Guide](Security-Hardening-Guide)**
  Kernel hardening, application sandboxing, secure coding practices.

- **[Btrfs Subvolumes & Send/Receive](Btrfs-Subvolumes-and-Send-Receive)**

- **[Filesystem Support Matrix](Filesystem-Support-Matrix)**
  ext4, XFS, Btrfs, ZFS, UFS, Hammer2, NFS, SMB, SSHFS, encryption.

- **[Networking Support Matrix](Networking-Support-Matrix)**
  IPv4/IPv6, bonding, VLANs, VPNs, namespaces, bridging.

- **[Btrfs Subvolumes & Send/Receive](Btrfs-Subvolumes-and-Send-Receive)**

- **[Container Orchestration](Container-Orchestration)**
  Kubernetes-compatible APIs, pod scheduling, service discovery, and autoscaling.

- **[Btrfs Subvolumes & Send/Receive](Btrfs-Subvolumes-and-Send-Receive)**

- **[System Monitoring & Observability](System-Monitoring-and-Observability)**
  Real-time metrics, distributed tracing, log aggregation, and alerting.

- **[Btrfs Subvolumes & Send/Receive](Btrfs-Subvolumes-and-Send-Receive)**

- **[Advanced NVIDIA GPU Support](Advanced-NVIDIA-GPU-Support)**
  NVIDIA GPU support with CUDA acceleration.

- **[Wi-Fi 6E/7 Support](Wi-Fi-6E-7-Support)**
  Ultra-high-speed wireless networking with 6 GHz support.

- **[USB3/4 xHCI Full Support](USB3-4-xHCI-Full-Support)**
  High-speed USB with Thunderbolt integration.

- **[Mach/Zircon Zero-Copy IPC](Mach-Zircon-Zero-Copy-IPC)**
  High-performance message passing with zero-copy.

- **[Gentoo Portage Integration](Gentoo-Portage-Integration)**
  Source-based package management with USE flags.

- **[Btrfs Subvolumes & Send/Receive](Btrfs-Subvolumes-and-Send-Receive)**
  Copy-on-write filesystem with snapshot replication.

- **[XDP Zero-Copy Networking](XDP-Zero-Copy-Networking)**
  Ultra-high-performance packet processing at NIC level.

- **[PF Firewall with CARP/pfsync](PF-Firewall-with-CARP-pfsync)**
  Stateful firewalling with high availability.

- **[Nix/Guix Hermetic Build Sandboxing](Nix-Guix-Hermetic-Build-Sandboxing)**
  Reproducible builds with complete isolation.


- **[Linux io_uring](Linux-io_uring-Implementation)**
  High-performance asynchronous I/O with zero-copy support.

- **[FreeBSD Capsicum](FreeBSD-Capsicum-Integration)**
  Capability-based access control and sandboxing.


- **[MGLRU & Memory Reclamation](Performance-Tuning-and-Kernel#mglru--memory-reclamation)**
  Multi-Gen LRU and memory reclamation optimization.

- **[eBPF / XDP Networking](Performance-Tuning-and-Kernel#ebpf--xdp-zero-copy-networking)**
  eBPF and XDP zero-copy networking support.

- **[Lock-Free Data Structures](SIGMA_CONCURRENCY_PRIMITIVES)**
  Lock-free concurrency primitives and data structures.

- **[Category:Performance](Category-Performance)**
  Performance and kernel articles.

---
## Maintenance & Recovery

- **[Rollback Engine & Snapshots](Maintenance-and-Rollback-Engine)**
  Atomic rollback engine and system snapshots.

- **[Declarative Generations](Maintenance-and-Rollback-Engine#declarative-system-generations)**
  Declarative system state and generation management.

- **[ZFS Boot Environments](freebsd-zfs)**
  ZFS boot environment management.

- **[Btrfs Subvolumes](SIGMA_VFS_LAYER)**
  Btrfs subvolume management and CoW filesystems.

---

## Hardware & Platform

- **[Hardware Support Matrix](SUPPORT_MATRIX)**
  Comprehensive hardware compatibility and support matrix.

- **[Multi-Architecture Support](ARCHITECTURE)**
  Support for x86_64, AArch64, and other architectures.

- **[Virtualization & Containers](Virtualization-and-Containers-Isolation)**
  Virtualization and container isolation features.

- **[Device Drivers](Device-Drivers)**
  Device driver support and management.

- **[Category:Hardware](Category:Hardware)**
  Hardware and platform articles.

---

## Our Community

- **[Code of Conduct](Code-of-Conduct)**
  Guidelines for the SigmaOS community.

- **[Getting Involved](Getting-Involved)**
  Describes various ways contributors can participate in the SigmaOS community.

- **[Contributing](Contributing)**
  Guide to contributing to SigmaOS development and documentation.

- **[International Communities](International-Communities)**
  Collection of links to SigmaOS communities around the world.

- **[Category:Community](Category-Community)**
  Community articles and guidelines.

---

## Wiki Interaction

- **[Help:Reading](Help-Reading)**
  Find clarifications if you struggle to understand instructions in some articles.

- **[Help:Browsing](Help-Browsing)**
  How to search the wiki, find related articles and view the wiki offline.

- **[Wiki:Contributing](Wiki-Contributing)**
  The starting point for those willing to contribute to the wiki.

- **[Help:Editing](Help-Editing)**
  Tutorial on editing articles and introduction to wiki text syntax.

---

## Reference Documentation

- **[Product Vision & Manifesto](PRODUCT_VISION)**
  Strategic vision and product manifesto.

- **[Release Criteria](RELEASE_CRITERIA)**
  Quality gates and release criteria.

- **[Architecture Decision Records](ARCHITECTURE_DECISIONS)**
  Key architectural decisions and their rationale.

- **[Comprehensive OS Analysis](SIGMAOS_COMPREHENSIVE_OS_ANALYSIS)**
  Consolidated analysis of SigmaOS architecture and comparisons.

- **[Category:Development](Category-Development)**
  Development and technical articles.

---

## Gap Closure Roadmap

- **[Phase 1: Critical Foundation](Phase-1-Gap-Closure-Implementation-Plan)**
  PCI/PCIe enumeration, GPU drivers, demand paging, pledge/unveil, eBPF JIT (Months 1-3)

- **[Phase 2: Core Features](Phase-2-Gap-Closure-Implementation-Plan)**
  Kernel modules, IRQ balancing, Cgroups v2, io_uring, Capsicum (Months 4-6)

- **[Phase 3: Advanced Features](Phase-3-Gap-Closure-Implementation-Plan)**
  ZFS, Btrfs, XDP, PF firewall, Nix/Guix builds (Months 7-12)

- **[Phase 4: Enterprise Features](Phase-4-Gap-Closure-Implementation-Plan)**
  NVIDIA GPU, Wi-Fi 6E/7, USB3/4, Mach/Zircon IPC, Portage (Months 13-18)

---

**[View all wiki pages](Table-of-contents)**
