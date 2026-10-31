# Palette's UX & Accessibility Journal 🎨

## 2026-10-31 - Universal Semantic ARIA Contrast & High-Visibility Terminal Focus Indicators
**Learning:** Terminal emulator windows, web status dashboards, and shell widgets frequently lack standardized ARIA live regions (`aria-live="polite"`) and focus outline tokens, leading to WCAG 2.1 AA failure states for screen-reader and keyboard-only operators.
**Action:** Always provide explicit `role="status"` and `aria-live="polite"` attributes on asynchronous terminal updates and ensure high-contrast `:focus-visible` outlines (minimum 3:1 contrast ratio) across all Zenith desktop shell web components.

## Critical UX & a11y Standards
- Explicit ARIA labels on icon-only shell buttons (`aria-label="..."`).
- Accessible color contrast ratios >= 4.5:1 for standard body text and >= 3:1 for large text / UI borders.
- Clear error messages that describe both the failure state and actionable remediation instructions.
