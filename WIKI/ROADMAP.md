# SigmaOS Canonical Roadmap: Desktop Edition & Beyond

> **Status Reference**:
> - `[x] Implemented`: Complete, tested, and actively compiling in the codebase.
> - `[~] Prototype`: In-tree prototype or specification ready for hardening.
> - `[ ] Planned`: Actively scheduled milestone task.
> - `[*] Research`: Long-term experimental exploration.

---

## 🎯 Primary Focus: SigmaOS Desktop Edition (Milestones M0 – M5)

### M0: Engineering Baseline (Status: ✅ Completed)
- [x] Strict zero-external-dependency policy in `Cargo.toml` (`[dependencies]` is empty).
- [x] Zero compilation errors across `cargo check --lib`.
- [x] Unified repository with git branches pruned to `main` only.
- [x] Clear engineering contract and separation between Desktop Edition and Research.
- [x] Standardized documentation in `docs/` (`PRODUCT_VISION.md`, `RELEASE_CRITERIA.md`, `ARCHITECTURE_DECISIONS.md`, `SUPPORT_MATRIX.md`).

### M1: QEMU Desktop Preview (Status: 🔄 Active)
- [x] Zenith Wayland Compositor Core (`src/compositor/zenith_core.rs`): Direct buffer compositing, damage tracking, and multi-monitor output.
- [x] Sub-second Boot Sequencer (`src/init/subsecond_boot_sequencer.rs`): Parallel stage activation (<250ms target).
- [x] Omarchy QuickShell HUD Bridge (`src/distro/omarchy_master_synthesis.rs`): Top bar, launcher, and control center.
- [~] Direct KMS/DRM scanout buffer integration with VirtIO-GPU.
- [ ] Automated QEMU boot test emitting serial heartbeat and reaching desktop.

### M2: Native Package MVP (`sigpkg`) (Status: 🔄 Active)
- [x] Content-addressed Merkle Store (`src/sigpkg/merkle_store.rs`).
- [x] Differential binary delta update engine with Adler-32 integrity (`src/sigpkg/delta_engine.rs`).
- [x] Boolean SAT dependency solver with Gentoo/APT algebra (`src/sigpkg/boolean_dep_solver.rs`).
- [x] Dilithium-5 cryptographic package signing authority (`src/sigpkg/package_signing.rs`).
- [~] Atomic snapshot creation and one-command rollback tool (`sigma-pkg rollback`).

### M3: Declarative Configuration & Theming (Status: 🔄 Active)
- [x] Dynamic Theme Live Switcher (`OmarchyThemeLiveEngine`): Instant Tokyo-Night, Catppuccin, Gruvbox, and Nord palette switching.
- [x] Web2App sandboxed launcher generator (`OmarchyAppSandbox`).
- [~] Declarative profile specification (`/system/profile.toml`, `/user/theme.toml`).
- [ ] Atomic generation switcher with recovery boot option.

### M4: Hardware Alpha (Status: 📅 Planned)
- [ ] Certified x86_64 bare-metal boot (Intel 8th+ Gen / AMD Ryzen).
- [ ] Integrated graphics hardware acceleration.
- [ ] Intel Wi-Fi / Realtek wired networking driver validation.
- [ ] Hardware compatibility reporting tool.

### M5: Stable Desktop Release (Status: 📅 Planned)
- [ ] Public bootable ISO image with minimal installer.
- [ ] Curated application bundle (terminal, editor, viewer, browser bridge).
- [ ] Fail-safe recovery menu with automatic generational rollback.

---

## 🔬 Long-Term Research Tracks (Post-Desktop Release)

| Track | Scope | Status |
|---|---|---|
| **Freestanding Microkernel** | Pure `#![no_std]` capability microkernel with seL4 formal isolation | [*] Research |
| **Post-Quantum Mesh VPN** | PQC-WireGuard (`Kyber-1024` / `Dilithium-5`) inter-node mesh | [~] Prototype |
| **Cluster Memory & Migration** | CRIU-style zero-downtime process migration across pooled nodes | [~] Prototype |
| **Multi-Architecture** | Tier-1 support for ARM64 (Apple Silicon / Pi 5) and RISC-V 64 | [*] Research |
| **Universal Distro ABI** | Complete binary compatibility for Linux ELF and FreeBSD binaries | [*] Research |
