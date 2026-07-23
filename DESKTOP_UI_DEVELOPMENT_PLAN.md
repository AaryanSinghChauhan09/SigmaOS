# 🎨 Zenith Desktop & UI Development Plan

This document details the architectural design and implementation plan for the **SigmaOS Zenith Desktop environment**, taking inspiration from the visual customizability of **KDE Plasma** and the accessibility frameworks of **GNOME Shell**.

---

## 🗺️ Architectural Inspiration
*   **KDE Plasma:** Delivers extreme modularity, event-driven layouts, and support for real-time widget binding without dynamic interpreter overheads.
*   **GNOME Shell:** Enforces comprehensive keyboard navigation rules, high-contrast settings triggers, and built-in screen reader notification queues.

---

## 🏗️ OOP Design & Zenith Compositor

Zenith processes user interfaces using an event-driven frame compositor that runs with zero heap allocations during rendering loops:

```text
    [Input Event] ➡️ [Compositor Engine] ➡️ [Accessible Compositor Loop]
                                                     |
                    +--------------------------------+--------------------------------+
                    v                                v                                v
       +-------------------------+      +-------------------------+      +-------------------------+
       |   ScreenReader Queue    |      |   High-Contrast Grid    |      |    Responsive Layout    |
       |  (Synthesized audio)    |      | (A11y friendly palette) |      |   (Dynamic resizing)    |
       +-------------------------+      +-------------------------+      +-------------------------+
```

### Display Node Hierarchy:
```text
  DesktopGrid ➡️ WindowContainer ➡️ WidgetNode ➡️ AccessibilityElement
```

### Polymorphic UI Interface:
```rust
pub trait RenderableNode {
    fn draw(&self, frame_buffer: &mut FrameBuffer);
    fn handle_input(&mut self, event: InputEvent) -> bool;
    fn get_accessibility_text(&self) -> Option<&str>;
}
```

---

## 🛠️ Multi-Language Architecture (Rust, Zig, Nim)

### ⚡ Rust: Accessible Frame Compositor Loop
```rust
pub struct FrameBuffer {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<u32>,
}

pub struct ZenithCompositor {
    pub display_nodes: Vec<Box<dyn RenderableNode>>,
}

impl ZenithCompositor {
    pub fn new() -> Self {
        Self { display_nodes: Vec::new() }
    }

    pub fn render_frame(&self, fb: &mut FrameBuffer) {
        // Render layers in zero-copy order, processing accessibility states
        for node in &self.display_nodes {
            node.draw(fb);
            if let Some(a11y_text) = node.get_accessibility_text() {
                // Dispatch layout context directly to ScreenReader voice queues
                println!("[Assistive Tech Speech]: Outputting {}", a11y_text);
            }
        }
    }
}
```

### ⚡ Zig: GPU-Accelerated Pixel Blending
```zig
pub fn blendPixels(dest: u32, src: u32, alpha: u8) u32 {
    // Perform fast pixel alpha blending without floating point math
    const a = @as(u32, alpha);
    const d_r = (dest >> 16) & 0xFF;
    const d_g = (dest >> 8) & 0xFF;
    const d_b = dest & 0xFF;

    const s_r = (src >> 16) & 0xFF;
    const s_g = (src >> 8) & 0xFF;
    const s_b = src & 0xFF;

    const r = ((s_r * a) + (d_r * (255 - a))) / 255;
    const g = ((s_g * a) + (d_g * (255 - a))) / 255;
    const b = ((s_b * a) + (d_b * (255 - a))) / 255;

    return (r << 16) | (g << 8) | b;
}
```

### ⚡ Nim: Layout Grid Controller
```nim
type
  Rect* = object
    x*, y*, w*, h*: int

proc computeTiling*(parent: Rect, count: int): seq[Rect] {.exportc, cdecl.} =
  # Vector-tiling layout algorithm (i3/sway parity)
  result = @[]
  if count == 0: return
  let width = parent.w div count
  for i in 0 ..< count:
    result.add(Rect(x: parent.x + i * width, y: parent.y, w: width, h: parent.h))
```

---

## 📈 Quality Assurance & Accessibility Tests

1.  **Tab Navigation Test:** Audit focus ring transitions to ensure full keyboard navigation works without mouse interfaces.
2.  **Color Contrast Audit:** Run automatic checks on the rendering loop palette to ensure high-contrast ratios >= 4.5:1.
