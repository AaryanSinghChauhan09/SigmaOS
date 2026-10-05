# Desktop Screen Lock Manager

## Overview

The Desktop Screen Lock Manager provides comprehensive screen lock management inspired by Linux Mint's screen lock and Omarchy's lock screen utilities. It supports multiple lock types, auto-lock configuration, lock time tracking, and failed attempt management.

## Features

- **Lock Types**: Password, PIN, Pattern, Biometric
- **Lock Status**: Unlocked, Locked, Unlocking
- **Lock Configuration**: Auto-lock timeout, lock on suspend, lock on lid close
- **Display Options**: Show clock, show notifications on lock screen
- **Lock Management**: Lock and unlock screen
- **Lock Time Tracking**: Track when screen was locked and lock duration
- **Failed Attempts**: Track failed unlock attempts with max attempt limit
- **Auto-Lock**: Determine if screen should auto-lock based on idle time
- **Default Configuration**: Password type, 5-minute auto-lock, lock on suspend/lid close

## Components

### LockScreenType

```rust
pub enum LockScreenType {
    Password,   // Password-based lock
    PIN,        // PIN-based lock
    Pattern,    // Pattern-based lock
    Biometric,  // Biometric lock
}
```

### LockStatus

```rust
pub enum LockStatus {
    Unlocked,   // Screen is unlocked
    Locked,     // Screen is locked
    Unlocking,  // Screen is being unlocked
}
```

### ScreenLockConfig

Screen lock configuration with:
- Lock type
- Auto-lock enabled flag
- Auto-lock timeout (seconds)
- Lock on suspend flag
- Lock on lid close flag
- Show clock flag
- Show notifications flag

### ScreenLockManager

Main management interface with:
- Lock and unlock operations
- Configuration management
- Lock time and duration tracking
- Failed attempt tracking
- Max attempt limit
- Auto-lock determination
- Statistics tracking

## Usage

### Basic Usage

```rust
use sigmaos::desktop::ScreenLockManager;

let mut manager = ScreenLockManager::new();

// Get status
println!("Status: {}", manager.get_status().as_str());

// Get configuration
let config = manager.get_config();
println!("Auto-lock: {}", config.auto_lock_enabled);
```

### Lock and Unlock

```rust
// Lock screen
manager.lock();
assert!(manager.is_locked());

// Unlock screen
if manager.unlock("password") {
    println!("Unlocked successfully");
}
```

### Configuration

```rust
// Set custom configuration
let config = ScreenLockConfig::new(LockScreenType::PIN);
config.set_auto_lock_timeout(600); // 10 minutes
manager.set_config(config);
```

### Lock Time

```rust
// Get lock time
if let Some(lock_time) = manager.get_lock_time() {
    println!("Locked at: {}", lock_time);
}

// Get lock duration
if let Some(duration) = manager.get_lock_duration() {
    println!("Locked for: {} seconds", duration);
}
```

### Failed Attempts

```rust
// Get failed attempts
println!("Failed attempts: {}", manager.get_failed_attempts());

// Reset failed attempts
manager.reset_failed_attempts();

// Check if max attempts reached
if manager.is_max_attempts_reached() {
    println!("Max attempts reached");
}
```

### Auto-Lock

```rust
// Check if should auto-lock based on idle time
if manager.should_auto_lock(300) {
    println!("Should auto-lock");
}
```

### Statistics

```rust
let stats = manager.get_statistics();
println!("Status: {}", stats.status.as_str());
println!("Lock type: {}", stats.lock_type.as_str());
println!("Auto-lock: {}", stats.auto_lock_enabled);
println!("Auto-lock timeout: {} seconds", stats.auto_lock_timeout);
println!("Failed attempts: {}", stats.failed_attempts);
```

## Default Configuration

The Screen Lock Manager includes default configuration:

- **Lock Type**: Password
- **Auto-Lock**: Enabled
- **Auto-Lock Timeout**: 300 seconds (5 minutes)
- **Lock on Suspend**: Enabled
- **Lock on Lid Close**: Enabled
- **Show Clock**: Enabled
- **Show Notifications**: Disabled
- **Max Attempts**: 5

## AI Agent Maintenance Instructions

When maintaining the Screen Lock Manager:

1. **Credential Validation**: Implement actual credential validation in production
2. **Auto-Lock Integration**: Integrate with idle detection system
3. **Suspend Detection**: Integrate with power management for lock on suspend
4. **Lid Detection**: Integrate with hardware for lock on lid close
5. **Max Attempts**: Implement appropriate handling when max attempts reached
6. **Biometric Support**: Integrate with actual biometric hardware
7. **Backend Integration**: Integrate with actual lock screen backend

## Testing

Run the unit tests with:

```bash
cargo test --lib desktop::screen_lock_manager
```

## Future Enhancements

- Integration with actual credential system (PAM, etc.)
- Real biometric support (fingerprint, face recognition)
- Pattern lock UI
- Custom lock screen themes
- Lock screen widgets
- Emergency unlock methods
- Per-user lock configuration
- Lock screen notifications
- Remote unlock support
