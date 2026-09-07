# AI Agent Design & UI/UX Management Specification for SigmaOS

This document provides specifications and guidelines for AI agents working on design management, desktop UI/UX, accessibility, and visual assets within the **SigmaOS** operating system.

---

## 1. Primary Strategy: Reducing Dependency on External CSS

SigmaOS prioritizes **native Rust UI toolkit widgets and compiled programmatic style engines** (`src/ui/gtk_toolkit.rs`, `src/ui/gtk.rs`) over reliance on raw, external CSS stylesheets.

- **Native Programmatic UI Toolkits**: Use `SovereignGtkToolkit`, `GtkSignalDispatcher`, and `GtkCssProvider` in `src/ui/gtk_toolkit.rs` to construct UI hierarchies (`GtkHeaderBar`, `AdwActionRow`, `AdwPreferencesGroup`, `AdwPreferencesPage`, `GtkBox`, `GtkButton`).
- **Native Desktop Shell & Dock**: `SovereignSystemStatusPanel` and `SovereignDockBar` handle top-bar applets and desktop dock items programmatically rather than depending on web-based CSS styling rules.
- **Dynamic Native Theme Engine**: System themes and accent color switches are managed via `OmarchySystemThemeStudio` (`src/distro/omarchy_inspiration.rs`) and `ThemeEngine` (`src/customization/theme.rs`).
- **Legacy CSS Minimization**: External CSS (`zenith_desktop.css`, `web_ui/styles/style.css`) is kept strictly to a minimum for legacy web shell rendering, with new UI components built natively in Rust.

---

## 2. Native Style Tokens & Widget Conventions

1. **Native Color Palettes & Tokens**:
   - Use `SovereignCssColorEngine` (`src/customization/theme.rs`) for native hex color parsing and RGBA conversion.
   - Maintain high contrast ratios (WCAG 2.1 Level AA compliant) across light and dark theme modes (`AdwColorScheme`).

2. **Programmatic Layouts**:
   - Prefer native container layouts (`GtkBox`, `GtkHeaderBar`, `AdwPreferencesGroup`) with explicit orientation (`GtkOrientation::Horizontal`, `GtkOrientation::Vertical`) over external CSS Grid/Flexbox stylesheets.

---

## 3. Accessibility & Visual Verification Protocols

1. **Accessibility (a11y) Standards**:
   - Ensure all interactive native widgets support keyboard focus (`focus-visible`) and signal emission (`g_signal_emit`).
   - Run accessibility test scripts when present (`./scripts/uiux_accessibility_test.sh`).

2. **Frontend Visual Verification Workflow**:
   - When introducing UI changes, capture screenshots or Playwright recordings using frontend verification instructions.
   - Verify visual rendering across display modes (light/dark) and high-DPI scaling factors.

---

## 4. Testing & Verification Commands

```bash
# Run atomic Rust UI toolkit unit tests
rustc --test --edition=2021 src/ui/gtk_toolkit.rs -o build/test_gtk_toolkit && ./build/test_gtk_toolkit

# Run UI/UX accessibility test script if present
./scripts/uiux_accessibility_test.sh

# Run full test runner
./run_sigma_tests.sh
```
