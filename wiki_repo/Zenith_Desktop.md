# 🖥️ Zenith Desktop — SigmaOS UI Compositor

> **Zenith** is SigmaOS's sovereign Wayland-compatible display compositor, designed to deliver a next-generation desktop experience without any X11 attack surface, legacy display server overhead, or proprietary GPU blobs.

---

## 🏗️ Architecture

```
┌─────────────────────────────────────────────────────────┐
│              Zenith Desktop Shell                        │
│   (App launcher, taskbar, notifications, workspaces)     │
├─────────────────────────────────────────────────────────┤
│             ZenithCompositor (Wayland Server)            │
│   (Window management, damage tracking, surface tree)     │
├────────────────────┬────────────────────────────────────┤
│   GPU Backend      │   Software Renderer                 │
│   (Vulkan/DRM/KMS) │   (VESA / framebuffer fallback)    │
├────────────────────┴────────────────────────────────────┤
│              S-SEC Capability Gate                        │
│   (GPU access, display server, input events)             │
├─────────────────────────────────────────────────────────┤
│   Hardware: GPU (NVLink/PCIe 5) + Display (HDMI/DP 2.1) │
└─────────────────────────────────────────────────────────┘
```

---

## ✨ Core Features

### Wayland Protocol Compatibility

ZenithCompositor implements the core Wayland protocol plus key extensions:

| Protocol | Status | Description |
|----------|--------|-------------|
| `wl_compositor` | ✅ | Core surface management |
| `wl_shm` | ✅ | Shared memory buffers |
| `xdg_shell` | ✅ | Desktop window model |
| `wl_seat` | ✅ | Pointer + keyboard input |
| `wp_presentation` | ✅ | Frame timing |
| `xwayland` | 🔄 | X11 app compatibility |
| `wp_fractional_scale` | ✅ | HiDPI support |
| `zwp_linux_dmabuf` | ✅ | Zero-copy GPU buffers |
| `ext_session_lock` | ✅ | Screen locker protocol |

### Rendering Pipeline

```
Application renders → wl_buffer (DMA-BUF or SHM) 
    → ZenithCompositor (damage tracking) 
    → Scene graph (sorted by z-order) 
    → GPU backend (Vulkan render pass) 
    → KMS/DRM (vsync atomic commit) 
    → Display
```

### Window Decorations

ZenithCompositor uses **server-side decorations (SSD)** with a clean sovereign aesthetic:

- Flat, minimal chrome — no window border gradients
- Accent color follows capability token color (per-app visual differentiation)
- Smooth 60/120/240Hz adaptive refresh
- Variable blur radius on glassmorphism panels

---

## 🎨 Zenith Desktop Shell

### Sigma Taskbar

- **Sigma Dock** — macOS-style app dock with magnetic hover animations
- **Smart Workspace** — AI-suggested workspace grouping (related apps grouped together)
- **Quick Settings** — One-click access to Wi-Fi, Bluetooth, Volume, Brightness
- **Notification Center** — Grouped, actionable notifications with rich media previews

### App Launcher

Natural language + fuzzy search:
```
> "open my tax calculation spreadsheet from last month"
→ [AI]: Found: /home/user/Documents/TaxCalc_March2025.xlsx
         Opening with SigmaOffice Calc...
```

### Workspaces

- **Dynamic workspaces** — Auto-created on demand, auto-removed when empty
- **Overview mode** — Pinch gesture or Super key shows all workspaces
- **Cross-workspace drag** — Drag windows between workspace tiles
- **Named workspaces** — AI suggests names based on open apps

---

## 🖱️ Input Handling

### Pointer (libinput replacement)

SigmaOS implements its own input event pipeline:

```rust
pub trait InputDriver {
    fn poll_events(&mut self) -> Vec<InputEvent>;
    fn set_acceleration(&mut self, accel: f64);
    fn set_scroll_factor(&mut self, factor: f64);
}
```

Supported input devices:
- USB HID keyboards and mice
- PS/2 legacy keyboard (via compatibility driver)
- Multi-touch touchpads (precision gestures: 2/3/4 finger)
- Stylus/pen input (Wacom + generic HID)
- Gamepad input (for gamepad-aware Wayland apps)
- Touchscreen (multi-point capacitive)

### Gesture Recognition

| Gesture | Action |
|---------|--------|
| 3-finger swipe left/right | Switch workspace |
| 3-finger swipe up | Show overview |
| 4-finger pinch | Mission Control |
| 2-finger scroll | Natural scrolling |
| Touchscreen long-press | Context menu |

---

## 🎨 Theming System

### SigmaTheme Engine

The theming engine uses a declarative theme specification:

```toml
# ~/.config/zenith/theme.toml
[colors]
accent = "#6C63FF"        # Sovereign purple
background = "#0D1117"    # Deep navy
surface = "#161B22"       # Card surface
text_primary = "#E6EDF3"  # Primary text
text_secondary = "#8B949E"

[blur]
enabled = true
radius = 20
saturation = 1.4

[animations]
duration_ms = 200
easing = "ease-out-cubic"
```

### Preset Themes

| Theme | Style |
|-------|-------|
| Sovereign Dark | Default — deep navy + sovereign purple |
| Banaras Gold | India-inspired warm gold on deep maroon |
| Kashmir Blue | Cool blue-grey inspired by Dal Lake |
| Midnight Teal | Cyberpunk-inspired teal on near-black |
| Paper White | Light mode — off-white with ink accents |

### Dynamic Theming

- **Time-based** — Auto-switch to light/dark at sunrise/sunset
- **Location-aware** — Sunrise/sunset calculated from device location
- **App-specific** — Different accent colors per application

---

## 🌐 Display Management

### Multi-Monitor Support

```bash
# List displays
sigma-display list

# Configure displays
sigma-display set --primary DP-1 --secondary HDMI-1 --arrange right

# HiDPI scaling
sigma-display scale --output DP-1 --factor 2.0
```

### HDR Support

ZenithCompositor supports HDR10 and HLG on compatible displays:
- Tone mapping for non-HDR app content on HDR displays
- Per-window HDR mode
- EDR (Extended Dynamic Range) in software for HiDPI displays

### Refresh Rates

- **Variable Refresh Rate (VRR)** — AMD FreeSync / NVIDIA G-Sync Compatible
- **Adaptive Sync** — Reduces screen tearing without fixed vsync latency
- **High Refresh** — Up to 360Hz on supported displays

---

## 📺 Screen Recording & Screenshotting

Built-in screen capture without external tools:

```rust
// Screen capture API (capability-gated)
let recorder = ScreenRecorder::new(cap_token)?;
recorder.start_recording(RecordingConfig {
    output_path: "/home/user/Videos/recording.mp4",
    codec: VideoCodec::AV1,
    quality: Quality::High,
    include_audio: true,
    region: CaptureRegion::FullScreen,
})?;
```

---

## 🔒 Security Properties

ZenithCompositor enforces:

1. **No ambient authority** — Apps cannot read other apps' window content
2. **Capability-gated screen capture** — Screenshots require explicit capability
3. **Secure lock screen** — Separate process with minimal capabilities
4. **Clipboard isolation** — Apps cannot read clipboard unless granted `clipboard_read` capability
5. **Input isolation** — Global keyboard hooks require `input_monitor` capability (must be user-approved)
6. **No X11 by default** — Eliminating the entire X11 attack surface

---

## 🔗 Related Pages

- [Security Framework](Security_Framework) — Capability-gated display APIs
- [SigmaMedia Frameworks](SigmaMedia-Frameworks) — Built-in media playback in Zenith
- [Advanced Absorption Matrix](Advanced_Absorption) — How Zenith replaces legacy DEs
- [Maturity & Distro-Parity Roadmap](Maturity_Parity_Roadmap) — Desktop milestone status
