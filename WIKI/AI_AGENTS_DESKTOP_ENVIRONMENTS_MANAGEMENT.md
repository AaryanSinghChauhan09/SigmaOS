# 🎨 AI Agents Desktop Environments Management Specification (`docs/AI_AGENTS_DESKTOP_ENVIRONMENTS_MANAGEMENT.md`)

This specification defines desktop environment management protocols, Wayland compositor operations, UI accessibility guidelines, and theme engine management for autonomous AI agents (Bolt ⚡, Palette 🎨, Sentinel 🛡️) in SigmaOS.

---

## 1. Zenith Wayland Microcompositor & Window Management (`zenith_desktop/`)

AI agents manage the native Wayland desktop compositor:
- **Hyprland-Style Dwindle Tiling**: Dynamic window tree layout calculation (`src/ui/window.rs`).
- **Visual Effects Pipeline**: Hardware-accelerated rounded corners (`rounding = 10`), GPU blur passes, and drop shadows.
- **Display Scaling & HiDPI**: Fractional scaling and per-monitor DPI autoconfiguration.

---

## 2. Control Center & Theme Engine (`src/ui/control_center.rs`, `src/ui/theme.rs`)

- **Dynamic Theme Cycling**: Live switching across curated themes (`TokyoNight`, `Catppuccin`, `Gruvbox`, `Nord`, `Adwaita Dark`).
- **Web2App Launchers**: Automated creation of desktop entries with Wayland Ozone support (`--ozone-platform=wayland`).

---

## 3. Accessibility & WCAG 2.1 AA Compliance (`src/ui/gtk_toolkit.rs`)

- **Focus Visible Outlines**: High-contrast keyboard focus indicators (`:focus-visible`).
- **ARIA & Screen Reader Annotations**: Semantic roles (`role="button"`, `aria-label`, `aria-expanded`).
- **Color Contrast**: Enforced minimum 4.5:1 text-to-background contrast ratio.

---

## 4. AI Agent Desktop Responsibilities

- **⚡ Bolt**: Profiles compositor rendering frame times (maintaining 60+ FPS), monitors GPU memory consumption, and optimizes window animation pipelines.
- **🎨 Palette**: Customizes visual themes, applies accessibility annotations, tunes font rendering, and designs responsive desktop layouts.
- **🛡️ Sentinel**: Enforces process isolation for desktop applets and web2app launchers, auditing IPC channels between renderer processes and the compositor.
