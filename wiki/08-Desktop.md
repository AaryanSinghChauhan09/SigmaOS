# Desktop

SigmaOS features the Zenith desktop environment for a modern, performant user experience.

## Zenith Compositor

Zenith is SigmaOS's native compositor providing direct hardware rendering.

### Features

- Direct GPU rendering (DRM/KMS)
- Zero-copy framebuffers
- Multi-monitor support
- Low-latency input handling
- Hardware acceleration

### Configuration

Configure Zenith:

```toml
# /etc/sigmaos/zenith.toml
[compositor]
backend = "drm"
output_scale = "auto"
vsync = true

[input]
keyboard_layout = "us"
mouse_acceleration = "adaptive"

[appearance]
theme = "dark"
font = "system-ui"
icon_theme = "sigma-icons"
```

## Applications

### Pre-installed Applications

SigmaOS includes essential applications:

- **Sigma Terminal**: Terminal emulator
- **Sigma File Manager**: File browser
- **Sigma Web**: Web browser
- **Sigma Settings**: System configuration
- **Sigma Calculator**: Calculator
- **Sigma Notes**: Note-taking application

### Installing Applications

Install applications via SigmaPkg:

```bash
# Install application
sigpkg install application-name

# List installed applications
sigpkg list

# Remove application
sigpkg remove application-name
```

## Keyboard Shortcuts

### Global Shortcuts

- **Super**: Open application launcher
- **Super+T**: Open terminal
- **Super+E**: Open file manager
- **Super+B**: Open web browser
- **Super+D**: Show desktop
- **Super+L**: Lock screen
- **Super+Shift+S**: Screenshot
- **Super+Shift+P**: Screen recording
- **Super+Esc**: Toggle theme

### Window Management

- **Super+Arrow Keys**: Move window
- **Super+Shift+Arrow Keys**: Resize window
- **Super+Enter**: Maximize window
- **Super+Shift+Enter**: Tile window
- **Super+Shift+Q**: Close window

## Themes

### Theme Selection

Change desktop theme:

```bash
# Set theme
sigma-theme set dark
sigma-theme set light
sigma-theme set auto
```

### Custom Themes

Install custom themes:

```bash
# Install theme
sigpkg install theme-name

# Apply theme
sigma-theme apply theme-name
```

## Display Configuration

### Multi-Monitor

Configure multiple monitors:

```bash
# List displays
sigdisplay list

# Configure display
sigdisplay configure --display 1 --primary
sigdisplay configure --display 2 --right-of 1
```

### Resolution

Change display resolution:

```bash
# List resolutions
sigdisplay resolutions

# Set resolution
sigdisplay set-resolution 1920x1080
```

## Accessibility

### Screen Reader

Enable screen reader:

```bash
# Enable screen reader
sigma-a11y enable screen-reader

# Configure screen reader
sigma-a11y configure screen-reader
```

### Screen Magnifier

Enable screen magnifier:

```bash
# Enable magnifier
sigma-a11y enable magnifier

# Configure magnifier
sigma-a11y configure magnifier --zoom 2.0
```

### Keyboard Accessibility

Configure keyboard accessibility:

```bash
# Enable sticky keys
sigma-a11y enable sticky-keys

# Enable slow keys
sigma-a11y enable slow-keys
```

## Next Steps

- [Packaging](09-Packaging.md) - Package management
- [Development](10-Development.md) - Development tools
- [Configuration](03-Configuration.md) - System configuration
