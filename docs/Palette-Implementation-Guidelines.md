# Palette Implementation Guidelines

This document contains UX and implementation learnings and actions for the SigmaOS desktop environment, specifically focused on accessibility, CLI design, compositor optimization, and UI layout patterns.

## 2026-07-12 - Zenith Desktop High Contrast and Keyboard Focus Indicators

**Learning:** Keyboard navigation (WCAG 2.1 Level AA) requires highly visible focus indicators (`:focus-visible`) to distinguish focused controls from surrounding elements. In glassmorphic UIs with transparent borders and dark background colors, default focus states may have insufficient color contrast. Explicitly defining custom `outline` and `box-shadow` properties on focused interactive elements ensures clarity and visual contrast, especially under high-contrast modes.

**Action:** Always add high-contrast `:focus-visible` styles with fallback support for `high-contrast-active` bodies to ensure inclusive designs.

## 2026-07-14 - Delightful CLI Empty States and Actionable Call-To-Actions

**Learning:** When queries or search terms return empty lists on CLI tools, users are often left confused about their next action or input validity. Providing a color-coded warning message with a clear "Protip" suggestions drastically reduces friction and guides them seamlessly.

**Action:** Always include delightful empty states with descriptive tips or actionable suggestions to help the user resolve the query.

## 2026-07-14 - Compositor Damage Tracking for Smooth Visual Updates

**Learning:** In `sigma_compositor`, blindly redrawing the entire screen on every frame is an enormous waste of GPU cycles. By tracking exactly which surface IDs have been "damaged" (changed), the compositor can limit redraws to only the affected regions. This is the same principle behind Wayland's `wl_surface.damage_buffer` — a `HashSet<u64>` of damaged surface IDs that gets drained on each frame commit reduces power consumption and increases perceived smoothness, especially on battery-powered devices.

**Action:** Always implement damage tracking in compositors. Use `take_damage()` to atomically drain the set and only composite the returned surfaces.

## 2026-07-14 - Predictable Spatial Models in Desktop Panels

**Learning:** When developing the `sigma_desktop` Dash and Panel, scattering system tray icons and application launchers haphazardly reduces spatial memory retention. The cognitive load required to find the network icon increases if it shifts position when a notification pops up. By adopting fixed spatial anchoring (e.g., launchers always left-aligned, clock always centered, tray always right-aligned), users can rely on muscle memory.

**Action:** Strictly enforce flexbox-style fixed alignments (Start, Center, End) in the `Panel` UI layout engine. Do not allow elements in one anchor group to displace elements in another group.

## 2026-07-14 - Neural UI Asynchronous Loading States

**Learning:** Integrating `sigma_ai` directly into `sigma_desktop` creates UI latency if inference blocks the main thread. "Smart" layout predictions take hundreds of milliseconds to compute. If the compositor halts frame rendering waiting for the AI backend, the entire OS feels sluggish.

**Action:** Isolate AI inference to a background thread and use placeholder skeleton UI elements (shimmer effects) in `sigma_desktop` while waiting for the model backend. Only swap the layout when the asynchronous RPC call returns.
