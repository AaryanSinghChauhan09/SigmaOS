# sigmaos-theme-icons

**Subcomponent of:** [SigmaOS-Themes](https://github.com/AaryanSinghChauhan09/SigmaOS-themes)

## Overview

SigmaOS icon theme pack, providing icon registration, lookup, and size-based selection with fallback mechanisms. Depends on sigmaos-theme-core for design tokens.

## Purpose

Following **SINGLE RESPONSIBILITY** and **COMPONENT ISOLATION**:

- Register icons from directories
- Lookup icons by name and size
- Select closest size with fallback
- Manage icon directories
- Integrate with core design tokens

## Architecture

### Modules

- **config.rs** - Configuration for icon theme
- **error.rs** - Error types with `thiserror`
- **icon.rs** - Icon data structures and lookup logic
- **palette.rs** - Color palette integration
- **service.rs** - Service lifecycle management

### Core Types

```rust
pub struct Icon { /* ... */ }
pub struct IconTheme { /* ... */ }
pub struct Config { /* ... */ }
```

## Installation

```bash
# Build within SigmaOS-themes workspace
cd SigmaOS-themes
cargo build -p sigmaos-theme-icons

# Run the binary
cargo run -p sigmaos-theme-icons
```

## Usage

### Register Icon Directory

```rust
use sigmaos_theme_icons::IconTheme;

let mut theme = IconTheme::new();
theme.register_directory("/usr/share/icons/hicolor")?;
```

### Lookup Icon

```rust
let icon = theme.lookup_icon("application-x-executable", 48)?;
// Returns closest available size
```

### Size Fallback

If exact size not available, automatically selects closest size:
- 48px requested → 48px if available
- Otherwise → 32px or 64px (closest match)
- Prevents broken icon rendering

## API

### IconTheme

```rust
impl IconTheme {
    pub fn new() -> Self
    pub fn register_directory(&mut self, path: &str) -> Result<()>
    pub fn lookup_icon(&self, name: &str, size: u32) -> Result<Icon>
    pub fn find_closest_size(&self, target: u32) -> Option<u32>
}
```

### Icon

```rust
pub struct Icon {
    pub name: String,
    pub size: u32,
    pub path: PathBuf,
}
```

## Testing

```bash
# Run icons tests
cargo test -p sigmaos-theme-icons
```

### Test Coverage

- `test_create_theme_with_standard_defaults` - Default configuration
- `test_register_and_lookup_exact_icon` - Icon registration and lookup
- `test_find_closest_size_fallback` - Size fallback logic
- `test_directory_registration` - Directory handling
- `test_error_display_formatting` - Error messages

All 5 tests passing.

## Icon Standards

Follows freedesktop.org icon theme specification:
- Standard icon directories (16x16, 24x24, 32x32, 48x48, 64x64, etc.)
- Icon naming conventions
- Category-based organization
- Scalable SVG support

## Integration with Core

Uses design tokens from sigmaos-theme-core:
- Color-aware icon selection
- Accent color integration
- Theme variant support

## Error Handling

Structured errors using `thiserror`:

```rust
pub enum IconError {
    NotFound(String),
    InvalidDirectory(String),
    Io(String),
}
```

Following **DEFENSIVE PROGRAMMING** and **FAIL-FAST**.

## Dependencies

- `anyhow` - Error handling
- `thiserror` - Structured error types
- `log` - Logging facade
- `env_logger` - Logger implementation
- `serde` - Serialization
- `toml` - TOML parsing
- **sigmaos-theme-core** - Design tokens (workspace dependency)

## Configuration

Icon theme configuration includes:
- Icon search paths
- Default icon size
- Fallback size preferences
- Theme variant selection

## Use Cases

- Application icons in desktop environment
- Status bar icons
- File manager icons
- Launcher icons
- System tray icons

## Contributing

Following **KISS** and **COMPONENT ISOLATION**:
- Keep icon lookup logic simple
- Don't add GUI rendering (use separate component)
- Test with various icon themes
- Support standard icon naming

## License

MIT OR Apache-2.0

## See Also

- [SigmaOS-Themes](SigmaOS-Themes.md) - Parent repository
- [sigmaos-theme-core](sigmaos-theme-core.md) - Core theming engine
- Freedesktop.org Icon Theme Specification
