# ⌨️ SigmaOS OOP Input Subsystem Development Plan

This document details the roadmap for developing the **SigmaOS Input Subsystem**. Drawing architectural inspiration from **evdev (Event Device)** and hardware abstraction layouts in Linux distributions like **Gentoo** and **Android**, this plan establishes a clean, extensible, and high-performance input event pipeline.

---

## 🏗️ 1. Input Subsystem Design

In SigmaOS, all physical user interactions (key presses, mouse coordinates, multi-touch coordinates) are processed into a unified, lightweight event package.

```
          +-------------------------------------------+
          |         Graphical Shell & Console         |
          +-------------------------------------------+
                                ^
                                |
          +-------------------------------------------+
          |            Unified Event Router           |
          +-------------------------------------------+
                                ^
                                |
             +------------------+------------------+
             | (Generic event translation)         |
             v                                     v
+------------------------+             +------------------------+
|      LegacyInput       |             |       ModernInput      | (OOP Traits)
+------------------------+             +------------------------+
| - Serial / PS2 Mouse   |             | - USB HID Keyboard     |
| - PS2 Keyboard         |             | - Multitouch Panel     |
+------------------------+             +------------------------+
```

### 1.1 The Core Trait (`InputDevice`)
Every input peripheral must conform to this `#![no_std]` trait:

```rust
pub trait InputDevice: PeripheralDevice {
    /// Forces the device to poll physical registers and publish standard input events
    fn poll_events(&mut self) -> Result<usize, &'static str>;

    /// Reads generated events from the driver internal buffer
    fn read_event_buffer(&mut self, buffer: &mut [InputEvent]) -> usize;
}
```

### 1.2 Event Format Structs
The system passes generic events without relying on dynamic allocations:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputType {
    KeyPress { keycode: u32, pressed: bool },
    MouseMove { dx: i32, dy: i32, scroll: i16 },
    Touch { id: u8, x: u32, y: u32, pressed: bool },
}

#[derive(Debug, Clone, Copy)]
pub struct InputEvent {
    pub event_type: InputType,
    pub timestamp_ms: u64,
}
```

---

## 🔌 2. Target Drivers & Hardware Evolution

We implement diverse input drivers to ensure support for both vintage workstations and modern embedded screens:

### 2.1 Legacy: SerialMouseDriver & Ps2KeyboardDriver
- **Serial Mouse**: Plugs into port COM1 (`0x3F8`) or COM2 (`0x2F8`). Receives raw 3-byte packets (Mousesystems or Microsoft Serial Mouse protocols) and transforms them into standard `MouseMove` events.
- **PS/2 Controller**: Communicates over legacy ports `0x60` and `0x64`. Supports multi-set scancodes using standard state machines to translate legacy key breaks into keycode presses.

### 2.2 Modern: USB HID & Multitouch Panels
- **USB HID Keyboard**: Operates on USB Universal Host Controller Interface (UHCI) or Extensible Host Controller Interface (xHCI). Decodes raw USB boot protocol packets into standard ASCII symbols.
- **Multitouch Panel**: Communicates via I2C or SPI (such as the STMPE610 controller). Extracts precise coordinates and publishes multi-finger touch streams.

---

## ⚡ 3. UDF Input Remapping Sandbox

For accessibility profiles, macro key injections, or localized keyboard layouts:
- Users register short **UDF input filters** into the event pipeline.
- These bytecodes intercept generated `InputEvent` packages, translating key values (e.g. from Dvorak layout mapping to QWERTY, or executing multi-key macro combos) without requiring high-level OS daemon dependencies.

---

## 📈 4. Roadmap and Milestones

1. **Phase 1: Input Event Definitions**
   - Create generic `InputEvent` formats and the `InputDevice` trait under `src/drivers/input/mod.rs`.
2. **Phase 2: PS/2 Keyboard and Mouse**
   - Implement interrupt handler for IRQ 1 and IRQ 12, decode scancodes, and manage Shift/Ctrl modifier flags.
3. **Phase 3: Serial COM Port Mouse**
   - Parse Microsoft serial protocols (1200 baud, 7 data bits, 1 stop bit) to enable classic cursor movements.
4. **Phase 4: USB HID Keyboard Driver**
   - Map standard USB class descriptors and decode incoming report buffers into corresponding key states.
5. **Phase 5: Event Multiplexer Router**
   - Link all active input devices to a central ring-buffer queue, delivering unified events to the desktop compositing manager.
