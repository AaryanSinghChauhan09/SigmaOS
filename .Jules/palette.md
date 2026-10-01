# Palette's UX Journal

## 2026-07-17 - Accessible Help Modal Overlays and Global Shortcut Guidance
**Learning:** When UI elements (like Dock icons, Context menus, or F1/? keyboard shortcuts) invoke global actions that toggle modal overlays (`#help-overlay`), ensuring the toggle function lazily populates accessible structure (`role="dialog"`, `aria-label`, `aria-modal="true"`) with clear keyboard shortcut guidance prevents broken click/shortcut handlers while delivering immediate discovery for keyboard users.
**Action:** Always verify that global modal trigger handlers exist, populate structured keyboard shortcut hints with proper ARIA dialog semantics, and automatically transfer focus to the primary action inside the dialog when opened.

## 2026-07-16 - Window Focus-to-Raise and Click-to-Raise Mouse/Keyboard Parity
**Learning:** In desktop window management environments with stacked glassmorphic windows (`.window`), mouse clicks (`mousedown`) and keyboard tab focus (`focusin`) inside child controls must both trigger window elevation (`zIndex`) and active state styling (`.active-focus`). Relying only on app launcher events leaves overlapping windows un-elevated when interacted with directly.
**Action:** Always bind both `mousedown` and `focusin` event listeners on window containers to raise window stack depth and update active focus indicators seamlessly across mouse and keyboard interactions.

## 2026-07-15 - ARIA Combobox Active Descendant and Keyboard Auto-Scroll in Command Palettes
**Learning:** In command palette comboboxes where focus remains on the input element (`#cmd-input`), screen readers rely on `aria-activedescendant` pointing to the highlighted `role="option"`'s unique ID (`cmd-option-${idx}`) and `aria-expanded` states. Visual keyboard users also require `scrollIntoView({ block: "nearest" })` on the selected option when list results exceed container height.
**Action:** When implementing combobox result lists, dynamically update `aria-activedescendant` on the input on selection change and ensure `scrollIntoView` is called for active option items.

## 2025-05-18 - Command Palette Keyboard Navigation and Listbox ARIA Attributes
**Learning:** Command center palettes that open on keyboard shortcuts (like Alt+Space) require active keyboard focus delegation (`#cmd-input`), visual selection states (`.command-item.selected`), and proper ARIA listbox roles (`role="listbox"`, `role="option"`, `aria-selected`) to ensure seamless usability for both power users and screen readers.
**Action:** Always pair command palette dialogs with dynamic result list rendering, keyboard arrow selection support, and explicit focus indicators on initial render.
