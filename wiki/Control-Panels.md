# SigmaOS Control Panels Components

## Overview
SigmaOS control panels providing system settings panels, competitive with Omarchy's control panels (Audio, Bluetooth, Network, Display, Power).

## Features
- **10 control panels**: Audio, Bluetooth, Network, Display, Power, Keyboard, Mouse, User, Security, About
- **Keyboard shortcuts** for quick access (SUPER+CTRL+A for Audio, etc.)
- **Active panel tracking**
- **Panel descriptions** and icons

## Default Panels

| Panel | Icon | Shortcut | Description |
|-------|------|----------|-------------|
| Audio | 🔊 | SUPER+CTRL+A | Audio controls and volume |
| Bluetooth | 📱 | SUPER+CTRL+B | Bluetooth devices |
| Network | 🌐 | SUPER+CTRL+W | Network connections |
| Display | 🖥️ | SUPER+CTRL+D | Display settings |
| Power | 🔋 | SUPER+CTRL+P | Power management |
| Keyboard | ⌨️ | - | Keyboard settings |
| Mouse | 🖱️ | - | Mouse settings |
| User | 👤 | - | User accounts |
| Security | 🔒 | - | Security settings |
| About | ℹ️ | - | System information |

## Structures

| Structure | Description |
|-----------|-------------|
| **ControlPanels** | Control panels manager |
| **PanelEntry** | Individual panel entry |
| **ControlPanel** | Audio, Bluetooth, Network, Display, Power, Keyboard, Mouse, User, Security, About |

## Usage Examples

```rust
// Create control panels
let mut panels = ControlPanels::new();

// Open audio panel
panels.open(ControlPanel::Audio);

// Handle keyboard shortcut
if let Some(panel) = panels.handle_shortcut("SUPER+CTRL+A") {
    panels.open(panel);
}

// Close active panel
panels.close();
```

## Status
- **Implementation**: Complete
- **Dependencies**: None (stdlib only)
- **Roadmap Phase**: Desktop & UX (P1)

## Related Files
- `src/control/panels.rs` - Control panels implementation
- `wiki/Control-Panels.md` - Wiki documentation

## Principles Followed
SINGLE_RESPONSIBILITY, COMPONENT_ISOLATION, MODULARITY, LAYERED DESIGN,
SEPARATION_OF_CONCERNS, KEEP_IT_SIMPLE_STUPID, DISCOVERABILITY,
USER_FRIENDLY, SIMPLE_USER_INTERFACE, CLEAR_ABSTRACTION, COMPOSITION_OVER_INHERITANCE