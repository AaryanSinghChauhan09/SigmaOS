# Gentoo Portage Clean-Room Subsystem in SigmaOS

## Overview

SigmaOS incorporates concepts from **Gentoo's Portage** package management and ebuild engine to provide fine-grained, source-level optimization and compilation flags (e.g., `-march=native`, AVX-512, LTO) directly tailored to target hardware.

---

## Key Modules

- [`src/sigpkg/portage.rs`](file:///home/aaryansinghchauhan/SigmaOS/src/sigpkg/portage.rs): Clean-room Portage engine, USE flags evaluation, and slotting support.
- [`src/toolchain/bootstrap.rs`](file:///home/aaryansinghchauhan/SigmaOS/src/toolchain/bootstrap.rs): Autonomous bootstrap compiler and linker pipeline.

---

## Core Capabilities

### 1. USE Flags System
SigmaOS simulates Portage USE flags to conditionally enable or disable features during package compilation:
```rust
let mut portage = PortageResolver::new();
portage.set_use_flag("wayland", true);
portage.set_use_flag("x11", false);
portage.set_use_flag("vulkan", true);
```

### 2. Slotting & Multi-Version Coexistence
Supports multiple simultaneous versions of libraries (e.g., Python 3.11 and 3.12, LLVM 17 and 18) installed side-by-side without symlink collisions.

### 3. Hardware-Optimized Binary Caching (Binhost)
Build artifacts compiled with host-specific CPU extensions are cached in `/var/cache/sigpkg/binpkgs` with content-addressable BLAKE3 hashes for instant subsequent deployment.
