# Palette's UX Journal

## 2025-05-18 - Command Palette Keyboard Navigation and Listbox ARIA Attributes
**Learning:** Command center palettes that open on keyboard shortcuts (like Alt+Space) require active keyboard focus delegation (`#cmd-input`), visual selection states (`.command-item.selected`), and proper ARIA listbox roles (`role="listbox"`, `role="option"`, `aria-selected`) to ensure seamless usability for both power users and screen readers.
**Action:** Always pair command palette dialogs with dynamic result list rendering, keyboard arrow selection support, and explicit focus indicators on initial render.
