# Node.js Binary Distribution & Runtime Guide

## Architecture Overview

The Node.js Binary Distribution Subsystem in SigmaOS (`NodeBinaryDistroEngine`) manages Node.js runtime environments, binaries, npm/corepack wrappers, and isolated store paths.

## Key Features Inspired by Linux & BSD Distros

### 1. NixOS / Guix Isolated Store Paths

Node.js versions are installed into immutable, isolated store paths:
`/sovereign/store/node-vX.Y.Z-<checksum-hash>`
This allows multiple Node.js versions (v18, v20, v22, Nightly) to coexist side-by-side without file or dependency collisions.

### 2. Debian `update-alternatives` & Gentoo `eselect` Active Version Switching

System-wide active Node.js version switching is managed dynamically via `set_active_version()`. Changing the active version updates system `/usr/bin/node` symlink targets cleanly.

### 3. Alpine & Debian C-Library ABI Targets

Node.js binaries are compiled and targeted for:

*   `Glibc`: Standard glibc ABI compatibility for Linux applications.
*   `Musl`: Lightweight, zero-overhead musl C library target inspired by Alpine Linux.
*   `SovereignKlib`: Native bare-metal ABI target for pure kernel environments.

### 4. Cryptographic Checksum & Signature Verification

Binary releases are cryptographically validated using SHA-256 hashes and Ed25519 signatures inspired by FreeBSD `signify` and Arch Linux `pacman`.

### 5. OpenBSD `pledge` / `unveil` & Linux `seccomp` Security Policies

Node.js execution paths enforce strict filesystem branch access limits (`unveiled_paths`) and block loading untrusted C++ native add-ons (`disable_native_addons`) when running untrusted scripts.
