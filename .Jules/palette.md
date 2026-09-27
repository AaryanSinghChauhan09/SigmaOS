# Palette's UX Journal

## 2026-07-15 - ARIA Combobox Active Descendant and Keyboard Auto-Scroll in Command Palettes
**Learning:** In command palette comboboxes where focus remains on the input element (`#cmd-input`), screen readers rely on `aria-activedescendant` pointing to the highlighted `role="option"`'s unique ID (`cmd-option-${idx}`) and `aria-expanded` states. Visual keyboard users also require `scrollIntoView({ block: "nearest" })` on the selected option when list results exceed container height.
**Action:** When implementing combobox result lists, dynamically update `aria-activedescendant` on the input on selection change and ensure `scrollIntoView` is called for active option items.

## 2025-05-18 - Command Palette Keyboard Navigation and Listbox ARIA Attributes
**Learning:** Command center palettes that open on keyboard shortcuts (like Alt+Space) require active keyboard focus delegation (`#cmd-input`), visual selection states (`.command-item.selected`), and proper ARIA listbox roles (`role="listbox"`, `role="option"`, `aria-selected`) to ensure seamless usability for both power users and screen readers.
**Action:** Always pair command palette dialogs with dynamic result list rendering, keyboard arrow selection support, and explicit focus indicators on initial render.

## 2026-09-28 - Desktop Window Click and Focus Elevation Handlers
**Learning:** In desktop web environments with floating window containers (`.window`), handling both `mousedown` and `focusin` events is required to ensure window elevation (`zIndex`) and active focus styling (`.active-focus`) work seamlessly for both mouse clicks and keyboard tab/focus navigation into window controls or input fields.
**Action:** Always pair pointer click elevation with `focusin` event listeners when managing multi-window desktop containers.
