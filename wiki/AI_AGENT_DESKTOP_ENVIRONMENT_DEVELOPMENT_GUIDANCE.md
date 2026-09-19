# AI Agent Desktop Environment Development & Maintenance Guidance

## Executive Overview

This document provides architectural standards, layout algorithms, and accessibility guidelines for AI coding agents developing and maintaining the **Zenith Desktop Environment (Zenith DE)** and desktop appearance engines in SigmaOS. Zenith DE incorporates Hyprland/Omarchy-inspired dwindle vs. scrolling window tiling modes (`WindowLayoutMode`), Quickshell widget integration (`OmarchyQuickshellEngine`), Zorin layout switching (`ZorinAppearanceSwitcher`), and WCAG AA accessibility bindings.

---

## Desktop Subsystem Architecture

```
                            +-----------------------------------+
                            |  Zenith Wayland Compositor Core   |
                            +-----------------------------------+
                                              |
                                              v
                            +-----------------------------------+
                            | Window Manager & Layout Engine    |
                            | (Dwindle / Scrolling / Master)    |
                            +-----------------------------------+
                             /                 |               \
                            /                  |                \
        +-----------------------+  +-----------------------+  +-----------------------+
        | Quickshell Widgets    |  | Zorin Layout Switch   |  | Omarchy Theme Studio  |
        | Bar / Walker Launcher |  | Win 11 / macOS / GNOME|  | Single-Pass Restyle   |
        +-----------------------+  +-----------------------+  +-----------------------+
```

---

## Engineering Guidelines for AI Agents

1. **Window Tiling Algorithms**:
   - `Dwindle`: Recursively split active workspace tiles horizontally and vertically as new windows open.
   - `Scrolling`: Line up open windows side-by-side with horizontal pan navigation (`Super+L`).

2. **Single-Pass System-Wide Restyling**:
   - Use `OmarchySystemThemeStudio` to apply palette changes across top bar, launcher, notifications, lock screen, terminal, and apps simultaneously.

3. **WCAG AA Accessibility Compliance**:
   - Ensure screen-reader ARIA label annotations and keyboard navigation handlers (`Super+Arrows`, `Super+G`, `Super+O`, `Super+Grave`) are present.

---

## Diagnostic Verification Protocol

AI agents modifying Zenith DE must verify clean execution:
1. Run `./scripts/uiux_accessibility_test.sh` to confirm compositor frame times ($< 16$ms) and app cold-start times ($< 500$ms).
2. Run `./run_sigma_tests.sh` to confirm overall system stability.
