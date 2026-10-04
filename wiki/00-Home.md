# SigmaOS Project Overview

SigmaOS is an operating-system project. Its repository contains kernel, memory, filesystem, networking, security, driver, desktop, installer, and package-management code. The presence of a module, API, or roadmap entry does not by itself establish runtime support.

## Project goals

- Build a system with clear subsystem boundaries and explicit resource ownership.
- Evaluate useful designs from Linux distributions and BSD systems, then adapt them to SigmaOS's architecture rather than claiming direct parity.
- Make reliability, security, performance, accessibility, and recoverability reviewable through source, documented behavior, and repeatable validation.
- Keep security-critical functions unavailable when a required audited provider is not integrated.

## Linux and BSD design references

| Area | Designs to study |
|---|---|
| Kernel and scheduling | Linux scheduler and cgroup design; xv6's compact process/trap model; Redox's Rust interfaces |
| Storage | Linux journaling and VFS; Btrfs copy-on-write/recovery; FreeBSD GEOM/ZFS boundaries |
| Networking and security | Linux namespaces/nftables; OpenBSD PF/pledge/unveil; FreeBSD jails/Capsicum |
| Packaging and updates | Arch's transparent package recipes; NixOS generations; Debian transactions; FreeBSD signed catalogs |
| Desktop and devices | Omarchy's keyboard workflow; Mint's onboarding/recovery; Linux DRM/KMS and hardware discovery |
| Documentation | ArchWiki's task-oriented pages, prerequisites, procedures, and troubleshooting |

These are inspiration sources for design review. They are not a feature-support list.

See [Inspiration and Reference Projects](Inspiration-and-References.md) for adaptation guidance. Keep behavior and future work on the owning component page, supported by source links and reproducible checks.

## Component documentation

- [Kernel](04-Kernel)
- [Filesystems](05-Filesystems)
- [Networking](06-Networking)
- [Security](07-Security)
- [Desktop](08-Desktop)
- [Packaging](09-Packaging)
- [Future development overview](14-Future-Development)
- [Component roadmap index](Home#component-future-development-roadmaps)

## Maintenance

Keep this page as a concise project overview. Link detailed behavior to source files and component pages. Move implementation details to their topic pages, label proposals as proposals, and remove unverified performance, test, hardware-support, or parity claims.
