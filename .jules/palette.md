# Palette's Journal 🎨

## 2026-10-23 - High-Contrast Focus Visible Rings and ARIA Attributes in Hybrid Terminal UI
**Learning:** Hybrid desktop/terminal components (like Ghostty, Waybar, and Quickshell widgets in Omarchy/Mint) require explicit `focus-visible` outline rings and `aria-label` screen reader bindings to maintain WCAG 2.1 AA compliance across desktop and CLI entrypoints.
**Action:** Always provide explicit keyboard navigation focus states (`focus-visible:ring-2`) and descriptive ARIA labels for icon-only action triggers in hybrid UI widgets.
