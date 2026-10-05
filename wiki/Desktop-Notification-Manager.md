# Desktop Notification Manager

## Overview

The Desktop Notification Manager provides a comprehensive notification system inspired by Linux Mint's notifications and Omarchy's notification utilities. It supports system and application notifications with urgency levels, Do Not Disturb mode, and notification lifecycle management.

## Features

- **Notification Urgency**: Low, Normal, Critical
- **Notification Types**: Application-specific and system notifications
- **Do Not Disturb**: Suppress non-critical notifications
- **Notification Lifecycle**: Send, dismiss, remove operations
- **Expiration**: Automatic expiration of notifications
- **Filtering**: By urgency, by application
- **Statistics**: Track notification counts and status

## Components

### DesktopNotificationUrgency

```rust
pub enum DesktopNotificationUrgency {
    Low,       // Low urgency notification
    Normal,    // Normal urgency notification
    Critical,  // Critical notification (bypasses DND)
}
```

### DesktopNotification

Represents a notification with:
- Unique ID
- Application name
- Title and body
- Urgency level
- Icon (optional)
- Creation timestamp
- Expiration timestamp (optional)
- Dismissed status

### DesktopNotificationManager

Main management interface with:
- Notification sending
- Do Not Disturb mode
- Notification retrieval
- Urgency-based filtering
- Application-based filtering
- Dismiss/remove operations
- Expired/dismissed cleanup
- Statistics tracking

## Usage

### Basic Usage

```rust
use sigmaos::desktop::DesktopNotificationManager;

let mut manager = DesktopNotificationManager::new();

// Send a notification
let id = manager.send(
    "TestApp".to_string(),
    "Test Title".to_string(),
    "Test Body".to_string(),
    DesktopNotificationUrgency::Normal,
);

// Get a notification
if let Some(notif) = manager.get(&id) {
    println!("Notification: {}", notif.title);
}
```

### Sending Notifications

```rust
// Normal notification
manager.send(
    "System".to_string(),
    "Update Available".to_string(),
    "A new update is available".to_string(),
    DesktopNotificationUrgency::Normal,
);

// Critical notification
manager.send(
    "System".to_string(),
    "Battery Critical".to_string(),
    "Battery level is critically low".to_string(),
    DesktopNotificationUrgency::Critical,
);
```

### Do Not Disturb Mode

```rust
// Enable DND
manager.set_dnd(true);

// Normal notifications won't be stored in DND mode
let id = manager.send(
    "App".to_string(),
    "Info".to_string(),
    "Body".to_string(),
    DesktopNotificationUrgency::Normal,
);
assert!(manager.get(&id).is_none());

// Critical notifications still work in DND mode
let id = manager.send(
    "System".to_string(),
    "Critical".to_string(),
    "Body".to_string(),
    DesktopNotificationUrgency::Critical,
);
assert!(manager.get(&id).is_some());
```

### Managing Notifications

```rust
// Dismiss a notification
manager.dismiss(&id)?;

// Dismiss all notifications
manager.dismiss_all();

// Remove a notification
manager.remove(&id)?;
```

### Filtering Notifications

```rust
// List all notifications
let all = manager.list_all();

// List active notifications
let active = manager.list_active();

// List by urgency
let critical = manager.list_by_urgency(DesktopNotificationUrgency::Critical);

// List by application
let system = manager.list_by_app("System");
```

### Cleanup Operations

```rust
// Clear expired notifications
let expired_count = manager.clear_expired();

// Clear dismissed notifications
let dismissed_count = manager.clear_dismissed();
```

### Statistics

```rust
let stats = manager.get_statistics();
println!("Total notifications: {}", stats.total_notifications);
println!("Active: {}", stats.active_count);
println("Dismissed: {}", stats.dismissed_count);
println("Critical: {}", stats.critical_count);
println("DND enabled: {}", stats.dnd_enabled);
```

## AI Agent Maintenance Instructions

When maintaining the Desktop Notification Manager:

1. **DND Mode**: Ensure critical notifications bypass DND mode
2. **Expiration**: Properly handle notification expiration
3. **Cleanup**: Regularly clean up expired and dismissed notifications
4. **ID Uniqueness**: Ensure notification IDs are unique
5. **Application Validation**: Validate application names before sending
6. **Urgency Levels**: Maintain proper urgency level handling

## Testing

Run the unit tests with:

```bash
cargo test --lib desktop::notification_manager
```

## Future Enhancements

- Integration with actual notification daemon (libnotify, mako)
- Notification actions and buttons
- Notification grouping
- Sound and vibration support
- Notification history
- Per-application notification settings
- Notification persistence across reboots
