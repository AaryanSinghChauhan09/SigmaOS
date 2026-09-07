# SigmaOS Linux & BSD Distribution Component & Architecture Guidelines

This document specifies the core architectural guidelines, packaging purity standards, security paradigms, and subsystem engineering rules for SigmaOS, drawing inspiration from leading Linux and BSD distribution ecosystems.

---

## 1. Arch Linux: Simplicity & Packaging Purity Guidelines
- **KISS Principle (Keep It Simple, Stupid):** Codebases must maintain minimal abstractions and explicit, transparent control flow without unneeded framework wrappers.
- **PKGBUILD Cleanliness & Zero Patch Overheads:** Package specifications must build directly from unmodified upstream sources whenever possible.
- **ALPM Topological Dependency Resolution:** Package dependency graphs must resolve cleanly without cyclic deadlocks, supporting atomic rolling releases.
- **AUR & Build Purity:** Sandboxed compilation environments (chroot/namespaces) must sanitize ambient environment variables (`SOURCE_DATE_EPOCH=...`, `LC_ALL=C`, `TZ=UTC`).

---

## 2. Debian & Ubuntu: Policy Stability & FHS/LSB Guidelines
- **Filesystem Hierarchy Standard (FHS):** Universal path canonicalization across `/usr/bin`, `/etc`, `/var`, and `/opt`.
- **Debian Policy Compliance:** Package metadata must define precise `Pre-Depends`, `Depends`, `Recommends`, and `Suggests` directives.
- **Transactional Maintainer Scripts:** Pre-install, post-install, pre-remove, and post-remove scriptlets must execute idempotently and support atomic unwinding on failure.
- **Release Stability Tiers:** Maintain distinct stability stages (`Staging`, `Testing`, `Stable/Core`) with mandatory QA signoff quorums before migration.

---

## 3. Fedora & Red Hat Enterprise Linux: MAC Security & Systemd Presets
- **SELinux Mandatory Access Control (MAC):** All processes run within strict security contexts (`u:r:s0`), enforcing MLS/MCS multi-level security compartments.
- **Systemd Service Supervision & Presets:** System services must define explicit unit files and systemd-preset activation policies (`enable`/`disable`) for default state management.
- **Cgroup v2 Resource Isolation:** CPU quotas, memory slices, and I/O pressure stall monitoring (PSI) must govern all containerized workloads and services.
- **Fedora Messaging Bus Audit:** Inter-service communications emit signed event messages onto the system bus for real-time auditability.

---

## 4. FreeBSD: Capsicum Capabilities, Jails, & bsdconfig Guidelines
- **Capsicum Capability Rights:** File descriptors and system handles operate under restricted capability mode (`cap_rights_limit`), preventing unauthorized ambient syscall access.
- **VNET & FreeBSD Jails:** Lightweight, isolated container environments featuring virtualized network stacks (`VNET`) and process namespace boundaries.
- **bsdconfig System Management:** Hierarchical, keyboard-accessible control panel tree for driver, networking, and user administration.
- **ULE Scheduler Interactivity Scoring:** Process scheduler evaluates interactive vs. batch workload heuristics to guarantee sub-millisecond desktop responsiveness.

---

## 5. OpenBSD: Pledge, Unveil, & Secure-By-Default Guidelines
- **Pledge & Unveil Privileges:** System binaries must explicitly drop unused system call rights (`pledge("stdio rpath", NULL)`) and restrict file system viewports (`unveil("/etc", "r")`).
- **W^X Memory Hardening:** Enforce strict Write-XOR-Execute memory page protections across userland and kernel memory allocators.
- **Secure-By-Default Configuration:** Every installed service defaults to disabled and hardened mode until explicitly enabled by an administrator.
- **Continuous Code Auditing:** Ongoing vulnerability inspection, safe arithmetic bounds checking, and null-byte injection prevention across all inputs.

---

## 6. Alpine, Void, & Nix/Guix: Hermetic CAS & Minimal Footprint Guidelines
- **Content-Addressed Store (CAS):** Packages and build artifacts reside in immutable, hash-addressed storage locations with Merkle tree closure verification.
- **Zero-Dependency `#![no_std]` Kernel Architecture:** Core OS components compile natively without external third-party dependencies.
- **Instant Rollback Generations:** System configurations maintain atomic generation snapshots allowing sub-second differential state rollbacks.
