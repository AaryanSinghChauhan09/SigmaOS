# SigmaOS AI Agents UX Management & Interface Guide

Welcome to the **SigmaOS AI Agents UX Management & Interface Guide**. This document details the visual, interaction, and user experience (UX) management guidelines for autonomous AI agents, UI toolkits, and desktop environment developers in SigmaOS.

---

## 1. User Experience Architecture & Philosophy

SigmaOS prioritizes a **visual-first, accessible, responsive, and adaptable UI/UX** powered by native safe Rust toolkits without heavy runtime dependencies.

### Key UX Pillars
1. **Zero-Latency Compositing**: Zenith Desktop Compositor (`src/desktop/zenith_compositor.rs`) provides hardware-accelerated 60+ FPS window rendering, animations, and gesture processing.
2. **Multi-Distro Layout Presets**: Adaptable visual layouts inspired by leading Linux & BSD desktop environments (`KdePlasma`, `GnomeShell`, `XfceModular`, `CinnamonMint`, `LuminaBsd`, `CosmicRust`).
3. **Accessibility-First Design**: Native WCAG 2.1 AA contrast verification, Text-To-Speech (TTS) synthesis, and Grade-1 Braille translation (`src/ui/skills.rs`).
4. **GTK & Libadwaita Parity**: Native Rust implementation of modern CSD window headers (`GtkHeaderBar`), preferences pages (`AdwPreferencesPage`), action rows (`AdwActionRow`), and CSS theme providers (`src/ui/gtk.rs`).

---

## 2. Zenith Desktop Layout Presets & Window Tiling

AI agents managing desktop layouts can trigger inspiration presets using `ZenithDesktopEnvironment` (`src/desktop/zenith_advanced_features.rs`):

```rust
use sigmaos::desktop::zenith_advanced_features::{ZenithDesktopEnvironment, DesktopInspirationPreset, WindowLayoutMode};

let mut env = ZenithDesktopEnvironment::new();

// Apply Pop!_OS COSMIC-inspired Rust auto-tiling preset
env.apply_desktop_inspiration(DesktopInspirationPreset::CosmicRust);
assert_eq!(env.layout_mode, WindowLayoutMode::DynamicBSP);

// Apply FreeBSD Lumina Qt/ZFS clean desktop preset
env.apply_desktop_inspiration(DesktopInspirationPreset::LuminaBsd);
assert_eq!(env.layout_mode, WindowLayoutMode::MasterStack);
```

### Supported Layout Modes
- `Floating`: Traditional free-floating windows with titlebars (KDE Plasma / Cinnamon style).
- `Stacking`: Stacking/overlapping windows with overview workspace grid (GNOME style).
- `MasterStack`: One prominent master window with stacked secondary windows (Lumina / DWM style).
- `DynamicBSP`: Automatic binary space partitioning tiling with dynamic splits (COSMIC / Sway style).

---

## 3. GTK & Libadwaita Sovereign UI Toolkit API

Agents building system controls or settings dialogs should use the native GTK/Libadwaita abstraction layer (`src/ui/gtk.rs`):

```rust
use sigmaos::ui::gtk::{AdwPreferencesPage, AdwActionRow, GtkHeaderBar};

let mut prefs_page = AdwPreferencesPage::new("System Settings");
let mut row = AdwActionRow::new("Dark Theme", "Toggle system-wide dark mode");
row.set_active(true);
prefs_page.add_row(row);

let header_bar = GtkHeaderBar::new("Control Center");
```

---

## 4. Accessibility & WCAG Compliance Protocol

When generating or updating UI themes and color palettes, AI agents MUST verify WCAG 2.1 AA contrast compliance using `LocaleManager` (`src/ui/skills.rs`):

- **Minimum Contrast Ratio**: 4.5:1 for standard text, 3:1 for large UI text.
- **Color Validation**:
  ```rust
  use sigmaos::ui::skills::LocaleManager;
  let manager = LocaleManager::new();
  // Verify white text on black background (21:1 ratio)
  assert!(manager.validate_wcag_contrast(0xFFFFFF, 0x000000));
  ```

---

## 5. UI/UX Verification & Playwright Automated Screenshots

For frontend and visual layout changes:
1. Run Playwright UI regression tests: `./scripts/uiux_accessibility_test.sh`.
2. Generate screenshot artifacts under `build/ui_screenshots/` to verify visual consistency across themes and display scales.
3. Ensure no DOM text is reinterpreted as unescaped HTML.

---

## 6. Checklist for AI Agents Managing UX Component Changes

- [ ] Used native `src/ui/` or `src/desktop/` zero-dependency Rust components.
- [ ] Verified layout responsiveness across 1080p, 1440p, 4K, and high-DPI displays.
- [ ] Verified WCAG AA contrast ratio compliance for foreground/background colors.
- [ ] Tested layout mode switching (`Floating`, `Stacking`, `MasterStack`, `DynamicBSP`).
- [ ] Executed `./run_sigma_tests.sh` to confirm zero test regressions.
