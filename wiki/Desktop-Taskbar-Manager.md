# Desktop Taskbar Manager

## Overview

The Desktop Taskbar Manager provides comprehensive taskbar management inspired by Linux Mint's panel/taskbar and Omarchy's taskbar utilities. It supports multiple taskbar positions, item types, pinning, and system tray integration.

## Features

- **Taskbar Positions**: Top, Bottom, Left, Right
- **Item Types**: Launcher, Running, Pinned, System Tray, Status Indicator
- **Item Management**: Add, remove, enable, disable taskbar items
- **Active State**: Track active and minimized windows
- **Window Count**: Track number of windows per item
- **Pinning**: Pin and unpin applications to taskbar
- **Custom Icons**: Set custom icons for each item
- **Auto-Hide**: Configurable auto-hide behavior
- **Size Control**: Taskbar size and icon size
- **System Tray**: Show/hide system tray
- **Status Indicators**: Show/hide status indicators
- **Item Filtering**: List items by type
- **Default Items**: Launcher, Terminal, Files, Web Browser
- **Statistics**: Track total, running, and pinned item counts

## Components

### DesktopTaskbarPosition

```rust
pub enum DesktopTaskbarPosition {
    Top,     // Top of screen
    Bottom,  // Bottom of screen
    Left,    // Left of screen
    Right,   // Right of screen
}
```

### DesktopTaskbarItemType

```rust
pub enum DesktopTaskbarItemType {
    Launcher,          // Application launcher
    Running,           // Running application
    Pinned,            // Pinned application
    SystemTray,        // System tray icon
    StatusIndicator,   // Status indicator
}
```

### DesktopTaskbarItem

Taskbar item structure with:
- Item ID and name
- Item type
- Custom icon (optional)
- Active status
- Minimized status
- Window count

### DesktopTaskbarManager

Main management interface with:
- Item registration and management
- Active and minimized state tracking
- Window count tracking
- Pinning and unpinning
- Custom icon setting
- Position and size configuration
- Auto-hide configuration
- System tray and status indicator visibility
- Item filtering by type
- Statistics tracking

## Usage

### Basic Usage

```rust
use sigmaos::desktop::DesktopTaskbarManager;

let mut manager = DesktopTaskbarManager::new();

// Get all items
let items = manager.get_items();
println!("Total items: {}", items.len());
```

### Item Management

```rust
// Add a new item
let id = manager.add_item("Text Editor".to_string(), DesktopTaskbarItemType::Pinned);

// Get item by ID
if let Some(item) = manager.get_item(&id) {
    println!("Item: {}", item.name);
}

// Remove item
manager.remove_item(&id);
```

### Active and Minimized State

```rust
// Set item as active
manager.set_item_active(&id, true);

// Set item as minimized
manager.set_item_minimized(&id, true);

// Set window count
manager.set_item_window_count(&id, 3);
```

### Pinning

```rust
// Pin item to taskbar
manager.pin_item(&id);

// Unpin item
manager.unpin_item(&id);
```

### Custom Icons

```rust
// Set custom icon
manager.set_item_icon(&id, "text-editor".to_string());
```

### Taskbar Configuration

```rust
// Set taskbar position
manager.set_position(DesktopTaskbarPosition::Top);

// Enable auto-hide
manager.set_auto_hide(true);

// Set taskbar size
manager.set_size(64);

// Set icon size
manager.set_icon_size(48);
```

### System Tray and Status Indicators

```rust
// Show/hide system tray
manager.set_system_tray_visible(false);

// Show/hide status indicators
manager.set_status_indicators_visible(false);
```

### Item Filtering

```rust
// Get running items
let running = manager.get_running_items();
println!("Running: {}", running.len());

// Get pinned items
let pinned = manager.get_pinned_items();
println!("Pinned: {}", pinned.len());

// Get items by type
let launchers = manager.get_items_by_type(DesktopTaskbarItemType::Launcher);
println!("Launchers: {}", launchers.len());
```

### Statistics

```rust
let stats = manager.get_statistics();
println!("Total items: {}", stats.total_items);
println!("Running items: {}", stats.running_items);
println!("Pinned items: {}", stats.pinned_items);
println!("Position: {}", stats.position.as_str());
println!("Auto-hide: {}", stats.auto_hide);
```

## Default Configuration

The Taskbar Manager includes default items:

- **Launcher**: Launcher type, icon "applications-menu"
- **Terminal**: Pinned type, icon "terminal"
- **Files**: Pinned type, icon "folder"
- **Web Browser**: Pinned type, icon "web-browser"

Default settings:
- **Position**: Bottom
- **Auto-Hide**: Disabled
- **Size**: 48 pixels
- **Icon Size**: 32 pixels
- **System Tray**: Visible
- **Status Indicators**: Visible

## AI Agent Maintenance Instructions

When maintaining the Taskbar Manager:

1. **Icon Validation**: Ensure icon names exist in icon theme
2. **Window Tracking**: Integrate with actual window manager for window counts
3. **Active State**: Sync active state with window manager
4. **Minimized State**: Sync minimized state with window manager
5. **Auto-Hide**: Implement actual auto-hide with edge detection
6. **System Tray**: Integrate with actual system tray (StatusNotifier, XEmbed)
7. **Backend Integration**: Integrate with actual panel backend (GNOME, KDE, XFCE)

## Testing

Run the unit tests with:

```bash
cargo test --lib desktop::taskbar_manager
```

## Future Enhancements

- Integration with actual panel backend (GNOME, KDE, XFCE)
- Window drag and drop to taskbar
- Taskbar widgets (clock, weather, system monitor)
- Multiple taskbars support
- Taskbar grouping
- Dynamic icon updates
- Taskbar context menus
- Per-monitor taskbar
- Taskbar transparency and blur
- Custom taskbar themes
