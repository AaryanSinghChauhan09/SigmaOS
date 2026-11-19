# sigmaos-theme-desktop

**Subcomponent of:** [SigmaOS-Themes](https://github.com/AaryanSinghChauhan09/SigmaOS-themes)

## Overview

SigmaOS desktop-shell theme, generating CSS stylesheets for desktop environments (panels, window decorations, launchers) with panel configuration, opacity settings, and border radius validation. Depends on sigmaos-theme-core for design tokens.

## Purpose

Following **SINGLE RESPONSIBILITY** and **SEPARATION OF MECHANISM & POLICY**:

- Generate desktop-shell CSS from design tokens
- Configure panel properties (height, position)
- Manage opacity and transparency
- Validate border radius bounds
- Integrate with core color palettes

## Architecture

### Modules

- **config.rs** - Configuration for desktop theme
- **error.rs** - Error types with `thiserror`
- **icon.rs** - Icon integration for desktop
- **palette.rs** - Palette token management
- **service.rs** - Service lifecycle management

### Core Types

```rust
pub struct DesktopTheme { /* ... */ }
pub struct PanelConfig { /* ... */ }
pub struct Config { /* ... */ }
```

## Installation

```bash
# Build within SigmaOS-themes workspace
cd SigmaOS-themes
cargo build -p sigmaos-theme-desktop

# Run the binary
cargo run -p sigmaos-theme-desktop
```

## Usage

### Generate Desktop CSS

```rust
use sigmaos_theme_desktop::DesktopTheme;

let theme = DesktopTheme::new(Config::default());
let css = theme.generate_css()?;
// Write to desktop environment config
```

### Panel Configuration

Configure panel properties:

```rust
let config = Config {
    panel_height: 48,
    panel_opacity: 0.9,
    border_radius: 12,
    ..Default::default()
};
```

### Opacity Settings

Control transparency levels:
- Panel opacity (0.0 - 1.0)
- Window decoration opacity
- Background blur effects

## API

### DesktopTheme

```rust
impl DesktopTheme {
    pub fn new(config: Config) -> Self
    pub fn generate_css(&self) -> Result<String>
    pub fn validate_config(&self) -> Result<()>
}
```

### PanelConfig

```rust
pub struct PanelConfig {
    pub height: u32,
    pub position: PanelPosition,
    pub opacity: f32,
    pub border_radius: u32,
}
```

## Testing

```bash
# Run desktop tests
cargo test -p sigmaos-theme-desktop
```

### Test Coverage

- `test_default_config_values_are_valid` - Default configuration
- `test_border_radius_validation_bounds` - Border radius validation
- `test_panel_height_validation_bounds` - Panel height validation
- `test_opacity_validation_bounds` - Opacity validation
- `test_generate_css_contains_properties` - CSS generation
- `test_error_display_formatting` - Error messages

All 6 tests passing.

## CSS Structure

Generated CSS includes:

```css
.panel {
    height: 48px;
    background-color: rgba(59, 130, 246, 0.9);
    border-radius: 12px;
}

.window-decoration {
    border-radius: 12px;
    opacity: 0.95;
}
```

## Validation Rules

Following **DEFENSIVE PROGRAMMING**:

- Border radius: 0-50px
- Panel height: 24-100px
- Opacity: 0.0-1.0
- Invalid values cause clear errors

## Integration with Core

Uses design tokens from sigmaos-theme-core:
- Color palette integration
- Accent color system
- Spacing tokens

## Error Handling

Structured errors using `thiserror`:

```rust
pub enum DesktopError {
    InvalidRadius(String),
    InvalidHeight(String),
    InvalidOpacity(String),
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

Desktop theme configuration includes:
- Panel height and position
- Opacity values
- Border radius settings
- Window decoration style

## Panel Positions

- **Top** - Panel at top of screen
- **Bottom** - Panel at bottom of screen
- **Left** - Panel at left edge
- **Right** - Panel at right edge

## Use Cases

- Waybar theming
- Hyprland decorations
- Panel configuration
- Window decorations
- Launcher styling

## Contributing

Following **KISS** and **MODULARITY**:
- Keep desktop CSS generation simple
- Don't add desktop shell logic (use separate component)
- Test with Waybar, Hyprland
- Support standard CSS properties

## License

MIT OR Apache-2.0

## See Also

- [SigmaOS-Themes](SigmaOS-Themes.md) - Parent repository
- [sigmaos-theme-core](sigmaos-theme-core.md) - Core theming engine
- [Hyprland-Configuration](Hyprland-Configuration.md) - Hyprland setup
