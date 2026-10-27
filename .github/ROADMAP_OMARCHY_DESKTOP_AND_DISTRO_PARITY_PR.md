# PR Proposal: AI Agent Future Development Roadmap — Omarchy Linux Desktop & Userland Parity

## Summary
This pull request specification establishes the AI Agent Future Development Roadmap for achieving complete, clean-room desktop and userland feature parity between SigmaOS and Omarchy Linux distributions. It outlines the architectural design, IPC interface protocols, theme synchronization, and security privilege shielding required for autonomous AI agents implementing Omarchy Linux desktop components.

## Target Components

### 1. Hyprland Compositor & Dynamic Tiling Engine
- **Upstream Inspiration**: Omarchy Hyprland Omakase setup.
- **Key Capabilities**:
  - Dwindle and Master workspace layout engines.
  - Multi-touch swipe gesture event router.
  - Dynamic window rules (`float`, `center`, `tile`, `noborder`) for quick launchers and lockscreens.
  - Custom bezier animation curve profiles (`md3_decel`, `ease_out`).
- **Implementation Goal**: `src/distro/omarchy_linux_missing_components_v75_pr.rs::OmarchyHyprlandCompositorLayoutEngineV75`.

### 2. Waybar Status HUD & Dynamic CSS Live Reloader
- **Upstream Inspiration**: Omarchy custom Waybar status bar.
- **Key Capabilities**:
  - Modular panel layout (Left: workspaces/window title, Center: clock/weather, Right: CPU/Memory/Audio/Tray).
  - Live CSS theme reload without panel restart.
  - Real-time IPC event listener for workspace status changes.
- **Implementation Goal**: `src/distro/omarchy_linux_missing_components_v75_pr.rs::OmarchyWaybarStatusHudEngineV75`.

### 3. Ghostty / Alacritty GPU Terminal PTY Engine
- **Upstream Inspiration**: Ghostty terminal emulator integration in Omarchy Linux.
- **Key Capabilities**:
  - PTY multiplexing with tab/window session state recovery.
  - GPU-accelerated rasterization with custom shader pipeline (`bloom_crt.glsl`).
  - Hot-swappable Nerd Font typography & color theme profile tuner.
- **Implementation Goal**: `src/distro/omarchy_linux_missing_components_v75_pr.rs::OmarchyGhosttyPtyTerminalEngineV75`.

### 4. Walker Fuzzy Search Launcher & Cliphist Clipboard Engine
- **Upstream Inspiration**: Walker launcher in Omarchy.
- **Key Capabilities**:
  - High-performance fuzzy matching for application executables, emojis, and math expressions.
  - SSH host execution and quick connect shortcuts.
  - Cliphist history daemon IPC integration.
- **Implementation Goal**: `src/distro/omarchy_linux_missing_components_v75_pr.rs::OmarchyWalkerLauncherEngineV75`.

### 5. Wallust Dynamic Palette Extractor & GTK/Qt Theme Sync
- **Upstream Inspiration**: Wallust color generation in Omarchy.
- **Key Capabilities**:
  - Dominant color extraction from desktop wallpaper via LAB/K-Means color space clustering.
  - Automated GTK3/4, Qt5/6, and Ghostty color configuration generation.
  - Swww wallpaper daemon transition controller.
- **Implementation Goal**: `src/distro/omarchy_linux_missing_components_v75_pr.rs::OmarchyWallustThemeSyncEngineV75`.

### 6. Hyprlock PAM Biometric Locker & Hypridle Governor
- **Upstream Inspiration**: Hyprlock and Hypridle security setup.
- **Key Capabilities**:
  - PAM authentication integration with fprintd fingerprint fallback.
  - Tiered idle power management (300s lock, 600s suspend).
  - Secure memory zeroing for credentials.
- **Implementation Goal**: `src/distro/omarchy_linux_missing_components_v75_pr.rs::OmarchyHyprlockPamSecurityEngineV75`.

### 7. Ananicy-cpp Auto-Nice Priority Scheduler & Game Mode
- **Upstream Inspiration**: Ananicy-cpp process rules in Omarchy.
- **Key Capabilities**:
  - eBPF-assisted automatic process niceness (`-15` for Hyprland, `-10` for Steam, `-5` for terminal).
  - CPU governor scaling threshold switching upon game detection.
- **Implementation Goal**: `src/distro/omarchy_linux_missing_components_v75_pr.rs::OmarchyAnanicyGameModeEngineV75`.

### 8. Herdr AI Agent Workflow Scheduler & Quickshell QML HUD
- **Upstream Inspiration**: Herdr local AI assistant integration.
- **Key Capabilities**:
  - Local AI endpoint (Ollama / LocalAI) RPC dispatcher.
  - Quickshell QML desktop overlay renderer for AI agent status feedback.
- **Implementation Goal**: `src/distro/omarchy_linux_missing_components_v75_pr.rs::OmarchyHerdrAiSchedulerEngineV75`.

### 9. Fail-Closed Sudo Privilege Shield & Polkit Gate
- **Upstream Inspiration**: Omarchy security/nopasswd-expiry-fail-closed shield.
- **Key Capabilities**:
  - Strict input validation preventing command injection and null byte truncation.
  - User privilege checking (root/wheel) with automatic timeout invalidation.
- **Implementation Goal**: `src/distro/omarchy_linux_missing_components_v75_pr.rs::OmarchyFailClosedSudoPrivilegeShieldEngineV75`.

### 10. Wallust Chord Theme Navigator
- **Upstream Inspiration**: Omarchy chord shortcuts for desktop theme cycling.
- **Key Capabilities**:
  - Hotkey chords (`SUPER+SHIFT+T`, `SUPER+SHIFT+W`, `SUPER+SHIFT+C`).
  - Seamless desktop theme, wallpaper, and color picker trigger dispatching.
- **Implementation Goal**: `src/distro/omarchy_linux_missing_components_v75_pr.rs::OmarchyWallustChordThemeNavigatorEngineV75`.

## Safety & Rollback
- All desktop state changes are managed through deterministic, rollback-safe configuration generators in `src/distro/`.
- In case of compositor or session failure, default safe fallback states are loaded without loss of user credentials or system files.

## Verification
- Run standalone module verification: `rustc --test --edition=2021 src/distro/omarchy_linux_missing_components_v75_pr.rs -o build/omarchy_v75_test && ./build/omarchy_v75_test`
- Run integration test suite: `./run_sigma_tests.sh`
