# Table of contents

This page provides a structured overview of all SigmaOS Wiki articles, organized by category.

---

## Getting started

- [About SigmaOS](About-SigmaOS) — Overview of SigmaOS describing what to expect from a SigmaOS system
- [Frequently Asked Questions](Frequently-Asked-Questions) — Notable questions and facts about the distribution
- [SigmaOS Compared to Other Distributions](SigmaOS-Compared-to-Other-Distributions) — Summarizes the similarities and differences between SigmaOS and other distributions
- [Installation Guide](Installation-Guide) — Guide through the process of installing SigmaOS
- [General Recommendations](General-Recommendations) — Annotated index of post-installation tutorials and other popular articles
- [List of Applications](List-of-Applications) — Categorized presentation of common applications and packages

---

## Desktop Environment

- [Zenith Compositor](Zenith-Compositor) — Overview of the Zenith desktop environment and compositor system
- [Zenith Desktop & Omarchy Workflow](Zenith-Desktop-and-Omarchy-Workflow) — Detailed guide to the Zenith desktop workflow and Omarchy integration
- [Bolt ⚡ Fast Launcher](Zenith-Desktop-and-Omarchy-Workflow#bolt-fast-path-launcher-and-agentic-steering) — Fast path launcher and agentic steering system
- [Palette 🎨 UI Themes](Zenith-Desktop-and-Omarchy-Workflow#palette-dynamic-theme--color-engine) — Dynamic theme and color engine for Zenith
- [Sentinel 🛡️ Exec Guard](Zenith-Desktop-and-Omarchy-Workflow#sentinel-sandboxing--exec-guard) — Capability-based sandboxing and execution guard

---

## Package Management

- [SigmaPkg - Universal Package Manager](SigmaPkg) — Universal package management system with multi-format support
- [Universal Package Engine (sigpkg)](Package-Management-and-Sigpkg) — Overview of the SigmaPkg package management system
- [Arch PKGBUILD & AUR Helper](ARCH_LINUX_PARITY_FEATURES) — Integration with Arch Linux package formats and AUR
- [Multi-Distro Adapters](UNIVERSAL_PACKAGE_SYSTEM_IMPLEMENTATION_PLAN) — Support for multiple Linux and BSD package formats
- [Content-Addressed Storage](Content-Addressed-Storage) — Immutable, cryptographically-verified package storage

---

## System Administration
- [System Monitoring & Observability](System-Monitoring-and-Observability) — Real-time metrics, distributed tracing, log aggregation, and alerting
- [Services & Supervision](System-Administration-and-Services) — Service management and supervision system
- [Privilege Delegation (doas)](SECURITY) — Secure privilege escalation and delegation
- [Networking & WireGuard](networking) — Network configuration and WireGuard VPN integration
- [Logging & Journalctl](syslog) — System logging and journal management

---

## Security & Hardening
- [Security Hardening Guide](Security-Hardening-Guide) — Kernel hardening, application sandboxing, secure coding practices
- [File Management](File-Management) — VFS, file permissions, file locking, directory operations
- [System Security](System-Security) — Access control, auditing, MAC framework, security policies
- [Security Sandbox & Isolation](Security-Sandbox-Isolation) — Comprehensive security sandboxing with Landlock, Capsicum, and pledge/unveil
- [Security & Sandboxing](Security-Sandboxing-and-Hardening) — Comprehensive security model and sandboxing capabilities
- [Zorin Exec Guard](Security-Sandboxing-and-Hardening#zorin-exec-guard) — Default-deny capability permission model
- [Hardware Enclaves & PQC](Security-Sandboxing-and-Hardening#hardware-enclaves--post-quantum-cryptography) — Hardware enclaves and post-quantum cryptography support
- [Memory Safety Guarantees](SECURITY) — Rust-enforced memory safety and kernel hardening features

---

## Performance & Kernel
- [Process Management](Process-Management) — Process scheduling, cgroups, resource limits, namespaces
- [Memory Management](Memory-Management) — Virtual memory, demand paging, slab allocator, memory compaction

- [Demand Paging & Swap](Demand-Paging-and-Swap) — Memory management with demand paging and swap support
- [Kernel Syscall Enforcement](Kernel-Syscall-Enforcement) — Kernel-space syscall restriction and security enforcement
- [eBPF JIT Compilation](eBPF-JIT-Compilation) — eBPF JIT compiler for high-performance packet filtering
- [Dynamic Kernel Module Loading](Dynamic-Kernel-Module-Loading) — Runtime kernel module loading and unloading
- [Interrupt Balancing & MSI-X](Interrupt-Balancing-and-MSI-X) — Advanced interrupt balancing and MSI-X support
- [Cgroups v2 Memory Controller](Cgroups-v2-Memory-Controller) — Fine-grained memory resource management and isolation
- [Linux io_uring](Linux-io_uring-Implementation) — High-performance asynchronous I/O with zero-copy support
- [FreeBSD Capsicum](FreeBSD-Capsicum-Integration) — Capability-based access control and sandboxing
- [ZFS Integration with ARC](ZFS-Integration-with-ARC) — Advanced storage management with data integrity and compression
- [Btrfs Subvolumes & Send/Receive](Btrfs-Subvolumes-and-Send-Receive) — Copy-on-write filesystem with snapshot replication
- [XDP Zero-Copy Networking](XDP-Zero-Copy-Networking) — Ultra-high-performance packet processing at NIC level
- [PF Firewall with CARP/pfsync](PF-Firewall-with-CARP-pfsync) — Stateful firewalling with high availability
- [Nix/Guix Hermetic Build Sandboxing](Nix-Guix-Hermetic-Build-Sandboxing) — Reproducible builds with complete isolation
- [Advanced NVIDIA GPU Support](Advanced-NVIDIA-GPU-Support) — NVIDIA GPU support with CUDA acceleration
- [Wi-Fi 6E/7 Support](Wi-Fi-6E-7-Support) — Ultra-high-speed wireless networking with 6 GHz support
- [USB3/4 xHCI Full Support](USB3-4-xHCI-Full-Support) — High-speed USB with Thunderbolt integration
- [Mach/Zircon Zero-Copy IPC](Mach-Zircon-Zero-Copy-IPC) — High-performance message passing with zero-copy
- [Gentoo Portage Integration](Gentoo-Portage-Integration) — Source-based package management with USE flags
- [Performance & BORE Scheduler](Performance-Tuning-and-Kernel) — Performance tuning and BORE scheduler configuration
- [MGLRU & Memory Reclamation](Performance-Tuning-and-Kernel#mglru--memory-reclamation) — Multi-Gen LRU and memory reclamation optimization
- [eBPF / XDP Networking](Performance-Tuning-and-Kernel#ebpf--xdp-zero-copy-networking) — eBPF and XDP zero-copy networking support
- [Lock-Free Data Structures](SIGMA_CONCURRENCY_PRIMITIVES) — Lock-free concurrency primitives and data structures

---

## Maintenance & Recovery
- [Container Orchestration](Container-Orchestration) — Kubernetes-compatible APIs, pod scheduling, service discovery, and autoscaling

- [Rollback Engine & Snapshots](Maintenance-and-Rollback-Engine) — Atomic rollback engine and system snapshots
- [Declarative Generations](Maintenance-and-Rollback-Engine#declarative-system-generations) — Declarative system state and generation management
- [ZFS Boot Environments](freebsd-zfs) — ZFS boot environment management
- [Btrfs Subvolumes](SIGMA_VFS_LAYER) — Btrfs subvolume management and CoW filesystems

---

## Hardware & Platform
- [Filesystem Support Matrix](Filesystem-Support-Matrix) — ext4, XFS, Btrfs, ZFS, UFS, Hammer2, NFS, SMB, SSHFS, encryption
- [Networking Support Matrix](Networking-Support-Matrix) — IPv4/IPv6, bonding, VLANs, VPNs, namespaces, bridging

- [Hardware Support Matrix](SUPPORT_MATRIX) — Comprehensive hardware compatibility and support matrix
- [Multi-Architecture Support](ARCHITECTURE) — Support for x86_64, AArch64, and other architectures
- [Virtualization & Containers](Virtualization-and-Containers-Isolation) — Virtualization and container isolation features
- [Device Drivers](Device-Drivers) — Device driver support and management

---

## Development

- [Contributing](Contributing) — How to contribute to SigmaOS development
- [Development Guide](DEVELOPMENT_GUIDE.md) — Guide for SigmaOS development
- [General Recommendations](General-Recommendations) — General post-installation recommendations
- [Wiki Contributing](Wiki-Contributing.md) — How to contribute to the SigmaOS Wiki
- [Naming Conventions & Rules](Naming-Conventions-and-Rules) — SigmaOS naming conventions and coding rules

---

## Community

- [Code of Conduct](Code-of-Conduct) — Guidelines for the SigmaOS community
- [Getting Involved](Getting-Involved) — Describes various ways contributors can participate in the SigmaOS community
- [Contributing](Contributing) — Guide to contributing to SigmaOS development and documentation
- [International Communities](International-Communities) — Collection of links to SigmaOS communities around the world

---

## Wiki Interaction

- [Help:Reading](Help-Reading) — Find clarifications if you struggle to understand instructions in some articles
- [Help:Browsing](Help-Browsing) — How to search the wiki, find related articles and view the wiki offline
- [Wiki:Contributing](Wiki-Contributing) — The starting point for those willing to contribute to the wiki
- [Help:Editing](Help-Editing) — Tutorial on editing articles and introduction to wiki text syntax

---

## Reference Documentation

- [Product Vision & Manifesto](PRODUCT_VISION) — Strategic vision and product manifesto
- [Release Criteria](RELEASE_CRITERIA) — Quality gates and release criteria
- [Architecture Decision Records](ARCHITECTURE_DECISIONS) — Key architectural decisions and their rationale
- [Comprehensive OS Analysis](SIGMAOS_COMPREHENSIVE_OS_ANALYSIS) — Consolidated analysis of SigmaOS architecture and comparisons

---

## Development & Technical

- [Architecture](ARCHITECTURE_DECISIONS) — std-based architecture decision and implementation strategy
- [Development Guide](DEVELOPMENT_GUIDE) — Development guidelines and best practices
- [Build Instructions](BUILD) — Build system and compilation instructions
- [Testing Strategy](TESTING_STRATEGY) — Testing methodology and quality assurance

---

## Advanced Topics

- [Sovereign VMM](Virtualization-and-Containers-Isolation) — Type-1 hypervisor integration and capability-gated ring boundaries
- [SovereignSched](SIGMA_EEVDF_SCHEDULER) — Dynamic workload scheduler with AMP and lock-free queue pools
- [ZenithNet](networking) — Custom networking stack with post-quantum cryptographic tunneling
- [SigmaFS](SIGMA_VFS_LAYER) — Next-generation crash-consistent filesystem with Merkle trees

---

## Roadmaps & Planning

- [Master Execution Roadmap](ROADMAP.md) — Overall project roadmap and staged rollout timeline
- [Future Course of Action](FUTURE_COURSE_OF_ACTION.md) — Strategic priorities and 6-week development plan
- [Future Development Roadmap](FUTURE-DEVELOPMENT-ROADMAP.md) — Ultimate development roadmap and system specification
- [Gap Analysis](OPEN_SOURCE_OS_COMPARATIVE_GAP_ANALYSIS.md) — Comprehensive comparative gap analysis with other OS projects

---

## Categories

- [Category:Installation](Category:Installation) — Installation-related articles
- [Category:Desktop Environment](Category:Desktop-Environment) — Desktop environment and compositor articles
- [Category:Package Management](Category:Package-Management) — Package management and software installation
- [Category:Security](Category:Security) — Security hardening and sandboxing
- [Category:Performance](Category:Performance) — Performance tuning and optimization
- [Category:Hardware](Category:Hardware) — Hardware support and device drivers
- [Category:Community](Category:Community) — Community and contribution guidelines
- [Category:Development](Category:Development) — Development and technical documentation

---

**[Return to Home](Home)**
