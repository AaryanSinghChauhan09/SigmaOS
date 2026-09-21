# 🖥️🔍 SIGMAOS OMARCHY DISPLAY, DPI SCALING & MULTI-MONITOR BLUEPRINT
## Comprehensive Architecture, Retina Scaling Matrix, and 4-Phase Execution Roadmap for High-PPI Displays, Multi-Monitor Topology, DDC/CI & Apple Display Controls for https://github.com/AaryanSinghChauhan09/SigmaOS

---

## EXECUTIVE SUMMARY & CONCEPTUAL VISION

Display technology ranges from standard 1080p/1440p screens to high-density Retina-class displays (>=218 PPI, such as 27" 5K Apple Studio Display / Asus ProArt PA27JCV / Samsung S9 / Kuycon G27P, and 32" 6K Apple Pro Display XDR / Asus ProArt PA32QCV / Kuycon G32P). Inspired by **Omarchy Linux** (the Arch Linux + Hyprland + Quickshell distribution designed by DHH & 37signals), **SigmaOS** delivers an integrated **Display, DPI Scaling & Multi-Monitor Subsystem** (`src/desktop/display_scaling.rs`).

This subsystem manages monitor scaling, Hyprland `monitors.lua` configuration, dynamic scaling shortcuts, unified font pixel size overrides, multi-monitor extend/mirror topologies, laptop lid state triggers, DDC/CI brightness controls, and Apple Display `asdcontrol` integration.

---

## PART 1: OMARCHY DISPLAY SCALING & MULTI-MONITOR SPECIFICATION

### 1. Default Display Scaling Matrix
- **2x Retina Default (>=218 PPI)**: Apple Studio Display 5K (218 PPI), Apple Pro Display XDR 6K (218 PPI), Asus ProArt PA27JCV/PA32QCV, Samsung S9 5K, Kuycon G27P/G32P. Configured with `omarchy_gdk_scale = 2` and `omarchy_monitor_scale = 2.0`.
- **4K Displays (27" / 32" 4K)**: Fractional scaling recommended at `omarchy_gdk_scale = 2` and `omarchy_monitor_scale = 1.6`.
- **1080p / 1440p Displays**: Standard 1x scaling configured with `omarchy_gdk_scale = 1` and `omarchy_monitor_scale = 1.0`.

### 2. Dynamic Scaling Shortcuts & Config Persistence
- **Step Higher (`Super + /`)**: Step up through major monitor scaling ratios: `1.0x` -> `1.25x` -> `1.6x` -> `2.0x` -> `3.0x` -> `4.0x`.
- **Step Lower (`Super + Alt + /`)**: Step down through major monitor scaling ratios.
- **Config Persistence**: Automatically persists active scaling configuration into `~/.config/hypr/monitors.lua`.

### 3. Unified Text Size Control (`omarchy display text size`)
- Single control knob adjusting shell, GTK, and terminal text sizes together (pixel size range: 9px to 20px).
- Command usage:
  - `omarchy display text size 14`: Set text size to 14px.
  - `omarchy display text size`: Display current text size.
  - `omarchy display text size reset`: Reset text size to default (11px).

### 4. Multi-Monitor Extension, Mirroring & Laptop Lid Controls
- **Automatic Extension**: External displays extend desktop space automatically on connection.
- **Mirroring Toggle (`Super + Ctrl + Alt + Delete`)**: Switch between extended and mirrored display mode.
- **Laptop Lid Toggle (`Super + Ctrl + Delete`)**: Closing lid turns off internal screen when extended; opening lid re-enables internal display.
- **Workspace Pinning**: Workspace-to-monitor bindings defined via `hl.monitor` entries in `monitors.lua`.

### 5. DDC/CI & Apple Display Brightness Control
- Dedicated brightness function keys (`Fn + BrightnessUp/Down`). Holding `Shift` jumps directly to minimum (0%) or maximum (100%) brightness.
- Hardware DDC/CI protocol support for external displays.
- Apple Display brightness control via native `asdcontrol` protocol. Disabled phantom screens (e.g., DP-2 on 6K XDR displays) via `hl.monitor({ output = "DP-2", disabled = true })`.

---

## PART 2: CORE ARCHITECTURAL PILLARS FOR SIGMAOS

```
                 +-------------------------------------------------+
                 |  SIGMAOS OMARCHY DISPLAY & SCALING ENGINE       |
                 +-------------------------------------------------+
                                          |
      +-------------------+---------------+---------------+-------------------+
      |                   |               |               |                   |
      v                   v               v               v                   v
🖥️ RETINA 2X / 1.6X   ⌨️ SCALING STEPPER 🎚️ UNIFIED TEXT SIZE 💻 EXTEND / MIRROR   ☀️ DDC/CI & APPLE
  HYPRLAND MONITORS.LUA Super+/ & Alt+/   omarchy display text LAPTOP LID TOGGLE    ASDCONTROL
  • >=218 PPI 5K/6K   • 1x, 1.25x, 1.6x,  • Pixel range 9-20px • Super+Ctrl+Alt+Del • Fn + Brightness
  • Fractional 1.6x    2x, 3x, 4x Stepper • Shell + GTK + Term• Lid Close/Open Sync• Shift Min/Max
  • Lua Config Export • Reboot Persist    • Reset Command    • Workspace Pinning • Phantom DP-2 Off
```

---

## PART 3: 4-PHASE DEVELOPMENT ROADMAP

### PHASE 1: Display Scaling & Hyprland `monitors.lua` Parser
- Implement `OmarchyDisplayScalingEngine` in `src/desktop/display_scaling.rs` for tracking monitor scales and exporting `monitors.lua`.
- Provide predefined presets for 5K/6K Retina (2.0x), 4K (1.6x), and 1080p/1440p (1.0x).

### PHASE 2: Unified Text Size Knob & Font DPI Override
- Implement `execute_text_size_command(args)` parsing `omarchy display text size <9-20>` and `reset`.
- Update system font DPI settings across Zenith shell, GTK 3/4 settings, and terminal profiles in proportion.

### PHASE 3: Multi-Monitor Extend/Mirror & Lid Toggle State Machine
- Implement display topology switcher toggling between `Extended` and `Mirrored` modes.
- Integrate laptop lid state sensor (`LidClosed` / `LidOpened`) automatically toggling internal panel state during multi-monitor extension.

### PHASE 4: DDC/CI & Apple Display `asdcontrol` Brightness Control
- Integrate DDC/CI I2C brightness adjustments for external monitors.
- Support Apple Studio/XDR `asdcontrol` USB control protocol and phantom screen disable rules (`DP-2` disabled).

---

## PART 4: VERIFICATION BENCHMARK & TEST CRITERIA

1. **Monitors.lua Serialization Unit Tests**: Confirm generated Lua config contains correct `omarchy_gdk_scale` and `omarchy_monitor_scale` values.
2. **Scaling Stepper Unit Tests**: Verify `Super+/` steps through 1x -> 1.25x -> 1.6x -> 2x -> 3x -> 4x and `Super+Alt+/` steps down correctly.
3. **Text Size CLI Unit Tests**: Validate pixel size bounds enforcement (9px - 20px) and reset behavior.
4. **Lid & Topology Unit Tests**: Test extended-to-mirrored toggles and internal panel disable upon laptop lid closure.

---
*End of SigmaOS Omarchy Display, DPI Scaling & Multi-Monitor Blueprint Specification.*
