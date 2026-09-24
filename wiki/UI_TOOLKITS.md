# SigmaOS UI Toolkits Architecture & Programming Guide

## 1. Overview

To empower application developers to create high-performance native graphical applications, SigmaOS provides a native UI toolkit architecture (`Zenith UI Framework`) alongside full binary compatibility bridges for established open-source toolkits like GTK 4 and Qt 6.

```
+------------------------------------------------------------------+
|                      SigmaOS Application Layer                   |
|  +--------------------+  +------------------+  +---------------+ |
|  | Sovereign Rust App |  | GTK4 / C App     |  | Qt6 / C++ App | |
|  +---------+----------+  +--------+---------+  +-------+-------+ |
+------------|----------------------|--------------------|---------+
             |                      |                    |
+------------v----------------------v--------------------v---------+
|                    UI Toolkit Abstraction Layer                  |
|  +-------------------------------------------------------------+ |
|  | Zenith UI Native Engine (Rust declarative widget tree)      | |
|  | GTK4 / Wayland Target Backend Bridge                        | |
|  | Qt6 / Wayland QPA (Qt Platform Abstraction) Plugin          | |
|  +-------------------------------------------------------------+ |
+-----------------------------------+------------------------------+
                                    |
+-----------------------------------v------------------------------+
|                     Zenith Display Server IPC                    |
+------------------------------------------------------------------+
```

## 2. Zenith UI Framework (Native Sovereign Toolkit)

`Zenith UI` is a zero-dependency, memory-safe Rust UI framework optimized for low memory usage and instant startup times.

### 2.1 Key Design Features
- **Declarative Composition**: UIs are constructed as immutable state trees with automatic reactive diffing.
- **Hardware-Accelerated Rendering**: Vector paths, text glyphs, and rounded rectangles are rendered via GPU shaders using a lightweight software/Vulkan pipeline.
- **CSS-Inspired Styling**: Modern flexbox layout engine with support for CSS variables, themes, dark/light modes, and animations.
- **Accessibility Integration**: Native Screen Reader and Assistive Technology (AT-SPI) accessibility node tree generation.

### 2.2 Native Zenith UI Code Example

```rust
use zenith_ui::prelude::*;

#[derive(Default)]
struct CounterApp {
    count: i32,
}

impl Component for CounterApp {
    type Message = Msg;

    fn update(&mut self, msg: Self::Message) {
        match msg {
            Msg::Increment => self.count += 1,
            Msg::Decrement => self.count -= 1,
        }
    }

    fn view(&self) -> Element<Self::Message> {
        Column::new()
            .padding(20)
            .spacing(10)
            .push(Text::new(format!("Current Count: {}", self.count)).size(24))
            .push(
                Row::new()
                    .spacing(10)
                    .push(Button::new("+ Increment").on_press(Msg::Increment))
                    .push(Button::new("- Decrement").on_press(Msg::Decrement)),
            )
            .into()
    }
}
```

## 3. GTK 4 Compatibility Bridge

SigmaOS includes a native GTK 4 backend plugin (`libwayland-zenith` / `gdk-zenith-backend`):
- Converts GTK Cairo/GSK rendering commands to Zenith display server DMA-BUFs.
- Maps GTK input signals directly to Zenith input events.
- Enforces system dark mode and theme settings across GTK applications.

## 4. Qt 6 Compatibility Bridge (QPA Plugin)

SigmaOS provides `qzenith` (Qt Platform Abstraction plugin):
- Enables native compilation and execution of Qt 6 and KDE applications.
- Supports Qt Quick / QML GPU acceleration via Vulkan / EGL sharing.
- Integrates Qt native file dialogs with the SigmaOS system file picker daemon.

## 5. UI Toolkit Comparison Matrix

| Feature | Zenith UI (Native) | GTK 4 Bridge | Qt 6 Bridge |
|---|---|---|---|
| Primary Language | Rust | C / Rust (`gtk4-rs`) | C++ / QML |
| Memory Footprint | Very Low (~4MB base) | Medium (~25MB base) | Medium-High (~35MB) |
| Cold Startup Time | < 10ms | ~40ms | ~50ms |
| Rendering Engine | Sovereign GPU Pipeline | GSK (GTK Scene Graph) | Qt Quick / RHI |
| Theme System | Native CSS / Accent | Libadwaita / GTK CSS | Breeze / KStyle |
