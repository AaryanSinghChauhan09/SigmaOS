# SigmaOS-Themes

**Repository:** https://github.com/AaryanSinghChauhan09/SigmaOS-themes

## Overview

SigmaOS-Themes is a unified theming system for SigmaOS, providing icons, GTK3 themes, desktop themes, and design tokens in a monorepo structure. Following **CONSOLIDATE GITHUB REPOS** and **MODULARITY** principles, it consolidates 4 separate theming repositories into one cohesive workspace.

## Components

The repository contains 4 subcomponents, each with its own Cargo crate:

- **sigmaos-theme-core** - Core theming engine, design tokens & color palettes
- **sigmaos-theme-icons** - Icon theme pack
- **sigmaos-theme-gtk3** - GTK3 theme engine
- **sigmaos-theme-desktop** - Desktop-shell theme

## Architecture

### Workspace Structure

```
SigmaOS-themes/
├── core/          # sigmaos-theme-core
├── icons/         # sigmaos-theme-icons
├── gtk3/          # sigmaos-theme-gtk3
├── desktop/       # sigmaos-theme-desktop
├── Cargo.toml      # Workspace configuration
└── README.md
```

### Component Dependencies

```
sigmaos-theme-icons → sigmaos-theme-core
sigmaos-theme-gtk3 → sigmaos-theme-core
sigmaos-theme-desktop → sigmaos-theme-core
```

All theme components depend on the core theming engine for design tokens and color palettes.

## Installation

### Build from Source

```bash
# Clone the repository
git clone https://github.com/AaryanSinghChauhan09/SigmaOS-themes.git
cd SigmaOS-themes

# Build all components
cargo build --release

# Build specific component
cargo build -p sigmaos-theme-core
cargo build -p sigmaos-theme-icons
cargo build -p sigmaos-theme-gtk3
cargo build -p sigmaos-theme-desktop
```

### Offline Build

```bash
cargo build --offline
```

## Usage

### Core Theming Engine

```bash
# Run the core theming engine
cargo run -p sigmaos-theme-core
```

The core engine provides:
- Design token management
- Color palette generation
- Theme registry and activation
- Accent color system

### Icon Theme

```bash
# Run icon theme tool
cargo run -p sigmaos-theme-icons
```

The icon theme provides:
- Icon registration and lookup
- Size-based icon selection
- Directory-based icon management
- Fallback mechanisms

### GTK3 Theme

```bash
# Run GTK3 theme generator
cargo run -p sigmaos-theme-gtk3
```

The GTK3 theme provides:
- CSS generation for GTK3
- Palette token management
- Border radius validation
- Theme variant support (latte, mocha)

### Desktop Theme

```bash
# Run desktop theme tool
cargo run -p sigmaos-theme-desktop
```

The desktop theme provides:
- Desktop-shell CSS generation
- Panel configuration
- Opacity and border radius settings
- Window decoration styling

## Development

### Running Tests

```bash
# Run all tests
cargo test --workspace

# Run tests for specific component
cargo test -p sigmaos-theme-core
cargo test -p sigmaos-theme-icons
cargo test -p sigmaos-theme-gtk3
cargo test -p sigmaos-theme-desktop
```

### Test Coverage

- **sigmaos-theme-core**: 6 tests (theme engine, registry, colors)
- **sigmaos-theme-desktop**: 6 tests (config validation, CSS generation)
- **sigmaos-theme-gtk3**: 5 tests (palette tokens, GTK3 CSS)
- **sigmaos-theme-icons**: 5 tests (icon lookup, directory registration)

Total: 22 tests, all passing

### Code Style

Following **KISS**, **DRY**, and **SOLID** principles:
- Each component has clear single responsibility
- Common patterns extracted to core
- Minimal dependencies between components
- Extensive error handling with `anyhow` and `thiserror`

## Configuration

### Design Tokens

Design tokens are defined in the core component and include:
- Color palettes (Sigma Blue, accent colors)
- Typography scales
- Spacing units
- Border radius values

### Component Metadata

Each subdirectory contains a `COMPONENT.toml` file with:
- Component name and description
- Repository URL
- Engine type
- Category
- Language
- Visibility
- Dependencies

## Integration with SigmaOS

The SigmaOS-themes repository is registered in the main SigmaOS component registry (`COMPONENTS-FOCUSED.toml`):

```toml
[[component]]
name = "SigmaOS-themes"
description = "Unified theming system: icons, GTK3, desktop themes, design tokens"
repo = "https://github.com/AaryanSinghChauhan09/SigmaOS-themes"
engine = "theme_system"
category = "Desktop/UI"
language = "rust"
inspirations = "SigmaOS"
visibility = "public"
depends_on = []
```

## Contributing

Following **TALK IS CHEAP, SHOW ME THE CODE** and **TEST CODE USING CI/CD**:

1. All changes must build: `cargo build --offline`
2. All tests must pass: `cargo test --offline`
3. Follow existing code patterns
4. Update COMPONENT.toml if adding new subcomponents
5. Use semantic versioning for releases

## License

MIT OR Apache-2.0

## Related Components

- [SigmaOS](https://github.com/AaryanSinghChauhan09/SigmaOS) - Main SigmaOS repository
- [sigmaos-wallust-themes](https://github.com/AaryanSinghChauhan09/SigmaOS) - Wallust color presets (included in main repo)

## Changelog

### Version 1.0.0 (2026-11-19)

- Initial consolidation of 4 theme repositories
- All 22 tests passing
- External inspirations removed (normalized to SigmaOS)
- Workspace structure with 4 subcomponents
