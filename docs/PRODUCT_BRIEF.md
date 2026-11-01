# SigmaOS Product Brief

## What Is SigmaOS?

SigmaOS is a sovereign, AI-native, modular Linux distribution built primarily in Rust. It is
designed to be a complete, self-sufficient operating system — one that can be installed, used,
updated, and recovered without depending on any external party's infrastructure or decisions.

The core design principles are modularity, user freedom, secure defaults, and long-term
sustainability. Every component is a separate, independently versioned repository. The kernel,
desktop, installer, package manager, and application layer are cleanly separated and can evolve
independently.

## Target Users

### Persona 1: General Desktop User
A non-technical user who wants a stable, polished Linux desktop that works on day one.
- Installs SigmaOS from a live ISO
- Expects a browser, office suite, media player, and file manager out of the box
- Wants automatic updates that don't break the system
- Expects to recover if something goes wrong

### Persona 2: Developer
A software developer who wants a productive, customizable workstation.
- Needs a terminal, editor, Git, SSH, and build tools
- Wants a clean package system with development libraries available
- Values system transparency and low overhead
- May want multiple workspace layouts and keyboard-driven workflows

### Persona 3: Privacy and Security User
A user who values control over their system and data.
- Wants secure defaults without manual hardening
- Expects no telemetry, no tracking, no forced cloud sign-in
- Values open-source transparency for every system component
- Needs disk encryption, firewall by default, app sandboxing

### Persona 4: AI-Native Workstation User
A researcher or engineer building and running local AI workloads.
- Needs GPU acceleration (Vulkan, OpenCL, VA-API)
- Needs Python, CUDA/ROCm drivers, containerization
- Values a clean, reproducible system environment
- May need fast suspend/resume and power profile management

## Core Differentiators vs Mainstream Distros

| Feature | Ubuntu | Linux Mint | SigmaOS |
|---|---|---|---|
| Architecture | Monolithic | Monolithic | Modular multi-repo |
| Language | Python/C/Shell | Python/C/Shell | Rust primary |
| Package signing | Yes | Yes | Yes + Ed25519 |
| Rollback | Limited | Limited | Full snapshot rollback |
| AI integration | None | None | Native AI agent layer |
| Recovery | Basic | Basic | Rescue mode + repair shell |
| Theming | GTK/GNOME | Cinnamon | Sovereign theme engine |
| Sovereignty | Depends on Canonical | Depends on Canonical | Self-sovereign |

## V1 Non-Goals

The following are explicitly out of scope for v1.0:

- Mobile / tablet support (ARM64 phones)
- Cloud-native OS variants
- RISC-V support
- Windows app compatibility layer (Wine is available but not a first-class feature)
- Enterprise directory integration (LDAP, Active Directory)
- Immutable root filesystem (planned for v2)
- Flatpak or Snap runtimes (native .deb packages only for v1)

## V1 Success Criteria

A v1.0 release is considered successful when:

1. A user can boot from the live ISO and complete installation in under 20 minutes
2. The installed system reaches a usable desktop on first boot without errors
3. All 24 default apps launch successfully on first login
4. `sigma update` completes without breaking the system on reference hardware
5. A failed update can be rolled back to the pre-update state via `sigma rescue`
6. The system boots clean after rollback
7. All 50 smoke tests pass on Tier 1 reference hardware
8. Zero P0 bugs, fewer than 5 P1 bugs in the release candidate

## Philosophy

- **FOSS first**: every component is free software under MIT, GPL-2.0, or Apache-2.0
- **User freedom**: no telemetry, no accounts required, no feature gates
- **Modularity**: each subsystem is a separate repo, testable and replaceable independently
- **Secure defaults**: hardening is on by default; users opt out, not in
- **Gall's Law**: start with a working simple system; add complexity only when the simple system is proven
- **Boy Scout Rule**: every contribution leaves the codebase cleaner than it found it
- **Spec-Driven**: features are specified before they are coded
