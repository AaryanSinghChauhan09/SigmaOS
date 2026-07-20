# 🎨 Zenith Desktop: Sovereign UI Compositor & Theming Plan

This plan details the architecture and roadmap for **Zenith Desktop**, SigmaOS’s bare-metal, high-performance UI compositor. Zenith Desktop abstracts rendering engines, window managers, and accessibility layers, delivering fluid 120 FPS window tiling and adaptive theming with zero dependencies.

---

## 1. Feature Ingestion & Architectural Parity

Zenith Desktop absorbs elite interface designs from modern systems and maps them to clean OOP layers:

*   **Tiling Window Management (inspired by i3/Sway):** Bypasses heavy X11 server overhead by executing lock-free binary-tree window partitioning directly in raw framebuffer buffers.
*   **Declarative Theming (inspired by NixOS/Svelte):** Window borders, spacing, effects, and text scaling are mapped dynamically to a single declarative, immutable settings graph.
*   **Zero-Allocation Accessibility (inspired by macOS VoiceOver):** Accessibility buffers use zero-copy string references (`.map(|s| s.as_str()).unwrap_or("")`) to avoid compositor jank.

---

## 2. Desktop Core Architecture (ZenithCompositor)

The compositor operates on an object-oriented state machine, maintaining private bounds for all window buffers and rendering them polymorphically.

```
       +---------------------------------------------+
       |             ZenithCompositor                |
       +---------------------------------------------+
       | - windows : Vec<Window>                     |
       | - active_profile : Option<String>           |
       | - active_theme : String                     |
       +---------------------------------------------+
                              |
                     [Polymorphic Layout]
                              v
       +---------------------------------------------+
       |            <<Interface>> Layout             |
       +---------------------------------------------+
       | + arrange_windows(windows, width, height)   |
       +---------------------------------------------+
            ^                 ^                 ^
            |                 |                 |
     [TilingLayout]    [FloatingLayout]   [TabbedLayout]
```

---

## 3. Implementation Roadmap

1.  **Phase 1: Binary Tree Tiling Engine (Milestone 1)**
    *   Implement binary-partition window tiling algorithms inside `WIKI/ZenithDesktop.md`.
    *   Expose safe bounds verification to prevent window geometry overflows.
2.  **Phase 2: Declarative Multi-Theme Engines (Milestone 2)**
    *   Write JSON-style parser definitions for runtime color-scheme switches.
    *   Integrate time-based auto-switching loops inside the visual composer.
3.  **Phase 3: Screen Reading & Accessibility Hook (Milestone 3)**
    *   Implement high-speed screen reader coordinate translation arrays.
    *   Conduct visual verification checking keyboard-focus outlines.
