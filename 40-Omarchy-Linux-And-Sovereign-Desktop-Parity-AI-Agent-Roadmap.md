# Omarchy Linux & Sovereign Desktop Parity AI Agent Roadmap

## Overview & Mission
This Wiki roadmap defines the engineering blueprint for autonomous AI agents implementing full desktop and system parity between SigmaOS and Omarchy Linux distributions.

## Architectural Objectives
- **Compositor & Tiling Layouts**:
  - Implement Hyprland-compatible window management, touch gestures, and window decoration rules.
- **Status Bar & Applets**:
  - Waybar-inspired status HUD with JSON configuration contracts, live CSS theme updates, and IPC status dispatching.
- **Terminal & Shell**:
  - GPU-accelerated terminal emulation (Ghostty/Alacritty) with PTY multiplexing and cross-shell prompt tuning.
- **Fuzzy Launcher & Clipboard**:
  - Low-latency Walker launcher with app, emoji, and calculator query engines plus `cliphist` clipboard integration.
- **Dynamic Color Extraction**:
  - Wallust dynamic palette extraction from SWWW/Hyprpaper wallpapers with GTK3/GTK4/Qt style synchronization.
- **Security & Process Priority**:
  - Hyprlock PAM biometric lock screen, Hypridle power governor, Ananicy-cpp auto-nice scheduler, and fail-closed sudo privilege shield.

## Technical Specifications & Parity Matrix
| Engine | Module Path | Responsibilities | Status |
| :--- | :--- | :--- | :--- |
| **Compositor Engine** | `src/distro/omarchy_linux_missing_components_v72_pr.rs` | Tiling layouts, gesture router, rules | Merged |
| **Status HUD Engine** | `src/distro/omarchy_linux_missing_components_v72_pr.rs` | Waybar modules, CSS reload, IPC | Merged |
| **PTY Terminal Engine** | `src/distro/omarchy_linux_missing_components_v72_pr.rs` | Ghostty PTY multiplexing, fonts, themes | Merged |
| **Fuzzy Search Launcher** | `src/distro/omarchy_linux_missing_components_v72_pr.rs` | Walker fuzzy launcher, cliphist | Merged |
| **Theme Sync Engine** | `src/distro/omarchy_linux_missing_components_v72_pr.rs` | Wallust color palette, GTK/Qt sync | Merged |
| **PAM Lock & Power** | `src/distro/omarchy_linux_missing_components_v72_pr.rs` | Hyprlock PAM, Hypridle timeouts | Merged |
| **Auto-Nice Scheduler** | `src/distro/omarchy_linux_missing_components_v72_pr.rs` | Ananicy-cpp process priority tuner | Merged |
| **AI Workflow Runner** | `src/distro/omarchy_linux_missing_components_v72_pr.rs` | Herdr AI task dispatcher, HUD overlay | Merged |
| **Privilege Shield** | `src/distro/omarchy_linux_missing_components_v72_pr.rs` | Fail-closed sudo & Polkit gate | Merged |

## Verification Suite
```bash
rustc --test --edition=2021 src/distro/omarchy_linux_missing_components_v72_pr.rs -o build/omarchy_v72_test && ./build/omarchy_v72_test
rustc --test --edition=2021 tests/test_omarchy_linux_v72_pr.rs -o build/test_omarchy_v72_pr && ./build/test_omarchy_v72_pr
./run_sigma_tests.sh
```
