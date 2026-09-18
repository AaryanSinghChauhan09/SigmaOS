# 🎨 Zenith Desktop & Omarchy Workflow Guide

SigmaOS features the **Zenith Desktop Ecosystem**, heavily inspired by **Omarchy Linux** and modern Wayland tiling workflows. Zenith combines a fast, keyboard-driven Wayland compositor with an **Agentic Steering Triad** (Bolt ⚡, Palette 🎨, Sentinel 🛡️) and curated Omakase presets (`sigomarchy`).

---

## 🧩 Zenith Desktop Architecture

```
   ┌────────────────────────────────────────────────────────────────────────┐
   │                         ZENITH DESKTOP SUITE                           │
   ├────────────────────────────────────────────────────────────────────────┤
   │  Wayland Compositor ➔ Agentic Triad ➔ Omakase Workstation Engine       │
   │  (wl_surface/xdg_shell)  (Bolt/Palette/Sentinel)  (sigomarchy / Ghostty)  │
   └────────────────────────────────────────────────────────────────────────┘
```

 Zenith is implemented in zero-dependency safe Rust (`src/desktop/`, `src/media/browser.rs`, `zenith_desktop/index.js`), eliminating reliance on legacy X11 protocols or heavy scripting interpreters.

---

## ⚡ Agentic Steering Triad

Zenith introduces **Agentic Desktop Steering**, providing intelligent automation, security enforcement, and theme management directly integrated into keybindings and system dialogs:

### 1. Bolt ⚡ — Fast-Path Launcher & Task Automation
- **Function**: High-speed, keyboard-driven command runner and task orchestrator (`tdl <ai>`, `herdr-run`).
- **Keybinding**: `Super + Space` or `Super + D`
- **Capabilities**: Launches native apps, WebApps, terminal scripts, and automated system commands instantly.

### 2. Palette 🎨 — Dynamic Theme & Color Engine
- **Function**: System-wide theme switcher supporting WCAG 2.1 AAA accessibility and automatic contrast adjustment.
- **Keybinding**: `Super + Alt + T`
- **Presets**: Dark Sovereign, Catppuccin Mocha, Nord, Gruvbox, High Contrast.

### 3. Sentinel 🛡️ — Sandboxing & Exec Guard
- **Function**: Real-time security auditor and capability permission model (Zorin Exec Guard parity).
- **Behavior**: Intercepts unverified execution requests, presenting capability options (Allow Once, Sandbox in Chroot, Block).

---

## ⌨️ Keybindings & Tiling Controls

Zenith utilizes an intuitive, keyboard-first tiling keybinding scheme:

| Keybinding | Action Description |
|------------|--------------------|
| `Super + Enter` | Open GPU-accelerated terminal (**Ghostty**) |
| `Super + Space` | Open **Bolt ⚡** launcher overlay |
| `Super + Q` | Close active window |
| `Super + Shift + C` | Reload Zenith configuration (`~/.config/zenith/config.toml`) |
| `Super + H / J / K / L` | Move window focus Left / Down / Up / Right |
| `Super + Shift + H / J / K / L` | Swap focused window position |
| `Super + 1 .. 9` | Switch to Workspace 1 .. 9 |
| `Super + Shift + 1 .. 9` | Move focused window to Workspace 1 .. 9 |
| `Super + F` | Toggle full-screen mode |
| `Super + V` | Toggle vertical / horizontal tiling split |
| `Super + Alt + T` | Open **Palette 🎨** theme selector |
| `Super + L` | Lock screen (`sigmalock`) |

---

## 🖥️ Omakase Developer Preset Workstation (`sigomarchy`)

SigmaOS provides curated **Omakase Workstation Presets** inspired by Omarchy Linux, allowing instant setup of developer tools with unified styling.

### Applying Presets via CLI:
```bash
# View available workstation presets
sigomarchy preset list

# Apply developer workstation preset
sigomarchy preset apply developer

# Apply minimalist workstation preset
sigomarchy preset apply minimalist
```

### Integrated Developer Stack:
1. **Ghostty Terminal**: GPU-accelerated Wayland terminal renderer with zero input latency (`OmarchyGhosttyTerminalEngine`).
2. **Fastfetch Sysinfo**: Instant ASCII diagnostic banner rendering CPU, memory, uptime, and kernel shard state (`OmarchyFastfetchSysinfoEngine`).
3. **Hyprpaper Wallpaper Daemon**: Smooth background image preloading and multi-monitor rendering (`OmarchyHyprpaperWallpaperEngine`).
4. **Waybar / Quickshell Bar**: Responsive status bar showing active workspaces, CPU/RAM usage, network status, and time.

---

## ⚙️ Customizing Preference Manifests

Zenith configuration is fully declarative and validated via TOML manifests:

### User Preferences (`~/.config/zenith/config.toml`):
```toml
[desktop]
layout = "dwindle"
border_width = 2
active_border_color = "#74c7ec"
inactive_border_color = "#313244"
gaps_in = 5
gaps_out = 10

[agentic_triad]
bolt_enabled = true
palette_theme = "catppuccin_mocha"
sentinel_strictness = "medium"

[terminal]
emulator = "ghostty"
font_family = "FiraCode Nerd Font"
font_size = 12.0
```

To test and verify Zenith desktop functionality, execute:
```bash
./scripts/uiux_accessibility_test.sh
```
