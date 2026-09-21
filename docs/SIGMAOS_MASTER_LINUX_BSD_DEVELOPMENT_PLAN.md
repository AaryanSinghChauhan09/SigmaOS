# 🌟 Master Linux & BSD Inspired SigmaOS Development Plan: Comprehensive Architecture & Roadmap

## 1. Executive Summary & Core Philosophy

**SigmaOS** is a secure, ultra-fast, post-Linux sovereign desktop operating system engineered in Safe-Rust with a strict **zero-external-dependency** policy in `Cargo.toml`.

By synthesizing the most robust paradigms from **Linux** (eBPF/io_uring performance, cgroup v2 container isolation, SchedExt BPF scheduling, CachyOS microarch tuning), **BSD** (FreeBSD ZFS/bectl boot environments, OpenBSD `pledge`/`unveil` path sandboxing, `kqueue` event loops), and **Modern Desktop Distros** (Omarchy/Hyprland auto-tiling, Pop!_OS COSMIC spatial grids, NixOS generational CoW state rollbacks), SigmaOS establishes a single demonstrable user journey:

```text
boot → install → login → Zenith desktop → package installation → atomic update → generational rollback
```

---

## 2. Synthesis of Linux & BSD Inspirations across 8 Core OS Pillars

| OS Pillar | Inspired Linux & BSD Ecosystem Paradigms | Key Capabilities Incorporated into SigmaOS |
|---|---|---|
| **1. Microkernel & Memory** | FreeBSD `kqueue`, Linux `io_uring`, `cgroup_v2`, SLUB allocator, Landlock LSM v5, eBPF VM | Sub-120ns zero-copy IPC, parallel sub-second boot sequencer (<250ms), and per-CPU kmalloc slab caches with redzone poisoning. |
| **2. Universal Package System (`sigpkg`)** | Gentoo ebuild algebra, Debian APT control, Arch `pacman`, FreeBSD `pkg`, Alpine `apk`, Nix/Guix Merkle stores | Universal PM handling 60+ package extensions (`.deb`, `.rpm`, `.pkg.tar.zst`, `.apk`, `.pkg`, `.nix`, `.snap`, `.flatpak`) with Merkle stores, boolean SAT solvers, Dilithium-5 PQC signatures, and containerized `apx` execution. |
| **3. Polyglot CLI Shell (`sigma-sh`)** | Fish auto-suggestions, Zsh prompts & completions, OpenBSD `doas`/`pledge`, FreeBSD `bectl` | Multi-format shell engine (`SovereignUniversalCliShellSystemEngine`) supporting 13 dialects (Bash, Zsh, Fish, Tcsh, Ksh, Dash, RC, Nushell, Ion, Elvish, Xonsh, Oil, PWSH), option flag translation, and dotfile parsing. |
| **4. Zenith Desktop & Compositor** | Omarchy/Hyprland dwindle tiling, KDE Plasma 6 spatial grid, Pop!_OS COSMIC stacking, GNOME Mutter damage tracking | Keyboard-driven Wayland compositor (`ZenithWaylandCore`), agentic steering tri-engine (Bolt ⚡, Palette 🎨, Sentinel 🛡️), zero-CSS native widget rendering, and live palette switching. |
| **5. Polyglot Toolchains** | Gentoo `emerge` flags, Fedora hardening, BSD `bmake`, CMake, Ninja, Cargo | Universal toolchain engine (`SovereignUniversalToolchainEngine`) supporting 13+ build drivers (GCC, Clang, Rustc, Zig, Nim, Go, Swift, Dmd, Fortran, Autotools, CMake, Meson/Ninja, Bmake) with linker selection (`ld.bfd`, `ld.lld`, `mold`). |
| **6. Security & Sandboxing** | OpenBSD `pledge`/`unveil`, FreeBSD Capsicum, Linux Landlock v5 LSM, Zorin Exec Guard | Default-deny capability permission model (`FineGrainedAccessControlMatrix`), path veiling, and interactive capability prompt popups for untrusted binaries. |
| **7. Atomic System Updates** | Fedora Silverblue / rpm-ostree, openSUSE Snapper, NixOS generations, FreeBSD ZFS `bectl` | Dual-root A/B images (`mkosi`/`sysupdate`), Adler-32 delta updates, and sub-second Copy-on-Write (CoW) system rollbacks (`sigma-pkg rollback`). |
| **8. Ecosystem & User Onboarding** | Pop!_OS / Ubuntu installer personas, Arch Wiki knowledge base, ItsFOSS productivity tools | Persona-guided installer onboarding (`Developer`, `Compliance`, `Student`, `Gaming`, `Minimal`), 10-domain FAQ knowledge base, and ItsFOSS-inspired system tools. |

---

## 3. Master Development Roadmap & Milestone Timeline

```
+-----------------------------------------------------------------------------------+
|                        SIGMAOS MASTER DEVELOPMENT TIMELINE                         |
+-----------------------------------------------------------------------------------+
  Milestone 0: Engineering Baseline & Zero-Dependency Contract    [COMPLETED]
  Milestone 1: QEMU Desktop Preview & Zenith Wayland Core         [ACTIVE FOCUS]
  Milestone 2: Native Package MVP (`sigpkg`) & SAT Dependency Solver [ACTIVE FOCUS]
  Milestone 3: Declarative Profile Specifications & Live Theming   [ACTIVE FOCUS]
  Milestone 4: Certified Bare-Metal Hardware Alpha (Intel/AMD)     [PLANNED]
  Milestone 5: Stable Public ISO Release & Generational Rollback  [PLANNED]
+-----------------------------------------------------------------------------------+
```

### Milestone 0: Engineering Baseline (✅ Completed)
- [x] Zero external dependencies in `Cargo.toml` (`[dependencies]` is empty).
- [x] Commitment to hybrid practical `std` architecture for userland/desktop per `ARCHITECTURE.md`.
- [x] Standardized documentation contracts in `docs/` (`PRODUCT_VISION.md`, `RELEASE_CRITERIA.md`, `SUPPORT_MATRIX.md`).

### Milestone 1: QEMU Desktop Preview (🔄 Active Focus)
- [x] Implement Zenith Wayland compositor protocol types in `src/compositor/zenith_core.rs`.
- [x] Implement sub-second boot sequencer in `src/init/subsecond_boot_sequencer.rs`.
- [x] Implement Omarchy QuickShell HUD bridge and live theme engine in `src/distro/omarchy_master_synthesis.rs`.
- [ ] Finalize direct DRM/KMS scanout buffer integration with VirtIO-GPU.
- [ ] Automated headless QEMU boot test emitting serial heartbeat string.

### Milestone 2: Native Package System (`sigpkg`) (🔄 Active Focus)
- [x] Merkle tree store (`src/sigpkg/merkle_store.rs`) and Adler-32 delta engine (`src/sigpkg/delta_engine.rs`).
- [x] Boolean SAT solver with Gentoo/APT algebra (`src/sigpkg/boolean_dep_solver.rs`).
- [x] Dilithium-5 post-quantum package signing authority (`src/sigpkg/package_signing.rs`).
- [x] Foreign package importer supporting 60+ Linux/BSD format extensions (`src/package/universal.rs`).
- [ ] Finalize atomic snapshot creation and single-command rollback tool (`sigma-pkg rollback`).

### Milestone 3: Declarative Configuration & User Personas (🔄 Active Focus)
- [x] Dynamic live theme switcher (`OmarchyThemeLiveEngine`) supporting Tokyo-Night, Catppuccin, Gruvbox, and Nord.
- [x] Web2App sandboxed launcher generator (`OmarchyAppSandbox`).
- [x] User onboarding persona selector (`Developer`, `Compliance`, `Student`, `Gaming`, `Minimal`) in `src/installer/gui_wizard.rs`.
- [ ] Declarative profile specification (`/system/profile.toml`, `/user/preferences.toml`).

### Milestone 4: Bare-Metal Hardware Alpha (📅 Planned)
- [ ] Certified x86_64 bare-metal boot (Intel 8th+ Gen / AMD Ryzen).
- [ ] Integrated graphics hardware acceleration and DRM/KMS driver validation.
- [ ] Intel Wi-Fi / Realtek wired networking status tray panel in Zenith QuickShell.

### Milestone 5: Stable Desktop Release & Public ISO (📅 Planned)
- [ ] Public bootable ISO image with minimal installer wizard.
- [ ] Curated application bundle (terminal, editor, viewer, browser bridge).
- [ ] Fail-safe recovery menu with automatic generational rollback.

---

## 4. Quality Assurance, Testing & CI Strategy

1. **Native Test Execution:** Verified via `./run_sigma_tests.sh` (running all 12 test suites across launch readiness, package systems, distro parity, and wiki implementations).
2. **Standalone Test Verification:** Verified via `./scripts/changed_files_rustc_tests.sh` executing standalone `rustc --test` binaries.
3. **Strict SHA Pinning:** All GitHub Actions workflows in `.github/workflows/` enforce SHA-pinned action references.
