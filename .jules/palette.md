## 2025-05-17 - Web Desktop Control Accessibility and ARIA Annotations
**Learning:** In web-based OS desktops (such as Zenith), interactive inputs, theme selectors, and toolbar controls often omit explicit `type="button"`, `aria-label`, and `title` attributes, rendering them invisible or ambiguous to screen reader users and breaking standard WCAG 2.1 form navigation.
**Action:** Always ensure all interactive controls and inputs in web UI components have explicit `aria-label` descriptions, `type="button"` attributes on non-submit buttons, and visible focus indicators.

## 2026-09-04 - Accessible Web Desktop Window Tab Navigation
**Learning:** Web OS window tabs built using generic `<div>` tags lack keyboard focusability, screen reader role identification, and active tab state announcements (`aria-selected`).
**Action:** Convert window tab navigation bars into `<div role="tablist">` with `<button type="button" role="tab">` elements linking via `aria-controls` to corresponding `<div role="tabpanel">` content blocks, dynamically synchronizing `aria-selected` upon selection.

## 2026-10-15 - System High-Contrast & Forced-Colors Media Query Support
**Learning:** Hardcoded `!important` hex colors in CSS high-contrast rules override user-configured system accessibility palettes in Windows High Contrast / Forced Colors mode; using standard CSS system colors (`Canvas`, `CanvasText`, `Highlight`, `HighlightText`) alongside media query feature detection (`prefers-contrast: high` and `forced-colors: active`) ensures compliance with WCAG 2.1 Level AA without breaking custom themes.
**Action:** Use CSS system keywords (`Canvas`, `CanvasText`, `Highlight`) inside `(forced-colors: active)` media queries and auto-initialize high-contrast detection listeners on DOM ready.

## 2026-11-20 - Desktop Right-Click Context Menu Accessibility & Keyboard Focus States
**Learning:** Web OS desktop context menus constructed with non-semantic `<div>` elements prevent screen readers from identifying menu items (`role="menu"` / `role="menuitem"`) and block keyboard navigation because `<div>` elements do not receive default focus or `:focus-visible` styling without explicit button resets and outline indicators.
**Action:** Structure context menus with `<div role="menu">` containers, `<button type="button" class="context-item" role="menuitem">` options, `<div role="separator">` dividers, explicit `aria-label` attributes, and CSS reset rules with `:focus-visible` outline indicators.
