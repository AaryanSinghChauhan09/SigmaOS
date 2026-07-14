## 2026-07-12 - [Zenith Desktop High Contrast and Keyboard Focus Indicators]
**Learning:** Keyboard navigation (WCAG 2.1 Level AA) requires highly visible focus indicators (`:focus-visible`) to distinguish focused controls from surrounding elements. In glassmorphic UIs with transparent borders and dark background colors, default focus states may have insufficient color contrast. Explicitly defining custom `outline` and `box-shadow` properties on focused interactive elements ensures clarity and visual contrast, especially under high-contrast modes.
**Action:** Always add high-contrast `:focus-visible` styles with fallback support for `high-contrast-active` bodies to ensure inclusive designs.

## 2026-07-14 - Delightful CLI Empty States and Actionable Call-To-Actions
**Learning:** When queries or search terms return empty lists on CLI tools, users are often left confused about their next action or input validity. Providing a color-coded warning message with a clear "Protip" suggestions drastically reduces friction and guides them seamlessly.
**Action:** Always include delightful empty states with descriptive tips or actionable suggestions to help the user resolve the query.
