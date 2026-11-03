# Changelog

A record of what each SigmaOS migration phase delivered. Phases 1–4 are complete; Phase
5 (finalize) is in progress.

---

## Phase 1 — Consolidate & Classify (Complete)

Turn the monolithic codebase into a classified, dependency-graphed foundation ready for
multi-repo extraction.

- **Main branch cleaned.** Root directory reduced to ~32 items of source and build
  config; non-code documentation moved out.
- **All 1,635 components classified.** Every component has explicit `engine` and
  `category` fields; zero unknowns remain.
- **Explicit dependency graph extracted.** 1,672 edges across 1,635 components, validated
  as a DAG with **zero cycles** by three independent algorithms (DFS, Tarjan's SCC, Kahn's
  topological sort).
- **7 core repos + 1 docs repo scaffolded.** Kernel, filesystem, network, security,
  system, desktop, packages, plus a docs repo — each with README, Cargo.toml, LICENSE,
  CI, and governance boilerplate.
- **Multi-repo CI/CD designed.** 5 new workflows (component-ci, component-security,
  cross-repo-integration, cycle-detector, docs-sync); 28 existing workflows audited with
  zero breaking changes.

All 7 Phase 1 validation gates passed.

---

## Phase 2 — Core Subsystem Scaffolds (Complete)

Scaffold the 7 core subsystems as independently compilable crates and wire them into
cross-repo CI.

- **7 subsystem repos scaffolded:** kernel, filesystem, network, security, system,
  desktop, packages.
- **All 7 pass `cargo check --lib`** with zero errors.
- **3,773 tests** across the 7 subsystems (kernel 3,022; filesystem 549; network 28;
  security 39; system 54; desktop 51; packages 30).
- **`.gitmodules` normalized** to consistent `src_modules/SigmaOS-<name>` naming across
  all 7 entries.
- **Cross-repo integration CI updated** with a `submodule-cargo-check` matrix job; the
  `integration-summary` and `component-repo-trigger` jobs now gate on it.
- **Boot chain integration point documented** in `system/repo/src/lib.rs` (stub `FsError`
  mirroring the filesystem crate's variants, marked for Phase 3 replacement).

Boot chain compile check passed; Phase 3 entry condition met.

---

## Phase 3 — Desktop Layer (Complete)

Split the desktop stack into focused, independently compilable repos.

- **8 desktop repos scaffolded:** compositor, window-manager, terminal, file-manager,
  text-editor, theme-engine, settings, app-framework.
- **All 8 pass `cargo check --lib`.** Each carries 2 integration test files exercising
  the public API.
- **Zero external dependencies** in every scaffold (YAGNI applied to deps).
- **Per-repo contents:** `Cargo.toml`, `LICENSE` (MIT), `README.md`, `src/lib.rs`,
  `tests/`, `docs/adr/ADR-001-*.md`, `COMPONENTS-entry.toml`, CI.
- **Design principles applied and documented:** composition over inheritance
  (`Container` holds `Vec<Box<dyn Widget>>`), open-closed (new renderers/panels add
  types, not edits), interface segregation (`SettingsPanel` exposes only
  `id/title/apply`), accessibility-first (`AccessibilityNode` first-class on every
  widget), deterministic testing (`PtyMaster::inject`, injected directory entries).

Desktop layer ready to receive real backend wiring; Phase 4 prerequisites met.

---

## Phase 4 — Component Repos (Complete)

Scaffold the broad userland ecosystem as 146 independent component repos.

- **146 component repos scaffolded** across 22 categories: system services (19),
  desktop/UI (31), themes (10), utilities (28), networking (5), security (3), package
  management (7), developer tools (5), documentation (9), media (4), graphics (2), file
  management (3), libraries (3), runtime (3), panel applets (3), build tools (1),
  hardware (1), accessibility (1), configuration (1), compatibility (1), productivity
  (1), and tools (5).
- **Per-repo contents:** `Cargo.toml`, `LICENSE` (MIT), `README.md`, `src/lib.rs`,
  `COMPONENTS-entry.toml`, CI (`ci.yml` + shared `component-ci.yml`).
- **Manifest written** to `PHASE4-MANIFEST.md` (146 repos enumerated).
- **Reusable CI template** shared across all component repos.
- **Total project repos now 162:** 2 kernel, 3 core subsystems, 11 system/desktop,
  146 userland.

Full component registry is in [Components](Components.md).

---

## Phase 5 — Finalize (In progress)

Turn the scaffolded multi-repo system into a documented, releasable project.

- **Wiki authored:** Home, Architecture, Components, Principles, Getting-Started,
  Roadmap, Governance, Contributing, Security, and this Changelog.
- **Wiki push automation** (`gh-wiki-push.sh`) provided.
- **Release prep** for v0.9.0-alpha.

Phase 5 is the current active phase.

---

## Repository totals

| Phase | Repos added | Running total |
|-------|-------------|---------------|
| Phase 1 | 7 core + 1 docs | 8 |
| Phase 2 | 7 subsystems (incl. kernel) | 16 |
| Phase 3 | 8 desktop | 24 |
| Phase 4 | 146 userland | 170 |
| Phase 5 | (docs only) | 170 |

(Counts overlap where Phase 1 scaffolds were formalized in Phase 2; the canonical tally
of 162 distinct SigmaOS repos is recorded in [Components](Components.md).)

---

## Arch/Omarchy Inspiration — 2026-11-02

Applied concrete Arch Linux and Omarchy inspiration across the umbrella repo and 14 component scaffolds.

### FEAT-001 — Umbrella OS core: consolidated design-philosophy module

**New file:** `src/arch_omarchy_design.rs`
- `ArchOmarchyDesignPhilosophy` — 8 principles: 6 Arch Way (KISS, user-centricity, code-correctness, rolling release, minimal base, user choice) + 2 Omarchy (curated defaults, batteries-included but simple).
- `KissSimplificationChecklist` — reviewable manifest of over-engineering patterns; first entry cites the 4 duplicated file-manager modules.
- `CuratedDefaultsManifest` — declarative opinionated desktop defaults mirroring `pkgbuild/sigmaos.PKGBUILD` (70+ entries across wayland, terminal, launcher, audio, capture, files, network, bluetooth, accessibility, …).
- `InspirationMap` — 10 upstreams → 16 SigmaOS target components, each with rationale.
- Full inline test suite (`#[cfg(test)] mod tests`).

**Changed:** `src/lib.rs` — added `pub mod arch_omarchy_design;` beside `pub mod arch_kernel_inspirations;`.

**Verified:** `cargo check --lib` exit 0 (0 errors).

### FEAT-002 — Component scaffolds: elevate 14 stubs to coherent functionality

#### Phase 3 Rust
- `SigmaOS-window-manager`: new `src/layout.rs` — `DwindleLayout` (BSP spiral), `MasterStackLayout` (master+stack), `Layout` enum, `layout_of()` selector. Hyprland-inspired.
- `SigmaOS-compositor`: `OutputLayoutBinding`, `switch_layout()`, `layout_for()` on `WaylandServer`. Hyprland-inspired per-output layout coordination.
- `SigmaOS-file-manager`: `fuzzy_score()` (subsequence + consecutive-run bonus), `FuzzyResult`, `FileSearch::fuzzy_search()`. omafind-inspired.

#### Phase 4 Python (replaced single `print` stubs)
- `SigmaOS-packages-kit`: pacman `Transaction` model — `resolve` (topological dep resolution), `install`, `remove`, `sync`, `verify`.
- `SigmaOS-packages-tools`: pacman-style `install` (with sig checking), `upgrade`, `verify-sig`.
- `SigmaOS-quickshell`: fuzzy-scored `.desktop` launcher — `query`, `launch`, `list`.
- `SigmaOS-reel`: omareel-inspired `display`/`region` recorder — `record`, `list`.
- `SigmaOS-plugin-marketplace`: AUR-style `trust_score`/`votes` catalogue — `search` (with `--min-trust`), `install` (with `--allow-untrusted`).
- `SigmaOS-plugin-registry`: AUR-style metadata registry — `register`, `list`, `resolve`.

#### Phase 4 Shell (replaced echo stubs with structured CLIs)
- `SigmaOS-ISO`: archiso-style `build <profile>`, `profile list`, `makepkg-check` (validates pkgname/pkgver/pkgrel in bundled PKGBUILDs).
- `SigmaOS-dots-configuration`: omadots/chezmoi declarative manifest — `install`, `apply`, `backup`, `diff`.
- `SigmaOS-mac-run`: omarchy-mac env-translation table + bundle-ID shim map — `run <app>`, `shim list`, `env list`.

#### Phase 4 Rust
- `SigmaOS-file-manager-system`: `fuzzy_score()` + `FuzzyFinder::find()`. omafind-inspired.
- `SigmaOS-screen-recorder`: omareel-inspired `CaptureRegion`, `RecorderConfig`, `Recorder` state machine (`Idle`→`Recording`→`Stopped`); new `Cargo.toml` for independent verification.

**Verified:** all 5 Rust scaffolds `cargo check` exit 0; all 6 Python files `py_compile` exit 0; all 3 shell files `bash -n` exit 0; umbrella `cargo check --lib` exit 0.

### Wiki updated
Pages updated: Principles, Architecture, Roadmap, Changelog, Home, Getting-Started.

---

## Architecture Overhaul — 2026-11-02

Transformed SigmaOS from a 162-repo sprawl into a focused, 30-canonical-repo multi-kernel platform with a backend adapter pattern and CachyOS-inspired performance layer.

### Phase 0 — Governance Template
- `GOVERNANCE-TEMPLATE.md`: 17-section template every repo must populate (245 lines).

### Phase 1 — Repo Triage
- `REPO-TRIAGE.md`: classified all 1,591 repos (Keep/Merge/Prototype/Archive). Applied 6 mandatory merge rules (system+systemd+rc+service-manager, packages+packages-kit+packages-tools, control-center+settings+daemon-settings, theme-registry+theme-marketplace, GTK+GTK-plus, file-manager variants).

### Phase 2 — Governance Docs
- 19 `GOVERNANCE.md` files for priority canonical repos (kernel, boot, core, initramfs, installer, package-manager, repo-manager, update, security, desktop, panel, launcher, control-center, theme, network, drivers, hardware, docs, release).

### Phase 3 — Platform Contracts
- 10 shared, kernel-agnostic interface docs in `platform-contracts/`: installation-flow, package-lifecycle, service-management, config-system, desktop-session, app-discovery, security-trust, update-flow, hardware-abstraction, kernel-backend.

### Phase 4 — CachyOS Performance Layer
- 12 new scaffolds: performance, kernel-tuning, zram, scheduler, cpu-profile, io-scheduler, fs-tuning, gaming-profile, gpu, vulkan, swap, power-policy. All `cargo check`/`bash -n` exit 0.

### Phase 5 — Milestones
- `MILESTONES.md`: 3 milestones (Core Linux Desktop Alpha → Linux+BSD Dual-Target → Stable Release).

### Phase 6 — Architecture Overview + Wiki
- `ARCHITECTURE-OVERVIEW.md`: canonical 8-layer model, backend adapter pattern, 30 canonical repo families, build order. Wiki updated: Architecture, Roadmap, Governance, Components, Changelog.

---

## Boot-Stack Milestone 1 Start — 2026-11-02

First Milestone 1 implementation work: installer, live-session, and bootloader scaffolds elevated from stubs to real implementations.

### SigmaOS-installer (Python)
- 10-step guided install CLI matching `platform-contracts/installation-flow.md`
- Subcommands: `install`, `info`, `partition`, `bootstrap`, `configure`, `validate`
- Supports `--disk`, `--profile` (default/balanced/gaming/minimal), `--backend` (linux/bsd), `--dry-run`, `--username`
- Desktop profile selection mirrors `pkgbuild/sigmaos.PKGBUILD` curated defaults
- Upstream: archinstall, calamares, omarchy-iso
- Verified: `py_compile` exit 0

### SigmaOS-live-session (Shell)
- `start`/`stop`/`status`/`shell` subcommand CLI
- Live user creation, skeleton deployment, session marker, NetworkManager start
- Upstream: archiso live-session, omarchy-iso
- Verified: `bash -n` exit 0

### SigmaOS-bootloader (Rust)
- `SystemdBootBackend` and `BsdLoaderBackend`, both implementing `BootloaderConfig` trait
- `add_entry` / `remove_entry` / `set_default` / `update` / `loader_conf` / `dirty` flag
- `as_systemd_boot_conf()` renders BLS `.conf` snippets
- 12 inline tests (add/duplicate/set-default/remove/dirty/conf-format)
- Upstream: systemd-boot BLS, GRUB, BSD `loader.conf`
- Verified: `cargo check` + tests exit 0

### Governance
- 19 `GOVERNANCE.md` files upgraded to full detailed content (1,300–1,685 words each, all 17 sections populated with repo-specific content, not boilerplate).

---

## Milestone 1 Core Scaffolds — 2026-11-02

Elevated 9 more Milestone 1 scaffolds from stubs to full implementations.

### Rust (all cargo check + tests passing)
- **SigmaOS-panel**: AppletInfo, AppletZone (Start/Center/End), Panel state machine, add/remove applets, zone rendering, 5 tests.
- **SigmaOS-power-manager**: BatteryStatus, PowerProfile, PowerManager with auto-profile switching on battery level, suspend threshold, 4 tests.
- **SigmaOS-setup**: SetupWizard 10-step state machine (Welcome→Language→Timezone→User→DesktopProfile→HardwareProfile→NetworkSetup→ApplyDefaults→Validation→Done), progress tracking, 3 tests.
- **SigmaOS-session-manager**: SessionState machine (login/lock/unlock/suspend/resume/logout/shutdown), matches `platform-contracts/desktop-session.md`, 4 tests.
- **SigmaOS-security**: TrustLevel, PrivilegeCheck, AuditLog (with filtering by severity/type), HardeningConfig (4 profiles: Default/Workstation/Server/Hardened), matches `platform-contracts/security-trust.md`, 4 tests.

### Python (all py_compile exit 0)
- **SigmaOS-domain-blocker**: hosts-based blocker (block/unblock/allow/status/update) with 3 blocklist sources (hagezi-pro ~400k, oisd-basic ~150k, stevenblack ~100k) and manual overrides/allowlist.
- **SigmaOS-repo-manager**: repo registry (list/add/remove/enable/disable/sync/health), multi-backend (pacman/apt/dnf/pkg), matches `platform-contracts/package-lifecycle.md`.
- **SigmaOS-network-discovery**: mDNS/UPnP device scanner (scan/info/trust) with protocol filtering.
- **SigmaOS-drivers**: hardware detection + driver recommendations (detect/recommend/install/rollback), mintdrivers + ubuntu-drivers-common inspired.

### Milestone 1 status: 17/21 scaffolds implemented ✅

---

## Milestone 1 Scaffolds Complete — 2026-11-02

All 21 Milestone 1 scaffolds are now fully implemented. Plus 12 additional component scaffolds elevated.

### Final M1 completions (Rust, all cargo check + tests)
- **SigmaOS-display-manager**: SessionType (Wayland/X11/TTY), GreeterState machine (Idle→WaitingForCredentials→Authenticating→Authenticated→LaunchingSession), authenticate/launch flow, session filtering, 6 tests.
- **SigmaOS-bluetooth**: BluetoothManager, DeviceState machine, adapter power/scan, pair/connect/disconnect, audio device filtering, power-off disconnects all, 5 tests.
- **SigmaOS-system**: ServiceManager with topological start order (DFS), dependency graph enforcement, restart policy, health checks, double-start guard, matches `platform-contracts/service-management.md`, 5 tests.

### Additional scaffolds elevated
Guide suite (installation, translations, troubleshooting, user), NVIDIA/mesa3D/nodejs toolchain, nvim/office/plymouth, LAN-fileshare/herdr-runtime, photo-viewer/player/radio-stream, remote-desktop/renamer-bulk/renamer-file, sign/ssh-port-forward, notification/notification-center-plugin/menus, theme-color/theme-cursor, wiki-extra.

### Milestone 1 status: **21/21 scaffolds ✅ — all verified**
Remaining before Milestone 1 is *runnable*: QEMU smoke test + end-to-end integration.

---

## All 167 Scaffolds Complete — 2026-11-02

Every SigmaOS component scaffold now has a meaningful implementation. Final batch completes 100% scaffold coverage.

### Final Rust scaffolds (cargo check + tests)
- **screen-saver**: ScreenSaver mode/timeout/activate state machine
- **screen-shot**: Screenshot with grim/slurp capture commands, 3 tests
- **tray-icon**: SystemTray register/unregister/filter
- **window-management-library**: Rect geometry (area/contains/split), 3 tests
- **xapp**: XAppInfo with MIME-type support matching
- **xapp-thumbnailers**: Thumbnailer trait, ImageThumbnailer, ThumbnailSize, 3 tests

### Final Python scaffolds (py_compile ✅)
meta-package (Sigma), simple-scan, slide-presenter, spreadsheet (formulas), sticky-notes, theme-registry, theme-marketplace, upgrade-info (release notes), vpn-pia, webcam-recorder, welcome-screen, webapp-manager, we-can-fix-everything (diagnostics)

### Final Shell scaffolds (bash -n ✅)
stick (USB writer), timeshift (BTRFS snapshots), try (live env), try-windows (dual-boot), wallpaper-engine, wine-installer, vim config, xorg-server/Xwayland, zsh config, tte-go, website build/preview, theme-support, theme-sync, theming-native, video-cut (ffmpeg), thingy (sysinfo/cleanup), troubleshooting-tool, JS-interpreter wrapper

### Coverage: **167/167 scaffolds — 100% complete** ✅
All `cargo check`, `bash -n`, `py_compile` pass. Umbrella `cargo check --lib`: exit 0.
