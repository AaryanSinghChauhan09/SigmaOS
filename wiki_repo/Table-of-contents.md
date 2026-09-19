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

- [Universal Package Engine (sigpkg)](Package-Management-and-Sigpkg) — Overview of the SigmaPkg package management system
- [Post-Quantum Cryptography Package Distribution](Post-Quantum-Cryptography-Package-Distribution) — Quantum-safe package distribution using Kyber-1024 and Dilithium-5
- [Arch PKGBUILD & AUR Helper](ARCH_LINUX_PARITY_FEATURES) — Integration with Arch Linux package formats and AUR
- [Multi-Distro Adapters](UNIVERSAL_PACKAGE_SYSTEM_IMPLEMENTATION_PLAN) — Support for multiple Linux and BSD package formats
- [Content-Addressed Storage](Content-Addressed-Storage) — Immutable, cryptographically-verified package storage

---

## System Administration

- [Services & Supervision](System-Administration-and-Services) — Service management and supervision system
- [Privilege Delegation (doas)](SECURITY) — Secure privilege escalation and delegation
- [Networking & WireGuard](networking) — Network configuration and WireGuard VPN integration
- [Logging & Journalctl](syslog) — System logging and journal management
- [Power Management](Power-Management) — CPU frequency scaling, device power states, battery management, and thermal control
- [Real-Time Computing](Real-Time-Computing) — Real-time scheduling, preemptible kernel, and deterministic latency
- [Virtualization and Containers](Virtualization-and-Containers) — Hardware virtualization, container technology, and orchestration
- [System Call Interface](System-Call-Interface) — Comprehensive system call interface and security features
- [Inter-Process Communication](Inter-Process-Communication) — Pipes, message queues, shared memory, and synchronization
- [Namespaces](Namespaces) — Process isolation with PID, IPC, Network, UTS, User, Cgroup, and Mount namespaces

---

## Security & Hardening

- [Security & Sandboxing](Security-Sandboxing-and-Hardening) — Comprehensive security model and sandboxing capabilities
- [Zorin Exec Guard](Security-Sandboxing-and-Hardening#zorin-exec-guard) — Default-deny capability permission model
- [Hardware Enclaves & PQC](Security-Sandboxing-and-Hardening#hardware-enclaves--post-quantum-cryptography) — Hardware enclaves and post-quantum cryptography support
- [Memory Safety Guarantees](SECURITY) — Rust-enforced memory safety and kernel hardening features

---

## Performance & Kernel

- [Performance & BORE Scheduler](Performance-Tuning-and-Kernel) — Performance tuning and BORE scheduler configuration
- [Low-Overhead Kernel Observability](Low-Overhead-Kernel-Observability) — High-performance kernel tracing with SigmaTrace and eBPF
- [Kernel Timers](Kernel-Timers) — High-resolution timer management with nanosecond precision
- [Workqueue](Workqueue) — Asynchronous work execution with priority-based scheduling
- [RCU Synchronization](RCU-Synchronization) — Scalable read-mostly data structure synchronization
- [Lockdep](Lockdep) — Deadlock detection and lock order validation
- [Cgroups](Cgroups) — Resource management and process grouping
- [Kobject](Kobject) — Kernel object management and hierarchy
- [MGLRU & Memory Reclamation](Performance-Tuning-and-Kernel#mglru--memory-reclamation) — Multi-Gen LRU and memory reclamation optimization
- [eBPF / XDP Networking](Performance-Tuning-and-Kernel#ebpf--xdp-zero-copy-networking) — eBPF and XDP zero-copy networking support
- [Lock-Free Data Structures](SIGMA_CONCURRENCY_PRIMITIVES) — Lock-free concurrency primitives and data structures

---

## Maintenance & Recovery

- [Rollback Engine & Snapshots](Maintenance-and-Rollback-Engine) — Atomic rollback engine and system snapshots
- [Declarative Generations](Maintenance-and-Rollback-Engine#declarative-system-generations) — Declarative system state and generation management
- [ZFS Boot Environments](freebsd-zfs) — ZFS boot environment management
- [Btrfs Subvolumes](SIGMA_VFS_LAYER) — Btrfs subvolume management and CoW filesystems

---

## Hardware & Platform

- [Hardware Support Matrix](SUPPORT_MATRIX) — Comprehensive hardware compatibility and support matrix
- [Multi-Architecture Support](ARCHITECTURE) — Support for x86_64, AArch64, and other architectures
- [Virtualization & Containers](Virtualization-and-Containers-Isolation) — Virtualization and container isolation features
- [Device Drivers](Device-Drivers) — Device driver support and management

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
