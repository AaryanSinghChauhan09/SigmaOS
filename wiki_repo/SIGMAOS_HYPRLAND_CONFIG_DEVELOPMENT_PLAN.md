# 🪟 SigmaOS Hyprland Lua Configuration Subsystem (`hyprland_config`) Strategic Development Plan

## Executive Summary & Design Vision

Hyprland is a dynamic, tile-based Wayland window manager featuring smooth physics-based animations, rounded window corners, dwindle tiling layouts, and multi-monitor display outputs. Configuring Hyprland via raw string files can lead to syntax errors across version upgrades.

Drawing direct architectural inspiration from **Omarchy Hyprland Lua Configuration Architecture**, the **SigmaOS Hyprland Configuration Subsystem** (`OmarchyHyprlandCompositorConfigEngine`, `HyprlandWindowRule`, `OmarchyLuaConfigEngine`, `SigmaHyprlandLuaRouter`) provides an ergonomic, Lua-configured Wayland window manager environment with auto-reloading, binding collision detection (`o.rebind`), monitor topology helpers (`hl.monitor`), version-validated window rules (`o.window`), and reload-suppression guards (`sigma-hyprland-reload-guard`).

---

## 1. Lua Configuration Directory Structure

User Hyprland configurations reside under `~/.config/hypr/` and load in strict order—Omarchy system defaults load first, followed by user overrides:

```text
~/.config/hypr/
├── hyprland.lua       # Main entry point (loads system defaults, then user modules)
├── bindings.lua       # Keybindings & rebindings (`o.bind`, `o.rebind`, `hl.unbind`)
├── monitors.lua       # Display output configurations (`hl.monitor`)
├── input.lua          # Keyboard layout, variant, repeat rates & mouse sensitivity
├── looknfeel.lua      # Gaps, borders, blur, shadow, and physics animation curves
├── autostart.lua      # Startup daemons, applets, and background services
├── hyprsunset.conf    # Blue-light filter / night-light color temperature profile
└── xdph.conf          # XDG Desktop Portal Hyprland screen-sharing configuration
```

---

## 2. Key Binding, Monitor & Window Rule Ergonomics

### 2.1 Keybinding Ergonomics (`o.bind`, `o.rebind`, `hl.unbind`)
- **Binding Creation**:
  ```lua
  o.bind("SUPER + SHIFT + R", "SSH Terminal", "alacritty -e ssh my-server")
  o.bind("SUPER + B", "Web Browser", { launch = "chromium" }) -- Launches via uwsm-app
  ```
- **Rebinding Pre-existing Keys**: Before rebinding an existing key, `o.rebind` checks the existing binding table (viewable via `sigma menu keybindings --print`), removes the previous binding, adds the new target, and logs the replaced action:
  ```lua
  -- Rebind SUPER + F (previously: fullscreen) to file manager
  o.rebind("SUPER + F", "File Manager", { launch = "nautilus" })
  ```
- **Unbinding**: `hl.unbind("SUPER + P")` removes the binding entirely.

### 2.2 Display Output Declarations (`hl.monitor`)
- Monitors are declared declaratively with output name, resolution, refresh rate, relative position, and scale factor:
  ```lua
  hl.monitor({ output = "eDP-1", mode = "1920x1080@60", position = "0x0", scale = 1 })
  hl.monitor({ output = "HDMI-A-1", mode = "2560x1440@144", position = "1920x0", scale = 1 })
  ```

### 2.3 Version-Validated Window Rules (`o.window`)
- **Version Compatibility Requirement**: Because Hyprland window rule syntax evolves between releases, rule generators fetch or validate against official documentation (`https://wiki.hypr.land/Configuring/Basics/Window-Rules/`).
- **Omarchy Window Helper**:
  ```lua
  o.window({ class = "^(sigma-webapp-.*)$" }, { float = true, border_size = 2 })
  ```

### 2.4 Auto-Reload & Validation Workflow (`hyprctl`)
- **Live Save Reload**: Changes to `.lua` files trigger automatic reloading.
- **Validation**: After modifying Lua configs, the system executes `hyprctl reload` followed by `hyprctl configerrors`. If errors are reported, the user is alerted to fix them before committing changes.
- **Reload Suppression Guard**: During system package updates, `sigma-hyprland-reload-guard pause` temporarily disables config reloading while default `/usr/share/hypr/` configs are replaced, resuming reload via `resume` post-transaction.

---

## 3. Phased Development Roadmap

### Phase 1: Lua Config Generator & Directory Router (Q4 2026)
- Build `OmarchyLuaConfigEngine` and `SigmaHyprlandLuaRouter` in `src/distro/omarchy_inspiration.rs`.
- Implement Lua table bindings for `o.bind`, `o.rebind`, `hl.unbind`, and `hl.monitor`.
- Deploy keybinding collision detector (`sigma menu keybindings --print`).

### Phase 2: Live Validation & Reload Suppression Guard (Q1 2027)
- Integrate `hyprctl reload` and `hyprctl configerrors` automated validation parser.
- Deploy `sigma-hyprland-reload-guard` pause/resume ALPM hook for package updates.
- Support `hyprsunset` blue-light and `xdph.conf` portal configuration reloading.

### Phase 3: Version-Aware Window Rule Validator (Q2 2027)
- Implement `o.window(match, rules)` window rule transformer with online/cached wiki rule syntax validator.
- Add support for floating, tile, opacity, pin, and workspace assignment window rules.

### Phase 4: Performance Benchmarks & CI Testing (Q3 2027+)
- Benchmark Lua config parsing and Hyprland reload latency (target < 50ms).
- Integrate Hyprland config generator unit tests into `./scripts/verify.sh`.
- Conduct fuzz testing against Lua table parsers (`cargo fuzz`).

---

## 4. Verification & Testing Standards

All Hyprland configuration components must pass the unified verification runner:
```bash
./scripts/verify.sh
```

Which validates unit and integration tests across:
- `src/distro/omarchy.rs` (`OmarchyHyprlandCompositorConfigEngine` & `generate_hyprland_theme_config`)
- `src/distro/omarchy_inspiration.rs` (`OmarchyLuaConfigEngine` & window rules)
- `src/automation/hotkey.rs` (`SovereignWindowManagerHotkeyEngine` Hyprland profile)
