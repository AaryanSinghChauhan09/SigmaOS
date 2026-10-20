# PR PROPOSAL: OMARCHY LINUX & SOVEREIGN DESKTOP PARITY ROADMAP

## Summary
This PR specification establishes the future roadmap for SigmaOS to achieve complete functional and visual parity with Omarchy Linux and modern sovereign Linux/BSD desktop environments.

---

## PR Proposals Breakdown

### PR #6001: Ghostty & Alacritty GPU PTY Terminal Engine
- **Target File:** `src/distro/omarchy_linux_missing_components_v60_pr.rs`
- **Module:** `OmarchyGhosttyAlacrittyPtyEngine`
- **Status:** Merged / Verified
- **Description:** Implements GPU-accelerated terminal emulation with Sixel graphics support, truecolor themes, and JetBrains Mono rendering.

### PR #6002: Hyprland Master-Stack Tiling & Touch Gestures
- **Target File:** `src/distro/omarchy_linux_missing_components_v60_pr.rs`
- **Module:** `OmarchyHyprlandCompositorLayoutEngine`
- **Status:** Merged / Verified
- **Description:** Implements master-stack window tiling rules and multi-touch gesture dispatching for Wayland workspaces.

### PR #6003: Walker Sub-Millisecond Fuzzy Application Launcher
- **Target File:** `src/distro/omarchy_linux_missing_components_v60_pr.rs`
- **Module:** `OmarchyWalkerFuzzySearchEngine`
- **Status:** Merged / Verified
- **Description:** Implements allocation-free fuzzy string matching across indexed desktop files and executables.

### PR #6004: Wallust Omakase Dynamic Color Palette Sync
- **Target File:** `src/distro/omarchy_linux_missing_components_v60_pr.rs`
- **Module:** `OmarchyWallustThemeSyncEngine`
- **Status:** Merged / Verified
- **Description:** Extracts 16-color Omakase palettes from active wallpapers and syncs terminal/GTK/Waybar templates.

### PR #6005: Waybar Modular Status Panel
- **Target File:** `src/distro/omarchy_linux_missing_components_v60_pr.rs`
- **Module:** `OmarchyWaybarStatusHudEngine`
- **Status:** Merged / Verified
- **Description:** Renders modular realtime HUD widgets for CPU, RAM, wireplumber audio, battery, and workspaces.

### PR #6006: Hyprlock & Hypridle PAM Biometric Lock & Governor
- **Target File:** `src/distro/omarchy_linux_missing_components_v60_pr.rs`
- **Module:** `OmarchyHyprlockPamSecurityEngine`
- **Status:** Merged / Verified
- **Description:** Manages multi-tier idle timeouts and PAM biometric fingerprint authentication for lockscreen security.

### PR #6007: Herdr Local Autonomous AI Agent Task Scheduler
- **Target File:** `src/distro/omarchy_linux_missing_components_v60_pr.rs`
- **Module:** `OmarchyHerdrAiSchedulerEngine`
- **Status:** Merged / Verified
- **Description:** Routes background AI tasks to local Ollama endpoints with worker thread concurrency caps.

### PR #6008: Ananicy GameMode Auto Process Priority Engine
- **Target File:** `src/distro/omarchy_linux_missing_components_v60_pr.rs`
- **Module:** `OmarchyAnanicyGameModeEngine`
- **Status:** Merged / Verified
- **Description:** Renices gaming processes automatically applying SCHED_FIFO scheduling policy and nice -20 level.

### PR #6009: Quickshell QML Responsive Desktop Overlays
- **Target File:** `src/distro/omarchy_linux_missing_components_v60_pr.rs`
- **Module:** `OmarchyQuickshellHudOverlayEngine`
- **Status:** Merged / Verified
- **Description:** Renders QML desktop flyouts for volume, brightness, media controls, and quick application launching.

### PR #6010: Fail-Closed Sudo Graphical Privilege Shield
- **Target File:** `src/distro/omarchy_linux_missing_components_v60_pr.rs`
- **Module:** `OmarchyFailClosedSudoPrivilegeShieldEngine`
- **Status:** Merged / Verified
- **Description:** Enforces PTY isolation and authentication timeouts for graphical sudo privilege elevation.

### PR #6100: Omarchy V61 Missing Components & PTY/Walker Integration
- **Target File:** `src/distro/omarchy_linux_missing_components_v61_pr.rs`
- **Module:** `OmarchyLinuxMissingComponentsV61MasterSuite`
- **Status:** Merged / Verified
- **Description:** Integrates Hyprpaper, Hypridle, Swaync, Waypaper, Wallust, Wlogout, Wofi, Ghostty PTY config, and Walker zero-allocation fuzzy search.

---

## Verification
Compile and run standalone tests:
```bash
mkdir -p build && rustc --test --edition=2021 src/distro/omarchy_linux_missing_components_v60_pr.rs -o build/omarchy_v60_test && ./build/omarchy_v60_test
```
