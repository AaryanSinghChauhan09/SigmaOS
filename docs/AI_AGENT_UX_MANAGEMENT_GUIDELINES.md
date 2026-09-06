# SigmaOS AI Agent User Experience (UX) Management Guidelines

## 1. Overview
SigmaOS features autonomous and interactive UX management agents (such as `AdaptiveUxAgent`, `GamifiedDesktopEngine`, `FolderColorSwitcherEngine`, `CinnamonThemeEngine`, and `CursorThemeEngine`). These agents allow natural language and predictive management of desktop layouts, desktop themes, accessibility features, and workspace organization.

## 2. Core UX Management Principles

### 2.1 Adaptive & Non-Intrusive Interaction
- **User Preference First**: UX management agents must prioritize direct user preferences over automated adjustments.
- **Predictive Layout Switching**: Agents automatically adapt workspace layouts (e.g., Zorin-style Windows, macOS, or GNOME Shell layouts) based on active workloads (e.g., coding, gaming, data science, or media editing).
- **Gamified Desktop & Workflows**: Agents manage taskbar progress, desktop streak rewards, and productivity task automation via `GamifiedDesktopEngine`.

### 2.2 Theme & Customization Framework
UX agents manipulate system appearance using structured native engines:
- **Cinnamon & GTK Themes**: `CinnamonThemeEngine` toggles presets (`MintYDark`, `MintYLight`, `MintYAqua`, `Adwaita`) and updates GTK CSS styles without requiring shell restarts.
- **Folder Color & Emblems**: `FolderColorSwitcherEngine` allows AI agents to dynamically color-code directory icons and apply status emblems (e.g., Git repository state, security sensitivity, or project phase).
- **XCursor & Wayland Cursor Management**: `CursorThemeEngine` dynamically scales cursor size and switches cursor themes (`Adwaita`, `Breeze`, `Bibata`) according to high-DPI scaling or user accessibility requests.

### 2.3 Accessibility & Inclusive Design
- **High-Contrast & Dyslexia-Friendly Fonts**: Agents can trigger high-contrast CSS modes and switch UI typography to OpenDyslexic or Atkinson Hyperlegible fonts upon user command or accessibility probe detection.
- **Screen Reader & Speech Integration**: AI agents interface with `WhisperSpeechToText` and speech synthesis pipelines for hands-free desktop navigation and command execution.

### 2.4 Desktop Portals & Safety Bounds
- **Capability Gated Portals**: UX agents modify desktop configurations via `DesktopPortal` IPC channels requiring user confirmation for destructive or high-impact actions (e.g., changing screen resolution, unmounting drives, or resetting desktop configurations).

---
*Maintained by the SigmaOS Desktop & UX Steering Committee.*
