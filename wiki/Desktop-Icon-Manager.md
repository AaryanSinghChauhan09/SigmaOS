# Desktop Icon Manager

## Overview

The Desktop Icon Manager provides comprehensive desktop icon management inspired by Linux Mint's desktop icons and Omarchy's icon utilities. It supports multiple icon types, position management, grid alignment, and auto-arrangement.

## Features

- **Icon Types**: Application, File, Folder, Link, Drive, Trash
- **Icon Management**: Add, remove, enable, disable desktop icons
- **Position Control**: Set icon position (x, y coordinates)
- **Size Control**: Per-icon size and default icon size
- **Visibility**: Show/hide icons
- **Locking**: Lock icons to prevent modification/deletion
- **Icon Path**: Custom icon path for each icon
- **Grid Alignment**: None, Left to Right, Top to Bottom
- **Auto-Arrange**: Automatic icon arrangement in grid
- **Icon Filtering**: List icons by type
- **Default Icons**: Home, Trash, File System
- **Statistics**: Track total, visible, and locked icon counts

## Components

### DesktopIconType

```rust
pub enum DesktopIconType {
    Application,  // Application launcher
    File,         // File icon
    Folder,       // Folder icon
    Link,         // Symbolic link
    Drive,        // Drive/mount point
    Trash,        // Trash can
}
```

### GridAlignment

```rust
pub enum GridAlignment {
    None,          // No alignment
    LeftToRight,   // Left to right grid
    TopToBottom,   // Top to bottom grid
}
```

### DesktopDesktopIcon

Desktop icon structure with:
- Icon ID and name
- Icon type
- Target path
- Custom icon path (optional)
- Position (x, y)
- Size
- Visibility status
- Lock status

### DesktopIconManager

Main management interface with:
- Icon registration and management
- Position and size control
- Visibility and locking
- Grid alignment configuration
- Auto-arrangement
- Icon filtering by type
- Statistics tracking

## Usage

### Basic Usage

```rust
use sigmaos::desktop::DesktopIconManager;
use std::path::PathBuf;

let mut manager = DesktopIconManager::new();

// Get all icons
let icons = manager.get_icons();
println!("Total icons: {}", icons.len());
```

### Icon Management

```rust
// Add a new icon
let id = manager.add_icon(
    "Documents".to_string(),
    DesktopIconType::Folder,
    PathBuf::from("/home/user/Documents"),
);

// Get icon by ID
if let Some(icon) = manager.get_icon(&id) {
    println!("Icon: {}", icon.name);
    println!("Path: {:?}", icon.path);
}

// Remove icon
manager.remove_icon(&id);
```

### Position and Size

```rust
// Set icon position
manager.set_icon_position(&id, 100, 200);

// Set individual icon size
manager.set_icon_size(&id, 64);

// Set default icon size for auto-arrange
manager.set_default_icon_size(64);
```

### Visibility and Locking

```rust
// Set icon visibility
manager.set_icon_visible(&id, false);

// Lock icon (prevents deletion)
manager.set_icon_locked(&id, true);

// Check if locked
if let Some(icon) = manager.get_icon(&id) {
    println!("Locked: {}", icon.is_locked);
}
```

### Custom Icon Path

```rust
// Set custom icon path
manager.set_icon_path(&id, PathBuf::from("/path/to/custom/icon.png"));
```

### Grid Alignment

```rust
// Set grid alignment
manager.set_grid_alignment(GridAlignment::LeftToRight);

// Set spacing between icons
manager.set_spacing(15);

// Auto-arrange icons
manager.auto_arrange();
```

### Icon Filtering

```rust
// Get icons by type
let folders = manager.get_icons_by_type(DesktopIconType::Folder);
println!("Folders: {}", folders.len());

let drives = manager.get_icons_by_type(DesktopIconType::Drive);
println!("Drives: {}", drives.len());

// Get only visible icons
let visible = manager.get_visible_icons();
println!("Visible: {}", visible.len());
```

### Statistics

```rust
let stats = manager.get_statistics();
println!("Total icons: {}", stats.total_icons);
println!("Visible icons: {}", stats.visible_icons);
println!("Locked icons: {}", stats.locked_icons);
println!("Grid alignment: {}", stats.grid_alignment.as_str());
```

## Default Configuration

The Icon Manager includes default icons:

- **Home**: Folder type, position (10, 10)
- **Trash**: Trash type, position (10, 70), locked
- **File System**: Drive type, position (10, 130)

Default settings:
- **Default Icon Size**: 48 pixels
- **Spacing**: 10 pixels
- **Grid Alignment**: None

## AI Agent Maintenance Instructions

When maintaining the Icon Manager:

1. **Path Validation**: Ensure icon paths exist before setting
2. **Lock Protection**: Prevent removal of locked icons
3. **Position Validation**: Ensure positions are within screen bounds
4. **Auto-Arrange**: Implement screen boundary detection in auto-arrange
5. **Icon Theme**: Integrate with icon theme system
6. **Drag and Drop**: Implement drag and drop in production
7. **Backend Integration**: Integrate with actual desktop backend (GNOME, KDE, XFCE)

## Testing

Run the unit tests with:

```bash
cargo test --lib desktop::icon_manager
```

## Future Enhancements

- Integration with actual desktop backend (GNOME, KDE, XFCE)
- Drag and drop support
- Icon themes and icon scaling
- Desktop widgets support
- Icon groups/folders
- Desktop background interaction
- Multi-monitor support
- Icon animations
- Custom icon rendering
- Desktop file system integration
