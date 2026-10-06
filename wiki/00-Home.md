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
| Kernel and scheduling | Linux scheduling and cgroups; FreeBSD ULE and cpusets; NetBSD modular kernel interfaces |
| Storage | Linux journaling and copy-on-write filesystems; FreeBSD GEOM and ZFS; NetBSD VFS boundaries |
| Networking and security | Linux namespaces and nftables; OpenBSD PF, pledge, and unveil; FreeBSD jails and Capsicum; NetBSD NPF |
| Packaging and updates | NixOS generations; Debian package transactions; Arch build recipes; Gentoo profiles; FreeBSD pkg and Poudriere |
| Desktop and devices | Linux DRM/KMS; FreeBSD device lifecycle; Pop!_OS COSMIC workspace design |

These are inspiration sources for design review. They are not a feature-support list.

## Component documentation

- [Kernel](04-Kernel)
- [Filesystems](05-Filesystems)
- [Networking](06-Networking)
- [Security](07-Security)
- [Desktop](08-Desktop)
- [Packaging](09-Packaging)
- [Future development overview](14-Future-Development)
- [Autonomous AI SysAdmin & Agent Orchestration](25-Roadmap-Autonomous-AI-SysAdmin-and-Agent-Orchestration)
- [Arch Linux Parity & Pacman ALPM Ecosystem](26-Roadmap-Arch-Linux-Parity-and-Pacman-ALPM-Ecosystem)
- [Debian Parity & APT Ecosystem](27-Roadmap-Debian-Parity-and-APT-Ecosystem)
- [Component roadmap index](Home#component-future-development-roadmaps)

## Maintenance

Keep this page as a concise project overview. Link detailed behavior to source files and component pages. Move implementation details to their topic pages, label proposals as proposals, and remove unverified performance, test, hardware-support, or parity claims.
