# Desktop Cursor Manager

## Overview

The Desktop Cursor Manager provides comprehensive cursor management inspired by Linux Mint's cursor settings and Omarchy's cursor utilities. It supports cursor themes, cursor size control, cursor speed adjustment, and cursor type configuration.

## Features

- **Cursor Sizes**: Small, Medium, Large, Extra Large (16-48 pixels)
- **Cursor Types**: Default, Pointer, Text, Move, Resize Horizontal, Resize Vertical, Resize Diagonal, Busy, Progress, Crosshair, Hand, Help, Not Allowed
- **Cursor Themes**: Multiple cursor themes with author information
- **Cursor Management**: Add, remove, and manage cursor themes
- **Current Theme**: Track and switch current cursor theme
- **Cursor Size Control**: Adjustable cursor size (Small, Medium, Large, Extra Large)
- **Cursor Speed Control**: Adjustable cursor speed (0-100)
- **Default Cursor**: Set default cursor type
- **Per-Theme Configuration**: Add and remove cursors from themes
- **Statistics**: Track theme count, current theme status, cursor size, and cursor speed

## Components

### DesktopCursorSize

```rust
pub enum DesktopCursorSize {
    Small,       // 16 pixels
    Medium,      // 24 pixels
    Large,       // 32 pixels
    ExtraLarge,  // 48 pixels
}
```

### DesktopCursorType

```rust
pub enum DesktopCursorType {
    Default,             // Default pointer
    Pointer,             // Link pointer
    Text,                // Text selection
    Move,                // Move object
    ResizeHorizontal,    // Horizontal resize
    ResizeVertical,      // Vertical resize
    ResizeDiagonal,      // Diagonal resize
    Busy,                // Working/busy
    Progress,            // Progress indicator
    Crosshair,           // Crosshair cursor
    Hand,                // Hand cursor
    Help,                // Help cursor
    NotAllowed,          // Not allowed cursor
}
```

### DesktopCursorTheme

Cursor theme with:
- Theme ID and name
- Author
- Cursor mappings (type → file path)

### DesktopCursorManager

Main management interface with:
- Theme management (add, remove, retrieve)
- Current theme tracking
- Cursor size control
- Cursor speed control
- Default cursor type
- Cursor retrieval for specific types
- Per-theme cursor configuration
- Statistics tracking

## Usage

### Basic Usage

```rust
use sigmaos::desktop::DesktopCursorManager;

let mut manager = DesktopCursorManager::new();

// Get configuration
println!("Total themes: {}", manager.get_themes().len());
println!("Current theme: {:?}", manager.get_current_theme());
println!("Cursor size: {}", manager.get_cursor_size().as_str());
println!("Cursor speed: {}", manager.get_cursor_speed());
```

### Theme Management

```rust
// Add theme
let theme = DesktopCursorTheme::new(
    "custom".to_string(),
    "Custom Theme".to_string(),
    "Author".to_string(),
);

let id = manager.add_theme(theme);

// Remove theme
manager.remove_theme(&id);

// Set current theme
manager.set_current_theme(&id);
```

### Cursor Size Control

```rust
// Set cursor size
manager.set_cursor_size(DesktopCursorSize::Large);
println!("Cursor size: {}", manager.get_cursor_size().as_str());
println!("Cursor pixels: {}", manager.get_cursor_size().as_pixels());
```

### Cursor Speed Control

```rust
// Set cursor speed
manager.set_cursor_speed(75);
println!("Cursor speed: {}", manager.get_cursor_speed());
```

### Default Cursor

```rust
// Set default cursor
manager.set_default_cursor(DesktopCursorType::Crosshair);
println!("Default cursor: {}", manager.get_default_cursor().as_str());
```

### Cursor Retrieval

```rust
// Get cursor for specific type
if let Some(cursor) = manager.get_cursor(DesktopCursorType::Pointer) {
    println!("Pointer cursor: {}", cursor);
}
```

### Per-Theme Cursor Configuration

```rust
// Add cursor to theme
manager.add_cursor_to_theme(
    &theme_id,
    DesktopCursorType::NotAllowed,
    "/custom/circle.png".to_string(),
);

// Remove cursor from theme
manager.remove_cursor_from_theme(&theme_id, DesktopCursorType::Busy);
```

### Statistics

```rust
let stats = manager.get_statistics();
println!("Total themes: {}", stats.total_themes);
println!("Current theme set: {}", stats.current_theme_set);
println!("Cursor size: {}", stats.cursor_size);
println!("Cursor speed: {}", stats.cursor_speed);
```

## Default Cursor Theme

The manager includes a default cursor theme with the following cursors:

- **Default**: /usr/share/cursors/default/left_ptr
- **Pointer**: /usr/share/cursors/default/hand2
- **Text**: /usr/share/cursors/default/text
- **Move**: /usr/share/cursors/default/fleur
- **Busy**: /usr/share/cursors/default/watch
- **Progress**: /usr/share/cursors/default/left_ptr_watch
- **Crosshair**: /usr/share/cursors/default/crosshair
- **Hand**: /usr/share/cursors/default/hand1
- **Not Allowed**: /usr/share/cursors/default/circle

## Default Configuration

The Cursor Manager includes default configuration:

- **Current Theme**: Default Cursor Theme
- **Cursor Size**: Medium (24 pixels)
- **Cursor Speed**: 50
- **Default Cursor**: Default

## AI Agent Maintenance Instructions

When maintaining the Cursor Manager:

1. **XCursor Integration**: Integrate with XCursor format for cursor themes
2. **Cursor Animation**: Add support for animated cursors
3. **Cursor Inversion**: Add cursor inversion for accessibility
4. **Cursor Preview**: Add cursor preview functionality
5. **Theme Import/Export**: Add theme import/export functionality
6. **Custom Cursor Support**: Add custom cursor upload support
7. **HotSpot Configuration**: Add cursor hotspot configuration
8. **Cursor Shaping**: Add cursor shaping for different displays
9. **Cursor Cache**: Add cursor caching for performance
10. **DPI Awareness**: Add DPI-aware cursor scaling

## Testing

Run the unit tests with:

```bash
cargo test --lib desktop::cursor_manager
```

## Future Enhancements

- XCursor format integration
- Animated cursor support
- Cursor inversion for accessibility
- Cursor preview functionality
- Theme import/export functionality
- Custom cursor upload support
- Cursor hotspot configuration
- Cursor shaping for different displays
- Cursor caching for performance
- DPI-aware cursor scaling
