## 2025-05-17 - Web Desktop Control Accessibility and ARIA Annotations
**Learning:** In web-based OS desktops (such as Zenith), interactive inputs, theme selectors, and toolbar controls often omit explicit `type="button"`, `aria-label`, and `title` attributes, rendering them invisible or ambiguous to screen reader users and breaking standard WCAG 2.1 form navigation.
**Action:** Always ensure all interactive controls and inputs in web UI components have explicit `aria-label` descriptions, `type="button"` attributes on non-submit buttons, and visible focus indicators.

## 2026-09-04 - Accessible Web Desktop Window Tab Navigation
**Learning:** Web OS window tabs built using generic `<div>` tags lack keyboard focusability, screen reader role identification, and active tab state announcements (`aria-selected`).
**Action:** Convert window tab navigation bars into `<div role="tablist">` with `<button type="button" role="tab">` elements linking via `aria-controls` to corresponding `<div role="tabpanel">` content blocks, dynamically synchronizing `aria-selected` upon selection.

## 2026-10-15 - System High-Contrast & Forced-Colors Media Query Support
**Learning:** Hardcoded `!important` hex colors in CSS high-contrast rules override user-configured system accessibility palettes in Windows High Contrast / Forced Colors mode; using standard CSS system colors (`Canvas`, `CanvasText`, `Highlight`, `HighlightText`) alongside media query feature detection (`prefers-contrast: high` and `forced-colors: active`) ensures compliance with WCAG 2.1 Level AA without breaking custom themes.
**Action:** Use CSS system keywords (`Canvas`, `CanvasText`, `Highlight`) inside `(forced-colors: active)` media queries and auto-initialize high-contrast detection listeners on DOM ready.

## 2026-08-01 - Proactive Focus States and ARIA Roles
**Learning:** Screen readers and keyboard-only users rely entirely on interactive controls having clear focus indicators and ARIA attributes (e.g., `aria-label`, `role="button"`). Icon-only buttons with no textual context must always expose descriptive labels, and focus rings must never be hidden or completely disabled.
**Action:** Always provide explicit `aria-label` tags for all visual icon elements and retain highly-visible focus indicators.

## 2026-08-01 - Elegant Loading Feedback for Async Tasks
**Learning:** Destructive or long-running operations (like package installations or database commits) without real-time state feedback cause users to double-submit, resulting in corrupted states or multiple duplicate network/disk requests.
**Action:** Enforce clear visual and textual transitions, and disable action controls during ongoing asynchronous execution.

## 2026-08-09 - High-Contrast focus boundaries
**Learning:** Screen layout changes or color saturation variations can render normal visual indicators invisible. Enforcing dynamic boundary contrasts makes keyboard navigation accessible.
**Action:** Ensure active outline color contrasts meet WCAG 2.1 AA ratios of at least 4.5:1.

## 2026-08-10 - EndeavourOS Calamares & EOS Welcome Application Parity
**Learning:** Desktop installers and welcome applications without clear progress feedback cause user frustration. Implementing `CalamaresInstaller` and `EosWelcomeApp` with structured status messages and pastebin log diagnostics provides a seamless setup and maintenance experience.
**Action:** Ensure all installation and maintenance wizards provide explicit status messages and clear diagnostic feedback.
