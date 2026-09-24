# Palette's Journal - Critical UX Learnings

## 2025-05-18 - Zenith Desktop Window Control Handlers & Accessibility
**Learning:** In desktop environments with DOM windows (like Zenith Desktop), inline `onclick` handlers on window controls (`closeWindow`, `maximizeWindow`, `launchApp`) require explicit global `window` assignment when loaded as JS modules. Toggling `display` alone without updating `aria-hidden` leaves screen readers unaware of window state changes.
**Action:** Always assign global window control handlers to `window.*` and synchronize `aria-hidden` alongside CSS class/display changes whenever opening, closing, or maximizing application windows.
