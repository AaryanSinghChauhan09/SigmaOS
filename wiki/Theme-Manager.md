# SigmaOS Theme Manager Components

## Overview
SigmaOS theme manager providing color palette definitions, template rendering, and runtime theme switching, competitive with Omarchy themes (base 8 to 24 colors), Mint themes, and Arch theming systems.

## Features
- **Color palette definitions** (TOML-based)
- **Template rendering** for terminal/shell/GTK
- **Theme switching at runtime**
- **User-defined themes**
- **Auto-generated themes** from wallpaper colors
- **Default themes**: dark, light, solarized
- **Hex color parsing** with validation
- **Accent/background/foreground color references**

## Default Themes

| Theme | Description | Accent | Background | Foreground |
|-------|-------------|--------|------------|------------|
| **dark** | Dark theme with low contrast | #89b4fa | #1e1e2e | #cdd6f4 |
| **light** | Light theme with high contrast | #1e66f5 | #f9f9f9 | #1e1e2e |
| **solarized** | Solarized color scheme | #268bd2 | #002b36 | #93a1a1 |

## Structures

| Structure | Description |
|-----------|-------------|
| **ThemeManager** | Theme manager with theme registry |
| **Theme** | Theme definition with colors |
| **ThemeColor** | Individual color (name, hex, RGB) |

## Usage Examples

```rust
// Create theme manager
let mut manager = ThemeManager::new("/usr/share/sigmaos/themes".to_string());

// Register default themes
manager.create_default_themes();

// Set active theme
manager.set_active_theme("dark").unwrap();

// Get active color
let accent = manager.get_active_color("accent");
```

## Status
- **Implementation**: Complete
- **Dependencies**: None (stdlib only)
- **Roadmap Phase**: Theming (P1)

## Related Files
- `src/theme/manager.rs` - Theme manager implementation
- `wiki/Theming-System.md` - Theming wiki

## Principles Followed
SEPARATION_OF_CONCERNS, COMPOSITION_OVER_INHERITANCE, MODULARITY,
LAYERED DESIGN, KEEP_IT_SIMPLE_STUPID, DISCOVERABILITY,
USER_FREEDOM_AND_CONTROL, CUSTOMIZATION