## 2025-05-17 - Web Desktop Control Accessibility and ARIA Annotations

**Learning:** In web-based OS desktops (such as Zenith), interactive inputs, theme selectors, and toolbar controls often omit explicit `type="button"`, `aria-label`, and `title` attributes, rendering them invisible or ambiguous to screen reader users and breaking standard WCAG 2.1 form navigation.
**Action:** Always ensure all interactive controls and inputs in web UI components have explicit `aria-label` descriptions, `type="button"` attributes on non-submit buttons, and visible focus indicators.
