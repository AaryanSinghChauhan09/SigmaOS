#!/bin/bash
cd "$(dirname "$0")"
mkdir -p wiki

cat << 'WIKI' > wiki/Home.md
# Welcome to the SigmaOS Wiki

SigmaOS is a sovereign operating system written in Rust.
Explore the sidebar to learn more about architecture, security, and driver development.
WIKI

cat << 'WIKI' > wiki/Installation.md
# Installation
1. Install Rust nightly.
2. Install QEMU.
3. Clone repo and run `cargo run`.
WIKI

cat << 'WIKI' > wiki/Architecture.md
# Architecture
Modular monolithic with strict capability-based isolation.
WIKI

cat << 'WIKI' > wiki/Security.md
# Security
Capability tokens restrict syscalls.
Pledge/unveil isolate processes.
WIKI

cat << 'WIKI' > wiki/Driver-Development.md
# Driver Development
Write drivers by implementing Rust traits like `StorageDriver` or `NetworkDriver`.
WIKI

cat << 'WIKI' > wiki/Package-Management.md
# Package Management
SigmaPkg uses SAT-solvers and declarative state for atomic installations.
WIKI

cat << 'WIKI' > wiki/Kernel-Development.md
# Kernel Development
Kernel subsystems are located in `src/`. Avoid unsafe code.
WIKI

cat << 'WIKI' > wiki/Compatibility.md
# Compatibility
SigmaOS supports POSIX compatibility via FHS virtualization.
WIKI

cat << 'WIKI' > wiki/Roadmap.md
# Roadmap
- V1.0: Full POSIX support
- V2.0: GUI stack
WIKI

cat << 'WIKI' > wiki/FAQ.md
# FAQ
Q: Why Rust?
A: Memory safety.
WIKI

cat << 'WIKI' > wiki/Contributing.md
# Contributing
See `CONTRIBUTING.md` in the main repo.
WIKI

cat << 'WIKI' > wiki/Code-Scanning-Fixes.md
# Code Scanning Fixes
Recent updates removed unsafe transmutes, unused variables, and potential security risks from the codebase.
WIKI

cat << 'WIKI' > wiki/Linux-Distros-Architecture.md
# Linux Distributions Architecture & Parity Guide

SigmaOS incorporates architectural paradigms from leading Linux distributions:
- **Arch Linux**: Rolling release dependency resolution (`ArchDependencyResolver`) and PKGBUILD recipe sandbox compilation (`ArchRecipeSandboxCompiler`).
- **NixOS / Guix**: Declarative system generations (`NixDeclarativeSystemState`), content-addressed store (`NixStyleStore`), and GNU Shepherd service graph manager (`ShepherdServiceManager`).
- **Clear Linux**: Stateless `/usr` configuration overlay architecture (`ClearLinuxStatelessOverlayEngine`).
- **Gentoo**: Portage USE-flags compilation and dependency resolution (`GentooPortageUseFlagResolver`).
- **Alpine / Void Linux**: Transactional trigger hooks (`AlpineVoidTriggerHookManager`) and Runit 3-stage service lifecycle supervision (`SovereignRunitSupervisor`).
- **Fedora / Ubuntu**: Fedora Toolbox dev containers and Ubuntu Pro Livepatch hot-patching.
WIKI

cat << 'WIKI' > wiki/BSD-Security-Hardening.md
# BSD Security Hardening & Isolation Guide

SigmaOS integrates security and containment mechanisms from BSD distributions:
- **OpenBSD**: Syscall process restriction (`pledge`), file path masking (`unveil`), W^X memory execution policies, and Retguard per-function return address cookies.
- **FreeBSD**: Jails virtualization with nested child jail hierarchies, RACCT/RCTL resource controls, and Capsicum capability delegation (`CapsicumCapability`).
- **DragonFly BSD**: HAMMER2 PFS multi-version B-tree filesystem and variant symlinks (`varsyms`) path resolution (`DragonFlyVarsymsPfsResolver`).
- **HardenedBSD**: PaX MPROTECT W^X protection and SegvGuard brute force crash mitigation.
WIKI

cat << 'WIKI' > wiki/Declarative-Package-Management.md
# Declarative Package Management & Automation

SigmaPkg leverages declarative system state management and transactional automation:
- **Atomic Rollbacks**: Instant generation rollbacks on configuration failures.
- **Hermetic Build Closures**: Content-addressed dependency tracking ensuring zero unreferenced state.
- **Automated Service Supervision**: Event-driven service dependency reconciliation with exponential backoff.
- **Storage Tiering & Scrubbing**: Bcachefs/ZFS automated extent promotion/demotion and data integrity scrubbing.
WIKI
