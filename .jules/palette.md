# Palette 🎨 Agent Journal - UX & Accessibility Learnings

## 2026-03-31 - Web UI & Desktop Control Center ARIA Compliance
**Learning:** Interactive desktop controls (such as Cinnamon Spices toggles and MintDrivers switcher in `src/ui/control_center.rs` and `web_ui/index.html`) require explicit ARIA attributes (`aria-label`, `aria-checked`, `role="switch"`) and keyboard focus rings for screen reader accessibility.
**Action:** Ensure all interactive UI elements maintain clear visual focus states (`focus-visible:ring-2`) and programmatic accessibility labels.

## 2026-03-31 - Unified Control Center Visual Feedback
**Learning:** Asynchronous operations (such as system restore point creation in Timeshift or theme switching) must provide instantaneous visual feedback or loading indicators to prevent double-clicks and confusion.
**Action:** Use inline status badges and disabled button states during ongoing configuration sync tasks.
