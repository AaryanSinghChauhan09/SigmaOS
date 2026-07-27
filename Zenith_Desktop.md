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

## 🐚 Unified CLI Control Suite (GUI-over-CLI)

SigmaOS bridges command-line productivity with Graphical interface richness. Every task that can be performed via the Zenith Compositor's graphical controls can easily be done using our interactive CLI control suite.

### 1. Display Configuration (`display`)
Enables full control over monitor arrangements, refresh rates, HiDPI scaling, and HDR settings:
```bash
# List all active and connected outputs
display list

# Set layout positioning
display set --primary DP-1 --secondary HDMI-1 --arrange right

# Configure HiDPI scaling
display scale --output DP-1 --factor 2.0

# Set refresh rate
display rate --output DP-1 --hz 144

# Toggle High Dynamic Range
display hdr --output DP-1 --enable true
```

### 2. Desktop Theming (`theme`)
Switch custom themes, adjust blur radius, or configure location-aware automatic dark mode switching:
```bash
# List preset themes
theme list

# Change active theme
theme set "Banaras Gold"

# Customize variables
theme configure --accent #6C63FF --blur true --radius 20

# Enable time-based theme transitions
theme auto --enable true --mode time
```

### 3. Desktop UX & Profile Swapper (`profile`)
Polymorphically swap between standard, developer, or guest environments with different layout algorithms:
```bash
# List custom user profiles
profile list

# Swap active profile
profile switch developer

# Override layout algorithm
profile layout tiling
```

### 4. Window Manager Controller (`window`)
Create, list, move, or destroy compositor windows directly:
```bash
# List running windows
window list

# Create a window
window create --title "SigmaBrowser" --app "sigma.browser" --geom "10,10,600,400"

# Change state
window state --id 1 --state Maximized

# Move / resize
window move --id 1 --x 20 --y 20 --w 800 --h 600

# Focus / Close
window focus --id 1
window close --id 2
```

### 5. Accessibility Controls (`accessibility` / `acc`)
Toggle screen readers, magnification zoom, or load color-blindness shader corrections:
```bash
# Query active accessibility settings
acc status

# Toggle screen reader
acc screen-reader --enable true --voice "default" --speed 1.0

# Adjust magnifier zoom level
acc magnifier --zoom 2.0

# Set color blind correction shader
acc colorblind protanopia
```

### 6. Screen Record & Screenshot (`screenshot` / `record`)
Capture high-quality, hardware-accelerated screenshots or video streams:
```bash
# Capture full screen
screenshot capture --output /home/ubuntu/screenshot.png --region full

# Start recording
record start --output /home/ubuntu/recording.mp4 --codec av1 --quality high

# Stop recording
record stop
```

### 7. Secure Clipboard Manager (`clipboard`)
Retrieve or write context safely with capability-gated validation checks:
```bash
# Set clipboard text
clipboard set "Sovereign Operating System"

# Retrieve clipboard text (Requires 0x5001 capability validation)
clipboard get
```

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
<<<<<<< HEAD
=======


---
## Merged from Zenith-Desktop.md
# Zenith Desktop Environment

The Zenith Desktop is SigmaOS's flagship UI — a sovereign, compositor-first desktop environment with AI-driven personalisation, spatial audio, and full accessibility.

---

## Architecture

```
Zenith Desktop (zenith_desktop/)
    │
    ├── Compositor (C++ native — Phase G)
    │     └── ZenithDesktopEnvironment.cpp
    ├── Window Manager
    │     └── zenith_desktop/wm/
    ├── Neural UI (AVX-512 accelerated)
    │     └── zenith_desktop/neural/sigma_neural_ui.cpp
    ├── Theme Engine
    │     └── zenith_desktop/theme/
    ├── Personalisation
    │     └── zenith_desktop/personalization/
    ├── App Store
    │     └── zenith_desktop/appstore/
    ├── Notifications
    │     └── zenith_desktop/notifications/
    ├── Settings
    │     └── zenith_desktop/settings/
    └── Accessibility
          └── zenith_desktop/a11y/
```

---

## Current State

| Component | Status |
|-----------|--------|
| JS prototype (browser) | ✅ Working in browser profile |
| C++ compositor | 🔄 In progress — Phase G |
| Auto-tiling WM | 🔄 Implemented, needs input integration |
| Theme engine | ✅ Implemented |
| Neural UI (AVX-512) | ✅ Implemented |
| Accessibility (SSR) | 🔄 Partial |
| Indian IME | ⬜ Phase H |
| sigma-ai LLM daemon | ⬜ Phase H |
| App store UI | 🔄 Demo in `release/app` |

---

## Window Management (`zenith_desktop/wm/`)

Zenith uses a **hybrid tiling + floating** window manager:

```
Workspaces: 1–9 (switch with Super+1 through Super+9)
Tiling modes:
  - Master/stack (default)
  - Grid (Super+G)
  - Fullscreen (Super+F)
  - Floating (Super+Shift+Space)

Key bindings (default):
  Super+Enter     New terminal
  Super+D         App launcher (sigma-spotlight)
  Super+Q         Close window
  Super+H/L       Resize master pane
  Super+Shift+R   Reload config
```

---

## Theme System (`zenith_desktop/theme/`)

```bash
# List available themes
sigma-theme list

# Apply a theme
sigma-theme apply midnight-sovereign

# Create a custom theme
sigma-theme new my-theme --base midnight-sovereign
```

**Built-in themes:**
- `midnight-sovereign` — dark blue/purple
- `solar-zenith` — warm amber + white
- `forest-minimal` — muted green
- `arctic-pure` — clean white/grey

Theme structure: CSS-like variables → compositor applies to all windows.

---

## Neural UI (`zenith_desktop/neural/sigma_neural_ui.cpp`)

AI-driven adaptive UI using AVX-512 SIMD acceleration:

- **Predictive pre-loading**: predicts next app the user will open based on context
- **Adaptive layouts**: rearranges widgets based on usage patterns
- **Smart notifications**: batches low-priority notifications, surfaces critical ones immediately
- **Energy-aware rendering**: reduces refresh rate when battery low

---

## Accessibility (`zenith_desktop/a11y/`)

- **Sovereign Screen Reader (SSR)**: reads UI elements aloud via sigma-voice
- **High-contrast themes**: automatic inversion for visual impairments
- **Keyboard-only navigation**: full WCAG 2.1 AA compliance target
- **Switch access**: single-switch scanning for motor impairments
- **Font scaling**: 75%–300% without layout break
- **Braille display**: planned via sigma-braille daemon (Phase H)

---

## Zenith Widgets (`zenith_desktop/modules/`)

Pre-installed widgets on the desktop panel:

| Widget | File | Function |
|--------|------|---------|
| System telemetry | `sigma_widget_sys_telemetry.cpp` | CPU/RAM/net graphs |
| AI monitor | `sigma_widget_ai_monitor.cpp` | LLM inference usage |
| Crypto shield | `sigma_widget_crypto_shield.cpp` | Active PQC connections |
| App launcher | `sigma_widget_app_launcher.cpp` | Quick-launch dock |
| Quick settings | `sigma_widget_quick_settings.cpp` | Wi-Fi, volume, brightness |

---

## Desktop Build & Run

```bash
# Build Zenith desktop
make PROFILE=standalone all -j$(nproc)

# Run JS prototype in browser (current demo)
open sigma-web/index.html

# Run native compositor in QEMU (Phase G)
make PROFILE=standalone qemu
```

---

*See also: [Architecture-Overview](Architecture-Overview) · [Release-Profiles](Release-Profiles) · [Sigma-Desktop-Environment](Sigma-Desktop-Environment)*
>>>>>>> wiki/master
