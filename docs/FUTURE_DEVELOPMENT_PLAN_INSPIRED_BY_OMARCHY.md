# SigmaOS Future Development Plan Inspired by Omarchy Linux

SigmaOS should evolve from a broad OS research platform into a **small, bootable, opinionated, and demonstrably usable desktop distribution**. The main lesson from Omarchy is not to reproduce its exact stack, but to adopt its product discipline: one polished user journey, strong defaults, integrated configuration, excellent documentation, and frequent end-to-end validation.

SigmaOS already defines this direction in `README.md`, `docs/PRODUCT_VISION.md`, and `ROADMAP.md`, but its largest gaps are still between prototype specifications and a reliable daily-driver system.

## 1. Strategic Direction

### Product goal

Prioritize this single path:

```text
Download ISO
  → Boot UEFI/QEMU
  → Install or try live session
  → Create user
  → Enter Zenith desktop
  → Launch applications
  → Install signed package
  → Change system configuration/theme
  → Update atomically
  → Recover using rollback
```

Every feature should either improve this path or be explicitly classified as long-term research.

### What to borrow from Omarchy

Omarchy provides useful patterns that SigmaOS can adapt:

- A highly opinionated desktop instead of a collection of loosely connected subsystems.
- A complete user manual, not only developer-oriented architecture documents.
- A centralized command-line interface with consistent command naming.
- A single long-running desktop shell with internal panels, widgets, notifications, and services.
- A manifest-based plugin model.
- Default configuration files with migrations and safe refresh behavior.
- Automated CLI, shell, and graphical acceptance tests.
- Visual verification for desktop changes.
- A curated application experience, including web applications and sensible defaults.

### What not to copy directly

SigmaOS should not blindly inherit:

- Omarchy’s Arch/`pacman` assumptions.
- Its shell and Quickshell implementation.
- Unsandboxed plugin execution.
- A large third-party dependency stack if that conflicts with SigmaOS’s security and dependency policy.
- Broad multi-distro compatibility before the primary desktop is stable.

---

# 2. Current State and Principal Gaps

SigmaOS has substantial architectural prototypes, including:

- Zenith compositor and desktop concepts.
- `sigpkg`, Merkle storage, dependency solving, signing, and delta update components.
- Declarative profile and theme concepts.
- Cross-platform security abstractions.
- A documented M0–M5 roadmap.
- A hardware support matrix centered on x86_64, QEMU, VirtIO, NVMe, USB HID, and future Intel/AMD hardware.

However, the repository still has several execution gaps.

## Gap A: The end-to-end bootable product is not yet proven

`docs/RELEASE_CRITERIA.md` identifies M1 as the active focus, while `ROADMAP.md` still lists automated QEMU boot-to-desktop validation as incomplete.

Needed:

- Reproducible ISO generation.
- UEFI boot validation.
- QEMU serial heartbeat.
- Automatic detection that Zenith reached its first frame.
- Automated keyboard and launcher smoke tests.
- Failure artifact collection: serial logs, screenshots, and disk images.

## Gap B: The installer and first-boot experience are incomplete

The intended product requires a user to install SigmaOS with fewer than three prompts, but the repository does not yet demonstrate a complete install-to-login path.

Needed:

- Live ISO mode.
- Guided installer.
- Disk selection and safe partitioning.
- User creation.
- Locale, timezone, keyboard, and network configuration.
- Bootloader installation.
- Recovery if installation is interrupted.
- First-login setup wizard.

## Gap C: The desktop shell needs productization

SigmaOS has Zenith compositor and Omarchy-inspired HUD references, but it needs the cohesive shell experience that makes Omarchy approachable.

Needed:

- Stable top bar.
- Application launcher.
- Workspace switcher.
- Notifications.
- Audio, network, Bluetooth, battery, and power panels.
- Lock screen and idle handling.
- Screenshot and screen-recording workflow.
- Clipboard history.
- Theme and wallpaper picker.
- Consistent keyboard shortcut system.
- Accessibility behavior and focus management.

## Gap D: Package functionality is ahead of package usability

The package documentation describes adapters for many package formats, Boolean dependency resolution, signed packages, delta updates, and rollback. The immediate product gap is a small, trustworthy native package workflow.

Needed before supporting dozens of formats:

- A stable `.sigpkg` manifest schema.
- A first-party repository format.
- Repository metadata signing and key rotation.
- Signature and trust errors understandable to users.
- Install, remove, upgrade, search, info, and rollback commands.
- Transaction previews and dry runs.
- Offline installation.
- Reproducible package builds.
- Sandboxed lifecycle hooks.
- Repository mirroring and recovery behavior.

The 60+ format goal should remain a compatibility track, not the primary release blocker.

## Gap E: Atomic updates are not yet an operational feature

SigmaOS documents A/B updates and generation rollback, but release readiness requires proving them under failure.

Needed:

- Bootable generation metadata.
- Health checks after reboot.
- Automatic rollback after failed boot or failed health check.
- Manual recovery menu.
- Interrupted-update recovery.
- Snapshot retention policy.
- Disk-space management.
- Signed update metadata.
- Test scenarios for power loss, corrupt downloads, and incomplete writes.

## Gap F: Architecture and policy documents conflict

`AGENTS.md` describes a strict `#![no_std]` design, while `ARCHITECTURE.md` records a decision to use a practical `std` architecture and notes thousands of `std` imports.

This is a major project-governance gap.

Recommended decision:

- Use `std` for the desktop runtime, installer, package manager, compositor, and tooling.
- Keep `no_std` only for explicitly freestanding kernel or boot components.
- Define this boundary in one canonical architecture document.
- Update `AGENTS.md`, `docs/AGENTS.md`, `README.md`, and contributor guidance so agents do not receive contradictory instructions.
- Add CI checks for the intended `std`/`no_std` boundary.

## Gap G: Documentation is developer-heavy and user-light

SigmaOS has many architecture and strategy documents, but the Omarchy model demonstrates the value of a structured end-user manual.

Needed:

```text
manual/
  01-welcome.md
  02-installation.md
  03-first-login.md
  04-navigation.md
  05-keyboard-shortcuts.md
  06-applications.md
  07-package-management.md
  08-updates-and-rollback.md
  09-themes.md
  10-networking.md
  11-displays-and-input.md
  12-security-and-permissions.md
  13-troubleshooting.md
  14-recovery.md
  15-developer-guide.md
```

The manual should be tested against clean installations, not merely written from implementation assumptions.

---

# 3. Proposed Development Phases

## Phase 0 — Product and Architecture Consolidation

**Priority:** P0
**Target:** First development cycle

### Objectives

Create one authoritative product contract and remove roadmap/documentation drift.

### Work items

- Reconcile `AGENTS.md` with `ARCHITECTURE.md`.
- Mark every major subsystem as one of:
  - Implemented and tested.
  - Prototype.
  - Specification only.
  - Deferred research.
- Remove duplicate or stale roadmap copies where practical.
- Define the supported release target as:
  - x86_64.
  - UEFI.
  - QEMU/KVM.
  - VirtIO GPU, network, and block.
  - USB HID.
- Freeze the initial release scope.
- Create a dependency and trust-boundary document.
- Define the desktop/runtime versus freestanding/kernel boundary.

### Exit criteria

- One canonical roadmap.
- One canonical architecture decision.
- No feature is described as complete without an executable test or demonstrable artifact.
- `cargo check --lib` and the documented verification commands pass consistently.

---

## Phase 1 — Reproducible Build and ISO Pipeline

**Priority:** P0
**Target:** Near term

### Objectives

Produce a bootable image from a clean checkout using a repeatable command.

### Work items

- Stabilize `sovereign_edition_builder`, `sigma_make`, and installer tooling.
- Define a deterministic build directory layout.
- Generate:
  - Kernel or runtime image.
  - EFI system partition.
  - Root filesystem.
  - Bootloader entries.
  - Checksums and signing metadata.
- Add a single command such as:

```bash
cargo run --bin sovereign_edition_builder -- --profile qemu
```

or a documented `make iso` target.

- Add reproducibility metadata:
  - Source revision.
  - Build profile.
  - Toolchain version.
  - Image hash.
- Validate measured-boot and bootloader path handling described in `AGENTS.md`.

### Exit criteria

A clean machine can build the same ISO twice and receive identical or explainably reproducible artifacts.

---

## Phase 2 — QEMU Desktop Preview

**Priority:** P0
**Target:** First major milestone

### Objectives

Turn the existing M1 prototype into a reliable graphical demo.

### Work items

- Boot directly into a Zenith session.
- Implement a minimal session manager.
- Confirm VirtIO-GPU scanout.
- Confirm keyboard and mouse input.
- Provide:
  - Top bar.
  - Launcher.
  - Terminal.
  - Workspace switching.
  - Window close and focus controls.
  - Power menu.
- Add QEMU test automation:

```bash
./scripts/test_qemu_desktop.sh
```

The test should:

1. Start QEMU.
2. Wait for a kernel heartbeat.
3. Wait for login/session readiness.
4. Detect Zenith’s first frame.
5. Send keyboard input.
6. Open the launcher.
7. Launch a test application.
8. Save serial output and screenshots.
9. Shut down cleanly.

### Exit criteria

An independent contributor can boot the preview in QEMU and interact with the desktop without manually repairing the environment.

---

## Phase 3 — Zenith Shell and Desktop Integration

**Priority:** P0
**Target:** After QEMU preview

### Objectives

Build an integrated shell rather than isolated desktop components.

### Recommended architecture

Create a long-lived `zenith-shell` process responsible for:

```text
zenith-shell
  ├── bar
  ├── launcher
  ├── notifications
  ├── workspace panel
  ├── audio panel
  ├── network panel
  ├── power panel
  ├── theme panel
  ├── clipboard service
  └── plugin registry
```

Unlike Omarchy’s unsandboxed QML plugins, SigmaOS plugins should use a capability-aware model.

### Plugin manifest concept

Each plugin should declare:

- ID and version.
- Entry point.
- UI kind: widget, panel, overlay, service, or launcher provider.
- Required capabilities.
- Configuration schema.
- Whether it may run at startup.
- Whether it may access files, network, devices, or process controls.

Example capability categories:

```text
ui.bar
ui.overlay
filesystem.read-user-config
network.connect
audio.control
power.manage
process.launch
```

### Exit criteria

- Desktop panels are loaded through one stable shell interface.
- A broken optional plugin cannot prevent the base shell from starting.
- Plugin permissions are visible and enforced.
- Shell startup and launcher latency meet the performance targets in `docs/RELEASE_CRITERIA.md`.

---

## Phase 4 — Installer, First Boot, and Hardware Detection

**Priority:** P0
**Target:** M4 preparation

### Objectives

Make SigmaOS installable on supported systems.

### Work items

- Implement the setup wizard.
- Add hardware detection and compatibility reporting.
- Support:
  - QEMU VirtIO.
  - NVMe.
  - USB HID.
  - Intel and AMD x86_64 systems.
  - Basic wired networking.
- Add a guided installer with safe defaults.
- Implement installation logs and recovery.
- Create a hardware report command:

```bash
sigma-hardware report
```

- Document unsupported hardware clearly instead of presenting partial support as complete.

### Exit criteria

A user can install SigmaOS on a certified test machine, reboot, log in, and reach Zenith without manual repair.

---

## Phase 5 — Native `sigpkg` MVP

**Priority:** P0
**Target:** Before public desktop beta

### Objectives

Deliver a small, secure package ecosystem before universal format absorption.

### Initial command surface

```bash
sigma-pkg search <query>
sigma-pkg info <package>
sigma-pkg install <package>
sigma-pkg remove <package>
sigma-pkg update
sigma-pkg list
sigma-pkg verify <package>
sigma-pkg rollback
```

### Work items

- Freeze the native manifest format.
- Define package repository metadata.
- Implement signed repository indexes.
- Add trust-store initialization and key rotation.
- Add human-readable transaction plans.
- Add dependency conflict explanations.
- Add package sandboxing based on SigmaOS capabilities.
- Make package lifecycle hooks opt-in and restricted.
- Add offline and interrupted-transaction tests.
- Publish a small first-party repository containing:
  - Terminal.
  - Editor.
  - File viewer.
  - Network tools.
  - Desktop utilities.
  - Browser/WebApp integration.

### Defer

Do not make every DEB, RPM, Pacman, Ports, Flatpak, Snap, AppImage, and BSD format a release requirement. First make `.sigpkg` reliable.

### Exit criteria

A clean installation can install, update, verify, remove, and roll back a package without corrupting the base system.

---

## Phase 6 — Atomic Updates and Recovery

**Priority:** P0
**Target:** Public beta

### Objectives

Make rollback a user-facing guarantee rather than only an internal engine.

### Work items

- Implement A/B root or generation switching.
- Add update preflight checks:
  - Signature.
  - Disk space.
  - Compatibility.
  - Dependency graph.
- Add post-boot health validation.
- Automatically mark a generation healthy only after:
  - Kernel startup.
  - Filesystem mount.
  - Session manager startup.
  - Zenith readiness.
- Add recovery boot menu.
- Add:

```bash
sigma-system generations
sigma-system rollback
sigma-system pin <generation>
sigma-system cleanup
```

- Test:
  - Interrupted downloads.
  - Interrupted writes.
  - Power loss during update.
  - Invalid signatures.
  - Failed desktop startup.
  - Full rollback after reboot.

### Exit criteria

A failed update cannot leave the machine unbootable, and a user can recover without a separate rescue environment.

---

## Phase 7 — Declarative Configuration and Theme System

**Priority:** P1
**Target:** Public beta

### Objectives

Provide the customization and consistency that make an opinionated distribution feel intentional.

### Work items

- Stabilize `/system/profile.toml` and `/user/preferences.toml`.
- Define schema versioning.
- Add atomic reconciliation.
- Add configuration migrations.
- Support:
  - Keyboard shortcuts.
  - Theme.
  - Wallpaper.
  - Bar layout.
  - Default applications.
  - Power behavior.
  - Display settings.
- Add a theme package format.
- Provide built-in themes such as Tokyo Night, Catppuccin, Gruvbox, and Nord.
- Add safe fallback to the last valid profile.

### Exit criteria

A malformed user configuration cannot prevent login, and theme changes apply without requiring a reboot.

---

## Phase 8 — Application Experience and Web Apps

**Priority:** P1
**Target:** Public beta

### Objectives

Deliver a curated application environment inspired by Omarchy’s application integration.

### Work items

- Define a `.desktop`-compatible application registry or SigmaOS-native equivalent.
- Add application metadata:
  - Name.
  - Icon.
  - Categories.
  - Required capabilities.
  - Default launch behavior.
- Provide curated defaults:
  - Terminal.
  - Text editor.
  - File manager/viewer.
  - Browser.
  - Media viewer.
  - Screenshot utility.
  - System settings.
- Add secure WebApp definitions.
- Route untrusted binaries through `Zorin Exec Guard`.
- Provide clear alternatives when an application is blocked.

### Exit criteria

A new user can find and launch useful applications without needing to understand package internals or kernel architecture.

---

## Phase 9 — Security UX and Permission Management

**Priority:** P1
**Target:** Beta hardening

### Objectives

Make SigmaOS security understandable instead of exposing only low-level policy engines.

### Work items

- Build a graphical permission viewer.
- Explain why an application was blocked.
- Support temporary and persistent grants.
- Display file, device, network, and process permissions.
- Integrate:
  - Landlock on Linux.
  - Capsicum on FreeBSD.
  - Pledge/unveil on OpenBSD.
- Ensure the desktop release has a clearly defined Linux security baseline first.
- Add audit logs that users can export.
- Add package and livepatch verification status indicators.

### Exit criteria

A non-expert user can understand, approve, revoke, and troubleshoot application permissions.

---

## Phase 10 — Hardware Qualification and Release Engineering

**Priority:** P0/P1
**Target:** M4–M5

### Work items

- Establish a certified hardware matrix.
- Test Intel and AMD integrated graphics.
- Validate Intel Wi-Fi and Realtek networking.
- Validate Intel HDA and USB audio.
- Test sleep, wake, display hotplug, multiple monitors, USB HID, and suspend recovery.
- Produce release artifacts:
  - ISO.
  - Checksums.
  - Signature.
  - Hardware compatibility report.
  - Known issues.
  - Upgrade notes.
- Use staged release channels:
  - `edge`
  - `preview`
  - `stable`

### Exit criteria

A release has a reproducible build, documented hardware coverage, tested rollback, and a clear support policy.

---

# 4. Testing and Verification Plan

SigmaOS should adopt Omarchy’s layered testing approach while retaining Rust-native verification.

## Required layers

### Unit tests

For:

- Package manifests.
- Dependency solver.
- Signature verification.
- Profile parser.
- Permission evaluator.
- Theme parser.
- Snapshot state transitions.
- Ring buffers and kernel primitives.

### Integration tests

For:

- Package installation.
- Repository metadata.
- Update transactions.
- Profile reconciliation.
- Permission enforcement.
- Installer workflows.

### QEMU smoke tests

For:

- Boot.
- Login.
- Desktop readiness.
- Keyboard input.
- Launcher.
- Terminal.
- Package installation.
- Update.
- Rollback.

### Graphical acceptance tests

For:

- Launcher visibility.
- Workspace changes.
- Theme switching.
- Notifications.
- Lock screen.
- Permission prompts.
- Recovery menu.

### Hardware tests

For:

- Display.
- Audio.
- Network.
- USB.
- NVMe.
- Suspend/resume.
- Multiple monitors.

The repository currently documents `./run_sigma_tests.sh` and `pytest tests/`; these should be consolidated into a CI matrix rather than relying only on a manually maintained shell script.

Recommended verification command:

```bash
./scripts/verify.sh
```

It should run:

```bash
cargo fmt --check
cargo check --lib
cargo check --all-targets
cargo test --all
./run_sigma_tests.sh
pytest tests/
./scripts/test_qemu_desktop.sh
```

---

# 5. Proposed Repository Improvements

SigmaOS would benefit from a clearer product-oriented structure:

```text
docs/
  architecture/
  development/
  security/
  testing/
  hardware/
manual/
  installation/
  desktop/
  packages/
  recovery/
  troubleshooting/
ci/
  qemu/
  hardware/
  release/
scripts/
  build-iso.sh
  run-qemu.sh
  test-qemu-desktop.sh
  verify.sh
src/
  compositor/
  desktop_shell/
  installer/
  package/
  security/
  update/
  profile/
tests/
  unit/
  integration/
  qemu/
  acceptance/
```

Also add:

- A canonical `docs/ROADMAP.md`.
- A release changelog generated from milestones.
- A user-facing support policy.
- A package/repository specification.
- A threat model.
- A hardware certification template.
- A migration policy for profile and package schemas.

---

# 6. Prioritized Backlog

## P0 — Must complete before a serious public preview

1. Resolve `std` versus `no_std` architectural contradictions.
2. Build a reproducible bootable ISO.
3. Automate QEMU boot-to-Zenith validation.
4. Finish minimal Zenith shell integration.
5. Implement installer and first-boot user creation.
6. Stabilize native `.sigpkg` installation and verification.
7. Implement tested atomic updates and rollback.
8. Add recovery boot behavior.
9. Establish a minimal supported hardware matrix.
10. Create an end-user installation and troubleshooting manual.

## P1 — Required for a usable beta

1. Audio, network, Bluetooth, power, and display panels.
2. Declarative profiles and configuration migrations.
3. Theme and wallpaper management.
4. Application registry and curated defaults.
5. Security permission UI.
6. WebApp support.
7. Hardware reporting.
8. Graphical acceptance tests.
9. Offline package installation.
10. Upgrade and rollback telemetry that preserves user privacy.

## P2 — Ecosystem and differentiation

1. Signed third-party plugins.
2. Package recipe SDK.
3. Community repository.
4. Sandboxed application extensions.
5. ARM64 and RISC-V desktop experiments.
6. BSD compatibility modes.
7. Post-quantum package and update signatures.
8. Livepatching.
9. Distributed or mesh features.
10. Full universal package-format absorption.

---

# 7. Release Sequence

## SigmaOS 0.1 — QEMU Preview

- Bootable ISO.
- Zenith desktop.
- Launcher and terminal.
- Basic keyboard navigation.
- Automated QEMU smoke test.
- No promise of broad hardware support.

## SigmaOS 0.2 — Installable Preview

- Guided installer.
- User creation.
- VirtIO and basic x86_64 support.
- Native `.sigpkg` installation.
- Basic profiles and themes.
- Recovery boot menu.

## SigmaOS 0.3 — Update-Safe Preview

- Signed repositories.
- Atomic updates.
- Automatic rollback.
- Generation management.
- Package transaction previews.
- Expanded acceptance testing.

## SigmaOS 0.5 — Hardware Beta

- Intel/AMD hardware validation.
- Integrated graphics.
- Audio and networking.
- Suspend/resume.
- Multi-monitor support.
- Public hardware compatibility reports.

## SigmaOS 1.0 — Stable Desktop Edition

- Reliable install, login, desktop, package, update, and recovery workflow.
- Documented supported hardware.
- Stable package repository.
- Security permission UX.
- Reproducible release artifacts.
- Complete user manual.
- Long-term maintenance policy.

---

# 8. Success Metrics

SigmaOS should not measure progress primarily by the number of implemented modules or supported package formats. Better metrics are:

- Percentage of clean QEMU boots reaching Zenith.
- Time from boot to usable desktop.
- Installer completion rate.
- Package install success rate.
- Update failure recovery rate.
- Rollback success rate.
- Number of certified hardware systems.
- Number of documented user workflows.
- Time to diagnose a failed package or desktop component.
- Number of regressions caught before release.
- Percentage of advertised features backed by executable tests.

## Recommended initial targets

| Area | Target |
|---|---:|
| QEMU boot-to-desktop success | 99% in CI |
| Installer completion in QEMU | 100% on reference image |
| Launcher latency | <50 ms |
| Package transaction rollback | <500 ms |
| Failed update recovery | 100% in fault-injection tests |
| Cold boot to login | <10 seconds in QEMU |
| Base idle memory | <250 MB where technically achievable |
| Documentation coverage | Every release-critical workflow documented |
| Reproducible ISO builds | 2 consecutive matching builds |

## Final recommendation

The next major SigmaOS milestone should not be another large cross-distro or kernel research expansion. It should be:

> **A reproducible QEMU image that installs, boots into Zenith, launches applications, installs one signed package, performs an atomic update, and successfully rolls back after an induced failure.**

That milestone directly addresses the gaps identified in the current M1–M5 roadmap and captures the strongest lesson from Omarchy: a focused, polished, opinionated desktop experience is more valuable than a large collection of unfinished subsystem promises.
