# HiDPI and Scale Factor Management

SigmaOS's **Omarchy HiDPI Scale Engine** provides automatic, per-app HiDPI scaling configuration for Wayland compositors — inspired by the Omarchy `1password-scale-factor` branch and vastly extended.

---

## Architecture Comparison

| Metric | Omarchy 1password-scale-factor | Linux Mint (HiDPI Wizard) | SigmaOS HiDPI Scale Engine |
|---|---|---|---|
| Core language | Shell / env vars | Python / GSettings | Safe Rust (`#![no_std]`) |
| App scope | 1Password only | Global only | Per-app + global override |
| Fractional scale | No | No | Yes (any `n/d` ratio) |
| Hyprland config gen | Manual | — | **Auto-generated** env block |
| XWayland DPI | Manual xrandr | Manual | Per-app xrandr override registry |
| External deps | Shell | Python | **Zero** |
| Scale profiles | Fixed | Fixed | 5 presets + custom fraction |

---

## Architectural Highlights

- **5 preset scales** — 1x, 1.5x, 2x, 2.5x, 3x; plus `Custom(n, d)` for any fraction
- **Per-app XWayland DPI** — registers expected XWayland DPI per app_id; eliminates blurry Electron apps
- **Hyprland env block generator** — emits `monitor = ,preferred,auto,<scale>` + `env = GDK_SCALE,<n>` lines
- **1Password-specific helper** — `onepassword_env_scale()` returns the exact `GDK_SCALE` / `GDK_DPI_SCALE` needed for fractional scales
- **Composable** — plugs into the `OmarchyHyprlandConfigEngine` for full config generation

---

## API & Usage

```rust
use sigmaos::desktop::omarchy_hidpi_scale_engine::{
    OmarchyHiDpiScaleEngine, HiDpiScale
};

let mut engine = OmarchyHiDpiScaleEngine::new();

// Set 4K display — 2x scale
engine.set_global_scale(HiDpiScale::Scale2x);

// Register apps that need explicit patching
engine.register_scale_target("1password", 192);
engine.register_scale_target("slack", 192);
engine.register_scale_target("zoom", 192);

// 1Password env var
let (key, val) = engine.onepassword_env_scale();
// → ("GDK_SCALE", "2")

// Inject into Hyprland config
let block = engine.generate_hyprland_env_block();
// monitor = ,preferred,auto,2
// env = GDK_SCALE,2   # 1password
// env = GDK_SCALE,2   # slack
```

---

## Supported Scale Presets

| Preset | Factor | Typical Display |
|---|---|---|
| `Scale1x` | 1.0 | 1080p (96 DPI) |
| `Scale1_5x` | 1.5 | 1440p laptop |
| `Scale2x` | 2.0 | 4K display |
| `Scale2_5x` | 2.5 | 5K iMac-equivalent |
| `Scale3x` | 3.0 | High-DPI mobile/tablet |
| `Custom(n, d)` | n/d | Any fractional ratio |

---

## Testing

```bash
rustc --test src/desktop/omarchy_hidpi_scale_engine.rs \
  --edition=2021 --cfg 'feature="standalone_test"' \
  -o build/test_hidpi && ./build/test_hidpi
# test result: ok. 1 passed; 0 failed
```

---

## Related Components

- [Desktop Environment](Desktop-Environment.md) — Hyprland integration
- [Omarchy Parity](Omarchy-Parity.md) — full feature comparison
- [Browser Theme Sync](Browser-Theme-Sync.md) — Wayland DPI-aware theme sync
