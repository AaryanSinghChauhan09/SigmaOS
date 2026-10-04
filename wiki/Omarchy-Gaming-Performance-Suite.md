# Omarchy Gaming Performance Suite

The **Omarchy Gaming Performance Suite** is SigmaOS's superior reimplementation of Omarchy's gaming-focused features. Implemented in `src/distro/omarchy_gaming_performance_suite.rs`, it provides automated CPU/GPU governor switching, in-game HUD telemetry, and developer stack management — all in pure Rust.

---

## Overview

Omarchy (`github.com/omacom/omarchy`) pioneered a gaming-first Arch Linux configuration with hyprland, automated GPU setup, and developer tooling. SigmaOS takes these ideas further with:

1. **SovereignOmarchyGamingGovernor** — automatic performance mode on game launch
2. **SovereignOmarchyHudEngine** — in-game overlay with zero overhead when hidden
3. **SovereignOmarchyDeveloperStacks** — one-command dev environment provisioning
4. **SovereignOmarchyVrrPacingController** — adaptive VRR/FreeSync/G-Sync frame pacing engine
5. **SovereignSteamShaderPrecacheManager** — intelligent shader cache validation and pre-compilation

---

## Gaming Governor (`SovereignOmarchyGamingGovernor`)

### What It Does
- Detects game launch via process name or `.desktop` category `Game`
- Switches CPU frequency governor to `performance`
- Increases GPU power limit (NVIDIA TDP boost, AMD overdrive)
- Enables AMDGPU overdrive for extra GPU headroom
- Disables compositor frame delay (direct scanout)
- Restores all settings on game exit

### Configuration
```toml
[gaming.governor]
enabled = true
cpu_governor = "performance"        # performance | schedutil | powersave
gpu_boost_nvidia_percent = 20       # TDP increase %
amdgpu_overdrive = true
compositor_bypass = true            # direct scanout in fullscreen
restore_on_exit = true
```

### Supported GPU Backends
| GPU | Feature | Method |
|-----|---------|--------|
| NVIDIA (proprietary) | TDP boost | `nvidia-smi -pl` |
| NVIDIA (nouveau) | Governor only | sysfs |
| AMD (amdgpu) | Overdrive, freq | `/sys/class/drm/card0/device/` |
| Intel Xe | Freq scaling | sysfs |

### Trigger Rules
```toml
[[gaming.triggers]]
name_contains = ["steam", "wine", "proton", "lutris"]
category = "Game"
exe_suffix = [".exe"]               # Wine/Proton games
```

---

## HUD Telemetry Engine (`SovereignOmarchyHudEngine`)

### What It Displays
- FPS (frames per second)
- Frame time (ms) with 1% low / 0.1% low
- GPU temperature and utilization
- VRAM usage
- CPU temperature and per-core usage
- RAM usage
- Network latency (if multiplayer)

### Rendering
- Rendered as a Wayland overlay layer (wlr-layer-shell)
- Zero GPU overhead when HUD is hidden (no rendering)
- Configurable position: TL / TR / BL / BR / center

### MangoHUD Compatibility
SigmaOS HUD is compatible with MangoHUD config format:

```ini
[MangoHud]
fps
gpu_temp
cpu_temp
vram
ram
frame_timing
position=top-left
font_size=24
```

### Shortcuts
| Shortcut | Action |
|----------|--------|
| `Shift+F12` | Toggle HUD |
| `Shift+F11` | Cycle position |
| `Shift+F10` | Screenshot current metrics |

---

## Developer Stacks (`SovereignOmarchyDeveloperStacks`)

One-command provisioning for complete development environments, inspired by Omarchy's opinionated dev setup:

### Supported Stacks

| Stack | Tools Installed | Version Manager |
|-------|----------------|----------------|
| `rust` | rustup, cargo, rust-analyzer, bacon | rustup |
| `go` | go toolchain, gopls, golangci-lint | direct |
| `node` | node, npm, pnpm, typescript, prettier | volta |
| `python` | python3, uv, ruff, pyright | uv |
| `zig` | zig compiler, zls | zigup |
| `nim` | nim, nimble, nimlsp | choosenim |
| `full` | All of the above | combined |

### Usage
```bash
sigma-devstack install rust
sigma-devstack install full
sigma-devstack list
sigma-devstack update all
```

### Implementation Details
- Each stack installs to `~/.sigma/stacks/<name>/`
- PATH injection via `~/.config/sigma/env.d/`
- Isolated: stacks don't conflict with each other
- Shell-agnostic: works with bash, zsh, fish, nushell

---

## VRR Adaptive Frame Pacing (`SovereignOmarchyVrrPacingController`)

### What It Does
- Detects display VRR range (min/max Hz) from EDID via DRM
- Monitors real-time frame deltas and adjusts present timing to avoid tearing at VRR boundaries
- Implements `VrrStrategy`: `Adaptive`, `Fixed(hz)`, `CapToRefresh`, `LatencyFavor`
- Emits `VrrEvent` telemetry: `FrameDropped`, `PaceAdjusted`, `RangeEnforced`
- Interfaces with `SovereignCompositor` for direct scanout scheduling

### Configuration
```toml
[gaming.vrr]
strategy = "adaptive"        # or "fixed", "cap_to_refresh", "latency_favor"
min_hz = 48
max_hz = 165
allow_freesync = true
allow_gsync = true
frame_drop_warn_threshold_ms = 5
```

### How It Beats the Competition
| | Omarchy | Linux Mint | **SigmaOS** |
|--|---------|-----------|-------------|
| VRR detection | ❌ | ❌ | ✅ EDID |
| Pacing algorithm | ❌ | ❌ | ✅ Rust, typed |
| FreeSync support | ✅ partial | ❌ | ✅ full |
| G-Sync compat | ✅ partial | ❌ | ✅ full |

---

## Shader Cache Manager (`SovereignSteamShaderPrecacheManager`)

### What It Does
- Scans `~/.local/share/Steam/steamapps/shadercache/` for all game shader blobs
- Validates each cache entry against current driver version (avoids stale shaders after Mesa/NVIDIA update)
- Invalidates caches that were compiled with an older driver (prevents stutters on first run)
- Optionally triggers pre-compilation via `glslc` / RADV shader pre-warm before game launch
- Reports cache hit ratio, total size, and per-game last-compile timestamps

### Commands
```bash
sigma-shader status          # show cache health for all games
sigma-shader validate 440    # validate cache for Steam AppID 440
sigma-shader precache 440    # pre-warm shaders for AppID 440
sigma-shader purge --stale   # remove all stale (driver-outdated) entries
```

### Implementation
- Driver version fingerprint: reads `/proc/driver/nvidia/version` or Mesa `GL_VERSION`
- Per-game metadata stored in `~/.config/sigma/shader_meta/`
- Runs as a low-priority background daemon (`nice 19`, `ionice idle`)
- Integrates with `SovereignOmarchyGamingGovernor`: auto-precache on governor activation

---

## Comparison with Omarchy Gaming Setup

| Feature | Omarchy (Arch) | **SigmaOS** |
|---------|---------------|-------------|
| GPU governor switch | ✅ (bash script) | ✅ (Rust, typed) |
| CPU governor | ✅ | ✅ |
| AMDGPU overdrive | ✅ manual | ✅ automatic |
| In-game HUD | MangoHUD (C) | ✅ native Rust HUD |
| MangoHUD compat | ✅ | ✅ |
| Dev stacks | ✅ ansible/bash | ✅ Rust, extensible |
| Auto restore | ❌ | ✅ |
| Wayland compositor bypass | ✅ hyprland | ✅ SigmaCompositor |
| VRR / FreeSync frame pacing | ❌ | ✅ adaptive pacing |
| Shader cache validation | ❌ | ✅ driver-aware invalidation |

---

## Integration with SigmaOS

- **Scheduler**: gaming governor requests `SCHED_FIFO` for the game process
- **Memory**: huge pages enabled for game workloads
- **Security**: game processes run in relaxed seccomp profile (allow `ioctl`, `futex`)
- **Thermal**: `src/kernel/acpi_pm.rs` cooling devices monitored; throttle if > 95°C

---

## Source File

- [`src/distro/omarchy_gaming_performance_suite.rs`](file:///home/aaryansinghchauhan/SigmaOS/src/distro/omarchy_gaming_performance_suite.rs)

Key types:
| Type | Description |
|------|-------------|
| `SovereignOmarchyGamingGovernor` | CPU/GPU performance mode manager |
| `GamingProfile` | Per-game settings profile |
| `GpuBackend` | Nvidia / AMD / Intel enum |
| `SovereignOmarchyHudEngine` | Telemetry overlay engine |
| `HudMetrics` | Live FPS/temp/util snapshot |
| `SovereignOmarchyDeveloperStacks` | Dev environment provisioner |
| `DevStack` | Individual stack definition |

---

## AI Agent Maintenance Instructions

> **For AI agents maintaining this page:**
> - Source: `src/distro/omarchy_gaming_performance_suite.rs`
> - Update GPU backend table when new GPU families are supported
> - Add new developer stacks as they are added to `SovereignOmarchyDeveloperStacks`
> - Keep comparison table current vs Omarchy's latest GitHub releases
> - Cross-reference with [Scheduler](Scheduler.md) for gaming scheduling details
