# SigmaOS Plymouth Boot Theme Components

## Overview
SigmaOS Plymouth boot theme manager providing boot theme management, competitive with Omarchy plymouth (omancy-plymouth-current, omancy-plymouth-list, omancy-plymouth-preview, omancy-plymouth-reset, omancy-plymouth-set, omancy-plymouth-set-by-theme, omancy-plymouth-switcher).

## Features
- **Theme installation** - Install Plymouth themes
- **Theme listing** - List available themes
- **Theme switching** - Set current boot theme
- **Theme reset** - Reset to default
- **Theme preview** - Preview theme
- **Theme removal** - Remove themes

## Structures

| Structure | Description |
|-----------|-------------|
| **PlymouthManager** | Plymouth manager |
| **PlymouthTheme** | Individual theme |

## Usage Examples

```rust
// Create Plymouth manager (comes with default themes)
let mut mgr = PlymouthManager::default();

// List themes
let themes = mgr.list_themes();

// Set theme
mgr.set_theme("sigmaos").unwrap();

// Reset theme
mgr.reset();

// Install custom theme
let theme = PlymouthTheme { ... };
mgr.install_theme(theme).unwrap();
```

## Status
- **Implementation**: Complete
- **Dependencies**: None (stdlib only)
- **Roadmap Phase**: Bootloader (P1)

## Related Files
- `src/plymouth/manager.rs` - Plymouth manager implementation
- `wiki/Plymouth.md` - Wiki documentation

## Principles Followed
SINGLE_RESPONSIBILITY, COMPONENT_ISOLATION, MODULARITY, LAYERED DESIGN,
KEEP_IT_SIMPLE_STUPID, DISCOVERABILITY, DEFENSIVE_PROGRAMMING, BOOTLOADER_INTEGRATION,
SECURE_DEFAULTS, USER_CUSTOMIZATION, AESTHETICS