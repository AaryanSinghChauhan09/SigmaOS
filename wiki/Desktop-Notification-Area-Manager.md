# Desktop Notification Area Manager

## Overview

The Desktop Notification Area Manager provides comprehensive system tray/notification area management inspired by Linux Mint's notification area and Omarchy's system tray utilities. It supports multiple item types, icons, tooltips, menus, and visibility control.

## Features

- **Item Types**: Application, System, Network, Volume, Battery, Bluetooth, Input Method, Custom
- **Item Management**: Add, remove, enable, disable notification area items
- **Icons**: Custom icon support for each item
- **Tooltips**: Custom tooltip text for each item
- **Menus**: Menu support for interactive items
- **Visibility**: Show/hide items
- **Auto-Hide**: Configurable auto-hide behavior
- **Icon Display**: Show/hide icons
- **Label Display**: Show/hide labels
- **Item Filtering**: List items by type
- **Default Items**: Network, Volume, Battery, Bluetooth
- **Statistics**: Track total, visible, and menu-enabled item counts

## Components

### TrayNotificationItemType

```rust
pub enum TrayNotificationItemType {
    Application,   // Application icon
    System,        // System icon
    Network,       // Network status
    Volume,        // Volume control
    Battery,       // Battery status
    Bluetooth,     // Bluetooth status
    InputMethod,   // Input method
    Custom,        // Custom item
}
```

### TrayNotificationItem

Notification item structure with:
- Item ID and name
- Item type
- Custom icon (optional)
- Tooltip (optional)
- Visibility status
- Menu support flag

### NotificationAreaManager

Main management interface with:
- Item registration and management
- Icon and tooltip management
- Menu support configuration
- Visibility control
- Auto-hide configuration
- Icon and label display control
- Item filtering by type
- Statistics tracking

## Usage

### Basic Usage

```rust
use sigmaos::desktop::NotificationAreaManager;

let mut manager = NotificationAreaManager::new();

// Get all items
let items = manager.get_items();
println!("Total items: {}", items.len());
```

### Item Management

```rust
// Add a new item
let id = manager.add_item("Clock".to_string(), TrayNotificationItemType::System);

// Get item by ID
if let Some(item) = manager.get_item(&id) {
    println!("Item: {}", item.name);
}

// Remove item
manager.remove_item(&id);
```

### Icon and Tooltip

```rust
// Set item icon
manager.set_item_icon(&id, "clock".to_string());

// Set item tooltip
manager.set_item_tooltip(&id, "Current time".to_string());
```

### Visibility and Menu

```rust
// Set item visibility
manager.set_item_visible(&id, false);

// Set menu support
manager.set_item_has_menu(&id, true);
```

### Display Configuration

```rust
// Enable auto-hide
manager.set_auto_hide(true);

// Show/hide icons
manager.set_show_icons(false);

// Show/hide labels
manager.set_show_labels(true);
```

### Item Filtering

```rust
// Get items by type
let network = manager.get_items_by_type(TrayNotificationItemType::Network);
println!("Network items: {}", network.len());

// Get only visible items
let visible = manager.get_visible_items();
println!("Visible: {}", visible.len());
```

### Statistics

```rust
let stats = manager.get_statistics();
println!("Total items: {}", stats.total_items);
println!("Visible items: {}", stats.visible_items);
println!("Items with menu: {}", stats.items_with_menu);
```

## Default Configuration

The Notification Area Manager includes default items:

- **Network**: Network type, icon "network-wired", tooltip "Network status", menu enabled
- **Volume**: Volume type, icon "audio-volume-high", tooltip "Volume control", menu enabled
- **Battery**: Battery type, icon "battery-full", tooltip "Battery status", menu enabled
- **Bluetooth**: Bluetooth type, icon "bluetooth-active", tooltip "Bluetooth status", menu enabled

Default settings:
- **Auto-Hide**: Disabled
- **Show Icons**: Enabled
- **Show Labels**: Disabled

## AI Agent Maintenance Instructions

When maintaining the Notification Area Manager:

1. **Icon Validation**: Ensure icon names exist in icon theme
2. **Tooltip Validation**: Ensure tooltips are descriptive
3. **Menu Integration**: Integrate with actual menu system for menu-enabled items
4. **Auto-Hide**: Implement actual auto-hide with edge detection
5. **Status Updates**: Integrate with actual system status updates
6. **Backend Integration**: Integrate with actual system tray (StatusNotifier, XEmbed)
7. **Item Ordering**: Support custom item ordering

## Testing

Run the unit tests with:

```bash
cargo test --lib desktop::notification_area_manager
```

## Future Enhancements

- Integration with actual system tray (StatusNotifier, XEmbed)
- Dynamic status updates (network, battery, volume)
- Item drag and drop reordering
- Item grouping
- Custom item development API
- Per-user item configuration
- Tray icon animations
- Notification badge support
- Multi-monitor support
