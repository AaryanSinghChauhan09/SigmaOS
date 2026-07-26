# 🎨 Wayland Zenith Desktop & Compositor Specification (`Zenith`)

This specification details the architecture, rendering loop, accessibility pipelines, and adaptive window behaviors of the **Zenith Desktop Compositor** (`zenith_desktop`) for SigmaOS.

Drawing inspiration from high-efficiency modern compositors like **Sway/Weston (wlroots)** and **Android SurfaceFlinger**, the Zenith compositor is a zero-dependency, hard real-time desktop environment built entirely with Object-Oriented Programming (OOP) principles and user-defined functions in Rust, Zig, and Nim.

---

## 🗺️ Zenith UI System Architecture

```
                    ┌────────────────────────────────────────┐
                    │      Zenith Wayland Input Listener     │
                    └───────────────────┬────────────────────┘
                                        │ (Pointer/Keyboard Events)
                    ┌───────────────────▼────────────────────┐
                    │   Interactive Window Tree (V-Layout)   │
                    └───────────────────┬────────────────────┘
                                        │ (DMA-buf Surface Blending)
                    ┌───────────────────▼────────────────────┐
                    │      S-Audio Voice Buffer Engine       │
                    └───────────────────┬────────────────────┘
                                        │ (Hardware Screen Reader Queue)
                    ┌───────────────────▼────────────────────┐
                    │     Vulkan/VESA Compositor Renderer    │
                    └────────────────────────────────────────┘
```

---

## 1. Zero-Dependency OOP Rust Specification (Compositor Loop)

The core compositor manages screen geometry, surface buffers, active input focal points, and window layers without external graphics library runtime utility blocks.

```rust
pub const SCREEN_WIDTH: u32 = 1920;
pub const SCREEN_HEIGHT: u32 = 1080;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowState {
    Active,
    Inactive,
    Minimized,
}

pub struct Geometry {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

pub struct WindowNode {
    pub id: u32,
    pub title: &'static str,
    pub geometry: Geometry,
    pub state: WindowState,
}

impl WindowNode {
    pub fn new(id: u32, title: &'static str, x: i32, y: i32, w: u32, h: u32) -> Self {
        Self {
            id,
            title,
            geometry: Geometry { x, y, width: w, height: h },
            state: WindowState::Inactive,
        }
    }

    pub fn resize(&mut self, w: u32, h: u32) {
        self.geometry.width = w;
        self.geometry.height = h;
    }

    pub fn move_to(&mut self, x: i32, y: i32) {
        self.geometry.x = x;
        self.geometry.y = y;
    }
}

pub struct ZenithCompositor {
    windows: [Option<WindowNode>; 32],
    active_window_id: Option<u32>,
}

impl ZenithCompositor {
    pub fn new() -> Self {
        const NONE_WINDOW: Option<WindowNode> = None;
        Self {
            windows: [NONE_WINDOW; 32],
            active_window_id: None,
        }
    }

    pub fn register_window(&mut self, window: WindowNode) -> Result<(), &'static str> {
        for slot in self.windows.iter_mut() {
            if slot.is_none() {
                *slot = Some(window);
                return Ok(());
            }
        }
        Err("Maximum active window boundary exceeded")
    }

    pub fn focus_window(&mut self, id: u32) {
        self.active_window_id = Some(id);
        for slot in self.windows.iter_mut() {
            if let Some(ref mut window) = slot {
                if window.id == id {
                    window.state = WindowState::Active;
                } else {
                    window.state = WindowState::Inactive;
                }
            }
        }
    }

    pub fn render_frame(&self, frame_buffer: &mut [u32]) {
        // Clear frame buffer to background color (0x000F172A - Tailwind Slate 900)
        for pixel in frame_buffer.iter_mut() {
            *pixel = 0x000F172A;
        }

        // Composite windows back-to-front (simple painter's algorithm)
        for slot in self.windows.iter() {
            if let Some(ref window) = slot {
                if window.state != WindowState::Minimized {
                    let g = &window.geometry;
                    for row in 0..g.height {
                        let y_coord = g.y + row as i32;
                        if y_coord >= 0 && y_coord < SCREEN_HEIGHT as i32 {
                            for col in 0..g.width {
                                let x_coord = g.x + col as i32;
                                if x_coord >= 0 && x_coord < SCREEN_WIDTH as i32 {
                                    let idx = (y_coord as usize * SCREEN_WIDTH as usize) + x_coord as usize;
                                    if idx < frame_buffer.len() {
                                        // Active windows rendered with bright primary borders
                                        if window.state == WindowState::Active {
                                            frame_buffer[idx] = 0x003B82F6; // Blue primary border
                                        } else {
                                            frame_buffer[idx] = 0x00475569; // Dim borders
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
```

---

## 2. Zero-Dependency OOP Zig Specification (Input & DMA Events)

Handles asynchronous hardware input events and coordinates DMA frame switches directly without calling libc or external input wrappers.

```zig
const std = @import("std");

pub const MouseEvent = struct {
    dx: i32,
    dy: i32,
    buttons: u8,
};

pub const KeyboardEvent = struct {
    scancode: u8,
    is_pressed: bool,
};

pub const HardwareEvent = union(enum) {
    mouse: MouseEvent,
    keyboard: KeyboardEvent,
};

pub const ZenithInputRouter = struct {
    cursor_x: i32,
    cursor_y: i32,
    click_state: u8,

    pub fn init() ZenithInputRouter {
        return ZenithInputRouter{
            .cursor_x = 960,
            .cursor_y = 540,
            .click_state = 0,
        };
    }

    pub fn dispatchEvent(self: *ZenithInputRouter, event: HardwareEvent) void {
        switch (event) {
            .mouse => |m| {
                self.cursor_x = @max(0, @min(1920, self.cursor_x + m.dx));
                self.cursor_y = @max(0, @min(1080, self.cursor_y + m.dy));
                self.click_state = m.buttons;
            },
            .keyboard => |k| {
                _ = k; // Handle hardware scancode translations directly
            },
        }
    }
};
```

---

## 3. Zero-Dependency OOP Nim Specification (Accessibility Screen Reader)

Exposes interactive voice buffer arrays and handles high-contrast display profiles natively inside userland translation boundaries.

```nim
type
  ScreenReaderVoice* = object
    pitch*: float32
    speed*: float32
    active*: bool

  AccessibilityChannel* = ref object of RootObj
    voiceConfig*: ScreenReaderVoice
    narratorBuffer*: array[128, string]
    nextNarratorIndex*: int

method announceText*(self: AccessibilityChannel, text: string) {.base.} =
  if self.voiceConfig.active and self.nextNarratorIndex < 128:
    self.narratorBuffer[self.nextNarratorIndex] = text
    self.nextNarratorIndex += 1

method clearNarratorQueue*(self: AccessibilityChannel) {.base.} =
  self.nextNarratorIndex = 0
  for i in 0 ..< 128:
    self.narratorBuffer[i] = ""

proc newAccessibilityChannel*(pitch: float32, speed: float32): AccessibilityChannel =
  new(result)
  result.voiceConfig = ScreenReaderVoice(pitch: pitch, speed: speed, active: true)
  result.nextNarratorIndex = 0
```

---

## 🔄 Synchronization & Performance Checklist

To secure ultimate frontend performance compared to Linux Mint:
1.  **Zero-Allocation Paint Pools:** Guarantee all buttons and panels draw textures to framebuffers without runtime page allocation.
2.  **DMA Buffer Swaps:** Double-buffered canvases must execute pointer swaps rather than physical memory copies inside visual loops.
3.  **Strict Context Sandbox:** Ensure that the interactive compositor executes under a dedicated `sigma_pledge` containing zero file-write permissions, protecting screen memory boundaries from host exploit leakages.
