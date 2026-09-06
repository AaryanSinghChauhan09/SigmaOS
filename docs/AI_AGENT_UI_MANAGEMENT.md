# AI Agent UI Management Architecture in SigmaOS

This document specifies UI management directives, accessibility standards, and compositor architectures for AI agents (such as "Palette") modifying or extending the SigmaOS user interface.

---

## 🎨 1. Desktop UI Architecture

SigmaOS provides a multi-desktop compositor framework written natively in Rust and WebAssembly:

```
+-----------------------------------------------------------------------+
| Zenith Desktop Compositor (zenith_desktop/ & src/desktop/)             |
| Native Rust/WASM UI Engine (NativeWasmDesktopEngine)                  |
+-----------------------------------------------------------------------+
| Window Management & Layouts                                           |
| Master-and-Stack Tiling, Cascaded Stacking, Tabbed & Floating Modes   |
+-----------------------------------------------------------------------+
| Styling & Accessibility Layer                                         |
| CSS Variables (zenith_desktop.css), ARIA Labels, Focus Indicators     |
+-----------------------------------------------------------------------+
```

---

## ♿ 2. Accessibility & Interaction Guidelines

1. **Accessible Interactive Controls**
   - Icon-only buttons **must** always specify `aria-label` or `set_aria_label("element-id", "Label")`.
   - Interactive elements must support both `Enter` and `Space` key activations (`handle_keydown`).

2. **Keyboard Navigation & Focus Indicators**
   - Ensure explicit focus states (`:focus-visible` or `.keyboard-focus` class).
   - Never suppress focus outlines without providing a high-contrast replacement focus indicator.

3. **DOM Security (XSS Protection)**
   - Use `set_secure_text_content` (or `textContent` in DOM bindings) instead of `innerHTML` when rendering user-supplied strings or dynamic titles.

4. **CSS Design Tokens (`zenith_desktop.css`)**
   - Use established CSS custom properties for themes:
     - `--accent-gold`
     - `--accent-blue`
     - `--accent-cyan`
     - `--accent-cyan-glow`

---

## ⚙️ 3. Verification Commands for UI Agents

- **Native Desktop & WASM UI Unit Tests:**
  `rustc --test src/desktop/web_wasm_bridge.rs --edition=2021 -o build/wasm_bridge_test && ./build/wasm_bridge_test`
- **Compositor Unit Tests:**
  `cargo test --lib -- desktop::zenith_compositor`
