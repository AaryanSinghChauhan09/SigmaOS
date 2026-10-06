# AI Agent Roadmap: Omarchy Linux Distro Feature Parity in SigmaOS

## Executive Overview & Architectural Vision

This specification defines the complete AI Agent Roadmap for achieving **100% Native Feature Parity with Omarchy Linux** within **SigmaOS**.
Omarchy Linux represents the zenith of modern Arch-based Linux desktop distributions, offering an curated, opinionated "Omakase" workstation experience with custom Wayland window management (Hyprland), seamless theme switching, terminal audio visualizers, multi-agent AI orchestration, and automated dotfiles lifecycle management.

SigmaOS absorbs, outclasses, and natively implements all Omarchy Linux subsystems as zero-dependency `#![no_std]` / `alloc` Rust modules with $O(1)$ constant-time data structures, capability-based sandboxing, and PR workflow integration.

---

## Key Subsystems & Native Component Specifications

### 1. Omarchy Omakase Setup & Bootstrap Installer
- **Component**: `OmarchyOmakaseSetupInstallerEngine`
- **Functionality**:
  - Automated desktop workstation profile installation (`OmakaseProfile`).
  - Pre-configured package selection (Hyprland, Waybar, Rofi/Wofi, Alacritty, SwayNC, Cava, Neovim, Fastfetch).
  - Dry-run installation plan generation and rollback tracking.

### 2. Omarchy Theme & Wallpaper Synchronization Daemon
- **Component**: `OmarchyThemeWallpaperDaemonEngine`
- **Functionality**:
  - Dynamic desktop theme & background wallpaper switching across 22 curated themes (Tokyo Night, Catppuccin, Nord, Gruvbox, Everforest, Kanagawa, Rose Pine, Dracula, Solarized, Cyberpunk, etc.).
  - Automated color palette extraction (`Pywal`/`Wallust` algorithm parity) from wallpaper images.
  - Multi-app live theme synchronization (Alacritty, Waybar, Rofi, GTK/Qt, SwayNC, Hyprland borders).

### 3. Omarchy Hyprland Hotkey & Gesture Binder
- **Component**: `OmarchyHyprlandKeybindingsEngine`
- **Functionality**:
  - Declaration and management of Hyprland window manager shortcuts (`Mod+Return`, `Mod+Q`, `Mod+Space`, `Mod+Shift+S`).
  - Detection of conflicting keybindings across desktop components.
  - Multi-touch touchpad gesture rules (3-finger swipe workspace switching, pinch zoom).

### 4. Omarchy Cava Audio Visualizer & PipeWire FFT Engine
- **Component**: `OmarchyAudioVisualizerCavaEngine`
- **Functionality**:
  - Cava terminal spectrum audio visualizer configuration generator.
  - PipeWire FFT audio frequency binning and frame rate throttling (30 FPS, 60 FPS, 144 FPS).
  - Stereo/Mono channel layout and dynamic gradient color mapping.

### 5. Omarchy Universal IDE Configuration Transpiler
- **Component**: `OmarchyIdeConfigTranspilerEngine`
- **Functionality**:
  - Transpilation of Omarchy developer environment configurations for Neovim/LazyVim and Helix modal editors.
  - Automatic LSP server mapping, Tree-sitter parser hooks, and statusline theme synchronization.

### 6. Omarchy SwayNC Notification Center & DND Applet
- **Component**: `OmarchySwayncNotificationEngine`
- **Functionality**:
  - SwayNC Wayland notification daemon policy and layout generator.
  - Do Not Disturb (DND) mode toggles, notification filtering by urgency, and inline widget applets (MPRIS audio player, Bluetooth status, Wi-Fi status).

### 7. Sovereign Master Omarchy Linux Synthesis Suite
- **Component**: `SovereignOmarchyLinuxMasterSynthesisSuite`
- **Functionality**:
  - Master orchestration and health monitoring across all 18 Omarchy Linux native parity engines in SigmaOS.
  - Continuous validation, telemetry reporting, and zero-downtime hot-reloading.

---

## Implementation Matrix & Subsystem Mapping

| Omarchy Subsystem | Legacy Project / Script | SigmaOS Native Module | Performance Advantage |
| :--- | :--- | :--- | :--- |
| **Omakase Installer** | `omarchy-setup.sh` | `OmarchyOmakaseSetupInstallerEngine` | Zero bash dependencies; transactional rollback |
| **Theme / Wallpaper** | `omarchy-theme`, `swww` | `OmarchyThemeWallpaperDaemonEngine` | $O(1)$ lockless theme switching |
| **Hyprland Shortcuts** | `hyprland.conf` | `OmarchyHyprlandKeybindingsEngine` | Compile-time hotkey conflict resolution |
| **Audio Visualizer** | `cava` | `OmarchyAudioVisualizerCavaEngine` | Direct PipeWire zero-copy FFT spectrum rendering |
| **IDE Transpiler** | `lazyvim`, `helix` configs | `OmarchyIdeConfigTranspilerEngine` | Instant config transpilation & validation |
| **Notification Center** | `swaync` | `OmarchySwayncNotificationEngine` | Low-latency Wayland notification pipeline |
| **Master Orchestrator** | `omarchy-ctl` | `SovereignOmarchyLinuxMasterSynthesisSuite` | Unified `#![no_std]` health monitoring & IPC |

---

## Verification & PR Criteria

1. **Standalone Test Coverage**: Every module must include `#[cfg(test)]` standalone unit tests executing clean-room verification.
2. **Zero Runtime Allocation Overhead**: Fixed-size structures, precomputed lengths, and zero-copy string slice indexing.
3. **Continuous Documentation Sync**: All changes must be synchronized across `wiki/`, `WIKI/`, and `docs/`.
