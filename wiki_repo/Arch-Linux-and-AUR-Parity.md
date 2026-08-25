# Arch Linux and AUR Parity in SigmaOS

## Overview

SigmaOS includes a zero-dependency, clean-room subsystem providing comprehensive compatibility with **Arch Linux**, **Pacman**, and the **Arch User Repository (AUR)**. This subsystem allows Arch Linux packages and PKGBUILD recipes to be compiled, resolved, and executed natively on SigmaOS.

---

## Key Modules

- [`src/sigpkg/arch_compat.rs`](file:///home/aaryansinghchauhan/SigmaOS/src/sigpkg/arch_compat.rs): PKGBUILD parser, dependency graph builder, and `makepkg` environment simulation.
- [`src/sigpkg/mod.rs`](file:///home/aaryansinghchauhan/SigmaOS/src/sigpkg/mod.rs): Unified package orchestrator integrating native `.spkg`, Arch `.pkg.tar.zst`, and Debian `.deb`.
- [`src/sigpkg/portage.rs`](file:///home/aaryansinghchauhan/SigmaOS/src/sigpkg/portage.rs): Source-based build and optimization system.

---

## Features

| Feature | SigmaOS Implementation | Notes |
|---------|------------------------|-------|
| **PKGBUILD Parsing** | Native pure-Rust lexer | Extracts `pkgname`, `pkgver`, `depends`, `makedepends` without bash |
| **AUR Integration** | RPC endpoint querying & build cache | Fetches sources, validates SHA256/BLAKE3, builds in chroot |
| **Pacman DB Format** | `/var/lib/pacman/local` compatibility | Emulates `desc`, `files`, and `depends` tracking |
| **Sandboxed Builds** | `SovereignLandlockLsm` containment | Ensures build scripts cannot alter host filesystem root |

---

## Architecture Flow

```
AUR / Official Arch Mirror
       │ (Download PKGBUILD or binary package)
       ▼
[PKGBUILD Parser / Verifier] ──> Validates BLAKE3 / SHA-256 signatures
       │
       ▼
[Dependency Graph Resolver] ───> Resolves in-memory DAG with cycle prevention
       │
       ▼
[Chroot Build Sandbox] ────────> Compiles inside Landlock + namespace isolate
       │
       ▼
[SigmaPkg Native Index] ───────> Atomically links into system root
```

---

## CLI Usage

```bash
# Search official and AUR packages
sigma-pkg search neofetch

# Build and install from AUR
sigma-pkg aur-install visual-studio-code-bin

# Sync package databases
sigma-pkg -Syu
```
