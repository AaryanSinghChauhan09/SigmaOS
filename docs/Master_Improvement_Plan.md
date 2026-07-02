# SigmaOS Master Improvement Plan

> A unified, actionable plan covering design, apps, UI/UX, performance,
> OOP architecture, and capabilities. Every item maps to a concrete file + owner.

---

## Executive Summary

| Area | Current State | Target (v1.0) | Target (v2.0) |
|---|---|---|---|
| Design System | ✅ Defined | Implemented in all widgets | Live theming |
| Applications | 0 installable | 8 Tier-1 apps | 20+ apps |
| UI/UX | ✅ Core engine | Full widget set + animation | Wayland compat |
| Performance | Unknown (no boot) | Boot < 2s, 60fps | Boot < 1s, 120fps |
| OOP Patterns | ✅ 20+ patterns used | Type-safe errors + effects | Async/await |
| Capabilities | 100+ subsystems | Bootable ISO | Mobile + Cloud |

---

## 1 · Design Improvements

### 1.1 Design Tokens (implement in code)

**Status**: Defined in `docs/Design_System.md`, not yet in code.  
**Action**: Create `userland/desktop/sigma_design_tokens.rs`

```rust
pub mod tokens {
    pub mod color {
        pub const BG:      Color = Color::hex(0x07080C);
        pub const SURFACE: Color = Color::rgba(31,33,42,153);
        pub const ACCENT:  Color = Color::hex(0x45F3FF);
        // ... all tokens as constants
    }
    pub mod spacing {
        pub const XS: u32 =  8;
        pub const SM: u32 = 12;
        pub const MD: u32 = 16;
        pub const LG: u32 = 24;
        pub const XL: u32 = 32;
    }
    pub mod radius {
        pub const SM: u32 =  6;
        pub const MD: u32 = 10;
        pub const LG: u32 = 14;
        pub const XL: u32 = 18;
        pub const FULL: u32 = 9999;
    }
    pub mod motion {
        pub const MICRO:  f32 = 0.080;
        pub const FAST:   f32 = 0.150;
        pub const NORMAL: f32 = 0.250;
        pub const SLOW:   f32 = 0.350;
    }
}
```

### 1.2 Icon System

**Status**: Planned.  
**Action**: Create `userland/desktop/sigma_icons.rs` — 24×24 bitmap icons, SVG-to-bitmap rasterizer.

```rust
pub enum Icon {
    Home, Settings, Search, User, Lock, Wifi, Battery,
    Add, Close, Edit, Delete, Copy, Share, Download, Upload,
    Terminal, Browser, Editor, Files, Calculator, Calendar,
    // 100+ icons
}
pub struct IconAtlas { data: Vec<u8>, w: u32, h: u32 }
impl IconAtlas {
    pub fn get(&self, icon: Icon, size: u32) -> ImageRef;
}
```

### 1.3 Typography Engine

**Status**: Placeholder (no font rendering).  
**Action**: Create `userland/desktop/sigma_font.rs`:
- Phase 1: 8×16 VGA bitmap font (already in sigma_vesa.zig)
- Phase 2: PSF (PC Screen Font) loader for richer glyphs
- Phase 3: TrueType outline rasterizer (Bézier curve renderer)

```rust
pub trait FontEngine {
    fn measure(&self, text: &str, size: u32) -> (u32, u32);
    fn render(&mut self, text: &str, x: i32, y: i32,
              size: u32, color: Color, canvas: &mut Canvas);
}
pub struct BitmapFont { glyphs: [[u8;16]; 256] }
pub struct VectorFont { /* TrueType data */ }
```

### 1.4 Dark/Light Adaptive Components

All components must respond to theme changes at runtime:

```rust
pub trait Themed {
    fn on_theme_change(&mut self, theme: &ThemeEngine);
}
// Every widget implements Themed for live theme switching
```

### 1.5 Visual Consistency Checklist

- [ ] All interactive elements have hover + active + focus states
- [ ] All text meets 4.5:1 contrast (auto-verified by `sigma-a11y-check`)
- [ ] Consistent 8px grid for all spacing
- [ ] Icons at 16/20/24/32px only (no non-standard sizes)
- [ ] Animations respect `reduce_motion` system preference

---

## 2 · Application Plan

### 2.1 Tier-1 Apps (ship before v0.1 release)

#### sigma-terminal

```
File:   userland/apps/sigma_terminal.rs
Lang:   Rust
Engine: VTE grid + PTY + sigma-renderer
Target: 60fps, < 100ms startup, 100K line scrollback
```

Key implementation tasks:
1. `TermGrid`: 2D array of `Cell { char, fg, bg, attrs }`
2. `PtyFork`: fork sigma-sh with PTY master/slave
3. `VtParser`: ANSI/VT100/VT220 escape sequence parser
4. `TermRenderer`: render grid to sigma_renderer DrawCmds
5. `InputBridge`: route keyboard events → PTY write

#### sigma-files

```
File:   userland/apps/sigma_files.rs
Lang:   Rust
Engine: sigma VFS API + sigma-renderer
Target: list 10K files < 50ms, navigate < 16ms
```

Key tasks:
1. `DirectoryModel`: async dir read + sort/filter
2. `ListView`: virtual scrolling (only render visible rows)
3. `PreviewPanel`: file type detection → preview renderer
4. `OperationQueue`: copy/move/delete as cancellable async ops
5. `BookmarkStore`: persistent bookmarks via sigma-vault

#### sigma-edit

```
File:   userland/apps/sigma_edit.rs
Lang:   Rust
Engine: Piece-tree buffer + incremental highlight
Target: keystroke < 5ms, open 10MB < 200ms
```

Key tasks:
1. `PieceTree`: O(1) insert/delete, O(log n) line query
2. `SyntaxHighlighter`: regex-based tokeniser per language
3. `CursorManager`: multiple cursors with selection regions
4. `LspClient`: Language Server Protocol over sigma-bus IPC
5. `GutterRenderer`: line numbers + git diff + breakpoints

### 2.2 App Framework (shared infrastructure)

```
File: userland/apps/sigma_app_framework.rs
```

Every app uses:
```rust
pub trait SigmaApp: Send {
    type Model: Clone + Default;
    type Msg: Send;

    // Lifecycle
    fn init(&self) -> Self::Model;
    fn update(&self, model: &mut Self::Model, msg: Self::Msg);
    fn view(&self, model: &Self::Model) -> Vec<DrawCmd>;
    fn subscriptions(&self, model: &Self::Model) -> Vec<Sub<Self::Msg>>;

    // Identity
    fn app_id(&self)    -> &'static str;
    fn app_name(&self)  -> &'static str;
    fn app_icon(&self)  -> Icon;
    fn version(&self)   -> &'static str;
}
```

---

## 3 · UI/UX Improvements

### 3.1 Missing Widgets (priority order)

| Widget | File | ETA |
|---|---|---|
| `Toggle` | sigma_widgets.rs | v0.1 |
| `Slider` | sigma_widgets.rs | v0.1 |
| `ListView` | sigma_widgets.rs | v0.1 |
| `DropDown` | sigma_widgets.rs | v0.1 |
| `ProgressBar` | sigma_widgets.rs | v0.1 |
| `Modal` | sigma_widgets.rs | v1.0 |
| `Tooltip` | sigma_widgets.rs | v1.0 |
| `TabBar` | sigma_widgets.rs | v1.0 |
| `TreeView` | sigma_widgets.rs | v1.0 |
| `ColorPicker` | sigma_widgets.rs | v1.5 |
| `DatePicker` | sigma_widgets.rs | v1.5 |
| `RichTextEditor` | sigma_widgets.rs | v1.5 |

### 3.2 UX Flows to Implement

| Flow | File | Priority |
|---|---|---|
| First-boot onboarding wizard | `userland/installer/sigma_onboarding.rs` | 🔴 |
| App permissions prompt | `userland/desktop/sigma_permission_dialog.rs` | 🔴 |
| Quick Settings panel (swipe down) | `userland/desktop/sigma_quick_settings.rs` | 🟠 |
| App Switcher (Super key) | `userland/desktop/sigma_app_switcher.rs` | 🟠 |
| Global Search | `userland/tools/sigma_search.rs` | 🟠 |
| Drag-to-tile window snapping | Extend sigma_wm.rs | 🟡 |
| Notification action centre | Extend sigma_notifications.rs | 🟡 |
| Screen lock with clock | `userland/desktop/sigma_lock_screen.rs` | 🟡 |

### 3.3 Accessibility Gaps

| Gap | Action | File |
|---|---|---|
| TTS audio output | Integrate espeak-ng-style synthesizer | `userland/accessibility/sigma_tts.rs` |
| Keyboard-only navigation | Focus order + ARIA roles on all widgets | sigma_widgets.rs |
| Screen magnifier | Pixel-doubled overlay compositing | `userland/desktop/sigma_magnifier.rs` |
| Voice control | sigma-ai intent → sigma-sh command | `userland/accessibility/sigma_voice_control.rs` |
| Braille display driver | HID USB braille protocol | `drivers/input/sigma_braille.rs` |

### 3.4 Mobile/Adaptive UI

```
Breakpoints:
  < 480px  → phone layout (bottom nav, full-screen apps)
  480-1024px → tablet layout (split-view, floating panels)
  > 1024px → desktop layout (current)

Required:
- BottomNavigationBar widget
- SplitView container
- AdaptiveLayout wrapper that switches based on screen size
```

---

## 4 · Performance Plan

### 4.1 Frame Rate Optimisation

```
Current bottleneck: Software fill_rect = O(W×H) pixels per frame
Solution path:
  1. Damage tracking (only repaint changed regions)       ← IMPLEMENT NOW
  2. Layer caching (cache static layers as bitmaps)       ← v0.1
  3. GPU compositing via VirtIO-GPU + Mesa Vulkan          ← v1.0
  4. Hardware KMS direct scanout (no compositor copy)      ← v1.5
```

**Immediate action — damage tracking**:

```rust
pub struct DamageTracker {
    dirty_rects: Vec<Rect>,
    merged:      Option<Rect>,
}
impl DamageTracker {
    pub fn mark(&mut self, r: Rect) { self.dirty_rects.push(r); self.merged = None; }
    pub fn merged_rect(&mut self) -> Option<Rect> {
        if self.dirty_rects.is_empty() { return None; }
        let mut m = self.dirty_rects[0];
        for &r in &self.dirty_rects[1..] { m = m.union(r); }
        self.merged = Some(m);
        self.merged
    }
    pub fn clear(&mut self) { self.dirty_rects.clear(); }
}
```

### 4.2 Memory Optimisation Plan

```
Target: < 256MB idle desktop RAM

Actions:
1. Lazy daemon startup: only start sigmad-metrics when first queried
2. Shard compression: compress loaded shards > 1MB with zstd
3. Arena allocator for widget trees (free all at once on screen change)
4. String interning for frequently repeated strings (paths, app names)
5. Shared read-only pages between processes (same code pages)
```

### 4.3 Boot Time Optimisation

```
Phase 1 (sigma-boot.zig):
  ├─ Read kernel + initramfs in parallel DMA transfers
  └─ Set GOP framebuffer before jumping to kernel (splash screen)

Phase 2 (kernel init):
  ├─ Initialise scheduler and memory manager in < 10ms
  ├─ Probe only necessary hardware (lazy driver init)
  └─ Mount tmpfs + extract initramfs via DMA

Phase 3 (sigma-init):
  ├─ Start sigmad-health and sigmad-netd in parallel
  ├─ Defer non-critical daemons (metrics, telemetry) by 5s
  └─ Pre-warm sigma-sh in background while desktop loads

Phase 4 (Zenith):
  ├─ Display splash screen immediately (< 100ms after kernel)
  ├─ Load launcher + panel before any apps
  └─ App thumbnails generate lazily in background
```

---

## 5 · OOP Architecture Improvements

### 5.1 Error Hierarchy (replace string errors)

```rust
// Current: Err(VfsError::IoError)  — lacks context
// Target:
#[derive(Debug)]
pub enum SigmaError {
    Kernel    { source: KernelError,   context: &'static str },
    Fs        { source: FsError,       path: Option<String>  },
    Net       { source: NetError,      addr: Option<String>  },
    Security  { source: SecurityError, pid: Option<u32>      },
    Crypto    { source: CryptoError,   algorithm: &'static str },
    Io        { source: IoError,       operation: &'static str },
}
impl SigmaError {
    pub fn context(self, ctx: &'static str) -> Self { ... }
}
```

### 5.2 Capability-Typed API

```rust
// Prevent calling privileged APIs without capability token
pub struct CapScope<R: RightKind>(PhantomData<R>);
pub struct ReadRight;
pub struct WriteRight;

impl CapScope<WriteRight> {
    pub fn write_file(&self, fd: u64, data: &[u8]) -> usize { ... }
}
// Compiler error if you try to call write without WriteRight capability
```

### 5.3 Plugin / Extension System

```rust
pub trait Plugin: Send + Sync {
    fn id(&self)      -> &'static str;
    fn version(&self) -> &'static str;
    fn init(&mut self, ctx: &mut PluginContext) -> Result<(), PluginError>;
    fn shutdown(&mut self);
}

pub struct PluginManager {
    plugins: Vec<Box<dyn Plugin>>,
}
impl PluginManager {
    pub fn load_sigpkg(&mut self, path: &str) -> Result<(), PluginError>;
    pub fn unload(&mut self, id: &str);
    pub fn get<T: Plugin + 'static>(&self) -> Option<&T>;
}
```

### 5.4 Reactive State (MVI pattern)

```rust
// Model-View-Intent: unidirectional data flow
pub struct Store<S, A> {
    state:      S,
    reducer:    fn(&S, A) -> S,
    subscribers: Vec<Box<dyn Fn(&S)>>,
}
impl<S: Clone, A> Store<S, A> {
    pub fn dispatch(&mut self, action: A) {
        self.state = (self.reducer)(&self.state, action);
        for sub in &self.subscribers { sub(&self.state); }
    }
    pub fn subscribe(&mut self, f: impl Fn(&S) + 'static) {
        self.subscribers.push(Box::new(f));
    }
}
```

---

## 6 · Capabilities Expansion

### 6.1 Near-term Capabilities

| Capability | Status | Action |
|---|---|---|
| Bootable ISO | ⬜ | kernel scheduler + MM + VFS + shell |
| sigma-pkg online | ⬜ | Set up pkg.sigmaos.app registry |
| Wi-Fi connection UI | ⬜ | sigma-netctl GUI in settings |
| Vulkan output | ⬜ | virtio-gpu + Mesa path |
| ARM64 boot (RPi4) | ⬜ | BCM2711 BSP + GIC driver |

### 6.2 Platform Capabilities Matrix

| Feature | Desktop | Mobile | Cloud | RTOS | Browser |
|---|---|---|---|---|---|
| Zenith DE | ✅ | Adaptive | ⬜ | — | ✅ WASM |
| sigma-pkg | ✅ | ✅ | ✅ | Minimal | ✅ |
| TLS 1.3+Kyber | ✅ | ✅ | ✅ | ✅ | ✅ |
| GPU acceleration | ⬜ v1.0 | ⬜ v1.5 | ⬜ v1.0 | — | ⬜ WebGPU |
| sigma-ai (LLM) | ✅ | ARM opt | Cloud API | — | WASM |
| OCI containers | ✅ | ⬜ | ✅ | — | — |

---

## 7 · Implementation Schedule

### Sprint 1 (Month 1–2): Foundation
- [ ] Design tokens → `sigma_design_tokens.rs`
- [ ] Damage tracking in compositor
- [ ] Toggle + Slider + ListView widgets
- [ ] sigma-terminal MVP (PTY + VTE + basic ANSI)

### Sprint 2 (Month 2–3): Usability  
- [ ] Onboarding wizard (5 steps)
- [ ] Quick Settings panel
- [ ] Font rendering (PSF bitmap)
- [ ] App Switcher (Super key)

### Sprint 3 (Month 3–4): Apps
- [ ] sigma-files MVP (browse + basic ops)
- [ ] sigma-edit MVP (open/edit/save)
- [ ] sigma-calc complete
- [ ] sigma-screenshot

### Sprint 4 (Month 4–6): Performance
- [ ] GPU compositing via VirtIO-GPU
- [ ] Boot time < 2.5s on QEMU
- [ ] sigma-pkg online registry live
- [ ] sigma-bench CI gates

### Sprint 5 (Month 6–9): Polish
- [ ] Accessibility TTS
- [ ] Mobile adaptive layouts
- [ ] ARM64 RPi4 boot
- [ ] First public alpha release

---

## 8 · Measurement & Tracking

### Weekly metrics (CI + manual)

```bash
sigma-bench all --output weekly-$(date +%Y%W).json
sigma-a11y-check userland/desktop/
sigma-lint --oop-patterns kernel/ userland/
sigma-loc-count  # lines of code by language
```

### Quality gates (no PR merges without passing)

| Gate | Threshold |
|---|---|
| Test coverage | ≥ 80% for kernel subsystems |
| OOP trait documentation | 100% of public traits |
| Performance regression | < 10% vs baseline |
| Accessibility contrast | ≥ 4.5:1 for all text |
| Reproducible build | Bit-identical on rebuild |

---

*References: [Design_System.md](Design_System.md) · [App_Development_Roadmap.md](App_Development_Roadmap.md) · [Performance_Targets.md](Performance_Targets.md) · [OOP_Architecture.md](OOP_Architecture.md) · [UI_UX_Performance_Plan.md](UI_UX_Performance_Plan.md) · [Adoption_Strategy.md](Adoption_Strategy.md)*
