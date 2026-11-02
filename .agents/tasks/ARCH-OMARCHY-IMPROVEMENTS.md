# SigmaOS Arch/Omarchy Improvements

**Date:** 2026-11-02
**Branch:** `improve/arch-omarchy-inspiration` → fast-forwarded to `main`
**Commits pushed to `origin/main`:**
- `682b474758` — FEAT-001: consolidated design-philosophy module
- `196c9e8eb0` — FEAT-002: 14 component scaffolds elevated
- `a291a83bc3` — docs(wiki): 6 wiki pages updated
**Wiki:** pushed to `origin/master` (SigmaOS.wiki.git) @ `ee82e62d76`

---

## What was inspired by Arch Linux

| Applied change | Upstream concept |
|---|---|
| `src/arch_omarchy_design.rs` — `ArchOmarchyDesignPhilosophy` (6 Arch Way principles) | The Arch Way |
| `KissSimplificationChecklist` tracking over-engineering | KISS |
| `SigmaOS-packages-kit` — `Transaction` model (resolve/install/remove/sync/verify) | pacman transaction lifecycle |
| `SigmaOS-packages-tools` — `install`/`upgrade`/`verify-sig` | pacman/ABS tooling |
| `SigmaOS-plugin-marketplace` — `trust_score` + `votes` gating | AUR community trust model |
| `SigmaOS-plugin-registry` — `register`/`list`/`resolve` metadata service | AUR package metadata |
| `SigmaOS-ISO` — `build <profile>`, `profile list`, `makepkg-check` | archiso profiles + makepkg PKGBUILD validation |
| `CuratedDefaultsManifest` — declarative desktop defaults in code | Arch minimal-base + PKGBUILD `depends=()` |
| `InspirationMap` — 10 upstreams → 16 SigmaOS components | Arch Wiki culture of explicit attribution |
| Wiki Principles page — The Arch Way table with applied-in-SigmaOS column | Arch Wiki documentation standards |

## What was inspired by Omarchy

| Applied change | Upstream concept |
|---|---|
| `src/arch_omarchy_design.rs` — `ArchOmarchyDesignPhilosophy` (2 Omarchy principles) | Curated defaults + batteries-included but simple |
| `CuratedDefaultsManifest` — Hyprland/foot/wofi/waybar/mako/pipewire/nautilus defaults | omarchy curated desktop |
| `SigmaOS-compositor` — `OutputLayoutBinding` + `switch_layout()/layout_for()` | Hyprland per-monitor layout config |
| `SigmaOS-window-manager` — `DwindleLayout` (BSP spiral) + `MasterStackLayout` (master+stack) + `Layout` enum | Hyprland dwindle/master-stack layouts |
| `SigmaOS-file-manager` search.rs — `fuzzy_score()` + `FuzzyResult` + `FileSearch::fuzzy_search()` | omafind fuzzy file finder |
| `SigmaOS-file-manager-system` — `FuzzyFinder::find()` + `fuzzy_score()` | omafind |
| `SigmaOS-quickshell` — fuzzy-scored `.desktop` launcher | omarchy shell overlay + wofi |
| `SigmaOS-reel` — `display`/`region` record CLI | omareel screen recorder |
| `SigmaOS-screen-recorder` — `CaptureRegion`/`RecorderConfig`/`Recorder` state machine | omareel recording pipeline |
| `SigmaOS-dots-configuration` — declarative source→target manifest (`install`/`apply`/`backup`/`diff`) | omadots/chezmoi config-as-code |
| `SigmaOS-mac-run` — env-translation table + bundle-ID shim map | omarchy-mac macOS compat shims |

---

## Files changed

### Umbrella repo (`src/`)
- `src/arch_omarchy_design.rs` — NEW (754 lines: 4 structs + inline tests)
- `src/lib.rs` — +1 line (`pub mod arch_omarchy_design;`)

### Phase 3 desktop Rust scaffolds
- `.agents/tasks/phase3-desktop/SigmaOS-window-manager/src/layout.rs` — NEW (411 lines)
- `.agents/tasks/phase3-desktop/SigmaOS-window-manager/src/lib.rs` — +3 lines
- `.agents/tasks/phase3-desktop/SigmaOS-compositor/src/lib.rs` — +88 lines
- `.agents/tasks/phase3-desktop/SigmaOS-file-manager/src/search.rs` — rewritten (+128 lines net)

### Phase 4 Python scaffolds
- `SigmaOS-packages-kit/src/main.py` — rewritten (174 lines)
- `SigmaOS-packages-tools/src/main.py` — rewritten (132 lines)
- `SigmaOS-quickshell/src/main.py` — rewritten (142 lines)
- `SigmaOS-reel/src/main.py` — rewritten (133 lines)
- `SigmaOS-plugin-marketplace/src/main.py` — rewritten (172 lines)
- `SigmaOS-plugin-registry/src/main.py` — rewritten (108 lines)

### Phase 4 Shell scaffolds
- `SigmaOS-ISO/src/main.sh` — rewritten (143 lines)
- `SigmaOS-dots-configuration/src/main.sh` — rewritten (177 lines)
- `SigmaOS-mac-run/src/main.sh` — rewritten (144 lines)

### Phase 4 Rust scaffolds
- `SigmaOS-file-manager-system/src/lib.rs` — +118 lines
- `SigmaOS-screen-recorder/src/lib.rs` — rewritten (224 lines)
- `SigmaOS-screen-recorder/Cargo.toml` — NEW

### Wiki sources
- `.agents/tasks/phase5-finalize/wiki/Principles.md` — Arch & Omarchy section
- `.agents/tasks/phase5-finalize/wiki/Architecture.md` — Hyprland layout + curated defaults + omafind
- `.agents/tasks/phase5-finalize/wiki/Roadmap.md` — rolling-release + curated desktop + KISS cleanup
- `.agents/tasks/phase5-finalize/wiki/Changelog.md` — Phase 5 Arch/Omarchy entry
- `.agents/tasks/phase5-finalize/wiki/Home.md` — design philosophy callout
- `.agents/tasks/phase5-finalize/wiki/Getting-Started.md` — build personas + component-level examples

---

## Build verification results

| Target | Command | Result |
|---|---|---|
| Umbrella crate | `cargo check --lib` | ✅ exit 0 (0 errors) |
| SigmaOS-window-manager | `cargo check` | ✅ exit 0 |
| SigmaOS-compositor | `cargo check` | ✅ exit 0 |
| SigmaOS-file-manager | `cargo check` | ✅ exit 0 (2 warnings, no errors) |
| SigmaOS-file-manager-system | `cargo check` | ✅ exit 0 |
| SigmaOS-screen-recorder | `cargo check` | ✅ exit 0 |
| packages-kit | `python3 -m py_compile` | ✅ exit 0 |
| packages-tools | `python3 -m py_compile` | ✅ exit 0 |
| quickshell | `python3 -m py_compile` | ✅ exit 0 |
| reel | `python3 -m py_compile` | ✅ exit 0 |
| plugin-marketplace | `python3 -m py_compile` | ✅ exit 0 |
| plugin-registry | `python3 -m py_compile` | ✅ exit 0 |
| SigmaOS-ISO | `bash -n` | ✅ exit 0 |
| dots-configuration | `bash -n` | ✅ exit 0 |
| mac-run | `bash -n` | ✅ exit 0 |
| Wiki links | link-integrity Python check | ✅ 0 broken links |

**What could not be verified (environment constraints):**
- `cargo test --lib arch_omarchy_design` — blocked by pre-existing test-build break in `src/drivers/pcie_hotplug.rs` (unrelated, tracked separately).
- QEMU end-to-end boot — no runner with KVM available; remains Phase 6 work.
- Live Wayland compositor test — no display server in this environment.

---

## GitHub sync

- `improve/arch-omarchy-inspiration` pushed to `origin` ✅
- `main` fast-forwarded to `a291a83bc3` and pushed to `origin` ✅
- Wiki (10 pages) pushed to `wiki-origin` (SigmaOS.wiki.git) @ `ee82e62d76` ✅

## Tracked separate issues (pre-existing, not introduced here)

- `src/drivers/pcie_hotplug.rs` `#[cfg(test)]` block fails to compile due to API drift (`InterruptController::allocate_irq`, `MsiVector` field renames). Does not affect `cargo check --lib` (test-only code). Should be fixed separately.
- `cargo fmt --check` exits 1 for 351 pre-existing files (rustfmt 1.10 vs. committed style). Repo-wide reformat is a separate cleanup commit.
- 2 moderate GitHub Dependabot vulnerabilities on `default branch` (pre-existing, flagged in push output).
