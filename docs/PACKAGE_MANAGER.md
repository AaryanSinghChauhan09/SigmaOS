# SigmaPkg & Universal Package Management Specification

## Overview
SigmaOS implements a universal, cross-distro package management engine capable of ingesting, translating, sandboxing, and executing packages across 120+ package formats (including `.deb`, `.rpm`, `.pkg.tar.zst`, `.apk`, `.ebuild`, `.xbps`, `.eopkg`, `.hpkg`, `.nixpkg`, `.guix`, `.snap`, `.flatpak`, AppImage, `.openbsd.tgz`, `.ports`, `.pkgsrc`, `.dports`, `.spack`, `.conan`, `.whl`, `.crate`, `.gem`, `.nupkg`, `.msi`, `.apex`, `.conda`, `.brew`, `.wasm`, `.oci`, `.tazpkg`, `.sif`, `.slp`, `.winget`, `.scoop`, `.choco`, `.pixi`, `.nimble`, `.zig`, `.swift`, `.dub`, `.opam`, `.shard`, `.plt`).

## Key Capabilities
1. **SAT-based Dependency Resolution**: Multi-distro DPLL/SAT constraint solver with canonical `sovereign-*` system package remapping and SONAME/libc translation across glibc, musl, Bionic, and BSD libc.
2. **Post-Quantum Cryptographic Verification**: Dilithium5 and Signify signature verification with SLSA v1.0 build provenance attestation auditing.
3. **Multi-Sandbox Execution**: Isolation using OpenBSD `pledge`/`unveil`, FreeBSD Capsicum, and Linux Landlock v5 / Seccomp filters.
4. **CAS Delta Store & Deduplication**: Zero-copy content-addressable storage deduplication using Blake3 and VCDIFF / XDELTA3 patch reconstitution.
5. **Boot Environment Snapshots**: Automatic transaction rollback support across FreeBSD `bectl`, OpenSUSE `snapper`, ZFS `zsys`, Btrfs, and HAMMER2 PFS.
