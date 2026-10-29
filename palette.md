# Palette's Journal — Critical UX & Accessibility Learnings

## 2026-04-12 - Keyboard Focus State Traps in Hybrid CLI & Zenith Desktop
**Learning:** Icon-only action buttons in desktop web UI and modal overlays lacked explicit `aria-label` attributes and focus visible ring indicators (`focus-visible:ring-2`), rendering them inaccessible to screen readers and keyboard tab navigation.
**Action:** Always wrap icon-only buttons with descriptive `aria-label` attributes and supply explicit high-contrast focus rings for all interactive elements.

## 2026-04-12 - Persona Switcher Screen Reader Live Announcements
**Learning:** Switching Linux distro persona modes (e.g., Arch, Debian, Mint, Omarchy) did not announce state changes to screen readers automatically, leaving visually impaired users unaware of persona transitions.
**Action:** Use `aria-live="polite"` regions and accessible toast notifications when dynamic desktop mode transformations occur.
