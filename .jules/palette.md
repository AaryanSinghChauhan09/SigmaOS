# Palette's Journal

## 2026-10-03 - High-Contrast Keyboard Focus Indicators & Live ARIA Telemetry in Desktop UI
**Learning:** In desktop compositors and visual installers (`OmarchyZenithDesktop` and `tools/installer/installer.qml`), custom UI widgets often lack explicit focus ring states and ARIA live region notifications for dynamic status changes (e.g. system update progress or mirror ranking). Adding standard 2px focus-visible rings and `aria-live="polite"` attributes ensures complete accessibility for screen readers and keyboard navigation.
**Action:** Always include visible focus states and accessible ARIA attributes when designing or refactoring desktop shell widgets and installation UI components.
