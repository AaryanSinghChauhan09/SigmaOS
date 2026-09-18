# Zenith Compositor

Zenith is SigmaOS's modern, Rust-based Wayland compositor designed for speed, security, and elegance. Built with inspiration from Hyprland, Mutter, and wlroots, Zenith provides a complete desktop environment with advanced features.

## Features

### Dynamic Window Tiling
- Automatic window arrangement with intelligent layout algorithms
- Support for floating windows alongside tiled windows
- Workspace management with multiple dynamic workspaces
- Master-Stack layout similar to i3/sway but for Wayland

### Performance
- Hardware-accelerated rendering with Vulkan and OpenGL ES
- Zero-copy buffer sharing between clients
- Optimized render pipeline with minimal CPU overhead
- Adaptive refresh rate synchronization

### Security
- Capability-based security model integrated with Sentinel
- XDG Activation protocol for secure client activation
- Screen capture protection and privacy controls
- Secure input handling with sandboxed input methods

### Visual Effects
- Smooth animations with configurable easing curves
- Blur and transparency effects
- Dynamic shadows with configurable blur radius
- Color temperature adjustment (blue light filter)

### Input Handling
- Advanced pointer gesture support
- Multi-touch touchpad support with natural scrolling
- Per-device input configuration
- Input method editor (IME) integration

### Multi-Monitor
- Full multi-monitor support with independent configurations
- Display profile support for different setups
- Hot-plug detection and configuration
- Custom DPI scaling per display

## Configuration

Zenith uses a declarative configuration file in `~/.config/zenith/config.toml`:

```toml
[general]
input_method = "fcitx"
# "wayland", "xwayland"
backend = "wayland"

[output]
scale = 1.0
adaptive_sync = true

[input]
touchpad_natural_scroll = true
touchpad_tap_to_click = true

[window]
border_size = 2
border_color = "#89b4fa"
gap_size = 8
```

## Keybindings

Default keybindings are designed for efficiency:

- `Super + Enter`: Open terminal
- `Super + Shift + Enter`: Open application launcher
- `Super + q`: Close focused window
- `Super + Shift + c`: Kill focused window
- `Super + [1-9]`: Switch to workspace
- `Super + Shift + [1-9]`: Move window to workspace
- `Super + Shift + Space`: Toggle floating mode
- `Super + j/k/l/;`: Move focus in layout

## Integration

Zenith integrates with SigmaOS's subsystems:

- **Sentinel**: Capability-based sandboxing for all applications
- **Bolt**: Fast path launcher for rapid application launch
- **Palette**: Dynamic theme engine for consistent theming
- **SigmaPkg**: Package manager integration for software installation

## Development

Zenith is built as a pure Rust Wayland compositor using:
- `wlroots` for Wayland protocol handling
- `smithay` for client-side decorations
- Custom rendering pipeline for optimal performance

The compositor is designed to be extensible with plugin support for custom layouts, effects, and input handlers.

## Troubleshooting

### Display Issues
If Zenith fails to start:
1. Check GPU driver support: `zenith --check`
2. Verify Wayland session: `echo $WAYLAND_DISPLAY`
3. Check logs: `journalctl -u zenith -f`

### Input Issues
If touchpad gestures don't work:
1. Check libinput configuration: `libinput list-devices`
2. Verify touchpad is recognized: `zenith --list-inputs`
3. Reset input configuration: `zenith --reset-input`

### Performance Issues
If animations are slow:
1. Check GPU acceleration: `zenith --check-gpu`
2. Reduce blur effects in config
3. Disable adaptive sync if unsupported

---

**[Desktop Environment](Category-Desktop-Environment)** | **[Hyprland Integration](Omarchy-Advanced-Parity)** | **[System Administration](System-Administration)**
