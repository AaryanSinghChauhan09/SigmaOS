# 🎯 SigmaOS Future Development Roadmap — Inspired by Omarchy Linux

> **Target Repository:** [https://github.com/AaryanSinghChauhan09/SigmaOS](https://github.com/AaryanSinghChauhan09/SigmaOS)
> **Document Status:** Proposed strategic roadmap (tracking document)
> **Last Updated:** 2026-09-27
> **Scope:** Desktop/UX, system configuration, hardware, developer tooling, security, and ecosystem work items that close known SigmaOS gaps using architectural patterns proven by [Omarchy](https://github.com/omacom/omarchy).
> **Related Documents:** [FUTURE-DEVELOPMENT-ROADMAP.md](../FUTURE-DEVELOPMENT-ROADMAP.md) · [WHAT_IS_WORKING_AND_NOT_WORKING.md](WHAT_IS_WORKING_AND_NOT_WORKING.md) · [ImprovementPlan.md](ImprovementPlan.md) · [OPERATIONS_AND_CONTINUOUS_IMPROVEMENT_GUIDE.md](OPERATIONS_AND_CONTINUOUS_IMPROVEMENT_GUIDE.md) · [CONTRIBUTING.md](../CONTRIBUTING.md)

> **Timeline note:** The quarter labels below are the originally planned windows from the roadmap proposal. As of this document's date (2026-09-27) the Q1–Q3 2026 windows have already elapsed, so treat the quarters as **sequence and priority**, not calendar commitments, until maintainers re-baseline them (see [Estimated Timeline](#-estimated-timeline)).

---

## 📚 Table of Contents

1. [How to Read This Document](#-how-to-read-this-document)
2. [Baseline: Where SigmaOS Stands Today](#-baseline-where-sigmaos-stands-today)
3. [Phase 1: Desktop Environment & User Experience](#phase-1-desktop-environment--user-experience-q1q2-2026)
4. [Phase 2: System Administration & Configuration](#phase-2-system-administration--configuration-q2q3-2026)
5. [Phase 3: Hardware Drivers & Optimization](#phase-3-hardware-drivers--optimization-q3q4-2026)
6. [Phase 4: Developer Experience & Tooling](#phase-4-developer-experience--tooling-q1q2-2027)
7. [Phase 5: Security Hardening & Sandboxing](#phase-5-security-hardening--sandboxing-ongoing)
8. [Phase 6: Community & Ecosystem](#phase-6-community--ecosystem-parallel-2026)
9. [Priority Implementation Order](#-priority-implementation-order)
10. [Omarchy Features to Directly Adopt](#-omarchy-features-to-directly-adopt)
11. [Estimated Timeline](#-estimated-timeline)
12. [Definition of Done & Verification](#-definition-of-done--verification)
13. [Tracking & Governance](#-tracking--governance)

---

## 📖 How to Read This Document

Each phase is organised the same way:

| Field | Meaning |
| :--- | :--- |
| **Gap** | What SigmaOS is missing or has only partially implemented today. |
| **Omarchy Inspiration** | The Omarchy pattern or strength being adopted. |
| **Deliverables** | Task checklist. Unchecked boxes are open work items; convert each into a GitHub issue before starting (see [Tracking & Governance](#-tracking--governance)). |
| **Where It Lives Today** | Existing modules under `src/` that the work should extend rather than duplicate. Most are scaffolding with standalone unit tests; see the caveat below. |
| **Exit Criteria** | Observable conditions that let maintainers mark the phase complete. |

> ⚠️ **Status caveat.** Per [WHAT_IS_WORKING_AND_NOT_WORKING.md](WHAT_IS_WORKING_AND_NOT_WORKING.md), individual modules pass standalone `rustc --test` runs via `./run_sigma_tests.sh`, while the unified `cargo check` / `cargo build` of the workspace still fails due to duplicate definitions and macro collisions. "Module present" in this document therefore means *scaffolding and unit tests exist*, not *ships in a bootable image*. Fixing the workspace build is the first critical item below precisely because every later phase depends on it.

---

## 🧭 Baseline: Where SigmaOS Stands Today

The following existing assets are the starting point for this roadmap (paths verified against the current tree):

| Area | Existing Modules | Notes |
| :--- | :--- | :--- |
| Zenith compositor | `src/compositor/zenith_core.rs`, `src/compositor/sigma_compositor.rs`, `src/desktop/zenith_compositor.rs`, `src/desktop/zenith.rs`, `src/desktop/zenith_advanced_features.rs`, `src/desktop/wayland_protocol.rs` | Several parallel implementations; needs consolidation into one production path. |
| Omarchy-inspired desktop pieces | `src/desktop/omarchy_theme.rs`, `src/desktop/omarchy_omakase.rs`, `src/desktop/omarchy_dynamic_workspace_suite.rs`, `src/distro/omarchy*.rs` | Theme system, Omakase presets, Super-key scratchpads/workspaces. |
| Launcher / menu / notifications | `src/launcher/app_launcher.rs`, `src/desktop/launcher.rs`, `src/desktop/menu.rs`, `src/notification/notification_system.rs`, `src/desktop/notifications.rs` | Fuzzy search, command palette, and recent-apps concepts already sketched. |
| Theming & UI toolkit | `src/theming/theme_engine.rs`, `src/ui/toolkit.rs`, `src/ui/widget_api.rs`, `src/accessibility/` | Theme engine is `#![no_std]`; widget API needs WCAG coverage. |
| Declarative config | `src/config/declarative.rs`, `src/config/manager.rs`, `src/config/loader.rs` | Generation tracking and rollback state machine exist; no DSL parser yet. |
| Packaging | `src/sigpkg/` (`boolean_dep_solver.rs`, `resolver.rs`, `aur_helper.rs`, `nix_dsl.rs`, `universal_adapter.rs`, …) | SAT solver and AUR helper scaffolds present; Flatpak/Snap bridges live in `src/compatibility/` and `src/container/runtime.rs`. |
| GPU | `src/driver/gpu_nvidia_nouveau.rs`, `src/driver/gpu_amd_rdna.rs`, `src/driver/gpu_intel_i915.rs`, `src/driver/gpu_drm_subsystem.rs`, `src/gpu/driver.rs`, `src/graphics/multi_monitor.rs` | NVIDIA GSP firmware translation is a known blocker. |
| Audio / Bluetooth | `src/audio/pipewire.rs`, `src/audio/alsa.rs`, `src/driver/audio_codec_hda.rs`, `src/bluetooth/` | PipeWire module is a thin scaffold (~135 lines). |
| USB / input | `src/driver/usb_xhci_host.rs`, `src/drivers/modern_usb.rs`, `src/driver/hid_input_device.rs`, `src/input/`, `src/touchscreen/` | xHCI ring management present; gestures and Wacom curves absent. |
| Power / thermal | `src/power/governor.rs`, `src/power/battery.rs`, `src/power/management.rs`, `src/thermal/manager.rs` | Governor and battery models exist; no hybrid-graphics switching. |
| Containers / toolchain | `src/container/oci_runtime.rs`, `src/container/oci_orchestrator.rs`, `src/toolchain/self_host.rs`, `src/toolchain/bootstrap.rs`, `src/compiler/` | OCI runtime scaffold; self-hosting compiler is a bootstrap plan, not a working compiler. |
| Security | `src/pledge.rs`, `src/security/bsd_pledge.rs`, `src/security/landlock.rs`, `src/security/landlock_sovereign.rs`, `src/security/capsicum.rs`, `src/security/mac.rs` | pledge/unveil and Landlock scaffolds exist; AppArmor/SELinux only referenced in compatibility modules. |
| Post-quantum crypto | `src/crypto/post_quantum.rs`, `src/crypto/postquantum.rs`, `src/crypto/pqc_dilithium.rs`, `src/crypto/vectorized_pqc.rs` | Duplicate module names (`post_quantum` vs `postquantum`) — consolidation needed. |
| Installer | `src/installer/gui_wizard.rs`, `src/installer/system_installer.rs`, `src/boot/sovereign_multi_distro_bootloader.rs` | Calamares-style dual-boot alongside mode and Limine scanning. |

---

## PHASE 1: DESKTOP ENVIRONMENT & USER EXPERIENCE (Q1–Q2 2026)

### 1.1 Zenith Desktop Compositor Enhancement

**Gap:** Zenith is in progress; it needs maturation for real-world desktop workflows, and the current tree carries several parallel compositor implementations that must converge into one.

**Omarchy Inspiration:** Beautiful, cohesive visual design with a modular theming system — every surface (compositor, terminal, editor, launcher) reads from one theme definition.

**Deliverables:**

- [ ] Complete Wayland-like protocol for client-server graphics architecture (surface, buffer, seat, output, and xdg-shell-equivalent objects) — extend `src/desktop/wayland_protocol.rs`
- [ ] Widget library (buttons, text fields, panels) with WCAG 2.1 AAA accessibility (focus rings, ARIA-equivalent roles, contrast ≥ 7:1, screen-reader hooks via `src/accessibility/screenreader.rs`)
- [ ] Theme engine supporting light/dark modes, custom accent colors, and per-app overrides — unify `src/theming/theme_engine.rs` and `src/desktop/omarchy_theme.rs`
- [ ] Multi-monitor support with HiDPI scaling and adaptive layout (fractional scale factors, per-output DPI, hot-plug re-layout) — extend `src/graphics/multi_monitor.rs`
- [ ] Keyboard-driven workflows with Super key bindings (`Super+Number` workspace jump, `Super+Arrow` focus/move navigation, `Super+Shift+Arrow` window move) — extend `src/desktop/omarchy_dynamic_workspace_suite.rs`
- [ ] Notification daemon with persistent notification center and do-not-disturb modes — extend `src/notification/notification_system.rs`
- [ ] Consolidate `src/compositor/zenith_core.rs`, `src/compositor/sigma_compositor.rs`, `src/desktop/zenith_compositor.rs`, and `src/graphics/zenith_compositor.rs` into a single canonical compositor crate path with re-exports (Algorithm A in `WHAT_IS_WORKING_AND_NOT_WORKING.md`)

**Exit Criteria:** A single Zenith binary boots in the QEMU smoke test (`scripts/qemu_smoke_test.sh`), renders two windows on two virtual outputs at different scale factors, switches theme at runtime, and passes `scripts/uiux_accessibility_test.sh`.

### 1.2 Application Menu & Launcher

**Omarchy Pattern:** Single-key access to everything; the launcher is the primary way users start work.

**Deliverables:**

- [ ] Application menu (`Super` key opens a grid of installed apps with incremental search) — extend `src/desktop/menu.rs`
- [ ] QuickRun launcher (`Alt+F2` or `Super+R` for command execution with shell completion) — extend `src/launcher/app_launcher.rs` command palette
- [ ] Recent applications sidebar (frequently used apps pinned, usage-weighted ordering)
- [ ] Workspace switcher with visual thumbnails (`M1–M4` keys or `Super+Page Up/Down`) — extend `src/desktop/workspace.rs`
- [ ] `.desktop`-entry–compatible application index so packages installed via `sigpkg` bridges appear automatically

**Exit Criteria:** Cold launcher open-to-render < 50 ms on the reference VM; search returns results per keystroke; all actions reachable without a pointer.

---

## PHASE 2: SYSTEM ADMINISTRATION & CONFIGURATION (Q2–Q3 2026)

### 2.1 Declarative System Configuration (NixOS/Omarchy-Inspired)

**Gap:** SigmaOS documents a declarative overlay (see `FUTURE-DEVELOPMENT-ROADMAP.md` §2.4) and has generation tracking in `src/config/declarative.rs`, but there is no configuration language, parser, or apply engine.

**Roadmap:**

- [ ] **SigmaConfig JSON/TOML DSL** — zero-dependency TOML/JSON parser in `src/config/loader.rs`, schema validation, and typed module options:

  ```toml
  # Example declarative system config
  [system]
  hostname = "sigma-workstation"
  timezone = "UTC"
  locale = "en_US.UTF-8"

  [services]
  ssh.enabled = true
  ssh.port = 22
  dhcp.enabled = true

  [packages]
  system_packages = ["git", "neovim", "firefox", "syncthing"]
  ```

- [ ] Atomic system state with instant rollback (< 50 ms using Btrfs snapshots) — wire `src/config/declarative.rs` generations to `src/filesystem/cow_snapshot.rs` and `src/filesystem/btrfs_inspired.rs`
- [ ] Idempotent configuration reapplication (running config twice = same result; apply plan is a diff against the active generation)
- [ ] Configuration version control integration (track config history in git; `sigma-config diff`, `sigma-config log`, `sigma-config rollback <generation>`)
- [ ] Omarchy-style "one command to reconfigure" CLI entrypoint (`sigma-config apply`) that stages, verifies, and commits a generation

**Exit Criteria:** Applying the example config on a clean image produces a running SSH service and installed package set; re-applying reports zero changes; rollback to the previous generation completes under 50 ms in the benchmark harness (`scripts/benchmark-boot.sh`).

### 2.2 Package Management Unification

**Gap:** The `sigpkg` framework exists (`src/sigpkg/`) but needs Omarchy's flexibility — Omarchy leans on the whole Arch/AUR ecosystem instead of maintaining everything itself.

**Integration Points:**

- [ ] Flatpak support (containerized GUI apps with sandboxing; portal integration via `src/desktop/desktop_portal.rs`)
- [ ] Snap bridge for Canonical packages (squashfs mount + confinement mapping onto pledge/unveil)
- [ ] AUR helper for Arch User Repository packages — harden `src/sigpkg/aur_helper.rs` / `src/sigpkg/makepkg.rs` with PKGBUILD sandboxing and signature checks
- [ ] Custom package recipes with dependency resolution (SAT solver ✅ already planned — `src/sigpkg/boolean_dep_solver.rs`)
- [ ] Binary package caching to avoid compilation delays (content-addressed cache keyed by recipe hash; `src/sigpkg/merkle_store.rs`)
- [ ] Unified `sigpkg install <name>` front-end that selects native → Flatpak → AUR → Snap according to a configurable policy

**Exit Criteria:** One CLI installs a native `.sigpkg`, a Flatpak, and an AUR package end-to-end; dependency conflicts are reported by the SAT solver before any filesystem mutation; failed transactions roll back atomically (`src/sigpkg/transaction.rs`).

---

## PHASE 3: HARDWARE DRIVERS & OPTIMIZATION (Q3–Q4 2026)

### 3.1 Modern Hardware Support Stack

**Currently Limited:** Driver scaffolds exist under `src/driver/` and `src/drivers/`, but real-hardware bring-up, firmware handling, and acceleration paths are incomplete.

- [ ] **GPU acceleration**
  - [ ] NVIDIA: Complete NVK (Nouveau Vulkan) integration; bypass the GSP firmware translation blocker documented in `WHAT_IS_WORKING_AND_NOT_WORKING.md` §3.C — `src/driver/gpu_nvidia_nouveau.rs`
  - [ ] AMD: AMDGPU KMS/DRM with RDNA 3 support — `src/driver/gpu_amd_rdna.rs`, `src/driver/gpu_drm_subsystem.rs`
  - [ ] Intel: Iris Xe driver with hardware video encoding — `src/driver/gpu_intel_i915.rs`
- [ ] **Audio subsystem**
  - [ ] PipeWire audio server (modern, low-latency audio routing graph) — grow `src/audio/pipewire.rs` from scaffold to a scheduling graph with quantum/latency control
  - [ ] ALSA compatibility layer for legacy apps — `src/audio/alsa.rs`
  - [ ] Bluetooth audio profiles (A2DP, HFP, LDAC) — `src/bluetooth/`
- [ ] **Input devices**
  - [ ] USB 3.2 (xHCI) controller (planned ✅; refine implementation, MSI-X, isochronous transfers) — `src/driver/usb_xhci_host.rs`
  - [ ] Multi-touch touchpad support with gestures (3/4-finger swipe → workspace switch, pinch → zoom)
  - [ ] Wacom tablet pressure curves (see Nice-to-Have tier)

**Exit Criteria:** Zenith renders with GPU acceleration on at least one card per vendor in the hardware matrix; audio plays through PipeWire with < 10 ms quantum; a USB 3 keyboard/mouse/touchpad enumerate on real hardware.

### 3.2 Power Management & Thermal Control

**Omarchy Pattern:** Efficient laptop/mobile support out of the box (Omarchy's primary audience is laptop developers).

- [ ] CPU frequency scaling (Intel SpeedStep/HWP, AMD P-States, ARM CPUFREQ) — `src/power/governor.rs`
- [ ] Battery management (suspend, hibernate, low-power modes, charge thresholds) — `src/power/battery.rs`, `src/power/management.rs`
- [ ] Thermal throttling with fan curve presets — `src/thermal/manager.rs`
- [ ] Hybrid graphics support (iGPU vs dGPU switching) — `src/graphics/nvidia_prime.rs`
- [ ] ACPI S3/S4 state machine integration with the scheduler and driver suspend/resume callbacks

**Exit Criteria:** Suspend/resume cycle survives on the reference laptop; idle power draw within 15% of a stock Omarchy install on the same machine; thermal presets selectable from the declarative config.

---

## PHASE 4: DEVELOPER EXPERIENCE & TOOLING (Q1–Q2 2027)

### 4.1 Integrated Development Environment

**Leverage SigmaOS's Rust-native kernel:** developers are Omarchy's core audience; SigmaOS should offer a first-party, lightweight environment that works offline.

- [ ] **Sigma IDE** (lightweight, Rust-based alternative to VS Code)
  - [ ] LSP support for Rust, C, Python, Go
  - [ ] Built-in terminal with shell integration (`src/desktop/terminal.rs`, `sigma-sh`)
  - [ ] Git integration with GitHub/GitLab
  - [ ] Debugger support (GDB/LLDB protocol adapters; `src/debugger/`)
- [ ] **Container runtime**
  - [ ] OCI-compatible container support (lightweight Docker alternative) — `src/container/oci_runtime.rs`
  - [ ] Pod definitions and orchestration — `src/container/oci_orchestrator.rs`
  - [ ] Network namespace isolation — `src/security/namespaces.rs`

**Exit Criteria:** Sigma IDE can open, build, and debug the SigmaOS repository itself; an OCI image pulled from a registry runs with network isolation and cgroup limits.

### 4.2 Self-Hosting Compiler Toolchain

**Currently Missing:** A self-hosted Rust compiler. `src/toolchain/self_host.rs` and `src/toolchain/bootstrap.rs` describe the bootstrap plan but do not compile Rust.

- [ ] `mrustc` (minimal Rust compiler) ported to SigmaOS as the stage-0 bootstrap
- [ ] Rust standard library recompilation for the bare-metal target (`rust-toolchain.toml` pinned; `alloc`/`core` first, `std` shim second)
- [ ] LLVM/Clang minimal distribution build (C toolchain for driver shims and mrustc output)
- [ ] Cranelift JIT for faster compilation cycles (debug builds and Sigma IDE incremental compile)
- [ ] Reproducible-build verification of the toolchain (ties into `.github/workflows/reproducible-sbom-cosign.yml`)

**Exit Criteria:** SigmaOS builds its own kernel image on SigmaOS with a bit-identical hash to the CI artifact.

---

## PHASE 5: SECURITY HARDENING & SANDBOXING (Ongoing)

### 5.1 Capability-Based Security

**Blend Omarchy's approach with FreeBSD/OpenBSD inspiration:** Omarchy ships sane defaults and trusts upstream sandboxing; SigmaOS should make sandboxing the default for every launched app.

- [ ] `pledge`/`unveil` implementation (✅ already planned — `src/pledge.rs`, `src/security/bsd_pledge.rs`), enforced at the syscall gate:

  ```rust
  // Allow only reading from /home and network access
  sigma_pledge("stdio rpath inet")?;
  sigma_unveil("/home", "r")?;
  sigma_unveil("/dev/null", "rw")?;
  ```

- [ ] Landlock LSM (Linux Landlock semantics ported to SigmaOS) — `src/security/landlock.rs`, `src/security/landlock_sovereign.rs`
- [ ] AppArmor profiles for common apps (profile parser + enforcement mode; today AppArmor is only referenced in `src/compatibility/linux_security.rs`)
- [ ] SELinux compatibility mode for cross-distro portability (label mapping onto `src/security/mac.rs`)
- [ ] Per-application sandbox manifests emitted by `sigpkg` so Flatpak/Snap/AUR installs receive pledge/unveil/Landlock policies automatically

**Exit Criteria:** Every app launched from the Zenith launcher runs under a declared pledge/unveil set; policy violations are logged to the audit subsystem (`src/security/audit.rs`) and surfaced in the notification center.

### 5.2 Post-Quantum Cryptography

- [ ] Kyber-1024 KEM (ML-KEM-1024 / FIPS 203) for key exchange — consolidate `src/crypto/post_quantum.rs` and `src/crypto/postquantum.rs`
- [ ] Dilithium-5 digital signatures (ML-DSA-87 / FIPS 204) for integrity — `src/crypto/pqc_dilithium.rs`; use for `sigpkg` package signing (`src/sigpkg/package_signing.rs`)
- [ ] SPHINCS+ hash-based signatures (SLH-DSA / FIPS 205) as fallback
- [ ] TLS 1.3 hybrid classical/PQC key exchange (X25519 + ML-KEM) in the native network stack, forward-compatible with future TLS revisions
- [ ] Known-answer tests (KATs) against the NIST reference vectors in `tests/`

**Exit Criteria:** Package signatures and system-update manifests are Dilithium-signed; the network stack negotiates a hybrid PQC handshake against a reference server; all KATs pass in `./run_sigma_tests.sh`.

---

## PHASE 6: COMMUNITY & ECOSYSTEM (Parallel, 2026+)

### 6.1 Omarchy-Inspired Community Features

- [ ] User theme marketplace (share custom Zenith themes) — `src/theming/theme_engine.rs` already reserves marketplace hooks
- [ ] Community-driven hardware support (crowdsourced driver testing and a compatibility database; `src/community/`, `src/driver/driver_test_framework.rs`)
- [ ] Package recipe repository (community-contributed `sigpkg` recipes; `src/sigpkg/recipe.rs`, `src/sigpkg/repository_manager.rs`)
- [ ] Documentation wiki with tutorials and troubleshooting guides (generated from `docs/` per `.github/workflows/documentation-checks.yml`)

### 6.2 Distro Bridges

From [FUTURE-DEVELOPMENT-ROADMAP.md](../FUTURE-DEVELOPMENT-ROADMAP.md): support 29+ Linux/BSD formats.

- [ ] NixOS Flakes compatibility layer — `src/sigpkg/nixos.rs`, `src/sigpkg/nix_dsl.rs`
- [ ] Guix Scheme package recipe support
- [ ] Homebrew bottle extraction and repackaging (`src/package/`)
- [ ] Linux kernel modules compatibility shim (for driver portability; `src/driver/shims.rs`, `src/driver/dkms_autoloader.rs`)

**Exit Criteria:** At least one third-party theme, one community driver report, and one community recipe merged through the documented contribution flow.

---

## 🚦 Priority Implementation Order

```
🔴 CRITICAL (Q1 2026):
  1. Fix cargo workspace compilation (remove duplicate definitions)
  2. Complete Zenith compositor (make it production-ready)
  3. Implement declarative system config (SigmaConfig DSL)
  4. Finish GPU driver support (NVIDIA, AMD, Intel)

🟡 HIGH (Q2–Q3 2026):
  5. Audio subsystem (PipeWire integration)
  6. Power management (suspend, battery, thermal)
  7. Application menu & launcher
  8. Package format bridges (Flatpak, Snap, AUR)

🟢 MEDIUM (Q4 2026 – Q2 2027):
  9.  Lightweight IDE (Sigma IDE)
  10. Container runtime (OCI support)
  11. Self-hosting compiler
  12. Post-quantum crypto stack
  13. Security hardening (pledge/unveil, Landlock)

⚪ NICE-TO-HAVE (Future):
  14. Wacom tablet support
  15. Theme marketplace
  16. Community hardware database
  17. Advanced debugging tools
```

| Priority | Item | Phase | Primary Modules | Blocking Dependency |
| :---: | :--- | :---: | :--- | :--- |
| 🔴 1 | Fix cargo workspace compilation | — | `src/lib.rs`, duplicated modules listed in `WHAT_IS_WORKING_AND_NOT_WORKING.md` §3.A | None — unblocks everything else |
| 🔴 2 | Complete Zenith compositor | 1.1 | `src/compositor/`, `src/desktop/zenith*.rs` | #1 |
| 🔴 3 | SigmaConfig DSL | 2.1 | `src/config/` | #1 |
| 🔴 4 | GPU driver support | 3.1 | `src/driver/gpu_*.rs` | #1 |
| 🟡 5 | PipeWire audio | 3.1 | `src/audio/` | #1 |
| 🟡 6 | Power management | 3.2 | `src/power/`, `src/thermal/` | #1 |
| 🟡 7 | Application menu & launcher | 1.2 | `src/launcher/`, `src/desktop/menu.rs` | #2 |
| 🟡 8 | Package format bridges | 2.2 | `src/sigpkg/`, `src/compatibility/` | #3 |
| 🟢 9 | Sigma IDE | 4.1 | new crate under `src/dev/` | #2, #7 |
| 🟢 10 | OCI container runtime | 4.1 | `src/container/` | #1 |
| 🟢 11 | Self-hosting compiler | 4.2 | `src/toolchain/`, `src/compiler/` | #1, #10 |
| 🟢 12 | Post-quantum crypto stack | 5.2 | `src/crypto/` | #1 |
| 🟢 13 | pledge/unveil + Landlock hardening | 5.1 | `src/pledge.rs`, `src/security/` | #1, #8 |
| ⚪ 14 | Wacom tablet support | 3.1 | `src/driver/hid_input_device.rs` | #4 |
| ⚪ 15 | Theme marketplace | 6.1 | `src/theming/` | #2 |
| ⚪ 16 | Community hardware database | 6.1 | `src/community/` | #4 |
| ⚪ 17 | Advanced debugging tools | 4.1 | `src/debugger/`, `src/tracing/` | #9 |

---

## 🧩 Omarchy Features to Directly Adopt

| Omarchy Component | SigmaOS Integration | Implementation Status | Where It Lives Today |
| :--- | :--- | :---: | :--- |
| Background/Wallpaper Engine | Zenith compositor | ✅ Present (listed in `WHAT_IS_WORKING_AND_NOT_WORKING.md` §2.D) | `src/desktop/zenith_advanced_features.rs`, `src/theming/theme_engine.rs` (wallpaper colour extraction) |
| Gaming Registry | Steam/Lutris support | ✅ Present (listed in `WHAT_IS_WORKING_AND_NOT_WORKING.md` §2.D) | `src/desktop/gaming_engine.rs`, `src/graphics/gaming_layer.rs` |
| Modular Theming | Zenith theme system | 🔄 In Progress | `src/desktop/omarchy_theme.rs`, `src/theming/theme_engine.rs` |
| Multi-OS Installer | Dual-boot with Limine | ✅ Partially done | `src/installer/gui_wizard.rs`, `src/boot/sovereign_multi_distro_bootloader.rs` |
| Keyboard-Driven Workflow | Super key bindings | 🔄 Partial (expand it) | `src/desktop/omarchy_dynamic_workspace_suite.rs`, `src/desktop/omarchy_omakase.rs` |
| Neovim Integration | LazyVim presets | ✅ Already included | `src/desktop/omarchy_omakase.rs` |
| Multi-Display Support | HiDPI scaling | 🔄 Needs work | `src/graphics/multi_monitor.rs`, `src/compositor/zenith_core.rs` |

---

## 📅 Estimated Timeline

| Window | Milestones |
| :--- | :--- |
| **Q1 2026** | Workspace fixes + Zenith v1 + SigmaConfig alpha |
| **Q2 2026** | GPU drivers + Audio + Menu system |
| **Q3 2026** | Power management + Package bridges |
| **Q4 2026** | IDE + Containers + PQC crypto |
| **Q1 2027** | Full self-hosting compiler |
| **2027+** | Community features + ecosystem growth |

**Re-baselining action:** Because the Q1–Q3 2026 windows had already elapsed when this document was committed (2026-09-27), maintainers should confirm the current status of items 1–8 in the priority table, then shift the remaining windows forward in a follow-up PR while keeping the sequence above intact.

---

## ✅ Definition of Done & Verification

Every work item derived from this roadmap must satisfy the repository's existing gates before it is checked off here:

1. **Unit tests** — `./run_sigma_tests.sh` passes at 100%; new modules include `#[cfg(test)]` suites (see `CONTRIBUTING.md` §2).
2. **`#![no_std]` compliance** — `./scripts/no_std_check.sh` passes; kernel/driver code uses `core`/`alloc` only (Algorithm C in `WHAT_IS_WORKING_AND_NOT_WORKING.md`).
3. **Workspace build** — once priority item #1 lands, `cargo check --lib` must stay green for every subsequent PR.
4. **Boot smoke test** — items touching `src/boot`, `src/kernel`, or the compositor run `scripts/qemu_smoke_test.sh`.
5. **Accessibility** — UI items run `scripts/uiux_accessibility_test.sh` and document keyboard reachability.
6. **Documentation** — update this file's checkboxes and, where behaviour changes, `WHAT_IS_WORKING_AND_NOT_WORKING.md`. Edit `docs/` only; `wiki/` mirrors are generated (`.github/workflows/documentation-checks.yml`).

---

## 🗂️ Tracking & Governance

- **Issues:** Open one GitHub issue per unchecked deliverable using the `feature_request.md` or `driver_submission.md` templates in `.github/ISSUE_TEMPLATE/`. Title format: `[Roadmap P<phase>.<section>] <deliverable>` (e.g. `[Roadmap P2.1] SigmaConfig TOML parser`).
- **Labels:** apply the phase's area label (`desktop`, `package-manager`, `drivers`, `security`, `documentation`, …) plus `enhancement`.
- **Pull requests:** reference the issue and this document; tick the corresponding checkbox in the same PR so the roadmap stays the single source of truth.
- **Review cadence:** maintainers revisit the priority table in the quarterly architecture review defined in [OPERATIONS_AND_CONTINUOUS_IMPROVEMENT_GUIDE.md](OPERATIONS_AND_CONTINUOUS_IMPROVEMENT_GUIDE.md), re-baseline dates, and move completed items into `WHAT_IS_WORKING_AND_NOT_WORKING.md` §2.
- **Owning specialist agents (defined in [FUTURE-DEVELOPMENT-ROADMAP.md](../FUTURE-DEVELOPMENT-ROADMAP.md) §142.5):** Palette 🎨 owns Phase 1 and 6.1, Bolt ⚡ owns Phases 3–4, Sentinel 🛡️ owns Phase 5; Phase 2 is shared between Bolt and Sentinel.

---

*End of Omarchy-Inspired Development Roadmap.*
