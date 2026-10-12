[[Category:Desktop]]
[[Category:Configuration]]

# Omarchy Linux Integration

SigmaOS incorporates several components and design philosophies inspired by [Omarchy Linux](https://omarchy.org/), a beautiful, modern, and opinionated Linux distribution based on Arch.

## Overview

Omarchy Linux is an omakase distribution that emphasizes beauty, productivity, and keyboard-first workflows. SigmaOS integrates key Omarchy concepts while maintaining its own identity and goals.

## Integrated Components

### Desktop Shell

SigmaOS includes a unified desktop shell inspired by Omarchy 4.0's Quickshell architecture:

- **Unified Shell Process** - Single process managing bar, launcher, menus, notifications, OSDs, and control panels
- **Plugin Architecture** - Extensible widget system for third-party customizations
- **Modular Bar** - Interactive widgets for workspaces, active window, clock, weather, media, system tray, battery, and keyboard layout
- **Native Launcher** - Fuzzy/acronym matching with live app icon indexing
- **Nested Search** - Unified menu for apps and commands (Super + Space)

### Theme System

Omarchy's coordinated theme system has been adapted for SigmaOS:

- **17 Built-in Themes** - One keystroke to switch between coordinated themes
- **Live Theming** - Real-time theme switching affecting terminal, editor, browser, wallpaper, and bar
- **Theme Picker** - Visual theme selector with preview
- **Custom Themes** - User-defined themes following the coordinated color scheme pattern

### Keyboard-First Workflow

Omarchy's keyboard-centric design is integrated:

- **Home Row Navigation** - All keybindings on home row where possible
- **Unified Menu** - Super + Space for app and command search
- **Quick Actions** - Keyboard shortcuts for common tasks
- **Modal Keybindings** - Chord-based commands for complex operations

### Quickshell Integration

SigmaOS includes Quickshell compatibility for:

- **Bar Widgets** - Custom status bar components
- **Menu System** - Application and command menus
- **Notifications** - Desktop notification system
- **On-Screen Displays** - Volume, brightness, and other OSD indicators
- **Control Panels** - Settings and configuration interfaces
- **Lock Screen** - Screen locking with widget support
- **Polkit Agent** - Authentication dialogs

## Installation

Omarchy-inspired components are included in the SigmaOS desktop environment:

```bash
sigma install sigma-desktop-shell
sigma install sigma-theme-engine
sigma install sigma-quickshell
```

## Configuration

### Desktop Shell Configuration

The desktop shell is configured in `~/.config/sigmaos/shell/`:

- `config.toml` - Main shell configuration
- `widgets/` - Custom widget definitions
- `themes/` - Theme configurations
- `plugins/` - Installed plugins

Example shell configuration:

```toml
[shell]
process = "unified"
plugins_enabled = true
widget_refresh_rate = 1000

[bar]
widgets = ["workspaces", "active_window", "clock", "tray"]
position = "top"
height = 32

[launcher]
fuzzy_match = true
acronym_match = true
live_indexing = true
```

### Theme Configuration

Themes are defined in `~/.config/sigmaos/themes/`:

```toml
[theme]
name = "Sigma Dark"
background = "#1a1a2e"
foreground = "#eaeaea"
accent = "#16213e"
highlight = "#0f3460"

[terminal]
background = "#1a1a2e"
foreground = "#eaeaea"

[editor]
background = "#1a1a2e"
foreground = "#eaeaea"
selection = "#16213e"
```

### Keybinding Configuration

Keybindings are configured in `~/.config/sigmaos/keybindings.toml`:

```toml
[keybindings]
super_space = "launcher"
super_k = "keybinding_list"
super_ctrl_shift_space = "theme_picker"
super_j = "next_workspace"
super_k = "previous_workspace"
```

## Usage

### Theme Switching

Switch themes with a single keystroke:

```bash
# Default keybinding
Super + Ctrl + Shift + Space
```

Or via command line:

```bash
sigma-theme-picker
```

### Unified Launcher

Access apps and commands from one interface:

```bash
# Default keybinding
Super + Space
```

### Plugin Management

Install, enable, and disable shell plugins:

```bash
# Install a plugin
sigma plugin add https://github.com/user/plugin

# List plugins
sigma plugin list

# Enable plugin
sigma plugin enable plugin-name

# Disable plugin
sigma plugin disable plugin-name
```

## Tips and Tricks

### Custom Widgets

Create custom bar widgets in `~/.config/sigmaos/shell/widgets/`:

```rust
// custom_widget.rs
use sigmaos::shell::Widget;

pub struct CustomWidget {
    // Widget state
}

impl Widget for CustomWidget {
    fn render(&self) -> String {
        // Render widget content
    }

    fn update(&mut self) {
        // Update widget state
    }
}
```

### Theme Creation

Create custom themes following the coordinated scheme:

1. Define color palette
2. Configure terminal colors
3. Configure editor colors
4. Configure bar and menu colors
5. Set wallpaper accent
6. Test with live preview

### Keyboard Shortcuts

Learn Omarchy-style keyboard shortcuts:

- `Super + Space` - Launcher
- `Super + K` - Keybinding list
- `Super + J/K` - Next/previous workspace
- `Super + Shift + J/K` - Move window to workspace
- `Super + Enter` - Terminal
- `Super + Shift + Q` - Close window

## Troubleshooting

### Shell Not Starting

If the unified shell process fails to start:

1. Check shell logs: `journalctl -u sigma-shell`
2. Verify configuration syntax: `sigma-shell --check-config`
3. Disable plugins temporarily: `sigma-shell --no-plugins`
4. Reset to default: `sigma-shell --reset`

### Theme Not Applying

If theme changes don't take effect:

1. Verify theme file syntax
2. Check theme directory permissions
3. Restart shell process: `sigma-shell --restart`
4. Clear theme cache: `sigma-theme --clear-cache`

### Plugin Issues

If a plugin causes problems:

1. Disable the problematic plugin: `sigma plugin disable plugin-name`
2. Check plugin logs: `journalctl -u sigma-plugin-plugin-name`
3. Update plugin: `sigma plugin update plugin-name`
4. Report issue to plugin maintainer

## See also

- [Omarchy Manual](https://omarchy.org/manual/)
- [Omarchy GitHub](https://github.com/basecamp/omarchy)
- [Desktop and Window Management](Desktop-and-Window-Management.md)
- [Theming Guide](Theming-Guide.md)
- [Configuration](Configuration.md)
