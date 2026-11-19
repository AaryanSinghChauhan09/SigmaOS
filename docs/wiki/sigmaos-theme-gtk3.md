# sigmaos-theme-gtk3

**Subcomponent of:** [SigmaOS-Themes](https://github.com/AaryanSinghChauhan09/SigmaOS-themes)

## Overview

SigmaOS GTK3 theme engine, generating CSS stylesheets for GTK3 applications with palette tokens, border radius validation, and theme variant support (latte, mocha). Depends on sigmaos-theme-core for design tokens.

## Purpose

Following **SINGLE RESPONSIBILITY** and **SEPARATION OF CONCERNS**:

- Generate GTK3 CSS from design tokens
- Manage palette tokens for colors
- Validate border radius bounds
- Support theme variants (latte, mocha)
- Integrate with core color palettes

## Architecture

### Modules

- **config.rs** - Configuration for GTK3 theme
- **error.rs** - Error types with `thiserror`
- **icon.rs** - Icon integration for GTK3
- **palette.rs** - Palette token management
- **service.rs** - Service lifecycle management

### Core Types

```rust
pub struct Gtk3Theme { /* ... */ }
pub struct Palette { /* ... */ }
pub struct Config { /* ... */ }
```

## Installation

```bash
# Build within SigmaOS-themes workspace
cd SigmaOS-themes
cargo build -p sigmaos-theme-gtk3

# Run the binary
cargo run -p sigmaos-theme-gtk3
```

## Usage

### Generate GTK3 CSS

```rust
use sigmaos_theme_gtk3::Gtk3Theme;

let theme = Gtk3Theme::new(Config::default());
let css = theme.generate_css()?;
// Write to ~/.config/gtk-3.0/gtk.css
```

### Theme Variants

Supports two main variants:
- **Latte** - Light theme with bright colors
- **Mocha** - Dark theme with muted colors

```rust
let theme = Gtk3Theme::with_variant(Config::default(), "mocha");
```

### Palette Tokens

Palette tokens define CSS color variables:
- Background colors
- Foreground colors
- Accent colors
- Border colors
- Selection colors

## API

### Gtk3Theme

```rust
impl Gtk3Theme {
    pub fn new(config: Config) -> Self
    pub fn with_variant(config: Config, variant: &str) -> Self
    pub fn generate_css(&self) -> Result<String>
    pub fn validate_config(&self) -> Result<()>
}
```

### Palette

```rust
impl Palette {
    pub fn new() -> Self
    pub fn set_token(&mut self, name: &str, value: &str)
    pub fn get_token(&self, name: &str) -> Option<&str>
    pub fn latte_palette() -> Self
    pub fn mocha_palette() -> Self
}
```

## Testing

```bash
# Run GTK3 tests
cargo test -p sigmaos-theme-gtk3
```

### Test Coverage

- `test_border_radius_bounds_check` - Border radius validation
- `test_compile_gtk3_css_contains_definitions` - CSS generation
- `test_error_display_formatting` - Error messages
- `test_latte_palette_token_values` - Latte variant colors
- `test_mocha_palette_token_values` - Mocha variant colors

All 5 tests passing.

## CSS Structure

Generated CSS includes:

```css
@define-color bg_color #ffffff;
@define-color fg_color #000000;
@define-color accent_color #3B82F6;
/* ... more tokens */

window {
    background-color: @bg_color;
    color: @fg_color;
}
```

## Border Radius Validation

Following **DEFENSIVE PROGRAMMING**:
- Validates border radius bounds (0-50px)
- Prevents invalid CSS
- Provides clear error messages

## Integration with Core

Uses design tokens from sigmaos-theme-core:
- Color palette integration
- Accent color system
- Theme variant selection

## Error Handling

Structured errors using `thiserror`:

```rust
pub enum Gtk3Error {
    InvalidRadius(String),
    InvalidToken(String),
    GenerationFailed(String),
}
```

## Dependencies

- `anyhow` - Error handling
- `thiserror` - Structured error types
- `log` - Logging facade
- `env_logger` - Logger implementation
- `serde` - Serialization
- `toml` - TOML parsing
- **sigmaos-theme-core** - Design tokens (workspace dependency)

## Configuration

GTK3 theme configuration includes:
- Border radius values
- Palette selection (latte/mocha)
- Custom token overrides
- CSS output path

## Theme Variants

### Latte (Light)
- Bright backgrounds
- Dark text
- Vibrant accents
- High contrast

### Mocha (Dark)
- Dark backgrounds
- Light text
- Muted accents
- Eye-friendly for night use

## Use Cases

- GTK3 application theming
- Desktop environment integration
- Wayland compositors with GTK3
- Legacy application support

## Contributing

Following **KISS** and **UPSTREAM FIRST**:
- Follow GTK3 CSS guidelines
- Test with real GTK3 applications
- Support standard GTK3 widgets
- Maintain backward compatibility

## License

MIT OR Apache-2.0

## See Also

- [SigmaOS-Themes](SigmaOS-Themes.md) - Parent repository
- [sigmaos-theme-core](sigmaos-theme-core.md) - Core theming engine
- GTK3 CSS Documentation
