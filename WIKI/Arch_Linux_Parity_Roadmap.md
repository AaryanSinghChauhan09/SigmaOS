# 🦅 SigmaOS Arch Linux Parity, AUR Integration, & PKGBUILD Support Roadmap

This document establishes the strategic engineering and architectural plan to bring complete compatibility and parity with the **Arch Linux ecosystem** (including `pacman`, `makepkg`, `PKGBUILD`, and `AUR` - Arch User Repository) to **SigmaOS**.

By adopting Arch's lightweight simplicity, bleeding-edge rolling releases, and community packaging standards, SigmaOS offers the ultimate power and flexibility for enthusiasts and enterprise engineers alike.

---

## 🏗️ 1. Technical Vision & Integration Matrix

Traditional distributions suffer from bloated, static release schedules. SigmaOS adopts Arch's **KISS (Keep It Simple, Stupid)** philosophy and implements a native **Rolling Release Engine** with sandboxed compilation tools.

```
       +-------------------------------------------------------+
       |                  Sovereign Packaging                  |
       +-------------------------------------------------------+
            |                        |                       |
            v                        v                       v
   +-----------------+      +-----------------+      +-----------------+
   |  PKGBUILD Spec  |      |   AUR Sandbox   |      |  ALPM Sync DB   |
   | (Source Rec)    |      | (Safe Compiler) |      | (Fast Tar.zst)  |
   +-----------------+      +-----------------+      +-----------------+
```

---

## 🦅 2. PKGBUILD Spec & makepkg compiler parity (Rust / Zig)

### 2.1 Native PKGBUILD Parsing
- **Inspiration**: Arch Linux `PKGBUILD` and `makepkg` scripts.
- **Implementation (Rust)**: We extend `PackageRecipe` inside `src/sigpkg/recipe.rs` to support structured representations of `PKGBUILD` standards:
  - `pkgrel`: Release incrementers.
  - `arch`: Target architectures (x86_64, aarch64, riscv64).
  - `prepare()`, `build()`, and `package()` hooks represented as ordered capability-gated commands execution sequences.

### 2.2 Sandboxed Compiler (Zig)
- Unprivileged user-space compilers in Zig compile the downloaded source packages safely without root permissions, generating secure `.tar.zst` equivalents with Dilithium-5 signatures.

---

## 📦 3. Arch User Repository (AUR) Integration (Nim / Rust)

### 3.1 AUR Client & Helper Subsystem
- **Inspiration**: `yay`, `paru`, and `aurweb`.
- **Implementation (Nim)**: High-speed, compiled Nim command-line helpers parse dependency trees from AUR metadata endpoints, flagging conflict intersections.
- **Implementation (Rust)**: Automated sandboxed isolation profiles inside the Universal Package Manager sandbox compile custom AUR recipes safely.

---

## 🔄 4. ALPM Database Sync & Rolling Releases

### 4.1 Local Metadata Sync DB
- **Inspiration**: Pacman `libalpm` library.
- **Implementation (Rust)**: Sync directories maintain structured, flat-text metadata databases caching active versions, preventing traditional distributed sync lag.

---

## 📅 5. Step-by-Step Implementation Roadmap

- [ ] **Phase 1 (Validation)**: Implement PKGBUILD and package release metadata inside `src/sigpkg/recipe.rs`.
- [ ] **Phase 2 (makepkg Sandbox)**: Bridge compiler scripts with the kernel capability sandboxes.
- [ ] **Phase 3 (AUR CLI Helper)**: Code userland helper utilities to download, compile, and sync AUR packages on-the-fly.
