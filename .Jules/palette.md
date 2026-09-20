## 2026-09-15 - Escape Key Dismissal for Desktop Overlays
**Learning:** Zenith Desktop modal overlays (`#cmd-palette`, `#context-menu`, `#help-overlay`) require centralized global `Escape` key event handling in `zenith_desktop/index.js` to satisfy WCAG 2.1 keyboard navigation standards.
**Action:** When adding modal or floating UI overlays in Zenith Desktop, always register `Escape` key listeners during initialization.
