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

## Tiling Window Manager

### Tiling Layouts

COSMIC-inspired safe multi-threaded tiling:

```bash
# Create workspace
tiling create-workspace "1" spiral

# Add window to workspace
tiling add-window 1 0 0 800 600

# Switch layout
tiling set-layout 1 grid

# Switch workspace
tiling switch-workspace 2
```

### Layout Types

Available tiling layouts:
- **Spiral**: Spiral arrangement of windows
- **Monocle**: Single focused window fullscreen
- **Columns**: Vertical column layout
- **Rows**: Horizontal row layout
- **Grid**: Grid-based layout

## Gamepad Input

### Gamepad Support

Linux evdev and Xbox controller support:

```bash
# Register gamepad
gamepad register "Xbox Controller" 0x045e 0x028e

# Handle button press
gamepad button-press 1 A

# Handle button release
gamepad button-release 1 A

# Handle axis movement
gamepad axis-move 1 LeftStickX 100

# Get button state
gamepad get-button 1 A

# Get axis value
gamepad get-axis 1 LeftStickX
```

### Supported Buttons

- A, B, X, Y
- Left/Right Bumper
- Left/Right Trigger
- Back, Start
- Left/Right Stick
- D-Pad (Up, Down, Left, Right)

## Next Steps

- [Packaging](09-Packaging.md) - Package management
- [Development](10-Development.md) - Development tools
- [Configuration](03-Configuration.md) - System configuration

## AI Agent Maintenance Instructions

- **Bolt ⚡**: Ensure documentation of any new zero-allocation optimizations or performance improvements are added concisely without marketing fluff.
- **Palette 🎨**: Maintain Arch Linux wiki style: clear, factual, one page per topic, using appropriate markdown formatting and tables where necessary.
- **Sentinel 🛡️**: Verify that no hardcoded credentials or unvetted cryptographic algorithms are documented as production-ready. Ensure security limitations are accurately stated.
- **General**: Keep pages up-to-date with current repository capabilities. Remove redundant files when consolidating information.

## Reference projects and future roadmap

Use Omarchy for coherent keyboard workflows and practical customization, Linux Mint for approachable onboarding and familiar system tools, and Arch Linux for transparent configuration and documentation. These are UX references, not evidence that the corresponding SigmaOS workflow is complete.

1. Define a first-session path for display/input setup, networking, launching an app, software installation, help, and recovery.
2. Make core flows usable by keyboard with visible focus, accessible labels, and clear error recovery.
3. Connect settings and panels to discovered hardware and real system capabilities; show unsupported operations clearly.
4. Test clean installation and upgrade flows with a new user profile and publish reproducible screenshots or test steps.

**Completion evidence:** a clean-install walkthrough completes without editing internal files; keyboard navigation and failure recovery are verified for each core workflow.
