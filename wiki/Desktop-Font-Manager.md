# Desktop Font Manager

## Overview

The Desktop Font Manager provides comprehensive font management inspired by Linux Mint's font settings and Omarchy's font utilities. It supports font families, font styles, font weights, font slants, font widths, and usage-based font configuration.

## Features

- **Font Families**: Multiple font families with generic and specific fonts
- **Font Styles**: Multiple styles per family (weight, slant, width, file path)
- **Font Weights**: Thin, Extra Light, Light, Regular, Medium, Semi Bold, Bold, Extra Bold, Black (100-900)
- **Font Slants**: Normal, Italic, Oblique
- **Font Widths**: Ultra Condensed, Extra Condensed, Condensed, Semi Condensed, Normal, Semi Expanded, Expanded, Extra Expanded, Ultra Expanded
- **Font Usage Types**: Default, Monospace, Sans Serif, Serif, Document, Interface, Title, Heading
- **Font Management**: Add, remove, and manage font families
- **Default Font**: Track and set default font
- **Usage-Based Configuration**: Set fonts for specific usage types
- **Font Size Control**: Adjustable font size (6-72)
- **Font Search**: Search font families by name
- **Statistics**: Track family count, default font status, and font size

## Components

### FontFamily

Font family with:
- Family name
- Generic flag (whether it's a generic family like "sans", "serif", "monospace")
- List of font styles

### FontStyle

Font style with:
- Style name
- Font weight (Thin to Black)
- Font slant (Normal, Italic, Oblique)
- Font width (Ultra Condensed to Ultra Expanded)
- File path to font file

### FontWeight

```rust
pub enum FontWeight {
    Thin,        // 100
    ExtraLight,  // 200
    Light,       // 300
    Regular,     // 400
    Medium,      // 500
    SemiBold,    // 600
    Bold,        // 700
    ExtraBold,   // 800
    Black,       // 900
}
```

### FontSlant

```rust
pub enum FontSlant {
    Normal,
    Italic,
    Oblique,
}
```

### FontWidth

```rust
pub enum FontWidth {
    UltraCondensed,
    ExtraCondensed,
    Condensed,
    SemiCondensed,
    Normal,
    SemiExpanded,
    Expanded,
    ExtraExpanded,
    UltraExpanded,
}
```

### FontUsageType

```rust
pub enum FontUsageType {
    Default,     // Default system font
    Monospace,   // Monospace font for code
    SansSerif,   // Sans-serif font
    Serif,       // Serif font
    Document,    // Document font
    Interface,   // UI interface font
    Title,       // Title font
    Heading,     // Heading font
}
```

### DesktopFontManager

Main management interface with:
- Family management (add, remove, retrieve)
- Default font tracking
- Usage-based font configuration
- Font size control
- Font search by name
- Statistics tracking

## Usage

### Basic Usage

```rust
use sigmaos::desktop::DesktopFontManager;

let mut manager = DesktopFontManager::new();

// Get configuration
println!("Total families: {}", manager.get_families().len());
println!("Default font: {:?}", manager.get_default_font());
println!("Font size: {}", manager.get_font_size());
```

### Family Management

```rust
// Add family
let family = FontFamily::new("Custom".to_string(), false);

let id = manager.add_family(family);

// Remove family
manager.remove_family(&id);
```

### Default Font

```rust
// Set default font
manager.set_default_font(&id);

// Get default font
if let Some(font) = manager.get_default_font() {
    println!("Default font: {}", font.name);
}
```

### Usage-Based Configuration

```rust
// Set font for specific usage
manager.set_font_for_usage(FontUsageType::Monospace, &id);
manager.set_font_for_usage(FontUsageType::Interface, &id);

// Get font for specific usage
if let Some(font) = manager.get_font_for_usage(FontUsageType::Monospace) {
    println!("Monospace font: {}", font.name);
}
```

### Font Size Control

```rust
// Set font size
manager.set_font_size(16);
println!("Font size: {}", manager.get_font_size());

// Font size is clamped to 6-72
manager.set_font_size(100);
assert_eq!(manager.get_font_size(), 72);
```

### Font Search

```rust
// Search families by name
let results = manager.search_families("sans");
for family in results {
    println!("Found: {}", family.name);
}
```

### Font Style

```rust
// Create font style
let style = FontStyle::new(
    "Regular".to_string(),
    FontWeight::Regular,
    FontSlant::Normal,
    FontWidth::Normal,
    "/path/to/font.ttf".to_string(),
);

// Add style to family
family.add_style(style);
```

### Statistics

```rust
let stats = manager.get_statistics();
println!("Total families: {}", stats.total_families);
println!("Default font set: {}", stats.default_font_set);
println!("Font size: {}", stats.font_size);
```

## Default Fonts

The manager includes default fonts:

- **Sans (Generic)**: DejaVu Sans with Regular, Bold, Italic styles
- **Serif (Generic)**: DejaVu Serif with Regular, Bold styles
- **Monospace (Generic)**: DejaVu Sans Mono with Regular, Bold styles

## Default Configuration

The Font Manager includes default configuration:

- **Default Font**: Sans
- **Monospace Font**: Monospace
- **Sans Serif Font**: Sans
- **Serif Font**: Serif
- **Document Font**: Serif
- **Interface Font**: Sans
- **Title Font**: Sans
- **Heading Font**: Sans
- **Font Size**: 12

## AI Agent Maintenance Instructions

When maintaining the Font Manager:

1. **FontConfig Integration**: Integrate with FontConfig for system font discovery
2. **Font Substitution**: Add font substitution for missing fonts
3. **Font Fallback**: Add font fallback chain configuration
4. **Font Rendering**: Add font rendering with anti-aliasing and hinting
5. **Font Installation**: Add font installation from font files
6. **Font Preview**: Add font preview functionality
7. **Font Metrics**: Add font metrics calculation (ascent, descent, line height)
8. **Font Caching**: Add font caching for performance
9. **Variable Fonts**: Add support for variable fonts (OpenType 1.8)
10. **Emoji Fonts**: Add emoji font configuration

## Testing

Run the unit tests with:

```bash
cargo test --lib desktop::font_manager
```

## Future Enhancements

- FontConfig integration for system font discovery
- Font substitution for missing fonts
- Font fallback chain configuration
- Font rendering with anti-aliasing and hinting
- Font installation from font files
- Font preview functionality
- Font metrics calculation (ascent, descent, line height)
- Font caching for performance
- Variable fonts support (OpenType 1.8)
- Emoji font configuration
