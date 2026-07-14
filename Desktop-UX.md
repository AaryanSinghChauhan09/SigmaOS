# SigmaOS Desktop UX — Zenith Control Center

> **Status**: ✅ Implemented — prototype in `userland/gui/sigma_control_center.nim`  
> **Language**: Nim (freestanding, OOP widget hierarchy)  
> **Branch**: `feature/multi-lang-impl-batch1`

---

## Overview

Zenith is SigmaOS's sovereign desktop compositor. It implements a full widget hierarchy using method-dispatch OOP without any OS windowing toolkit (no GTK, no Qt, no SDL). All rendering is done via a hand-rolled framebuffer with ARGB8888 pixels.

## Architecture

```
WindowManager
├── Framebuffer (1920×1080, ARGB8888 pixels)
├── PanelWidget (top status bar — derives DesktopWidget)
└── AppLauncher  (dock — derives DesktopWidget)
```

## Widget Hierarchy (OOP)

```
DesktopWidget (ref object of RootObj)  ← abstract base
├── PanelWidget                         ← top bar
├── AppLauncher                         ← dock with icon grid
└── (future) WindowTile                 ← tiling window frame
```

## Framebuffer Engine

- Resolution: 1920 × 1080 (configurable via `const SCREEN_W/H`)
- Format: ARGB8888 (`Pixel = SigmaU32`)
- Operations: `setPixel`, `fillRect` — all bounds-checked, no exceptions
- Background: `0xFF1A1A2E` (deep navy Sigma theme)

## Event System

```nim
type EventKind = enum
  evMouseDown, evMouseUp, evMouseMove, evKeyDown, evKeyUp, evResize, evPaint
```

Events dispatched via `WindowManager.dispatchEvent()` → routed to each widget.

## Color Palette

| Color | Hex | Use |
| :--- | :--- | :--- |
| Sigma Purple | `#6C63FF` | Panel accent, active states |
| Deep Navy | `#1A1A2E` | Background |
| Sky Blue | `#00BFFF` | Terminal icon |
| Sigma Orange | `#FF8C00` | Files icon |
| Dock Glass | `#0D0D1A CC` | Semi-transparent dock bar |

## Implementation Files

| File | Language | Description |
| :--- | :--- | :--- |
| `userland/gui/sigma_control_center.nim` | Nim | Full desktop compositor |

## Test Coverage

```nim
proc testDesktopUX*(): bool
# WindowManager init → Dock populated → Frame painted → Pixel verified
```

## Future Work

- [ ] Text rendering (bitmap font from `sigma_font.bin`)
- [ ] Window tiling manager (`WindowTile` widget)
- [ ] Animation system (delta-time frame loop)
- [ ] Wayland protocol adapter via SigmaOS IPC
- [ ] Touchscreen event normalization
