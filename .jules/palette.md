## 2025-05-17 - Web Desktop Control Accessibility and ARIA Annotations
**Learning:** In web-based OS desktops (such as Zenith), interactive inputs, theme selectors, and toolbar controls often omit explicit `type="button"`, `aria-label`, and `title` attributes, rendering them invisible or ambiguous to screen reader users and breaking standard WCAG 2.1 form navigation.
**Action:** Always ensure all interactive controls and inputs in web UI components have explicit `aria-label` descriptions, `type="button"` attributes on non-submit buttons, and visible focus indicators.

## 2026-09-04 - Accessible Web Desktop Window Tab Navigation
**Learning:** Web OS window tabs built using generic `<div>` tags lack keyboard focusability, screen reader role identification, and active tab state announcements (`aria-selected`).
**Action:** Convert window tab navigation bars into `<div role="tablist">` with `<button type="button" role="tab">` elements linking via `aria-controls` to corresponding `<div role="tabpanel">` content blocks, dynamically synchronizing `aria-selected` upon selection.

## 2026-09-05 - Accessible Web Desktop Context Menu Navigation
**Learning:** Right-click context menus built with generic `<div>` items are inaccessible to screen readers and keyboard users because they lack `role="menu"`, `role="menuitem"`, `role="separator"`, button semantics, and focus-visible indicators.
**Action:** Ensure context menu containers have `role="menu"` and `aria-label`, items use `<button type="button" role="menuitem">` with full-width CSS reset and `:focus-visible` highlights, and menu dividers use `role="separator"`.
