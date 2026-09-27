# Palette's Journal

## 2026-07-20 - [A11y ARIA Focus Trap & Keyboard Navigation in Zenith Desktop Compositor]
**Learning:** In terminal and desktop compositor interfaces, icon buttons and interactive status widgets frequently lack explicit `aria-label` attributes and keyboard focus indicators, rendering screen readers and keyboard-only users unable to navigate window groups.
**Action:** Ensure all interactive widgets have explicit visual hover states, keyboard `focus-visible` outlines, and descriptive ARIA labels (`aria-label`, `role="button"`).
