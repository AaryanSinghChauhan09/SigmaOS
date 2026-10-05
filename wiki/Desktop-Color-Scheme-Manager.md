# Desktop Color Scheme Manager

## Overview

The Desktop Color Scheme Manager provides comprehensive color scheme management inspired by Linux Mint's appearance settings and Omarchy's color utilities. It supports light, dark, high-contrast, and custom color schemes with full color customization.

## Features

- **Scheme Types**: Light, Dark, High Contrast, Custom
- **Color Customization**: Background, foreground, accent, secondary, success, warning, error colors
- **Scheme Management**: Add, remove, and manage color schemes
- **Current Scheme**: Track and switch current color scheme
- **Light/Dark Toggle**: Quick toggle between light and dark schemes
- **Scheme Filtering**: Filter schemes by type
- **Search**: Search schemes by name and author
- **Default Schemes**: Pre-configured schemes from GNOME, Linux Mint, and high-contrast standards
- **Statistics**: Track scheme counts by type and current scheme status

## Components

### ColorSchemeType

```rust
pub enum ColorSchemeType {
    Light,         // Light scheme
    Dark,          // Dark scheme
    HighContrast,  // High contrast scheme
    Custom,        // Custom scheme
}
```

### ColorScheme

Color scheme with:
- Scheme ID and name
- Scheme type
- Background color
- Foreground color
- Accent color
- Secondary color
- Success color
- Warning color
- Error color
- Author

### DesktopColorSchemeManager

Main management interface with:
- Scheme management (add, remove, retrieve)
- Current scheme tracking
- Light/dark toggle
- Color customization
- Scheme filtering by type
- Search functionality
- Default schemes from major projects
- Statistics tracking

## Usage

### Basic Usage

```rust
use sigmaos::desktop::DesktopColorSchemeManager;

let mut manager = DesktopColorSchemeManager::new();

// Get configuration
println!("Total schemes: {}", manager.get_schemes().len());
println!("Current scheme: {:?}", manager.get_current_scheme());
```

### Scheme Management

```rust
// Add scheme
let new_scheme = ColorScheme::new(
    "custom".to_string(),
    "Custom Scheme".to_string(),
    ColorSchemeType::Custom,
    "#000000".to_string(),
    "#FFFFFF".to_string(),
    "#FF0000".to_string(),
    "Author".to_string(),
)
.with_secondary("#222222".to_string())
.with_success("#00FF00".to_string())
.with_warning("#FFFF00".to_string())
.with_error("#FF0000".to_string());

let id = manager.add_scheme(new_scheme);

// Remove scheme
manager.remove_scheme(&id);
```

### Set Current Scheme

```rust
// Set current scheme
manager.set_current_scheme(&scheme_id);

// Get current scheme
if let Some(scheme) = manager.get_current_scheme() {
    println!("Current scheme: {}", scheme.name);
    println!("Background: {}", scheme.background);
    println!("Foreground: {}", scheme.foreground);
}
```

### Light/Dark Toggle

```rust
// Toggle between light and dark
manager.toggle_light_dark();
```

### Scheme Filtering

```rust
// Get light schemes
let light_schemes = manager.get_schemes_by_type(ColorSchemeType::Light);

// Get dark schemes
let dark_schemes = manager.get_schemes_by_type(ColorSchemeType::Dark);

// Get high-contrast schemes
let hc_schemes = manager.get_schemes_by_type(ColorSchemeType::HighContrast);
```

### Search

```rust
// Search schemes
let results = manager.search_schemes("Adwaita");
for scheme in results {
    println!("Found: {}", scheme.name);
}
```

### Statistics

```rust
let stats = manager.get_statistics();
println!("Total schemes: {}", stats.total_schemes);
println!("Light schemes: {}", stats.light_schemes);
println!("Dark schemes: {}", stats.dark_schemes);
println!("High contrast schemes: {}", stats.high_contrast_schemes);
println!("Custom schemes: {}", stats.custom_schemes);
```

## Default Schemes

The manager includes default schemes from major projects:

### Light Schemes
- **Adwaita Light**: GNOME default light scheme
- **Mint Light**: Linux Mint light scheme

### Dark Schemes
- **Adwaita Dark**: GNOME default dark scheme
- **Mint Dark**: Linux Mint dark scheme

### High Contrast Schemes
- **High Contrast Light**: Universal high-contrast light
- **High Contrast Dark**: Universal high-contrast dark

## Default Configuration

The Color Scheme Manager includes default configuration:

- **Current Scheme**: Adwaita Dark

## Color Values

Each color scheme includes:

- **Background**: Primary background color
- **Foreground**: Primary text color
- **Accent**: Accent/highlight color
- **Secondary**: Secondary/neutral color
- **Success**: Success/positive color (green)
- **Warning**: Warning color (orange/yellow)
- **Error**: Error/negative color (red)

## AI Agent Maintenance Instructions

When maintaining the Color Scheme Manager:

1. **Color Validation**: Validate color hex codes for correctness
2. **Color Import/Export**: Add color scheme import/export functionality
3. **Color Picker**: Integrate with system color picker
4. **Palette Generation**: Generate color palettes from accent color
5. **Theme Sync**: Sync color schemes with theme preferences
6. **Accessibility**: Ensure color contrast ratios meet WCAG standards
7. **Adaptive Colors**: Add adaptive color schemes based on time of day
8. **Color Blind Friendly**: Add color-blind-friendly schemes
9. **Gradient Support**: Add gradient color support
10. **Per-Application Schemes**: Add per-application color scheme overrides

## Testing

Run the unit tests with:

```bash
cargo test --lib desktop::color_scheme_manager
```

## Future Enhancements

- Color validation and preview
- Color scheme import/export
- System color picker integration
- Palette generation from accent color
- Theme synchronization
- WCAG accessibility compliance checking
- Adaptive color schemes (time-based)
- Color-blind-friendly schemes
- Gradient color support
- Per-application color scheme overrides
