# SigmaOS Improvement Sprints — Detailed Task Breakdown

> Sprint-level task breakdown for all improvement areas.
> Each task maps to: file path + estimated effort + acceptance criteria.

---

## Sprint 1: Foundation (Month 1–2)

### S1.1 Design Token Implementation

**File**: `userland/desktop/sigma_design_tokens.rs`  
**Effort**: 1 day  
**AC**: All widgets source colour/spacing/radius from tokens, not hardcoded hex

```rust
// Task: convert all Color::hex(0x45F3FF) → tokens::color::ACCENT
// Run: grep -r "Color::hex" userland/desktop/ | wc -l
// Target: 0 hardcoded colors in widget files
```

### S1.2 Damage Tracking

**File**: `userland/desktop/sigma_compositor.rs` (extend)  
**Effort**: 2 days  
**AC**: Frame time at 1080p drops from ~8ms to ~2ms for static screen

```rust
// Rect::union(a, b) helper
// DamageTracker::mark(rect) / merged_rect() / clear()
// Compositor::render() only repaints merged_rect
```

### S1.3 Widget Expansion — Toggle + Slider + ListView

**File**: `userland/desktop/sigma_widgets.rs` (extend)  
**Effort**: 3 days  
**AC**: Toggle animates with spring, Slider responds to drag, ListView virtual scrolls

```rust
pub struct Toggle    { base: BaseWidget, on: bool, spring: Spring }
pub struct Slider    { base: BaseWidget, value: f32, min: f32, max: f32, dragging: bool }
pub struct ListView  { base: BaseWidget, items: Vec<ListItem>, scroll_y: f32, row_height: u32 }
```

### S1.4 sigma-terminal MVP

**File**: `userland/apps/sigma_terminal.rs` (new)  
**Effort**: 5 days  
**AC**: Can run sigma-sh, display coloured output, scroll, copy/paste

```rust
struct TermGrid  { cells: Vec<Cell>, cols: u32, rows: u32 }
struct Cell      { ch: char, fg: Color, bg: Color, bold: bool, underline: bool }
struct VtParser  { state: VtState, params: Vec<u32> }
struct PtyMaster { fd: i32 }
```

---

## Sprint 2: Usability (Month 2–3)

### S2.1 Onboarding Wizard

**File**: `userland/installer/sigma_onboarding.rs` (new)  
**Effort**: 3 days  
**AC**: 5-step wizard completes, writes user config, starts desktop

```
Step 1: Welcome screen + language picker
Step 2: Privacy consent (all off default)
Step 3: Create user account + set hostname
Step 4: Disk partition wizard (calls sigma-disks)
Step 5: Hardware detection + driver status summary
```

### S2.2 Quick Settings Panel

**File**: `userland/desktop/sigma_quick_settings.rs` (new)  
**Effort**: 2 days  
**AC**: Swipe-down from panel shows animated overlay with 12 toggles + sliders

```rust
pub struct QuickSettingsPanel {
    visible:   bool,
    y_spring:  Spring,    // slides down from -panel_height to 0
    toggles:   [QuickToggle; 8],
    sliders:   [QuickSlider; 2],  // brightness + volume
}
```

### S2.3 App Switcher

**File**: `userland/desktop/sigma_app_switcher.rs` (new)  
**Effort**: 2 days  
**AC**: Super key shows window thumbnails, click to focus, drag to workspace

```rust
pub struct AppSwitcher {
    visible:    bool,
    thumbnails: Vec<WindowThumbnail>,
    selected:   Option<SurfaceId>,
    animation:  Animation,    // scale 0.0→1.0 on open
}
```

### S2.4 Font Rendering — PSF Bitmap

**File**: `userland/desktop/sigma_font.rs` (new)  
**Effort**: 3 days  
**AC**: Text renders in sigma-terminal and settings hub; Outfit Regular at 14px

```rust
pub struct PsfFont { glyphs: Vec<[u8;32]>, width:u32, height:u32, n_glyphs:u32 }
impl FontEngine for PsfFont {
    fn render(&mut self, text: &str, x:i32, y:i32, _size:u32, color:Color, canvas:&mut Canvas);
}
```

---

## Sprint 3: Apps (Month 3–4)

### S3.1 sigma-files MVP

**File**: `userland/apps/sigma_files.rs` (new)  
**Effort**: 5 days  
**AC**: Browse filesystem, copy/move/delete files, show file info

```rust
struct DirectoryModel { path: PathBuf, entries: Vec<DirEntry>, sort: SortBy, filter: String }
struct ListView       { /* virtual scrolling — see S1.3 */ }
struct FileOp         { kind: FileOpKind, src: Vec<PathBuf>, dst: PathBuf, progress: f32 }
```

### S3.2 sigma-edit MVP

**File**: `userland/apps/sigma_edit.rs` (new)  
**Effort**: 7 days  
**AC**: Open/edit/save files; basic syntax highlighting for Rust, Python, plain text

```rust
struct PieceTree     { original: String, adds: Vec<String>, pieces: Vec<Piece> }
struct TextEditor    { buffer: PieceTree, cursors: Vec<Cursor>, viewport: Rect }
struct SyntaxTheme   { keywords: Color, strings: Color, comments: Color, types: Color }
```

### S3.3 sigma-calc

**File**: `userland/apps/sigma_calc.nim` (new)  
**Effort**: 2 days  
**AC**: Scientific calculator with unit converter; trig functions, base conversion

```nim
type
  CalcState = object
    display:  string
    memory:   float64
    history:  seq[string]
    mode:     CalcMode  # Standard | Scientific | Base
```

### S3.4 sigma-screenshot

**File**: `userland/apps/sigma_screenshot.nim` (new)  
**Effort**: 2 days  
**AC**: Capture full screen, window, or region; annotate; save PNG; copy to clipboard

```nim
type ScreenshotMode = enum Full, Window, Region, Timed
proc capture(mode: ScreenshotMode): string  # returns temp path
proc annotate(path: string): string
proc copy_to_clipboard(path: string)
```

---

## Sprint 4: Performance (Month 4–6)

### S4.1 GPU Compositing

**Files**: `drivers/gpu/sigma_vulkan.rs`, extend `userland/desktop/sigma_compositor.rs`  
**Effort**: 10 days  
**AC**: 60fps at 1080p via VirtIO-GPU in QEMU; software path as fallback

```
VirtIO-GPU → DRM/KMS atomic → Mesa Gallium Vulkan ICD → VkSwapchain
```

### S4.2 Boot Time

**Target**: < 2.5s from UEFI to sigma-sh prompt  
**Effort**: 5 days

```
1. sigma-boot.zig: parallel DMA for kernel + initramfs load
2. kernel: lazy driver init (probe only NVMe + e1000 at boot)
3. sigma-init: parallel daemon start with dependency graph
4. Zenith: show splash frame within 500ms of kernel start
```

### S4.3 sigma-pkg Online Registry

**Files**: New registry server (Go or Nim), CI publish workflow  
**Effort**: 5 days  
**AC**: `sigma-pkg install sigma-hello` downloads from pkg.sigmaos.app

```
Registry API:
  GET /v1/index                 → TOML package index (Dilithium-5 signed)
  GET /v1/pkg/{name}/{ver}/{arch} → .sigpkg download
  POST /v1/publish              → upload signed package (CI only)
```

### S4.4 sigma-bench CI Gates

**File**: `.github/workflows/sigma_bench.yml` (new)  
**Effort**: 2 days  
**AC**: PR fails if any P1 metric regresses > 10%

```yaml
- name: Run benchmarks
  run: sigma-bench all --baseline baseline.json --fail-on-regression 0.10
- name: Post results
  uses: actions/github-script@v7
  with: script: github.issue.createComment(benchmarkReport)
```

---

## Sprint 5: Polish + Alpha (Month 6–9)

### S5.1 Accessibility TTS

**File**: `userland/accessibility/sigma_tts.rs` (new)  
**Effort**: 5 days  
**AC**: Screen reader announces focused element; espeak-compatible phoneme synthesizer

### S5.2 Mobile Adaptive Layout

**File**: extend all widgets + add `userland/desktop/sigma_adaptive.rs`  
**Effort**: 5 days  
**AC**: sigma-settings and sigma-files look correct on 480px-wide display

### S5.3 ARM64 / Raspberry Pi 4 Boot

**Files**: `kernel/hal/sigma_bcm2711.zig`, `arch/arm64/sigma_gic.zig`  
**Effort**: 7 days  
**AC**: SigmaOS boots to sigma-sh on RPi4; e1000 equivalent via USB Ethernet

### S5.4 Public Alpha Release

**Tasks**:
- [ ] Create v0.1.0 GitHub Release tag
- [ ] Upload signed ISO to release assets
- [ ] Write "Getting Started" blog post
- [ ] Post to HackerNews + r/linux
- [ ] Open GitHub Discussions for feedback

---

## Quality Gates per Sprint

| Gate | S1 | S2 | S3 | S4 | S5 |
|---|---|---|---|---|---|
| All new Rust has trait docs | ✓ | ✓ | ✓ | ✓ | ✓ |
| No hardcoded colors in widgets | ✓ | ✓ | ✓ | ✓ | ✓ |
| Text contrast ≥ 4.5:1 | ✓ | ✓ | ✓ | ✓ | ✓ |
| sigma-bench baseline set | — | ✓ | ✓ | ✓ | ✓ |
| Reproducible build verified | — | — | ✓ | ✓ | ✓ |
| QEMU boot CI green | — | — | — | ✓ | ✓ |
| Accessibility audit pass | — | — | — | — | ✓ |

---

*See: [Master_Improvement_Plan.md](Master_Improvement_Plan.md) · [Performance_Targets.md](Performance_Targets.md)*
