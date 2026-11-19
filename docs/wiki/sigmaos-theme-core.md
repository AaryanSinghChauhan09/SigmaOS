# sigmaos-theme-core

**Subcomponent of:** [SigmaOS-Themes](https://github.com/AaryanSinghChauhan09/SigmaOS-themes)

## Overview

The core theming engine for SigmaOS, providing design tokens, color palettes, and the foundational theming infrastructure used by all other SigmaOS theme components.

## Purpose

Following **SINGLE RESPONSIBILITY** and **ARCHITECTURE** principles, sigmaos-theme-core:

- Defines design tokens (colors, spacing, typography)
- Provides color palette generation
- Implements theme registry and activation
- Supplies accent color system
- Serves as dependency for icons, GTK3, and desktop themes

## Architecture

### Modules

- **config.rs** - Configuration structures and defaults
- **error.rs** - Error types with `thiserror`
- **service.rs** - Service lifecycle management
- **theme_engine.rs** - Core theming logic and registry

### Core Types

```rust
pub struct Config { /* ... */ }
pub struct ThemeComponent { /* ... */ }
pub struct Service { /* ... */ }
pub enum ServiceStatus { Stopped, Running, Failed(String) }
```

## Installation

```bash
# Build within SigmaOS-themes workspace
cd SigmaOS-themes
cargo build -p sigmaos-theme-core

# Run the binary
cargo run -p sigmaos-theme-core
```

## Usage

### Initialize Theme Component

```rust
use sigmaos_theme_core::{Config, ThemeComponent};

let mut component = ThemeComponent::new(Config::default());
component.initialize()?;
// ... use theme component
component.shutdown()?;
```

### Color Palette

The core includes predefined color palettes:
- **Sigma Blue** - Primary brand color
- **Accent colors** - Secondary and accent variants
- **Theme variants** - Different color schemes

### Theme Registry

The registry manages theme activation and ensures consistency:
- Register themes with metadata
- Activate themes by name
- Prevent duplicate registrations
- Validate theme availability

## API

### Config

```rust
impl Config {
    pub fn default() -> Self
    // Configuration fields for theming behavior
}
```

### ThemeComponent

```rust
impl ThemeComponent {
    pub fn new(config: Config) -> Self
    pub fn initialize(&mut self) -> Result<()>
    pub fn shutdown(&mut self) -> Result<()>
}
```

### Service

```rust
impl Service {
    pub fn new(config: Config) -> Self
    pub fn start(&mut self) -> Result<()>
    pub fn stop(&mut self) -> Result<()>
    pub fn is_running(&self) -> bool
}
```

## Testing

```bash
# Run core tests
cargo test -p sigmaos-theme-core
```

### Test Coverage

- `test_accent_color_hex` - Validates hex color format
- `test_sigma_blue` - Checks Sigma Blue color value
- `test_theme_variants` - Tests theme variant generation
- `test_registry_new_has_default` - Registry initialization
- `test_registry_register_and_activate` - Theme registration
- `test_registry_activate_missing_fails` - Error handling

All 6 tests passing.

## Design Tokens

### Color Palette

The core defines the SigmaOS color palette:

- **Primary**: Sigma Blue (#3B82F6)
- **Accent**: Various accent colors for different UI elements
- **Neutral**: Grayscale for text and backgrounds
- **Semantic**: Success, warning, error colors

### Typography

Font sizes, weights, and line heights defined as design tokens.

### Spacing

Consistent spacing scale (4px, 8px, 16px, 24px, 32px, etc.)

## Error Handling

Using `thiserror` for structured error types:

```rust
pub enum ComponentError {
    Service(String),
    Theme(String),
    Config(String),
}
```

Following **DEFENSIVE PROGRAMMING** and **FAIL-FAST** principles.

## Dependencies

- `anyhow` - Error handling
- `thiserror` - Structured error types
- `log` - Logging facade
- `env_logger` - Logger implementation
- `serde` - Serialization
- `toml` - TOML parsing

## Integration

Used by:
- **sigmaos-theme-icons** - For color-aware icon selection
- **sigmaos-theme-gtk3** - For GTK3 CSS generation
- **sigmaos-theme-desktop** - For desktop theme styling

## Contributing

Following **KISS** and **DRY**:
- Keep theme logic simple and composable
- Reuse design tokens across components
- Add tests for new color schemes
- Update COMPONENT.toml for API changes

## License

MIT OR Apache-2.0

## See Also

- [SigmaOS-Themes](SigmaOS-Themes.md) - Parent repository
- [sigmaos-theme-icons](sigmaos-theme-icons.md) - Icon theme component
- [sigmaos-theme-gtk3](sigmaos-theme-gtk3.md) - GTK3 theme component
- [sigmaos-theme-desktop](sigmaos-theme-desktop.md) - Desktop theme component
