# SigmaOS-theme-engine

Theme rendering and synthesis for SigmaOS. Handles CSS variables, dark/light mode switching, icon theming, and palette generation.

## Upstream Inspiration

- [adwaita](https://gnome.pages.gitlab.gnome.org/libadwaita/) — GNOME HIG design system
- [arc-theme](https://github.com/jnsh/arc-theme) — flat GTK theme
- [papirus](https://github.com/PapirusIconTheme/Papirus-Icon-Theme) — icon theme
- [nwg-look](https://github.com/nwg-piotr/nwg-look) — GTK theme switcher for wlroots

## Purpose

- **CSS variable resolution**: themed color tokens → CSS variable map
- **Dark/light switching**: OS-level preference → theme variant
- **Icon theming**: icon name → themed SVG path with fallback chain
- **Palette generation**: accent color → full color palette
- **Real-time reload**: inotify-based hot reload of theme files
- **GTK4 apply**: programmatic application of theme to GTK4 apps

## Build Instructions

```bash
sudo apt install libgtk-4-dev

cargo build --release
cargo test
```

## API Overview

- `CssTheme` — CSS variable map with dark/light variants
- `IconTheme` — icon lookup with inheritance chain
- `Palette` — accent color → full color scale
- `ThemeManager` — top-level theme state with reload support
