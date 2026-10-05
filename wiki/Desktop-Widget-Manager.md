# Desktop Widget Manager

## Overview

The Desktop Widget Manager provides comprehensive widget management inspired by Linux Mint's desklets and Omarchy's widget utilities. It supports multiple widget types, positioning, sizing, configuration, and visibility control.

## Features

- **Widget Types**: Clock, Calendar, Weather, System Monitor, Network Monitor, Disk Monitor, CPU Usage, Memory Usage, Battery, Notes, Custom
- **Widget Positions**: Top Left, Top Center, Top Right, Middle Left, Middle Center, Middle Right, Bottom Left, Bottom Center, Bottom Right, Custom
- **Widget Management**: Add, remove, enable, disable desktop widgets
- **Position Control**: Set widget position (preset or custom coordinates)
- **Size Control**: Set widget size (width, height)
- **Visibility**: Show/hide widgets
- **Locking**: Lock widgets to prevent modification/deletion
- **Configuration**: Per-widget key-value configuration
- **Widget Filtering**: List widgets by type
- **Default Widgets**: Clock, System Monitor
- **Statistics**: Track total, visible, and locked widget counts

## Components

### DesktopWidgetType

```rust
pub enum DesktopWidgetType {
    Clock,           // Clock widget
    Calendar,        // Calendar widget
    Weather,         // Weather widget
    SystemMonitor,   // System monitor widget
    NetworkMonitor,  // Network monitor widget
    DiskMonitor,     // Disk monitor widget
    CPUUsage,        // CPU usage widget
    MemoryUsage,     // Memory usage widget
    Battery,         // Battery widget
    Notes,           // Notes widget
    Custom,          // Custom widget
}
```

### DesktopWidgetPosition

```rust
pub enum DesktopWidgetPosition {
    TopLeft,         // Top left corner
    TopCenter,       // Top center
    TopRight,        // Top right corner
    MiddleLeft,      // Middle left
    MiddleCenter,    // Middle center
    MiddleRight,     // Middle right
    BottomLeft,      // Bottom left corner
    BottomCenter,    // Bottom center
    BottomRight,     // Bottom right corner
    Custom(i32, i32), // Custom coordinates
}
```

### DesktopWidget

Desktop widget structure with:
- Widget ID and name
- Widget type
- Position
- Size (width, height)
- Visibility status
- Lock status
- Configuration (key-value pairs)

### DesktopWidgetManager

Main management interface with:
- Widget registration and management
- Position and size control
- Visibility and locking
- Configuration management
- Widget filtering by type
- Statistics tracking

## Usage

### Basic Usage

```rust
use sigmaos::desktop::DesktopWidgetManager;

let mut manager = DesktopWidgetManager::new();

// Get all widgets
let widgets = manager.get_widgets();
println!("Total widgets: {}", widgets.len());
```

### Widget Management

```rust
// Add a new widget
let id = manager.add_widget("Weather".to_string(), DesktopWidgetType::Weather);

// Get widget by ID
if let Some(widget) = manager.get_widget(&id) {
    println!("Widget: {}", widget.name);
}

// Remove widget
manager.remove_widget(&id);
```

### Position and Size

```rust
// Set widget position
manager.set_widget_position(&id, DesktopWidgetPosition::TopLeft);

// Set custom position
manager.set_widget_position(&id, DesktopWidgetPosition::Custom(100, 200));

// Set widget size
manager.set_widget_size(&id, 300, 200);
```

### Visibility and Locking

```rust
// Set widget visibility
manager.set_widget_visible(&id, false);

// Lock widget (prevents deletion)
manager.set_widget_locked(&id, true);

// Check if locked
if let Some(widget) = manager.get_widget(&id) {
    println!("Locked: {}", widget.is_locked);
}
```

### Configuration

```rust
// Set widget configuration
manager.set_widget_config(&id, "location".to_string(), "London".to_string());
manager.set_widget_config(&id, "units".to_string(), "metric".to_string());

// Get widget configuration
if let Some(location) = manager.get_widget_config(&id, "location") {
    println!("Location: {}", location);
}
```

### Widget Filtering

```rust
// Get widgets by type
let clocks = manager.get_widgets_by_type(DesktopWidgetType::Clock);
println!("Clocks: {}", clocks.len());

// Get only visible widgets
let visible = manager.get_visible_widgets();
println!("Visible: {}", visible.len());
```

### Statistics

```rust
let stats = manager.get_statistics();
println!("Total widgets: {}", stats.total_widgets);
println!("Visible widgets: {}", stats.visible_widgets);
println!("Locked widgets: {}", stats.locked_widgets);
```

## Default Configuration

The Widget Manager includes default widgets:

- **Clock**: Clock type, Top Right position, 200x100 size, format "%H:%M"
- **System Monitor**: System Monitor type, Bottom Right position, 250x150 size

## AI Agent Maintenance Instructions

When maintaining the Widget Manager:

1. **Position Validation**: Ensure custom positions are within screen bounds
2. **Size Validation**: Ensure widget sizes are reasonable
3. **Lock Protection**: Prevent removal of locked widgets
4. **Config Validation**: Validate configuration values per widget type
5. **Widget Refresh**: Implement periodic refresh for dynamic widgets
6. **Widget Rendering**: Integrate with actual widget rendering system
7. **Backend Integration**: Integrate with actual widget backend (Conky, Ksysguard, etc.)

## Testing

Run the unit tests with:

```bash
cargo test --lib desktop::widget_manager
```

## Future Enhancements

- Integration with actual widget rendering system
- Widget marketplace/store
- Widget templates
- Widget themes and styling
- Drag and drop widget positioning
- Widget grouping
- Widget animations
- Custom widget development API
- Widget sandboxing
- Per-monitor widget placement
- Widget synchronization across devices
