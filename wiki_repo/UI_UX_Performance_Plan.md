# SigmaOS UI/UX & Performance Improvement Plan

> A structured, prioritised roadmap for making SigmaOS visually excellent,
> fast, and delightful to use. Grounded in concrete implementation tasks.

---

## Current State (v15.0 Baseline)

| Area | Status | Gap |
|---|---|---|
| Animation | ✅ Spring + easing engine | No GPU acceleration yet |
| Renderer | ✅ Software 2D renderer | No Vulkan/GL path |
| Input | ✅ Keyboard/mouse/touch/gestures | No haptics, no eye tracking |
| Panel | ✅ Dynamic Island panel | Static layout only |
| Settings | ✅ 3-panel hub | No search, no per-app permissions |
| Accessibility | ✅ 6 features | No TTS audio output |
| Compositor | ✅ Alpha-blend scene graph | No GPU compositing |
| Window Manager | ✅ Tiling + BSP + Grid | No snap zones, no drag-to-tile |
| Theme Engine | ✅ 3 themes + CSS vars | No live theme preview |
| Notifications | ✅ Full notification system | No grouped notifications |
| Widgets | ✅ Button + TextInput | No ListView, Slider, Toggle |
| Launcher | ✅ Fuzzy search | No voice search, no history learn |

---

## Priority 1 — Core Performance (v0.1 target)

### 1.1 GPU-accelerated Compositing

**Problem**: Software renderer is CPU-bound. At 1920×1080, fill_rect takes ~8ms.
**Solution**: VirtIO-GPU + Mesa path for Vulkan 1.3 compositing.

```
kernel/drivers/gpu/sigma_virtio_gpu.zig  ✅ already done
drivers/gpu/sigma_vulkan.rs              → add VkDevice + VkSwapchain
userland/desktop/sigma_compositor.rs    → add GPU submit path
```

**Target**: 60fps at 1080p on VirtIO-GPU (QEMU). 16ms frame budget.

### 1.2 Zero-Copy Framebuffer Path

**Problem**: Every frame copies pixel data through multiple buffers.
**Solution**: DMA-buf shared buffer between compositor and KMS.

```rust
// target API:
let buf = DmaBuf::alloc(width, height, PixelFormat::BGRA8888)?;
compositor.set_output_buffer(buf.clone());
drm.scanout(buf);   // no copy — same physical pages
```

### 1.3 Async I/O (io_uring equivalent)

**Problem**: File and network I/O blocks the render thread.
**Solution**: sigma_io_uring ring-buffer async syscalls.

```
kernel/core/sigma_io_uring.rs   → new file
```

**Target**: sigma-pkg install downloads at full NIC speed, UI stays at 60fps.

### 1.4 Startup Optimisation

**Problem**: Cold boot to usable desktop takes >5s (no boot yet).
**Target timeline** once bootable:

| Milestone | Target |
|---|---|
| UEFI → kernel_main | < 500ms |
| Kernel init (sched+mm+irq) | < 200ms |
| sigma-init + daemons | < 800ms |
| Zenith compositor first frame | < 500ms |
| **Total boot to desktop** | **< 2 seconds** |

**Techniques**: pre-fork daemons, tmpfs early mounts, lazy shard loading.

---

## Priority 2 — UI Quality (v1.0 target)

### 2.1 Font Rendering (sigma-font-engine)

**Problem**: No font rendering yet — text is placeholder.
**Plan**: Cleanroom bitmap font renderer → vector font rasteriser.

```rust
// Phase 1: 8x8 bitmap font (already stubbed in sigma_vesa.zig)
// Phase 2: TrueType/OpenType outline rasterizer
pub struct GlyphCache { atlas: Vec<u8>, glyphs: BTreeMap<(char,u32), GlyphRect> }
pub trait FontEngine { fn rasterize(&mut self, c: char, size: u32) -> &[u8]; }
```

**Files to create**: `userland/desktop/sigma_font.rs`

### 2.2 Widget Expansion

**Current**: Button, TextInput
**Planned** (in priority order):

| Widget | Description | Use in |
|---|---|---|
| `Toggle`    | On/off switch with slide anim | Settings, quick toggles |
| `Slider`    | Range input with drag handle | Volume, brightness |
| `ListView`  | Scrollable list with virtual rendering | File manager, app list |
| `DropDown`  | Select from options | Settings selects |
| `Progress`  | Determinate + indeterminate bar | Downloads, loading |
| `Tooltip`   | Hover info overlay | All interactive elements |
| `Modal`     | Blocking dialog | Confirm/cancel prompts |
| `TabBar`    | Horizontal/vertical tab strip | Settings panels, browser |
| `Sidebar`   | Collapsible side navigation | File manager, settings |
| `SearchBar` | Input with live filter + icons | Launcher, settings |

**File**: `userland/desktop/sigma_widgets.rs` (extend existing)

### 2.3 Glassmorphism Improvements

**Current**: 3×3 box blur only
**Plan**:

- 5×5 Gaussian blur (better visual quality)

- Luminance-adaptive border (brighter border on dark bg, dimmer on light)

- Vibrancy: average surrounding pixel colour as tint

### 2.4 Responsive Layouts

**Problem**: Fixed-pixel layouts break at different resolutions.
**Plan**: Flexbox-inspired layout engine.

```rust
pub enum FlexDir { Row, Column }
pub struct FlexLayout { dir: FlexDir, gap: u32, align: Align, justify: Justify }
impl FlexLayout { pub fn arrange(&self, children: &[Rect], container: Rect) -> Vec<Rect> }
```

### 2.5 Dark/Light Mode Transitions

**Plan**: Animated theme switch — interpolate all colours over 400ms.

```rust
fn transition_theme(&mut self, from: &dyn Theme, to: &dyn Theme, progress: f32) -> ThemeSnapshot {
    // Lerp every colour at ease_in_out_sine(progress)
}
```

---

## Priority 3 — UX Improvements (v1.0–v1.5)

### 3.1 Onboarding Wizard

**Current**: None.
**Plan**: 5-step first-boot flow:

```
Step 1: Welcome + language selection
Step 2: Privacy choices (all off by default, explicit opt-in)
Step 3: User account + hostname
Step 4: Disk setup (partition wizard)
Step 5: Hardware detection summary + driver status
```

**File**: `userland/installer/sigma_onboarding.rs`

### 3.2 Unified Search (sigma-search)

**Plan**: Global search across apps, files, settings, web.

```
Score = file_name_score × 3 + content_score × 1 + recent_boost × 2
Query pipeline: tokenise → search_apps → search_files → search_settings → merge → rank
```

**File**: `userland/tools/sigma_search.rs`

### 3.3 Notification Action Centre

**Current**: Basic notification display.
**Plan**:

- Grouped notifications (by app)

- Swipe-to-dismiss gesture

- Action buttons rendered in-notification

- History panel (last 48h)

### 3.4 Quick Settings Panel

**Plan**: Swipe-down panel with 12 toggles + sliders:

```
[Wi-Fi ▲] [Bluetooth] [Do Not Disturb] [Airplane Mode]
[Brightness ████░░] [Volume ███░░░]
[VPN] [Hotspot] [Mirror] [Dark Mode] [Night Light] [Battery Saver]
```

### 3.5 App Switcher (Mission Control)

**Plan**: Super key → show all windows as thumbnails, draggable to workspaces.

```rust
pub struct AppSwitcher {
    thumbnails: Vec<WindowThumbnail>,
    layout:     ThumbnailGrid,
    animation:  Animation,   // scale from 1.0 → 0.7
}
```

---

## Priority 4 — Performance Engineering

### 4.1 Memory Optimisation

| Target | Current | Plan |
|---|---|---|
| Idle desktop RAM | Unknown | < 256 MB |
| sigma-sh startup | Unknown | < 50ms |
| sigma-pkg install | Unknown | < 1s for typical package |
| Kernel size | Unknown | < 2 MB stripped |

**Techniques**:

- `#[repr(packed)]` structs to reduce memory layout waste

- Arena allocators for short-lived objects

- Slab cache reuse for frequent allocations

- Lazy shard loading (load on first use)

### 4.2 I/O Stack Optimisation

```
Current: syscall → VFS → Tmpfs → copy to user buffer
Target:  syscall → VFS → io_uring ring → zero-copy to mmap'd user buffer
```

### 4.3 Network Performance

| Path | Current | Target |
|---|---|---|
| sigma-pkg download (1MB) | Unknown | < 100ms on GbE |
| TLS handshake | Unknown | < 5ms |
| DNS resolution | Unknown | < 10ms (DoH cached) |
| Firewall per-packet | Unknown | < 100ns (AVC cache hit) |

### 4.4 Scheduler Tuning

- **Interactive boost**: bump priority +2 for process receiving keyboard input

- **Anticipatory I/O**: prefetch next likely read based on access pattern

- **CPU affinity**: pin kernel threads to CPU 0, user threads spread across rest

---

## Priority 5 — Capabilities Expansion

### 5.1 Multi-Monitor Support

```rust
pub struct OutputManager { outputs: Vec<Output>, arrangement: MultiMonArrangement }
pub enum MultiMonArrangement { Mirror, Extend(ExtendDir), Independent }
```

### 5.2 HiDPI / Fractional Scaling

- 1.25×, 1.5×, 2.0× rendering scale

- Logical pixel coordinates throughout UI

- Automatic detection from EDID display info

### 5.3 Wayland Protocol Compatibility

- Implement xdg-shell, xdg-output, wlr-layer-shell protocols

- Allow Wayland apps to run on Zenith compositor

- `navigator.sigmaos.wayland` API for web apps

### 5.4 Mobile UI Adaptation

- Breakpoints: phone (< 600px) / tablet (600-1024px) / desktop (> 1024px)

- Touch-first interactions on phone

- Bottom navigation bar on phone, side rail on tablet

---

## Implementation Schedule

| Quarter | UI/UX Focus | Performance Focus |
|---|---|---|
| Q3 2026 | Font engine + 5 new widgets | io_uring + GPU compositor path |
| Q4 2026 | Onboarding wizard + quick settings | Boot time < 2s |
| Q1 2027 | App switcher + multi-monitor | Memory < 256MB idle |
| Q2 2027 | Wayland compat + HiDPI | Network < 5ms TLS handshake |
| Q3 2027 | Mobile UI adaptation | 60fps on ARM64 Raspberry Pi |

---

## Measurement Framework

Every UI/UX change must be measured with `sigma-bench` and `sigma-perf`:

```bash

# Frame time regression test

sigma-bench compositor-60fps --target 16ms

# Memory regression

sigma-bench idle-ram --target 256MB

# Startup time

sigma-bench boot-to-desktop --target 2000ms

# Input latency

sigma-bench keystroke-to-screen --target 5ms
```

All benchmarks run on every PR via CI (`sigma_qemu.yml`).

---

*See also: [docs/Adoption_Strategy.md](Adoption_Strategy.md) · [wiki/UI-UX-Performance](../wiki_repo/UI-UX-Performance.md) · [ROADMAP.md](../ROADMAP.md)*
