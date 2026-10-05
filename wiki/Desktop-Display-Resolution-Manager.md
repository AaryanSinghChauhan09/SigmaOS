# Desktop Display Resolution Manager

## Overview

The Desktop Display Resolution Manager provides comprehensive display and resolution management inspired by Linux Mint's display settings and Omarchy's display utilities. It supports multiple displays, resolution configuration, refresh rate selection, and display positioning.

## Features

- **Resolution Management**: Configure display resolutions (width, height)
- **Refresh Rate**: Configure refresh rates for each resolution
- **Multiple Displays**: Support for multiple displays
- **Primary Display**: Set and track primary display
- **Display Enable/Disable**: Enable or disable individual displays
- **Display Positioning**: Configure display position (x, y coordinates)
- **Display Modes**: Multiple display modes per display
- **Preferred Mode**: Mark preferred/resolution mode
- **Aspect Ratio Calculation**: Calculate aspect ratio for resolutions
- **Default Display**: Pre-configured display with common resolutions

## Components

### Resolution

```rust
pub struct Resolution {
    pub width: u32,
    pub height: u32,
}
```

### RefreshRate

```rust
pub struct RefreshRate {
    pub hz: u32,
}
```

### DisplayMode

Display mode with:
- Mode ID
- Resolution
- Refresh rate
- Preferred flag

### Display

Display with:
- Display ID and name
- Primary display flag
- Enable/disable status
- Current mode
- Available modes
- Position (x, y)

### DisplayResolutionManager

Main management interface with:
- Display management (add, remove, enable, disable)
- Primary display selection
- Display positioning
- Display mode management
- Resolution and refresh rate configuration
- Default display with common resolutions
- Statistics tracking

## Usage

### Basic Usage

```rust
use sigmaos::desktop::DisplayResolutionManager;

let mut manager = DisplayResolutionManager::new();

// Get displays
let displays = manager.get_displays();
println!("Total displays: {}", displays.len());
```

### Display Management

```rust
// Add display
let id = manager.add_display("HDMI-1".to_string());

// Remove display
manager.remove_display(&id);

// Enable/disable display
manager.set_display_enabled(&id, false);
```

### Primary Display

```rust
// Set primary display
manager.set_primary_display(&id);

// Get primary display
if let Some(primary) = manager.get_primary_display() {
    println!("Primary: {}", primary.name);
}
```

### Display Positioning

```rust
// Set display position
manager.set_display_position(&id, 1920, 0);
```

### Display Modes

```rust
// Add display mode
let mode = DisplayMode::new(
    "mode_id".to_string(),
    Resolution::new(2560, 1440),
    RefreshRate::new(144),
);
manager.add_display_mode(&display_id, mode);

// Set current mode
manager.set_display_mode(&display_id, "mode_id".to_string());
```

### Resolution

```rust
// Create resolution
let res = Resolution::new(1920, 1080);

// Get string representation
println!("{}", res.as_str()); // "1920x1080"

// Calculate aspect ratio
let ratio = res.aspect_ratio();
println!("Aspect ratio: {:.2}", ratio); // 1.78
```

### Statistics

```rust
let stats = manager.get_statistics();
println!("Total displays: {}", stats.total_displays);
println!("Enabled displays: {}", stats.enabled_displays);
println!("Primary display set: {}", stats.primary_display);
```

## Default Display

The manager includes a default display with common resolutions:

- **1920x1080 @ 60Hz** (Preferred)
- **2560x1440 @ 60Hz**
- **3840x2160 @ 60Hz**

## Default Configuration

The Display Resolution Manager includes default configuration:

- **Default Display**: eDP-1 (laptop display)
- **Primary Display**: eDP-1
- **All Displays Enabled**: true
- **Default Position**: (0, 0)

## AI Agent Maintenance Instructions

When maintaining the Display Resolution Manager:

1. **Display Detection**: Integrate with actual display detection (EDID, xrandr, etc.)
2. **Mode Detection**: Detect available modes from display hardware
3. **Display Switching**: Implement actual display mode switching
4. **Hot-plug Support**: Add support for display hot-plug events
5. **Scaling**: Implement display scaling configuration
6. **Rotation**: Add display rotation support
6. **Mirroring**: Implement display mirroring configuration
7. **Color Profile**: Add color profile management
8. **HDR Support**: Add HDR display support
9. **VRR Support**: Add variable refresh rate support
10. **Multi-Monitor**: Implement multi-monitor setups (extended, mirrored)

## Testing

Run the unit tests with:

```bash
cargo test --lib desktop::display_resolution_manager
```

## Future Enhancements

- Integration with actual display detection (EDID, xrandr)
- Display mode switching
- Display hot-plug support
- Display scaling configuration
- Display rotation support
- Display mirroring configuration
- Color profile management
- HDR display support
- Variable refresh rate support
- Multi-monitor setups (extended, mirrored)
- Custom resolution support
