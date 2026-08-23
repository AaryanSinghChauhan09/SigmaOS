# SigmaOS Desktop Environment — Zenith

## Architecture

```
Applications (GTK4, Qt6, native Rust)
  ↓
Zenith Compositor (DRM/KMS Wayland, GPU-accelerated)
  ↓
Sigma Shell | Palette Theme | XWayland (X11 compat)
  ↓
Wayland Protocol
  ↓
DRM/KMS / Mesa / Vulkan
```

## Zenith Compositor

### Rendering

Scene Graph → Vulkan Command Buffer → GPU → DRM Framebuffer → Display

### Wayland Protocols

| Protocol | Purpose |
|----------|---------|
| xdg_shell | App windows |
| wl_seat | Input devices |
| zwp_linux_dmabuf_v1 | Zero-copy GPU |
| zwlr_layer_shell_v1 | Panels/overlays |
| wp_fractional_scale_v1 | HiDPI scaling |
| ext_session_lock_v1 | Screen lock |

### Features

| Feature | Detail |
|---------|--------|
| VSync | FreeSync / G-Sync |
| HDR | HDR10 / HLG |
| Multi-monitor | Independent configs |
| HiDPI | Fractional scaling |
| Blur | GPU background blur |
| Animations | 60Hz+ smooth |

## Layout Modes

| Mode | Inspiration |
|------|------------|
| Zenith | macOS (dock + menu bar) |
| Classic | GNOME/MATE |
| Cinnamon | Linux Mint |
| Tiling | Sway/i3 |
| Touch | GNOME Shell |

## Tiling Window Manager

**BSP (Binary Space Partitioning)**:
```
Monitor
├── Left: Firefox
└── Right:
    ├── Top: Terminal
    └── Bottom: File Manager
```

**Master-Stack**:
```
Monitor
├── Master (60%): Active window
└── Stack (40%): Window 1 / Window 2 / Window 3
```

### Keybindings

| Key | Action |
|-----|--------|
| Super+Enter | Open terminal |
| Super+d | Open launcher |
| Super+←/→/↑/↓ | Focus direction |
| Super+f | Toggle fullscreen |
| Super+Shift+q | Close window |
| Super+1-9 | Switch workspace |

## Theme Engine (Palette)

### Dr460nized Theme (Default)

Inspired by **Garuda Linux**:
- Colors: #1a1b2e, #16213e (deep blue-purple dark)
- Blur: frosted glass panels
- Rounded corners: 12px
- Accent: vibrant magenta/cyan

### Available Themes

| Theme | Style |
|-------|-------|
| Dr460nized | Dark blur (Garuda-inspired) |
| Nord | Arctic blue |
| Catppuccin | Warm dark |
| Gruvbox | Warm retro |
| Adwaita | GNOME default |

## Accessibility

WCAG 2.1 AA compliance: screen reader (Orca), high contrast, large text, keyboard navigation, magnifier, color blindness modes.
