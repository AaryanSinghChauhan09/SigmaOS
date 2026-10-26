# Palette's Journal 🎨 - UX & Accessibility Critical Insights

This journal contains ONLY critical UX and accessibility learnings, design system edge cases, and user interaction insights for SigmaOS.

---

## 2025-05-15 - ARIA Focus Indicators in Zenith Desktop QML Components
**Learning:** Custom QML/Wayland desktop widgets lacked visible focus outlines when navigated via Tab key, making keyboard-only navigation invisible to screen reader and keyboard users. Adding a 2px high-contrast `focus-visible` ring resolved accessibility issues across desktop panels.
**Action:** Ensure every interactive Zenith QML/CSS widget includes an explicit focus ring state.

## 2025-05-16 - Async Shell Command Visual Feedback
**Learning:** Shell launcher commands with > 200ms latency felt unresponsive without immediate visual feedback. Adding a subtle loading spinner animation in `sigmashell` improved perceived responsiveness significantly.
**Action:** Always provide instant micro-animations for async task triggers.
