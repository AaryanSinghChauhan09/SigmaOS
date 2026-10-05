# Desktop Theme Manager

## Overview

The Desktop Theme Manager provides comprehensive theme management inspired by Linux Mint's theme system and Omarchy's appearance utilities. It supports GTK, icon, cursor, sound, window, and application themes with dark variant support.

## Features

- **Theme Types**: GTK, Icon, Cursor, Sound, Window, Application
- **Theme Management**: Add, remove, and manage themes
- **Dark Variants**: Support for dark theme variants
- **Per-Type Configuration**: Separate configuration for each theme type
- **Current Theme Tracking**: Track currently active themes for each type
- **Theme Filtering**: Filter themes by type
- **Search**: Search themes by name and author
- **Default Themes**: Pre-configured themes from GNOME, Linux Mint, KDE, and Papirus
- **Statistics**: Track theme counts by type and current theme status

## Components

### DesktopThemeType

```rust
pub enum DesktopThemeType {
    Gtk,         // GTK theme
    Icon,        // Icon theme
    Cursor,      // Cursor theme
    Sound,       // Sound theme
    Window,      // Window theme
    Application, // Application theme
}
```

### DesktopTheme

Theme entry with:
- Theme ID and name
- Theme type
- File path
- Author and version
- Dark variant (optional)
- Dark mode flag

### DesktopThemeManager

Main management interface with:
- Theme management (add, remove, retrieve)
- Per-type current theme configuration
- Dark variant support
- Theme filtering by type
- Search functionality
- Default themes from major projects
- Statistics tracking

## Usage

### Basic Usage

```rust
use sigmaos::desktop::DesktopThemeManager;

let mut manager = DesktopThemeManager::new();

// Get configuration
println!("Total themes: {}", manager.get_themes().len());
println!("Current GTK theme: {:?}", manager.get_current_gtk_theme());
```

### Theme Management

```rust
// Add theme
let new_theme = DesktopTheme::new(
    "custom".to_string(),
    "Custom Theme".to_string(),
    DesktopThemeType::Gtk,
    "/usr/share/themes/Custom".to_string(),
    "Author".to_string(),
    "1.0".to_string(),
);

let id = manager.add_theme(new_theme);

// Remove theme
manager.remove_theme(&id);
```

### Theme Configuration

```rust
// Set GTK theme
manager.set_gtk_theme(&theme_id);

// Set icon theme
manager.set_icon_theme(&theme_id);

// Set cursor theme
manager.set_cursor_theme(&theme_id);

// Set sound theme
manager.set_sound_theme(&theme_id);

// Set window theme
manager.set_window_theme(&theme_id);
```

### Get Current Themes

```rust
if let Some(gtk) = manager.get_current_gtk_theme() {
    println!("GTK theme: {}", gtk.name);
}

if let Some(icon) = manager.get_current_icon_theme() {
    println!("Icon theme: {}", icon.name);
}
```

### Theme Filtering

```rust
// Get GTK themes
let gtk_themes = manager.get_themes_by_type(DesktopThemeType::Gtk);

// Get icon themes
let icon_themes = manager.get_themes_by_type(DesktopThemeType::Icon);
```

### Search

```rust
// Search themes
let results = manager.search_themes("Adwaita");
for theme in results {
    println!("Found: {}", theme.name);
}
```

### Statistics

```rust
let stats = manager.get_statistics();
println!("Total themes: {}", stats.total_themes);
println!("GTK themes: {}", stats.gtk_themes);
println!("Icon themes: {}", stats.icon_themes);
println!("Cursor themes: {}", stats.cursor_themes);
println!("Sound themes: {}", stats.sound_themes);
```

## Default Themes

The manager includes default themes from major projects:

### GTK Themes
- **Adwaita**: GNOME default with dark variant
- **Yaru**: Ubuntu default with dark variant
- **Mint-Y**: Linux Mint default with dark variant

### Icon Themes
- **Adwaita**: GNOME default icons
- **Mint-X**: Linux Mint icons
- **Papirus**: Modern flat icon theme

### Cursor Themes
- **Adwaita**: GNOME default cursors
- **Breeze**: KDE cursors

### Sound Themes
- **freedesktop**: freedesktop.org sound theme
- **Mint**: Linux Mint sound theme

## Default Configuration

The Theme Manager includes default configuration:

- **GTK Theme**: Adwaita
- **Icon Theme**: Adwaita
- **Cursor Theme**: Adwaita
- **Sound Theme**: freedesktop
- **Window Theme**: Adwaita

## AI Agent Maintenance Instructions

When maintaining the Theme Manager:

1. **Theme Detection**: Detect installed themes from standard directories
2. **Theme Installation**: Implement theme installation from archives
3. **Theme Preview**: Add theme preview functionality
4. **Sync with Applications**: Sync theme preferences with GTK/GNOME settings
5. **Theme Download**: Implement theme download from online repositories
6. **Custom Theme Creation**: Add tools for creating custom themes
8. **Theme Validation**: Validate theme files for compatibility
9. **Color Scheme Sync**: Sync theme with color scheme preferences
10. **Per-Application Themes**: Add per-application theme overrides

## Testing

Run the unit tests with:

```bash
cargo test --lib desktop::theme_manager
```

## Future Enhancements

- Theme detection from standard directories
- Theme installation from archives
- Theme preview functionality
- Sync with GTK/GNOME settings
- Theme download from online repositories
- Custom theme creation tools
- Theme validation for compatibility
- Color scheme synchronization
- Per-application theme overrides
