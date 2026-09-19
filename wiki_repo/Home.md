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

- **[Universal Package Engine (sigpkg)](Package-Management-and-Sigpkg)**
  Overview of the SigmaPkg package management system.

- **[Post-Quantum Cryptography Package Distribution](Post-Quantum-Cryptography-Package-Distribution)**
  Quantum-safe package distribution using Kyber-1024 and Dilithium-5.

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

- **[Power Management](Power-Management)**
  CPU frequency scaling, device power states, battery management, and thermal control.

- **[Real-Time Computing](Real-Time-Computing)**
  Real-time scheduling, preemptible kernel, and deterministic latency.

- **[Virtualization and Containers](Virtualization-and-Containers)**
  Hardware virtualization, container technology, and orchestration.

- **[System Call Interface](System-Call-Interface)**
  Comprehensive system call interface and security features.

- **[Inter-Process Communication](Inter-Process-Communication)**
  Pipes, message queues, shared memory, and synchronization.

- **[Namespaces](Namespaces)**
  Process isolation with PID, IPC, Network, UTS, User, Cgroup, and Mount namespaces.

---

## Security & Hardening

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

---

## Performance & Kernel

- **[Performance & BORE Scheduler](Performance-Tuning-and-Kernel)**
  Performance tuning and BORE scheduler configuration.

- **[Low-Overhead Kernel Observability](Low-Overhead-Kernel-Observability)**
  High-performance kernel tracing with SigmaTrace and eBPF.

- **[Kernel Timers](Kernel-Timers)**
  High-resolution timer management with nanosecond precision.

- **[Workqueue](Workqueue)**
  Asynchronous work execution with priority-based scheduling.

- **[RCU Synchronization](RCU-Synchronization)**
  Scalable read-mostly data structure synchronization.

- **[Lockdep](Lockdep)**
  Deadlock detection and lock order validation.

- **[Cgroups](Cgroups)**
  Resource management and process grouping.

- **[Kobject](Kobject)**
  Kernel object management and hierarchy.

- **[Ftrace](Ftrace)**
  Function tracing and instrumentation.

- **[Perf](Perf)**
  Performance monitoring and profiling.

- **[Seccomp](Seccomp)**
  System call filtering and security sandboxing.

- **[IOMMU](IOMMU)**
  Device memory isolation and DMA remapping.

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

---

## Omnipresent Self-Sufficiency Reference
- **[Sovereign OS Absolute Omnipresent Self-Sufficiency Ultra Encyclopedia V29](SOVEREIGN_OS_ABSOLUTE_OMNIPRESENT_SELF_SUFFICIENCY_ULTRA_ENCYCLOPEDIA_V29)**
  Master architectural reference detailing native zero-dependency replacements for external software, AI models, frameworks, database engines, robotics tools, codecs, and operating systems.
