## 2026-09-15 - Escape Key Dismissal for Desktop Overlays
**Learning:** Zenith Desktop modal overlays (`#cmd-palette`, `#context-menu`, `#help-overlay`) require centralized global `Escape` key event handling in `zenith_desktop/index.js` to satisfy WCAG 2.1 keyboard navigation standards.
**Action:** When adding modal or floating UI overlays in Zenith Desktop, always register `Escape` key listeners during initialization.

## 2026-09-20 - Synchronized ARIA State for Keyboard-Toggled Modal Overlays
**Learning:** When toggling modal dialog overlays (such as `#cmd-palette`) via keyboard shortcuts (`Alt+Space`) and dismissal keys (`Escape`), `aria-hidden` ("false" when visible, "true" when hidden) must be synchronized on both opening and closing triggers alongside `aria-modal="true"` to prevent accessibility regressions for screen reader users on repeated toggles.
**Action:** Always update `aria-hidden` and shift focus in both toggle-open and toggle-close handlers for modal dialog components.

## 2026-09-25 - ARIA Switch Role & State Synchronization for Custom Toggle Controls
**Learning:** Custom UI toggle switches using `<input type="checkbox">` elements (e.g., inside `.toggle-switch` wrappers in Zenith Desktop settings) are exposed to assistive technology as standard checkboxes unless annotated with `role="switch"` and dynamically synchronized `aria-checked` ("true"/"false") attributes on `change` events to comply with WAI-ARIA switch patterns.
**Action:** Always assign `role="switch"` and attach change event listeners to synchronize `aria-checked` whenever introducing custom toggle controls in Zenith Desktop.

## 2026-10-01 - Standard `tabindex` Selector & WAI-ARIA Menu Navigation
**Learning:** Query selectors for interactive focusable elements must target standard `[tabindex="0"]` (avoiding typos like `[tab-index="0"]`) to ensure keyboard listeners and focus indicators attach reliably, and `role="menu"` components require Arrow key (`ArrowDown`/`ArrowUp`) and `Home`/`End` listeners for WCAG 2.1 menu keyboard navigation.
**Action:** Always include `[tabindex="0"]` in DOM query selectors for interactive controls and register `initMenuNavigation()` for `role="menu"` containers.

## 2026-10-15 - Backing UI Tooltip & ARIA Shortcut Hints with Active Global Keydown Handlers
**Learning:** Advertised keyboard shortcuts in UI tooltips (`data-tooltip`) and ARIA labels (`aria-label`, e.g., "Launch File Manager (Alt+F)") create an accessibility expectation for keyboard users that must be backed by active global keydown listeners in `zenith_desktop/index.js`, while bypassing key events when focus is inside editable form controls (`input`, `textarea`, `isContentEditable`).
**Action:** Always ensure any keyboard shortcut hints presented in UI labels/tooltips are hooked up to active global keydown handlers during desktop initialization.
