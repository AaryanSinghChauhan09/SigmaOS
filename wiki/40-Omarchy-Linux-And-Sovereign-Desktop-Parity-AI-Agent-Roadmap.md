# 40. Omarchy Linux & Sovereign Desktop Parity AI Agent Roadmap

## Executive Summary
This document specifies the future development roadmap for SigmaOS desktop compositing, terminal user experience, dynamic styling, application launching, and local AI agent workflow parity formatted as Pull Request (PR) specifications for autonomous AI agents. Inspirations are synthesized from **Omarchy Linux**, **Arch Linux**, **CachyOS**, **Wayland Protocol Workgroups**, and modern Linux/BSD desktop environments.

---

## AI Agent Parity Roadmap Matrix

| PR # | Title & Target Desktop Component | Inspired Sources | Key Architectural Specifications |
| :--- | :--- | :--- | :--- |
| **#4001** | `feat(desktop): Ghostty & Alacritty GPU PTY Terminal with Sixel Graphics` | Omarchy Linux, Ghostty, Alacritty | GPU-accelerated PTY terminal emulator supporting sub-millisecond rendering, Sixel inline image protocols, truecolor 24-bit palettes, and zero-allocation VT sequence parsing. |
| **#4002** | `feat(compositor): Hyprland Master-Stack Layout & Dynamic Touch Gestures` | Hyprland, Omarchy Linux, Wayland 1.24 | Master-stack window tiling layout algorithm with configurable split ratios, dynamic window swallowing, and multi-finger touch gesture tracking. |
| **#4003** | `feat(launcher): Walker Sub-Millisecond Fuzzy Launcher & Plugin Engine` | Walker, Omarchy Linux, Rofi | Asynchronous sub-millisecond fuzzy search launcher across desktop entries, terminal commands, SSH hosts, emoji tables, and window switcher queues. |
| **#4004** | `feat(theme): Wallust Omakase Dynamic Color Palette Theme Synchronizer` | Wallust, Pywal, Omarchy Linux | Dynamic wallpaper-driven color palette extractor broadcasting standardized Omakase color schemes across GTK, Waybar, Ghostty, Alacritty, and Walker templates. |
| **#4005** | `feat(security): Hyprlock & Hypridle PAM Biometric Lock & Idle Governor` | Hyprlock, Hypridle, FreeBSD PAM | Multi-stage desktop idle state governor triggering screen dimming, lockscreen activation, and PAM biometric/fingerprint authentication with failsafe timeouts. |
| **#4006** | `feat(ai): Herdr Local Autonomous AI Agent Task Scheduler` | Herdr AI, Omarchy Linux, Ollama | Local LLM/agent task scheduler managing background AI workflow queues, prompt routing, and zero-trust local inference model sandbox execution. |

---

## Detailed PR Specifications for AI Agents

### PR #4001: `feat(desktop): Ghostty & Alacritty GPU PTY Terminal with Sixel Graphics`
- **PR Branch:** `feature/ghostty-alacritty-pty-sixel`
- **Target Subsystem:** Desktop Terminal Emulator (`src/desktop/`, `src/distro/omarchy_linux_missing_components_v60_pr.rs`)
- **Primary Inspiration:** *Omarchy Linux Desktop & Ghostty Terminal Engine*
- **Key Capabilities:**
  1. Clean-room `#![no_std]` Rust implementation of GPU PTY terminal emulator with Sixel graphics decoding.
  2. Integration with JetBrains Mono / Nerd Font font rasterizers and truecolor palette mappings.
  3. Zero-copy buffer swapping for high-FPS terminal animation and scrolling performance.

### PR #4002: `feat(compositor): Hyprland Master-Stack Layout & Dynamic Touch Gestures`
- **PR Branch:** `feature/hyprland-master-stack-gestures`
- **Target Subsystem:** Wayland Compositor (`src/desktop/hyprland.rs`, `src/desktop/wayland_protocol.rs`)
- **Primary Inspiration:** *Hyprland Wayland Compositor & Omarchy Window Rules*
- **Key Capabilities:**
  1. Master-stack layout manager maintaining primary focus window with secondary stack balance.
  2. Multi-touch gesture dispatcher supporting 3-finger swipe workspace transitions and 4-finger pinch window scaling.
  3. Direct scanout support for zero-latency fullscreen gaming and video playback.

### PR #4003: `feat(launcher): Walker Sub-Millisecond Fuzzy Launcher & Plugin Engine`
- **PR Branch:** `feature/walker-fuzzy-launcher`
- **Target Subsystem:** Application Launcher (`src/desktop/walker_launcher.rs`)
- **Primary Inspiration:** *Walker Launcher & Omarchy Omakase Stack*
- **Key Capabilities:**
  1. Allocation-free ASCII and UTF-8 fuzzy matching algorithm (`contains_ignore_case`) filtering desktop candidates in under 500 microseconds.
  2. Extensible plugin API supporting calculation engines, custom bash scripts, and system service control.
  3. Instant keyboard navigation with vim-style keybindings (`ctrl+j`/`ctrl+k`).

### PR #4004: `feat(theme): Wallust Omakase Dynamic Color Palette Theme Synchronizer`
- **PR Branch:** `feature/wallust-omakase-theme-sync`
- **Target Subsystem:** Desktop Theming (`src/theming/`, `src/desktop/omarchy_theme.rs`)
- **Primary Inspiration:** *Wallust Color Engine & Omarchy Omakase Presets*
- **Key Capabilities:**
  1. Fast K-means color quantization extracting 16 dominant colors from active desktop background wallpaper.
  2. Realtime IPC broadcast updating CSS variables across Waybar, terminal configs, and GTK theme templates.
  3. 12 pre-configured Omakase theme presets with instant hot-swapping.

### PR #4005: `feat(security): Hyprlock & Hypridle PAM Biometric Lock & Idle Governor`
- **PR Branch:** `feature/hyprlock-hypridle-pam-security`
- **Target Subsystem:** Desktop Security (`src/security/pam.rs`, `src/desktop/`)
- **Primary Inspiration:** *Hyprlock Screen Shield & FreeBSD PAM Modules*
- **Key Capabilities:**
  1. PAM-backed multi-factor biometric authentication support (fprintd fingerprint, facial recognition, password fallback).
  2. Hypridle time-decay state machine transitioning through screen dimming, lockscreen, display DPMS off, and system suspend.
  3. Secure memory wiping for PAM secrets upon authentication completion.

### PR #4006: `feat(ai): Herdr Local Autonomous AI Agent Task Scheduler`
- **PR Branch:** `feature/herdr-local-ai-agent-scheduler`
- **Target Subsystem:** AI Orchestration (`src/ai/`, `src/distro/omarchy_linux_missing_components_v60_pr.rs`)
- **Primary Inspiration:** *Herdr AI & Omarchy Agent Workflows*
- **Key Capabilities:**
  1. Local REST & Unix domain socket API client connecting to local Ollama / Llama.cpp endpoints.
  2. Multi-agent priority task queue scheduling asynchronous code analysis, document summary, and system administration tasks.
  3. Resource cap governor dynamically throttling AI inference thread count based on system CPU and thermal load.

---

## Verification & Validation Commands

To verify this roadmap slice:
```bash
mkdir -p build && rustc --test --edition=2021 src/distro/omarchy_linux_missing_components_v60_pr.rs -o build/omarchy_v60_test && ./build/omarchy_v60_test
./run_sigma_tests.sh
```
