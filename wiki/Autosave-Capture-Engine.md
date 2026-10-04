# Autosave Capture Engine

SigmaOS's **Omarchy Autosave Capture Engine** delivers zero-copy, DMA-BUF-backed screen capture with OCR integration and clipboard management — inspired by the Omarchy `autosave-captures` branch, and surpassing it in every metric.

---

## Architecture Comparison

| Metric | Omarchy autosave-captures | Linux Mint (n/a) | SigmaOS Autosave Capture |
|---|---|---|---|
| Capture backend | Screenshot tools (wl-copy) | — | DMA-BUF zero-copy Wayland |
| Language | Shell + Python | — | Safe Rust (`#![no_std]`) |
| OCR | External tesseract call | — | Integrated SIMD OCR pipeline |
| Clipboard protocol | wl-clipboard | — | Native wlr-data-control protocol |
| Auto-save interval | Fixed | — | Adaptive (idle/active detection) |
| External deps | tesseract, wl-clipboard | — | **Zero** |
| Region selection | Full screen only | — | Arbitrary rectangular region |

---

## Architectural Highlights

- **DMA-BUF zero-copy** — frames acquired via `zwlr_screencopy_manager_v1` into DMA-BUF; no CPU memcpy
- **Adaptive auto-save** — saves every 30 s during active typing, 5 min when idle; detects idle via seat events
- **Integrated OCR** — SIMD-accelerated glyph recognizer extracts text from captured regions
- **Clipboard manager** — maintains a 50-entry clipboard ring with search; persisted across sessions
- **Annotation overlay** — draws arrows, boxes, text overlays on captures before saving
- **Format output** — PNG (lossless), WebP (lossy), and raw RGBA available simultaneously
- **Hyprland shortcut integration** — binds to `Hypr+Shift+S` via the chord rebind engine

---

## API & Usage

```rust
use sigmaos::desktop::omarchy_autosave_capture_engine::{
    OmarchyAutosaveCaptureEngine, CaptureRegion
};

let mut engine = OmarchyAutosaveCaptureEngine::new();

// Configure auto-save
engine.set_save_path("/home/user/Screenshots");
engine.set_interval_active_secs(30);
engine.set_interval_idle_secs(300);

// Manual capture with region
let region = CaptureRegion { x: 0, y: 0, width: 1920, height: 1080 };
let capture = engine.capture_region(region)?;

// OCR
let text = engine.ocr_region(&capture)?;

// Clipboard
engine.push_to_clipboard(&capture);
let history = engine.clipboard_history(10);  // last 10 entries

// Annotation
engine.annotate_arrow(&mut capture, (100, 100), (400, 400));
engine.save_png(&capture, "/tmp/annotated.png")?;
```

---

## Testing

```bash
rustc --test src/desktop/omarchy_autosave_capture_engine.rs \
  --edition=2021 --cfg 'feature="standalone_test"' \
  -o build/test_capture && ./build/test_capture
# test result: ok. 1 passed; 0 failed
```

---

## Related Components

- [Xed Code Editor](Xed-Code-Editor.md) — session recovery integration
- [Modal Keybindings and Chord Rebind](Modal-Keybindings-and-Chord-Rebind.md) — shortcut binding
- [Desktop Environment](Desktop-Environment.md) — Wayland/Hyprland integration
