# OSS Absorption: SerenityOS — Desktop & Browser Integration

> **Status**: 🔄 Active | **Source Project**: SerenityOS (Andreas Kling) | **Target Shard**: `SigmaOS Graphical Desktop Environment`

---

## 1. Executive Summary

SerenityOS is a custom Unix-like operating system built from scratch in C++, famous for its cohesive, classic 90s-style desktop environment and its ground-up web browser (Ladybird/LibWeb).

SigmaOS absorbs the **design philosophy of extreme cohesiveness** and specifically forks and adapts the **Ladybird web engine** as a Sovereign Sandbox component, providing a modern, independent browser engine that doesn't rely on Chromium or Gecko, ensuring complete supply-chain independence.

---

## 2. Key Features Absorbed

### 2.1 Cohesive Desktop Framework (`sigma-gui`)

Unlike standard Linux where Qt, GTK, and EFL applications all look and behave differently, SigmaOS enforces a unified toolkit (`libsigma-ui`) inspired by SerenityOS's `LibGUI`.

```rust
// userland/libsigma-ui/widget.rs
// SPDX-License-Identifier: MIT

pub trait Widget {
    fn paint(&self, ctx: &mut PaintContext);
    fn handle_event(&mut self, event: UIEvent) -> EventResult;
    
    // Enforced global styling
    fn theme(&self) -> &Theme {
        SigmaTheme::current()
    }
}
```

### 2.2 Ladybird Web Engine Integration

SigmaOS integrates a Rust-hardened variant of the Ladybird engine for embedded web views and the default `sigma-browser`.

```bash
# Launch sovereign browser
$ sigma run sigma-browser https://sigmaos.org
Σ [BROWSER] Using Ladybird-derived LibWeb engine
  Sandboxed: YES (Landlock + seccomp)
  Engine   : LibWeb (SigmaOS variant)
  JS       : LibJS (JIT disabled for security)
```

---

## 3. Architecture Comparison

| Component | Linux Standard | SerenityOS | SigmaOS |
|:----------|:---------------|:-----------|:--------|
| UI Toolkit | Qt, GTK, FLTK | LibGUI | libsigma-ui |
| Compositor | Mutter, KWin | WindowServer | Zenith (Wayland) |
| Web Engine | Blink, Gecko | LibWeb | LibWeb (sandboxed) |
| Audio | PulseAudio | AudioServer | PipeWire |

---

## 4. References & Standards

- SerenityOS — `serenityos.org` (BSD-2-Clause)
- Ladybird Browser — `ladybird.dev` (BSD-2-Clause)
