# Desktop Screen Saver Manager

## Overview

The Desktop Screen Saver Manager provides comprehensive screen saver management inspired by Linux Mint's screen saver and Omarchy's screen utilities. It supports screen locking, idle detection, and customization with configurable modes and timeout settings.

## Features

- **Screen Saver Modes**: Disabled, Blank, Photos, Clock, Matrix
- **Lock on Sleep**: Never, When Suspended, When Screen Saver
- **Idle Detection**: Track user activity and idle time
- **Screen Locking**: Manual and automatic screen lock
- **Idle Timeout**: Configurable idle timeout before screen saver activation
- **Activity Updates**: Update activity timestamp to prevent activation
- **Default Configuration**: 5-minute idle timeout, blank mode, lock on screen saver
- **Statistics**: Track mode, timeout, lock status, and idle time

## Components

### DesktopScreenSaverMode

```rust
pub enum DesktopScreenSaverMode {
    Disabled,  // Screen saver disabled
    Blank,    // Blank screen
    Photos,   // Photo slideshow
    Clock,    // Clock display
    Matrix,   // Matrix rain effect
}
```

### DesktopLockOnSleep

```rust
pub enum DesktopLockOnSleep {
    Never,             // Never lock on sleep
    WhenSuspended,     // Lock when system suspends
    WhenScreenSaver,   // Lock when screen saver activates
}
```

### DesktopScreenSaverManager

Main management interface with:
- Mode configuration
- Idle timeout setting
- Lock on sleep behavior
- Screen lock/unlock operations
- Activity tracking
- Idle time calculation
- Activation and lock checking
- Statistics tracking

## Usage

### Basic Usage

```rust
use sigmaos::desktop::DesktopScreenSaverManager;

let mut manager = DesktopScreenSaverManager::new();

// Get current mode
let mode = manager.get_mode();
println!("Mode: {}", mode.as_str());

// Get idle timeout
let timeout = manager.get_idle_timeout();
println!("Idle timeout: {} seconds", timeout);
```

### Mode Configuration

```rust
// Set screen saver mode
manager.set_mode(DesktopScreenSaverMode::Photos);
manager.set_mode(DesktopScreenSaverMode::Clock);
manager.set_mode(DesktopScreenSaverMode::Matrix);
manager.set_mode(DesktopScreenSaverMode::Disabled);
```

### Idle Timeout

```rust
// Set idle timeout (in seconds)
manager.set_idle_timeout(600); // 10 minutes
manager.set_idle_timeout(1800); // 30 minutes
manager.set_idle_timeout(3600); // 1 hour
```

### Lock on Sleep Behavior

```rust
// Get lock on sleep setting
let setting = manager.get_lock_on_sleep();
println!("Lock on sleep: {}", setting.as_str());

// Set lock on sleep behavior
manager.set_lock_on_sleep(DesktopLockOnSleep::Never);
manager.set_lock_on_sleep(DesktopLockOnSleep::WhenSuspended);
manager.set_lock_on_sleep(DesktopLockOnSleep::WhenScreenSaver);
```

### Screen Locking

```rust
// Lock screen
manager.lock();
assert!(manager.is_locked());

// Unlock screen
manager.unlock();
assert!(!manager.is_locked());
```

### Activity Tracking

```rust
// Update activity (call this on user input)
manager.update_activity();

// Get idle time
let idle_time = manager.get_idle_time();
println!("Idle time: {} seconds", idle_time);
```

### Activation and Lock Checking

```rust
// Check if screen saver should activate
if manager.should_activate() {
    println!("Screen saver should activate");
}

// Check if screen should lock
if manager.should_lock() {
    println!("Screen should lock");
    manager.lock();
}
```

### Statistics

```rust
let stats = manager.get_statistics();
println!("Mode: {}", stats.mode.as_str());
println!("Idle timeout: {} seconds", stats.idle_timeout);
println!("Lock on sleep: {}", stats.lock_on_sleep.as_str());
println!("Is locked: {}", stats.is_locked);
println!("Idle time: {} seconds", stats.idle_time);
```

## Default Configuration

The Desktop Screen Saver Manager includes default settings:

- **Mode**: Blank
- **Idle Timeout**: 300 seconds (5 minutes)
- **Lock on Sleep**: When Screen Saver
- **Locked**: False

## AI Agent Maintenance Instructions

When maintaining the Desktop Screen Saver Manager:

1. **Timeout Validation**: Ensure idle timeouts are reasonable (minimum 60 seconds)
2. **Mode Validation**: Ensure modes are valid before setting
3. **Activity Updates**: Update activity on all user input events
4. **Lock Security**: Ensure lock is properly authenticated
5. **DPMS Integration**: Integrate with actual DPMS for screen power management
6. **Power Management**: Respect system power state when activating

## Testing

Run the unit tests with:

```bash
cargo test --lib desktop::screen_saver_manager
```

## Future Enhancements

- Integration with actual screen saver (XScreenSaver, xscreensaver)
- Custom photo sources for Photos mode
- Custom clock styles for Clock mode
- Matrix color customization
- Per-user screen saver settings
- Password protection on unlock
- Hot corners for activation
- Power management integration
- External screen saver modules
- Preview screen saver
