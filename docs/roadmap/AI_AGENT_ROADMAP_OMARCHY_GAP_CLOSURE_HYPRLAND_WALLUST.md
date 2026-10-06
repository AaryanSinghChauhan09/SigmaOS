# AI Agent Future Development Roadmap: Omarchy Linux Gap Closure (Hyprland, Omakub, Wallust, Zellij)

## Architectural Directive
This document specifies the AI agent development directives for complete architectural parity between **SigmaOS** and **Omarchy Linux**. AI agents extending SigmaOS userland and compositor layers must implement the following 4 core capabilities:

### 1. Hyprland & Wayland Tiling Window Manager Parity
- **Dynamic Dwindle Tiling**: Automatic binary-tree splitting of windows upon launch on Wayland outputs.
- **Touchpad Swipe Gestures**: 3-finger and 4-finger swipe gesture tracking mapped to workspace switching (`process_swipe_gesture`).
- **Focus & Master Layout Rules**: Direct window focus shifting via keyboard shortcuts and IPC event hooks.

### 2. Omakub Out-of-the-Box Developer Environment Installer
- **Zero-Touch Bootstrap**: Automated setup of Neovim, Zellij, Rust, Go, Node.js, and Docker developer tools.
- **Categorized Presets**: Grouping presets into `Editors`, `TerminalTools`, `ProgrammingLanguages`, and `DatabaseAndStorage`.

### 3. Wallust Dynamic Color Palette Generator
- **Wallpaper Color Extraction**: Zero-copy sampling of background image color histograms.
- **Live Terminal & Compositor Sync**: Generating 16-color ANSI palettes and broadcasting updates to Ghostty, Waybar, Hyprland, and GTK themes.

### 4. Zellij Terminal Session Persistence
- **Detachable Session Management**: Creating, attaching, and persisting named terminal workspace layouts across system reboots.
- **Multi-Tab Layout Definitions**: Supporting custom layout templates (e.g. `compact`, `strider`).
