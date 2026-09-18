# AI Agent GUI Operations Management in SigmaOS

## Overview
SigmaOS incorporates a modern desktop environment and Wayland microcompositor (Zenith Desktop) governed by autonomous AI Agents (**Palette** 🎨, **Bolt** ⚡, **Sentinel** 🛡️). This document defines operational directives, window management protocols, XDG desktop portal interactions, theme engine rules, and WCAG 2.1 AA accessibility guidelines for AI agents supervising GUI operations across SigmaOS.

AI agents interact directly with `src/desktop/compositor.rs` (Zenith Compositor), `src/desktop/desktop_portal.rs`, `src/desktop/launcher.rs`, and `zenith_desktop/`.

---

## 1. GUI Subsystems & Architecture

### 1.1 Zenith Wayland Microcompositor (`src/desktop/compositor.rs`)
Implemented in `src/desktop/compositor.rs`. Manages zero-copy Wayland buffer blitting, multi-monitor display outputs, Hyprland-inspired dwindle window tiling, and sub-surface compositing:
* **Window Tiling & Scaling**: Dynamically resizes window frames based on active workspace layout and display DPI scale.
* **Double-Buffered ARGB Blitting**: Renders window surfaces to linear framebuffers (`GopLinearFramebufferDriver`) with hardware-accelerated VSync synchronization.

### 1.2 XDG Desktop Portals (`src/desktop/desktop_portal.rs`)
Implemented in `src/desktop/desktop_portal.rs`. Mediates sandboxed desktop portal requests for flatpak/containerized apps (file chooser, screenshot, screen cast, wallpaper, secret storage).

### 1.3 Application Launcher & Web2App Engine (`src/desktop/launcher.rs`)
Implemented in `src/desktop/launcher.rs`. Indexes system applications (`.desktop` entries) and converts web URLs into standalone, sandboxed Web2App desktop windows.

### 1.4 Zenith Web Desktop UI & Accessibility (`zenith_desktop/`)
Implemented in `zenith_desktop/`. Provides the web-based OS control center, taskbar panel, and desktop window manager.

---

## 2. AI Agent Operational Directives & Protocols

### 2.1 UX & Accessibility Protocols (🎨 Palette)
1. **WCAG 2.1 AA Compliance**:
   **Palette** 🎨 enforces high-contrast themes (TokyoNight, Catppuccin, Adwaita) and visible focus indicators (`:focus-visible`).
2. **Accessible Control Annotations**:
   Interactive buttons and tab bars must include explicit `type="button"`, `role="tablist"`, `role="tab"`, and `aria-label` attributes for screen readers.

### 2.2 Desktop Portal Security (🛡️ Sentinel)
* **Sandboxed Permission Prompts**:
  When a sandboxed app requests file chooser or screen recording access, **Sentinel** 🛡️ verifies user authorization via `XdgDesktopPortal` before granting temporary path unveil permissions.

---

## 3. Sample Agent Commands & CLI Interactions

```bash
# Query active Zenith compositor display surfaces and window layout
sigma-gui compositor-status

# Toggle window tiling layout mode (dwindle / master-stack)
sigma-gui set-layout --mode dwindle

# Audit Web UI accessibility attributes and WCAG compliance
sigma-gui audit-a11y --dir zenith_desktop/
```
