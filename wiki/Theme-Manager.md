# Theme Manager

## Overview

The Theme Manager provides comprehensive theme management inspired by Omarchy's theme system and Linux Mint's theme preferences. It supports dark/light modes, accent colors, custom theme creation, and theme variants.

## Features

- **Theme Modes**: Light, Dark, and Auto (system-controlled) modes
- **Accent Colors**: Predefined accent colors (Blue, Green, Red, Orange, Purple, Pink, Teal, Yellow) with custom RGB support
- **Window Styles**: Traditional, Modern, Compact, and Floating window styles
- **Font Management**: System and custom font families with configurable sizes
- **Color Palettes**: Predefined light and dark color palettes with customizable accent colors
- **Custom Themes**: Create and manage custom themes
- **Theme Variants**: Create variants of existing themes with different accent colors
- **Default Themes**: Built-in Sigma Dark, Sigma Light, and Sigma Auto themes

## Components

### ThemeMode

```rust
pub enum ThemeMode {
    Light,  // Light mode
    Dark,   // Dark mode
    Auto,   // Automatically switch based on system preference
}
```

### AccentColor

```rust
pub enum AccentColor {
    Blue,      // Blue accent
    Green,     // Green accent
    Red,       // Red accent
    Orange,    // Orange accent
    Purple,    // Purple accent
    Pink,      // Pink accent
    Teal,      // Teal accent
    Yellow,    // Yellow accent
    Custom,    // Custom RGB color
}
```

### WindowStyle

```rust
pub enum WindowStyle {
    Traditional,  // Traditional window decorations
    Modern,       // Modern window decorations
    Compact,      // Compact window decorations
    Floating,     // Floating window style
}
```

### ColorPalette

```rust
pub struct ColorPalette {
    pub background: (u8, u8, u8),
    pub foreground: (u8, u8, u8),
    pub surface: (u8, u8, u8),
    pub border: (u8, u8, u8),
    pub accent: (u8, u8, u8),
}
```

### ThemeConfig

Complete theme configuration including:
- Theme name and mode
- Accent color
- Window style
- Font family and size
- Color palette
- Custom theme flag

### ThemeManager

Main management interface with:
- Default theme management
- Custom theme creation
- Theme variant generation
- Current theme selection
- Theme removal (custom only)
- Statistics tracking

## Usage

### Basic Usage

```rust
use sigmaos::theming::ThemeManager;

let mut manager = ThemeManager::new();

// List all themes
let themes = manager.list_themes();

// Set current theme
manager.set_current_theme("Sigma Light")?;

// Get current theme
let current = manager.get_current_theme();
```

### Creating Custom Themes

```rust
use sigmaos::theming::{ThemeConfig, ThemeMode, AccentColor};

let custom_theme = ThemeConfig::new("My Theme".to_string(), ThemeMode::Dark)
    .with_accent(AccentColor::Purple)
    .with_window_style(WindowStyle::Modern)
    .with_font_size(16);

manager.add_theme(custom_theme);
```

### Creating Theme Variants

```rust
// Create a variant with different accent color
manager.create_variant("Sigma Dark", "Sigma Dark Teal".to_string(), AccentColor::Teal)?;
```

### Filtering Themes

```rust
// List custom themes only
let custom = manager.list_custom_themes();
```

### Removing Custom Themes

```rust
manager.remove_theme("My Theme")?;
```

## AI Agent Maintenance Instructions

When maintaining the Theme Manager:

1. **Color Accuracy**: Ensure color palette RGB values are accurate for light and dark modes
2. **Accent Application**: Verify accent colors are properly applied to all UI elements
3. **Theme Persistence**: Ensure theme configurations are properly saved and loaded
4. **Variant Generation**: Maintain proper variant creation from base themes
5. **Default Protection**: Prevent deletion of default themes
6. **Font Management**: Ensure font family and size are properly applied

## Testing

Run the unit tests with:

```bash
cargo test --lib theming::theme_manager
```

## Future Enhancements

- Integration with actual window manager/compositor
- Theme import/export functionality
- Wallpaper-based theme generation
- GTK/Qt theme integration
- Icon theme management
- Cursor theme support
