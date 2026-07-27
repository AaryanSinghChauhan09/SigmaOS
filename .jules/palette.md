# 🎨 Palette's Journal — SigmaOS UX & Accessibility Polish

This journal captures CRITICAL accessibility (a11y) and user experience (UX) insights obtained while developing and auditing the Zenon/Zenith desktop compositors and global settings framework in SigmaOS.

---

## 2024-07-15 - Zero-Allocation Configuration Routing for Accessibility Features
**Learning:** Using temporary heap allocations (such as `unwrap_or(&String::new())`) inside accessibility triggers causes frequent, short-lived heap allocations during window compositing and input processing. Replacing these with `.map(|s| s.as_str()).unwrap_or("")` completely avoids allocations, allows visual modes to evaluate layout configurations in zero-copy speed, and prevents compositor micro-stutter (jank).
**Action:** Always keep user preference evaluation and theme rendering loops free of temporary allocations to maintain a fluid 120 FPS desktop experience.

## 2024-07-15 - Global Hash Map Keys for Screen Readers and Assistive Technologies
**Learning:** Using raw strings for accessibility features like Screen Readers, High Contrast, and Magnifiers leads to typo-prone, fragile settings routing. Standardizing settings using structured Rust enums (e.g., `AccessibilityFeature`) deriving `Hash + Eq + Copy` guarantees compile-time validation, enables instant key lookups in global hash maps, and ensures seamless integrations with audio and haptic drivers.
**Action:** Always wrap assistive technology config identifiers in Copy-safe enums to prevent lookup failures and guarantee flawless accessibility fallback layers.

## 2026-07-20 - Multi-Language Accessibility Support and Auto-Theming Contrasts
**Learning:** Dynamic time-based color switching in visual compositors can introduce layout recalculation costs if screen boundaries or contrast ratios are re-evaluated synchronously. Separating theme loading into a pre-compiled layout structure and adjusting contrast ratios dynamically using simple bitwise pixel color maps reduces visual jitter and guarantees consistent focus styling.
**Action:** Always pre-allocate style constraints for time-based screen contrast switching to prevent layout jank.

## 2025-10-24 - [Legacy Windowing Protocol Bottlenecks]
**Learning:** Traditional window display architectures (e.g., X11, Wayland) introduce latency overhead due to heavy event marshaling and context switches.
**Action:** Direct frame buffers coupled with a dedicated lightweight Vulkan compositing thread achieve latency parity with dedicated gaming consoles.
