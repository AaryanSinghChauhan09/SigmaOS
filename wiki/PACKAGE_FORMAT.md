# SigmaOS Package Format Specification (`.sigpkg` Container)

## 1. Executive Summary

The `.sigpkg` format is the primary binary package container format for SigmaOS. Designed for streaming verification, high compression ratios, zero-copy installation, and cryptographic integrity, `.sigpkg` unifies application deployment, system library updates, and flatpak/containerized sandbox bundles.

## 2. Binary Container Layout

A `.sigpkg` file is a contiguous binary archive structured into four main sections:

```
 0                   1                   2                   3
 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|                       Magic: "SIGPKG1"                         |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|       Header Length           |       Signature Length        |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|                                                               |
|                1. Detached Cryptographic Header               |
|            (Ed25519 / Dilithium-3 Post-Quantum Signature)      |
|                                                               |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|                                                               |
|                   2. Package Manifest (TOML)                  |
|    (Package Name, Version, Architecture, Dependencies, Hashes)|
|                                                               |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|                                                               |
|               3. Payload Archive (Zstd / LZ4 Tar)             |
|                  (Filesystem Directory Tree)                  |
|                                                               |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|                                                               |
|                4. Lifecycle Scriptlets & Triggers             |
|             (pre-install, post-install, pre-remove)           |
|                                                               |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
```

## 3. Package Manifest Specification (`manifest.toml`)

```toml
[package]
name = "zenith-terminal"
version = "1.2.0"
release = 1
arch = "x86_64"
license = "MIT"
summary = "GPU-accelerated terminal emulator for SigmaOS"
description = "A fast, memory-safe terminal emulator supporting truecolor and ligatures."
url = "https://sigmaos.org/apps/zenith-terminal"
maintainer = "SigmaOS Core Team <dev@sigmaos.org>"
priority = "optional"
category = "System/Terminals"

[dependencies]
depends = ["zenith-compositor>=1.0.0", "font-noto-sans>=2024.1"]
opt-depends = ["zsh: Alternative default interactive shell"]
conflicts = ["legacy-term"]
provides = ["terminal-emulator"]

[files]
blake3_merkle_root = "f4e2a1b9c8d7e6f5a4b3c2d1e0f9a8b7c6d5e4f3a2b1c0d9e8f7a6b5c4d3e2f1"
total_installed_size_bytes = 14285024

[scriptlets]
post_install = "scripts/post_install.sh"
post_remove = "scripts/post_remove.sh"
```

## 4. Delta Packages & Incremental Updates

`.sigpkg` supports block-level binary diffing (`.sigdelta`). When a user updates a package, the `sigpkg` daemon downloads only the changed LZ4/Zstd compressed chunks, reconstructing the target `.sigpkg` container locally before applying Merkle store verification.
