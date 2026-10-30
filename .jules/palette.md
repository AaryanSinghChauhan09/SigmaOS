# Palette's Journal - Critical Learnings Only

## 2026-10-27 - Keyboard Navigation Focus Indicators
**Learning:** Explicit focus-visible ring styling on custom desktop widgets improves screen reader and keyboard accessibility without visual clutter for mouse users.
**Action:** Include `focus-visible:ring-2` on all interactive Wayland/desktop controls.

## 2026-10-28 - WAI-ARIA Combobox Listbox Linking
**Learning:** Options with `role="option"` require a parent container explicitly marked with `role="listbox"` and linked to the search input via `aria-controls` for screen readers to properly announce options and option count.
**Action:** Always pair `aria-haspopup="listbox"` and `aria-controls` on combobox inputs with `role="listbox"` on the suggestions popup container.
