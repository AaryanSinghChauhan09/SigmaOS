## 2026-09-15 - Escape Key Dismissal for Desktop Overlays
**Learning:** Zenith Desktop modal overlays (`#cmd-palette`, `#context-menu`, `#help-overlay`) require centralized global `Escape` key event handling in `zenith_desktop/index.js` to satisfy WCAG 2.1 keyboard navigation standards.
**Action:** When adding modal or floating UI overlays in Zenith Desktop, always register `Escape` key listeners during initialization.

## 2026-09-20 - Synchronized ARIA State for Keyboard-Toggled Modal Overlays
**Learning:** When toggling modal dialog overlays (such as `#cmd-palette`) via keyboard shortcuts (`Alt+Space`) and dismissal keys (`Escape`), `aria-hidden` ("false" when visible, "true" when hidden) must be synchronized on both opening and closing triggers alongside `aria-modal="true"` to prevent accessibility regressions for screen reader users on repeated toggles.
**Action:** Always update `aria-hidden` and shift focus in both toggle-open and toggle-close handlers for modal dialog components.
