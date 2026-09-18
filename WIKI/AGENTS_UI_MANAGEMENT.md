# AGENTS_UI_MANAGEMENT.md — AI Agent UI Management Guidelines for SigmaOS

Welcome, AI Agent! This document defines the standards, architectural models, and verification guidelines for managing, developing, and extending the **User Interface (UI) and Desktop Systems** in **SigmaOS**.

---

## 1. SigmaOS UI Architecture Overview

SigmaOS features a flexible, multi-environment desktop framework capable of hosting native Wayland surface compositing as well as absorbed Linux and BSD desktop environments.

### Core UI Modules
* **Zenith Compositor (`src/desktop/zenith_compositor.rs`, `src/desktop/zenith.rs`)**: Native Wayland/X11 display compositor supporting surface buffers, damage regions (`DamageRegion`), window geometries, state transitions (`WindowState`), and input event dispatching.
* **Distro Desktop Parity (`src/desktop/ultimate_distro_desktop.rs`, `src/desktop/pantheon.rs`, `src/desktop/moksha.rs`, `src/distro/omarchy.rs`)**:
  - **KDE Plasma 6 (`KdePlasma6Engine`)**: KWin Wayland split-tiling grid calculation, KRunner search dispatching, wallpaper accent color extraction.
  - **GNOME 46 Mutter (`Gnome46MutterEngine`)**: Fractional scaling factor management (1.0x to 4.0x), Quick Settings system toggle panel, Shell Extension sandbox validator.
  - **XFCE 4.18 (`Xfce418Engine`)**: Thunar custom context menu actions, XFCE4 panel plugin IPC channels.
  - **Lumina BSD (`LuminaBsdDesktopEngine`)**: Native BSD sysctl hardware telemetry (battery, thermals), Lumina-FM ZFS snapshot file restoration.
  - **Sway / Regolith (`SwayRegolithWmEngine`)**: Tree-based container layout tiling nodes (Horizontal, Vertical, Tabbed, Stacked) and keybinding action dispatcher.
  - **Omarchy Hyprland (`OmarchyModernDesktopEngine`)**: Dwindle layout generation, curated theme switcher (Tokyo-Night, Catppuccin, Gruvbox, Nord, Everforest, Kanagawa), Web2App PWA desktop entry generator.
  - **Mint Cinnamon (`CinnamonThemeEngine`) & MATE (`mate_betsy.rs`)**: LMDE 2 Betsy applet themes, panel transparency, desklets, and sound schemes.
  - **Pantheon (`pantheon.rs`) & Moksha (`moksha.rs`)**: Plank dock, Wingpanel indicators, Slingshot launcher, Evas canvas manager.
* **System Customization & Theme Engines (`src/ui/folder_color.rs`, `src/customization/cursor.rs`, `src/boot/plymouth.rs`)**:
  - **Folder Color Switcher**: Papirus/Yaru icon palette inheritance, emblem overlays, GTK/Papirus CSS theme generator.
  - **Cursor Theme Engine**: XCursor specifications (Adwaita, Breeze, Bibata), hotspot offsets, animated multi-frame cursors.
  - **Plymouth Bootsplash**: GTK bootsplash spinner/logo themes, LUKS password prompt dialogs, boot progress bars.

---

## 2. UI Development Guidelines for AI Agents

When modifying or adding UI components, follow these strict directives:

### 1. Compositor & Geometry Safety
* **Damage Region Tracking**: Always mark damaged bounding boxes (`DamageRegion`) when updating Wayland surfaces to avoid full-screen redraw overhead.
* **Fractional Scaling Alignment**: Ensure coordinates and surface dimensions are properly rounded when fractional scaling factors (e.g. 1.25x or 1.5x) are active in `Gnome46MutterEngine`.
* **Tiling Grid Limits**: In `KdePlasma6Engine` and `SwayRegolithWmEngine`, gracefully handle zero windows or screen boundary overflow cases.

### 2. Theme Palette & Asset Standards
* **Color Extraction**: Extract dynamic accent colors using RGB histogram sampling or standard hex strings (`#RRGGBB`).
* **Icon & CSS Generation**: When generating CSS themes for GTK/Papirus or Folder Color switchers, maintain standard CSS class names (`.folder`, `.folder-color-badge`) and subfolder inheritance rules.

### 3. Accessibility & Keyboard Navigation
* **Keyboard Navigation**: Ensure all UI applets, launchers (KRunner, Slingshot), and menus support full keyboard navigation (Up, Down, Tab, Escape, Return).
* **High Contrast & Screen Reader Parity**: Integrate with the accessibility manager (`src/accessibility/`) to expose UI component roles and ARIA-equivalent labels.

---

## 3. Visual Verification Protocol

When making user-visible UI or CSS modifications:
1. **Accessibility Verification**: Execute the UI accessibility script:
   ```bash
   ./scripts/uiux_accessibility_test.sh
   ```
2. **Playwright Visual Testing (If Applicable)**: If editing HTML, CSS, or web desktop components (e.g. `zenith_desktop.css` or `web_ui/`), use Playwright scripts to capture screenshots and verify rendering fidelity according to `frontend_verification_instructions`.
3. **Unit Test Verification**: Run the core Rust test suite to verify desktop engine invariants:
   ```bash
   ./run_sigma_tests.sh
   ```

---

## 4. Pre-Commit Checklist for UI Changes

Before finalizing UI contributions:
- [ ] Added unit tests covering new UI engine methods or state transitions.
- [ ] Confirmed `./run_sigma_tests.sh` passes without errors.
- [ ] Requested automated code review using `request_code_review`.
- [ ] Recorded UI patterns using `initiate_memory_recording`.
