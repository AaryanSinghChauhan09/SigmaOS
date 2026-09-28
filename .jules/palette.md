# 🎨 Palette's UX Journal

## 2026-03-31 - Accessibility Indicators for Desktop Window Compositors
**Learning:** Terminal and desktop compositor widgets lack screen reader ARIA live region updates during workspace switching, causing blind and low-vision users to lose focus context.
**Action:** Ensure all Wayland window switcher overlays and QuickRun app launcher grids trigger ARIA focus events and announce active workspace state changes.
