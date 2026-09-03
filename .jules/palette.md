# Palette's Journal - UX & Accessibility Learnings

## Philosophy
- Users notice the little things.
- Accessibility is not optional.
- Every interaction should feel smooth.
- Good UX is invisible - it just works.

## Critical Learnings

## 2025-05-18 - Desktop Shell & Terminal Accessibility Standards
**Learning:** Icon-only controls in terminal multiplexers and graphical control centers lack screen reader announcements if `aria-label` or explicit tooltip titles are omitted. Adding distinct keyboard focus indicators improves navigation efficiency for power users and screen readers alike.
**Action:** Every interactive UI component must include high-contrast focus states, explicit accessibility labels, and keyboard shortcuts.

## 2025-05-18 - Semantic Controls for Desktop Dock Icons
**Learning:** Non-semantic `<div>` elements used for icon-only dock controls are unreachable via standard keyboard navigation (Tab/Shift+Tab) and ignored by assistive technologies unless converted to semantic `<button>` elements with `type="button"`, explicit `aria-label` names, and `:focus-visible` CSS rules.
**Action:** Always replace icon-only `<div>` controls in desktop docks and toolbars with semantic `<button type="button">` elements equipped with explicit `aria-label` descriptions and hover/focus tooltip parity.
