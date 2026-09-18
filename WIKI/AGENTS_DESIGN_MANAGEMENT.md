# AI Agent Design & UI/UX Management Specification for SigmaOS

This document provides specifications and guidelines for AI agents working on design management, desktop UI/UX, accessibility, and visual assets within the **SigmaOS** operating system.

---

## 1. UI/UX & Zenith Desktop Architecture

SigmaOS features the **Zenith Desktop Environment**, driven by lightweight, sovereign web and native rendering engines:

- **Desktop Shell**: `zenith_desktop/` (`index.html`, `zenith_desktop.css`, `dashboard/`)
- **Control Center / Settings**: Integrated with `DeepinDdeControlCenterEngine` and `Yast2ModulePlug`.
- **Dynamic Theming**: Live theme switching supported via `OmarchySystemThemeStudio` (`src/distro/omarchy_inspiration.rs`).
- **Compositing**: Wayland/DRM atomic modesetting (`DrmModeInfo`) and `SteamOsGamescopeCompositorEngine` (FSR upscaling & FPS limiting).

---

## 2. CSS & Design Tokens Conventions

1. **CSS Variables & Color Palettes**:
   - Primary theme tokens defined in `zenith_desktop.css`:
     - `--bg-primary`, `--bg-secondary`, `--accent-color`, `--text-primary`, `--border-color`.
   - Maintain high contrast ratios (WCAG 2.1 AA compliant) for readability across dark and light modes.

2. **Typography & Layout**:
   - Use system sans-serif fonts with fallback stacks (`Inter`, `system-ui`, `-apple-system`, `sans-serif`).
   - Use CSS Grid and Flexbox for responsive desktop layouts across varying display resolutions.

---

## 3. Accessibility & Visual Verification Protocols

1. **Accessibility (a11y) Standards**:
   - Ensure all interactive elements include proper ARIA attributes (`aria-label`, `role`, `tabindex`).
   - Support keyboard navigation (`Tab`, `Enter`, `Escape`, arrow keys) for all desktop widgets and windows.
   - Run accessibility test scripts when present (`./scripts/uiux_accessibility_test.sh`).

2. **Frontend Visual Verification Workflow**:
   - When introducing UI changes, capture screenshots or Playwright recordings using frontend verification instructions.
   - Verify visual rendering across display modes (light/dark) and high-DPI scaling factors.

---

## 4. Testing & Verification Commands

```bash
# Run UI/UX accessibility test script if present
./scripts/uiux_accessibility_test.sh

# Run full test runner
./run_sigma_tests.sh
```
