# Desktop Screenshot Manager

## Overview

The Desktop Screenshot Manager provides comprehensive screenshot management inspired by Linux Mint's screenshot tool and Omarchy's screenshot utilities. It supports multiple capture modes, image formats, save directory configuration, and screenshot history.

## Features

- **Capture Modes**: Full Screen, Window, Selection, Screen
- **Image Formats**: PNG, JPEG, BMP, WebP
- **Screenshot Management**: Capture, remove, and track screenshots
- **Default Format**: Configurable default image format
- **Default Mode**: Configurable default capture mode
- **Save Directory**: Configurable save directory
- **Timestamp Tracking**: Track when each screenshot was taken
- **Dimensions**: Track screenshot width and height
- **Screenshot Filtering**: List screenshots by mode or format
- **Convenience Methods**: Quick capture methods for common modes
- **Statistics**: Track total, full screen, window, and selection counts

## Components

### DesktopScreenshotMode

```rust
pub enum DesktopScreenshotMode {
    FullScreen,  // Capture entire screen
    Window,      // Capture active window
    Selection,   // Capture selected region
    Screen,      // Capture specific screen
}
```

### DesktopScreenshotFormat

```rust
pub enum DesktopScreenshotFormat {
    PNG,   // PNG format
    JPEG,  // JPEG format
    BMP,   // BMP format
    WebP,  // WebP format
}
```

### DesktopScreenshot

Screenshot structure with:
- Screenshot ID and filename
- File path
- Capture mode
- Image format
- Timestamp
- Dimensions (width, height)

### DesktopScreenshotManager

Main management interface with:
- Screenshot capture in various modes
- Format and mode configuration
- Save directory configuration
- Screenshot history management
- Screenshot filtering by mode and format
- Statistics tracking

## Usage

### Basic Usage

```rust
use sigmaos::desktop::DesktopScreenshotManager;

let mut manager = DesktopScreenshotManager::new();

// Get default settings
println!("Default format: {}", manager.get_default_format().as_str());
println!("Default mode: {}", manager.get_default_mode().as_str());
println!("Save directory: {:?}", manager.get_save_directory());
```

### Capture Screenshots

```rust
// Capture with specific mode
let id = manager.capture(DesktopScreenshotMode::FullScreen);

// Capture full screen (convenience method)
let id = manager.capture_full_screen();

// Capture window (convenience method)
let id = manager.capture_window();

// Capture selection (convenience method)
let id = manager.capture_selection();
```

### Configuration

```rust
// Set default format
manager.set_default_format(DesktopScreenshotFormat::JPEG);

// Set default mode
manager.set_default_mode(DesktopScreenshotMode::Window);

// Set save directory
manager.set_save_directory(PathBuf::from("/custom/path"));
```

### Screenshot Management

```rust
// Get screenshot by ID
if let Some(screenshot) = manager.get_screenshot(&id) {
    println!("Filename: {}", screenshot.filename);
    println!("Path: {:?}", screenshot.path);
    println!("Mode: {}", screenshot.mode.as_str());
    println!("Format: {}", screenshot.format.as_str());
}

// Remove screenshot
manager.remove_screenshot(&id);

// Get all screenshots
let screenshots = manager.get_screenshots();
println!("Total screenshots: {}", screenshots.len());
```

### Screenshot Filtering

```rust
// Get screenshots by mode
let full_screen = manager.get_screenshots_by_mode(DesktopScreenshotMode::FullScreen);
println!("Full screen: {}", full_screen.len());

let windows = manager.get_screenshots_by_mode(DesktopScreenshotMode::Window);
println!("Windows: {}", windows.len());

// Get screenshots by format
let png = manager.get_screenshots_by_format(DesktopScreenshotFormat::PNG);
println!("PNG: {}", png.len());
```

### Statistics

```rust
let stats = manager.get_statistics();
println!("Total screenshots: {}", stats.total_screenshots);
println!("Full screen: {}", stats.full_screen_count);
println!("Window: {}", stats.window_count);
println!("Selection: {}", stats.selection_count);
```

## Default Configuration

The Screenshot Manager includes default configuration:

- **Default Format**: PNG
- **Default Mode**: Full Screen
- **Save Directory**: `/home/user/Pictures/Screenshots`

## AI Agent Maintenance Instructions

When maintaining the Screenshot Manager:

1. **Directory Validation**: Ensure save directory exists before capturing
2. **Format Validation**: Ensure formats are supported by image backend
3. **Timestamp Handling**: Ensure timestamps are unique for filename generation
4. **Capture Integration**: Integrate with actual screen capture backend
5. **Window Detection**: Integrate with window manager for window capture
6. **Selection UI**: Implement selection UI for selection mode
7. **Backend Integration**: Integrate with actual screenshot backend (gnome-screenshot, etc.)

## Testing

Run the unit tests with:

```bash
cargo test --lib desktop::screenshot_manager
```

## Future Enhancements

- Integration with actual screen capture backend
- Delayed capture with countdown
- Multiple monitor support
- Screenshot editing
- Screenshot sharing
- OCR text recognition
- Screenshot annotations
- Automatic upload to cloud
- Screenshot history with thumbnails
- Keyboard shortcuts
- Screenshot preview
