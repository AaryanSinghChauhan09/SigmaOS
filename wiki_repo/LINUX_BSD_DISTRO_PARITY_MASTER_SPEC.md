# Linux & BSD Distribution Parity Master Specification

## Overview

SigmaOS incorporates design paradigms from major open-source Linux and BSD operating systems (Debian, Ubuntu, Arch Linux, Fedora, Alpine, Void, Gentoo, Slackware, FreeBSD, OpenBSD, DragonFly BSD, NixOS, openSUSE, and Qubes OS).

## 1. Package Management & Package Format Subsystem

*   **Universal Package Manager (`UniversalPackageManager`)**:
    *   Native format: `SigmaPkg` (`.sigpkg`).
    *   Supported foreign package formats: `.deb` (Debian/Ubuntu), `.rpm` (Fedora/RHEL/openSUSE), `.pkg.tar.zst` (Arch), `.apk` (Alpine), `.xbps` (Void), `.pkg` (FreeBSD), `.ebuild` (Gentoo), `.nixpkg` (NixOS), `.appimage`, `.snap`, `.flatpak`.
    *   **Universal Package Translator (`UniversalPackageTranslator`)**: Translates foreign package metadata directly into native `SigmaPkg` format with virtual dependency normalization (e.g. `libssl-dev` / `openssl-devel` -> `sovereign-openssl`).
    *   **Isolated Store Paths**: NixOS/Guix-inspired isolated store paths (`/sovereign/store/node-vX.Y.Z-hash`) preventing version collisions.

## 2. Node.js Binary Distribution (`NodeBinaryDistroEngine`)

*   **Release Stream Management**: LTS, Current, Maintenance, Nightly streams.
*   **Target C-Library ABIs**: Glibc (Debian/Fedora), Musl (Alpine), SovereignKlib (bare-metal).
*   **Cryptographic Verification**: SHA-256 checksums and Ed25519 signatures inspired by FreeBSD signify & Arch pacman.
*   **Active Version Switching**: System-wide switching inspired by Debian `update-alternatives` and Gentoo `eselect`.
*   **Runtime Sandboxing**: OpenBSD `pledge`/`unveil` & Linux `seccomp` policy enforcement for Node processes and native `.node` C++ add-ons.

## 3. Hybrid Graphics & NVIDIA PRIME (`NvidiaPrimeEngine`)

*   **Operating Profiles**: Offload (`__NV_PRIME_RENDER_OFFLOAD=1`), OnDemand, DiscreteNvidia, IntegratedOnly, ReversePrime.
*   **Power Management**: Dynamic D3cold runtime suspend/resume inspired by FreeBSD `bbswitch` and Linux kernel PCIe power management.
*   **Buffer Sharing**: DRM PRIME DMA-BUF buffer sharing between dGPU render engine and iGPU display server.

## 4. Security & Isolation Architecture

*   **Sandboxing**: OpenBSD `pledge`/`unveil` syscall restriction gates.
*   **Capabilities**: FreeBSD Capsicum capability mode sandboxing.
*   **Qubes Isolation**: Micro-VM / domain security isolation.
