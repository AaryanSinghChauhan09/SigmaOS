## 2026-09-02 - Accessible Window Controls in Web Desktop Interfaces

**Learning:** Custom window controls (e.g. minimize, maximize, close dots) in web-based desktop operating systems are frequently styled as simple `div` or `span` tags, making them invisible to keyboard focus traps and screen readers. Converting these to semantic `<button type="button">` elements with explicit `aria-label` and `title` attributes (e.g., `aria-label="Minimize OmniShell Terminal"`) alongside CSS reset rules (`border: none; padding: 0; outline: none`) preserves exact visual aesthetics while making window management WCAG 2.1 AA compliant.

**Action:** Whenever building custom window titlebars or desktop controls, always use native `<button>` tags with explicit, context-aware `aria-label`s and `:focus-visible` focus rings.
