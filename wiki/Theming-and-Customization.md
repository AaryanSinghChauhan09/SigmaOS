# Theming and Customization

SigmaOS provides a deep, consistent theming system that covers the compositor, GTK apps, Qt apps, the terminal, icons, cursors, and system sounds — all driven by a single TOML theme file. Inspired by Omarchy's opinionated Catppuccin aesthetic and Mint's cross-desktop theming, SigmaOS goes further with AI-generated accent palettes and live theme switching without logout.

---

## Theme Architecture

```
 ~/.config/sigma/theme.toml
          │
          ├──► SigmaCompositor (window borders, blur, animations)
          ├──► GTK 3/4 theme   (native GTK apps)
          ├──► Qt 5/6 theme    (Qt apps via sigma-qt-theme)
          ├──► sigma-term       (terminal color scheme)
          ├──► Icon theme       (file manager, panel)
          ├──► Cursor theme     (pointer shape + size)
          └──► sigma-sh prompt  (shell color scheme)
```

All applied **live** — no logout required.

---

## Theme File Format

```toml
[meta]
name = "sigma-dark"
variant = "dark"      # dark | light | auto (time-based)
author = "SigmaOS Team"

[colors]
# Base palette
background  = "#1e1e2e"
surface     = "#313244"
overlay     = "#45475a"
text        = "#cdd6f4"
subtext     = "#a6adc8"
accent      = "#89b4fa"    # primary accent (blue)
accent2     = "#cba6f7"    # secondary (mauve)
green       = "#a6e3a1"
red         = "#f38ba8"
yellow      = "#f9e2af"
orange      = "#fab387"
pink        = "#f5c2e7"
teal        = "#94e2d5"

[compositor]
border_color        = "#89b4fa"
border_width        = 2
corner_radius       = 8
blur_enabled        = true
blur_radius         = 12
blur_passes         = 3
shadow_enabled      = true
shadow_color        = "#00000080"
shadow_range        = 20
inactive_opacity    = 0.90

[gtk]
theme        = "sigma-gtk-dark"
icon_theme   = "sigma-icons"
cursor_theme = "sigma-cursors"
cursor_size  = 24
font         = "Inter 11"
monospace    = "JetBrains Mono 10"

[terminal]
color_scheme = "sigma-dark"
opacity      = 0.95
font         = "JetBrains Mono"
font_size    = 13.0
```

---

## Built-in Themes

| Theme | Base | Variant |
|-------|------|---------|
| `sigma-dark` | Catppuccin Mocha-inspired | Dark |
| `sigma-light` | Catppuccin Latte-inspired | Light |
| `sigma-nord` | Nord palette | Dark |
| `sigma-gruvbox` | Gruvbox | Dark |
| `sigma-dracula` | Dracula | Dark |
| `sigma-solarized` | Solarized | Auto |
| `sigma-mint` | Linux Mint green | Dark/Light |
| `sigma-omarchy` | Omarchy catppuccin | Dark |

---

## AI Accent Generator

SigmaOS can generate a personalized color accent from any source:

```bash
# Generate theme from wallpaper
sigma-theme generate --from-wallpaper ~/wallpapers/mountain.jpg

# Generate theme from a color hex
sigma-theme generate --accent "#ff6b6b"

# Apply generated theme
sigma-theme apply --name my-red-theme
```

The AI engine:
1. Extracts dominant colors from wallpaper (k-means clustering)
2. Selects a harmonious accent palette
3. Ensures WCAG AA contrast ratios for text readability
4. Generates complete `theme.toml` with all variants

---

## Omarchy-Style Dotfile Versioning

Inspired by Omarchy's Git-based dotfile management:

```bash
# Initialize dotfile tracking
sigma-dots init

# Track a config file
sigma-dots track ~/.config/sigma/theme.toml
sigma-dots track ~/.config/sigma/sh/config.toml

# Take a snapshot
sigma-dots snapshot --name "catppuccin-setup"

# Switch to a saved profile
sigma-dots restore catppuccin-setup

# Share profile
sigma-dots export --output ~/my-sigma-profile.tar.zst
sigma-dots import ~/my-sigma-profile.tar.zst
```

---

## Compositor Visual Effects

| Effect | Config Key | Default |
|--------|-----------|---------|
| Window blur | `blur_enabled` | true |
| Blur radius | `blur_radius` | 12px |
| Corner rounding | `corner_radius` | 8px |
| Drop shadow | `shadow_enabled` | true |
| Inactive dimming | `inactive_opacity` | 0.90 |
| Fade in/out | `animations.enable` | true |
| Animation duration | `animations.duration_ms` | 180ms |
| Easing | `animations.easing` | ease-out-cubic |

---

## Wallpaper Management

```bash
# Set wallpaper
sigma-wallpaper set ~/wallpapers/mountain.jpg

# Slideshow mode
sigma-wallpaper slideshow ~/wallpapers/ --interval 30m --transition fade

# Sync wallpaper with time of day
sigma-wallpaper dynamic --dawn dawn.jpg --day day.jpg --dusk dusk.jpg --night night.jpg
```

---

## GTK / Qt Theming Implementation

### GTK (`src/theming/gtk.rs`)
- Generates `gtk.css` from `theme.toml` color values
- Supports GTK 3.20+ and GTK 4.0+
- Injects via `GTK_THEME` and `~/.config/gtk-*/settings.ini`

### Qt (`src/theming/qt.rs`)
- Generates `~/.config/qt5ct/qt5ct.conf` and qt6ct equivalents
- Sets color palette via QApplication style override
- Supports Qt5 and Qt6

---

## Comparison vs Mint / Omarchy Theming

| Feature | Linux Mint | Omarchy | **SigmaOS** |
|---------|-----------|---------|-------------|
| GTK theming | ✅ | ✅ | ✅ |
| Qt theming | ✅ | ✅ | ✅ |
| Live switch | ❌ (logout) | ✅ | ✅ |
| AI color gen | ❌ | ❌ | ✅ |
| Dotfile versioning | ❌ | ✅ (Git) | ✅ |
| Single config file | ❌ | ❌ | ✅ TOML |
| Dynamic wallpaper | ❌ | ❌ | ✅ |

---

## Source Files

| File | Description |
|------|-------------|
| `src/theming/` | Theme engine |
| `src/compositor/` | Compositor visual effects |
| `src/customization/` | User customization system |
| `src/desktop/` | Desktop shell theming hooks |

---

## AI Agent Maintenance Instructions

> **For AI agents maintaining this page:**
> - Source: `src/theming/`, `src/customization/`
> - Add new built-in themes to the theme table as they are added
> - Update compositor effects table when new effects are implemented
> - Keep comparison table current vs Omarchy's Catppuccin config
