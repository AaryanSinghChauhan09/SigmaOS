> Imported repository document from [`Agents/INPUT_AGENTS.md`](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/Agents/INPUT_AGENTS.md). For current component status and roadmap, use the canonical component page linked from [Home](Home.md).

---

# SigmaOS Input Device Component Agents

## Component Overview

The input subsystem manages keyboards, mice, touchpads, touchscreens, game controllers, and input event routing (evdev, libinput). This component is currently **MISSING** in SigmaOS but is critical for user interaction.

**Status**: 🔴 **NOT IMPLEMENTED** - Critical gap for desktop/mobile use

## Linux & BSD Inspiration Sources

### Primary References
- **Linux Input Subsystem** (`drivers/input/`): evdev, input_event protocol
- **libinput** (Linux): High-level input handling (gestures, acceleration)
- **FreeBSD evdev** (`sys/dev/evdev/`): Linux-compatible event device interface
- **OpenBSD wsmux** (`sys/dev/wscons/`): Console multiplexer
- **Wayland libinput** (compositor integration): Direct device access

### Key Capabilities to Absorb
1. **evdev Protocol** (Linux `/dev/input/event*`)
2. **Touchpad Gestures** (libinput: pinch, swipe, two-finger scroll)
3. **Keyboard Layouts** (XKB keymaps)
4. **Hot-Plug Support** (USB HID devices)
5. **Input Multiplexing** (multiple keyboards/mice to single logical device)

## Agent Role: ⌨️ Input

### Core Mission
Implement evdev-compatible input subsystem with HID device support, libinput integration, keyboard layout handling, and Wayland/X11 event routing for SigmaOS.

### Operational Boundaries

**Always Do**:
- Generate timestamps for all input events (for gesture recognition)
- Support hot-plug (USB keyboard/mouse connect/disconnect)
- Implement rate limiting (prevent input flooding DoS)
- Respect keyboard grab semantics (Wayland/X11)
- Test with real hardware (USB, Bluetooth, PS/2)

**Ask First**:
- Adding support for exotic input devices (eye trackers, VR controllers)
- Implementing custom gesture recognition beyond libinput
- Supporting legacy protocols (PS/2, serial mice)

**Never Do**:
- Log keystrokes (privacy violation)
- Allow unprivileged access to `/dev/input/*` (keylogger risk)
- Skip input validation (check all coordinates, button IDs)
- Trust HID descriptors without bounds checking

### Philosophy
Input is user intent. Responsiveness is paramount: < 1ms latency from hardware to compositor. Security: input devices are trusted; access control is mandatory. Accessibility: keyboard-only navigation must work.

### Required Components

#### 1. **Input Core** (`src/drivers/input/core.rs`)
```rust
#![no_std]
// Input device registration
// - Device enumeration (/dev/input/event0, event1, ...)
// - Event routing to subscribers (Wayland compositor, X11 server)
// - Input device capabilities (keyboard, mouse, touchpad, etc.)
// - Hot-plug event handling (USB device add/remove)
```

#### 2. **evdev Interface** (`src/drivers/input/evdev.rs`)
```rust
#![no_std]
// Linux-compatible evdev protocol
pub struct InputEvent {
    pub time: TimeVal,
    pub type_: u16, // EV_KEY, EV_REL, EV_ABS
    pub code: u16,  // KEY_A, REL_X, ABS_X
    pub value: i32, // Press/release, delta, absolute value
}

// Device capabilities bitmask
// - EVIOCGBIT (get supported event types)
// - EVIOCGABS (get absolute axis info: min, max, resolution)
// - EVIOCGRAB (exclusive device access)
```

#### 3. **HID (Human Interface Device) Parser** (`src/drivers/input/hid.rs`)
```rust
#![no_std]
// USB HID report descriptor parser
// - Parse HID descriptors (keyboards, mice, game controllers)
// - Map HID usage pages to evdev codes
// - Handle quirks (specific devices with broken descriptors)
// - Support HID-over-I2C (touchpads on laptops)
```

#### 4. **Keyboard Driver** (`src/drivers/input/keyboard.rs`)
```rust
#![no_std]
// Keyboard event handling
// - Scancode → keycode translation (using XKB)
// - Key repeat (initial delay + repeat rate)
// - Modifier state tracking (Shift, Ctrl, Alt, Super)
// - Compose key sequences (dead keys, multi-key)
// - LED control (Caps Lock, Num Lock, Scroll Lock)
```

#### 5. **Mouse/Touchpad Driver** (`src/drivers/input/pointer.rs`)
```rust
#![no_std]
// Pointer device handling
// - Relative motion (mice)
// - Absolute position (touchscreens, graphics tablets)
// - Button mapping (left, right, middle, side buttons)
// - Scroll wheel (vertical, horizontal)
// - Touchpad gestures (via libinput integration)
```

#### 6. **Touchscreen Driver** (`src/drivers/input/touchscreen.rs`)
```rust
#![no_std]
// Multi-touch protocol
// - Type A (slot-less) vs Type B (slot-based)
// - Touch tracking (ID assignment, position, pressure)
// - Gesture recognition (tap, double-tap, long-press)
// - Palm rejection
```

#### 7. **Game Controller Driver** (`src/drivers/input/gamepad.rs`)
```rust
#![no_std]
// Gamepad support (Xbox, PlayStation, Nintendo controllers)
// - Button mapping (A/B/X/Y, D-pad, bumpers, triggers)
// - Analog stick dead zones
// - Force feedback (rumble motors)
```

### Verification Protocol

```bash
# List input devices
ls -la /dev/input/
# Should show event0, event1, ..., mice

# Dump input events
evtest /dev/input/event0
# Press keys, move mouse, verify events appear

# Test keyboard layout
setxkbmap us # US QWERTY
xev # Type keys, check keycodes

# Test touchpad gestures
libinput debug-events --device /dev/input/event5
# Perform two-finger scroll, pinch, swipe

# Test game controller
jstest /dev/input/js0
# Press buttons, move sticks, verify values

# Hot-plug test
# Unplug USB mouse, plug back in
dmesg | grep input
# Should show device removal and re-addition
```

### Security Hardening Rules

1. **Device Permissions**: Only `input` group can read `/dev/input/*`
2. **Grab Enforcement**: Respect EVIOCGRAB (screen lockers, Wayland)
3. **Rate Limiting**: Max 1000 events/second per device
4. **Bounds Checking**: Validate all HID descriptor values
5. **No Keystroke Logging**: Never log actual key values

### Integration Points

**Dependencies**:
- `src/drivers/usb.rs` - USB HID device enumeration
- `src/drivers/i2c.rs` - Laptop touchpad (HID-over-I2C)
- `src/drivers/bluetooth.rs` - Bluetooth mice/keyboards
- `src/desktop/wayland.rs` - Wayland input event routing
- `src/desktop/xorg.rs` - X11 input event routing (legacy)

**Exports to Userland**:
- `/dev/input/event*` - Raw event devices (evdev)
- `/dev/input/mice` - Multiplexed mouse events
- `/dev/input/js*` - Joystick devices
- `libsigma-input.so` - libinput-compatible library
- `/etc/sigma/xkb/` - Keyboard layout definitions

### Zero-Dependency Philosophy

**No libinput in Kernel**: Gesture recognition belongs in userspace compositor. Kernel only provides raw events.

**Direct Hardware Access**:
```rust
// Example: Read USB HID report
unsafe fn read_hid_report(device: &UsbDevice) -> Result<InputEvent, Error> {
    let mut buf = [0u8; 64];
    device.interrupt_read(&mut buf)?;
    
    // Parse HID report (device-specific)
    let buttons = buf[0];
    let x_delta = buf[1] as i8;
    let y_delta = buf[2] as i8;
    
    Ok(InputEvent {
        type_: EV_REL,
        code: REL_X,
        value: x_delta as i32,
        ..
    })
}
```

### Component Milestones

1. **Phase 1**: PS/2 keyboard + mouse drivers (legacy support)
2. **Phase 2**: USB HID parser + evdev interface
3. **Phase 3**: Hot-plug support (USB device add/remove)
4. **Phase 4**: Touchpad driver (HID-over-I2C)
5. **Phase 5**: Touchscreen multi-touch support
6. **Phase 6**: Game controller support (Xbox/PS)
7. **Phase 7**: Bluetooth input device support

### Testing Requirements

- Unit tests: HID descriptor parsing, keycode translation
- Integration tests: Type into text editor, move mouse cursor
- Stress tests: Rapid keypress (1000 events/sec), verify no drops
- Latency tests: Measure input-to-compositor latency (< 1ms)
- Compatibility: Test 10+ different mice/keyboards/touchpads

### Performance Targets

- **Input latency**: < 1 millisecond (hardware IRQ to evdev event)
- **Event throughput**: 10K events/second (gaming mouse polling rate)
- **Hot-plug latency**: < 100 milliseconds (USB detect to device ready)
- **Battery impact**: < 1% CPU for Bluetooth polling
- **Memory per device**: < 4 KB (device state + buffers)

### Error Handling

All input errors MUST:
1. Log to kernel ring buffer (device path, error code)
2. Never panic kernel (input is non-critical)
3. Drop malformed events (invalid type/code/value)
4. Notify userland via udev event (device disconnected)

### evdev Event Types

Support these event types:
- `EV_KEY (0x01)` - Key/button press and release
- `EV_REL (0x02)` - Relative motion (mouse X/Y, wheel)
- `EV_ABS (0x03)` - Absolute position (touchscreen, tablet)
- `EV_MSC (0x04)` - Miscellaneous (scancodes)
- `EV_LED (0x11)` - LED state (Caps Lock, Num Lock)
- `EV_SND (0x12)` - Sound (PC speaker beep)
- `EV_REP (0x14)` - Key repeat
- `EV_FF (0x15)` - Force feedback (rumble)
- `EV_SYN (0x00)` - Event separator (end of atomic update)

### Keyboard Layout Support

Implement XKB (X Keyboard Extension) subset:
- **Layouts**: us, gb, de, fr, es, ru, jp, ...
- **Variants**: dvorak, colemak, azerty, qwertz
- **Options**: Compose key, Caps Lock → Ctrl remapping
- **Dead keys**: ´ + e → é, ` + a → à

Configuration files:
- `/etc/sigma/xkb/rules/evdev.xml` - Layout definitions
- `/etc/sigma/xkb/symbols/us` - US layout keycodes
- `/etc/sigma/xkb/keycodes/evdev` - Scancode → keycode map

### Touchpad Gestures (libinput Subset)

Support these gestures:
- **Two-finger scroll** (vertical, horizontal)
- **Pinch to zoom** (two-finger pinch)
- **Swipe** (three-finger swipe for workspace switch)
- **Tap to click** (single, double, triple tap)
- **Palm rejection** (ignore large contact areas)

### Wayland Input Integration

Wayland compositor receives:
1. Keyboard events → `wl_keyboard.key`, `wl_keyboard.modifiers`
2. Pointer events → `wl_pointer.motion`, `wl_pointer.button`
3. Touch events → `wl_touch.down`, `wl_touch.motion`, `wl_touch.up`
4. Tablet events → `zwp_tablet_tool_v2.*`

Compositor responsibilities:
- Open `/dev/input/event*` directly (no X server)
- Handle keyboard focus (which window receives keys)
- Implement pointer constraints (gaming mouse lock)

### Documentation Requirements

- evdev protocol reference (event types, codes)
- Keyboard layout configuration guide
- Touchpad gesture tuning (acceleration, sensitivity)
- Game controller button mapping
- Troubleshooting (input not working, wrong layout)

---

## Journaling Rules (`.jules/input.md`)

Record critical insights:
- Device-specific HID quirks (broken descriptors)
- Input latency regressions
- Gesture recognition edge cases
- Keyboard layout bugs (specific language layouts)

**Journal Entry Template**:
```
## [Date] - [Input Issue Summary]
**Problem**: [No input / wrong keys / laggy mouse]
**Root Cause**: [Missing driver / wrong layout / USB polling]
**Solution**: [Driver added / layout fixed / polling adjusted]
**Hardware**: [Keyboard/mouse model]
```

---

*This agent file defines the currently MISSING input subsystem. Implementation is CRITICAL for interactive desktop/mobile use.*
