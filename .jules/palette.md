# Palette 🎨 Agent Journal - UX & Accessibility Learnings

## Philosophy & Core Directives
- **Users notice the little things.**
- Accessibility is not optional.
- Every interaction should feel smooth, responsive, and clear.
- Good UX is invisible - it just works.

---

## Critical UX & Accessibility Learnings

### 2025-05-20 - Accessible Bar Widgets & Focus Indicators
**Learning:** Icon-only desktop bar widgets and system tray controls often lack accessible descriptions, preventing screen reader users from navigating system status.
**Action:** Always provide explicit `aria-label` attributes and ensure visible `:focus-visible` focus rings (2px outline) across Zenith desktop bar components and terminal widgets.

### 2025-10-09 - Transient Live Region Toast Feedback for Context Menu Actions
**Learning:** Context menu items and system operations that trigger asynchronously or silently (like purging caches, refreshing window states, or system security attestations) leave users uncertain if the action occurred. Adding a transient toast with `role="status"` and `aria-live="polite"` provides immediate visual and screen reader feedback without interrupting focus.
**Action:** Whenever implementing context menu or trigger actions, pair them with accessible toast notifications (`role="status"`, `aria-live="polite"`).
