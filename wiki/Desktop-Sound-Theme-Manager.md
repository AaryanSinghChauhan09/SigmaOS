# Desktop Sound Theme Manager

## Overview

The Desktop Sound Theme Manager provides comprehensive sound theme management inspired by Linux Mint's sound settings and Omarchy's sound utilities. It supports sound event configuration, theme management, volume control, and event-based sound playback.

## Features

- **Sound Event Types**: Boot, Shutdown, Login, Logout, Desktop Login, Desktop Logout, Notification, Message, Error, Warning, Success, Battery Low, Battery Full, Volume Change, File Copy Complete, File Move Complete, Device Connect, Device Disconnect
- **Sound Themes**: Multiple sound themes with author information
- **Event-Based Sounds**: Assign sounds to specific events
- **Theme Management**: Add, remove, and manage sound themes
- **Current Theme**: Track and switch current sound theme
- **Volume Control**: Global volume control (0-100)
- **Sound Playback**: Play sounds for specific events
- **Per-Theme Configuration**: Add and remove sounds from themes
- **Statistics**: Track theme count, current theme status, and volume

## Components

### SoundEventType

```rust
pub enum SoundEventType {
    Boot,                 // System boot sound
    Shutdown,             // System shutdown sound
    Login,                // User login sound
    Logout,               // User logout sound
    DesktopLogin,         // Desktop session login
    DesktopLogout,        // Desktop session logout
    Notification,         // Notification sound
    Message,              // Message sound
    Error,                // Error sound
    Warning,              // Warning sound
    Success,              // Success sound
    BatteryLow,           // Battery low warning
    BatteryFull,          // Battery full notification
    VolumeChange,         // Volume change sound
    FileCopyComplete,     // File copy complete
    FileMoveComplete,     // File move complete
    DeviceConnect,        // Device connected sound
    DeviceDisconnect,     // Device disconnected sound
}
```

### DesktopSoundTheme

Sound theme with:
- Theme ID and name
- Author
- Sound mappings (event → file path)

### DesktopSoundThemeManager

Main management interface with:
- Theme management (add, remove, retrieve)
- Current theme tracking
- Volume control
- Sound playback for events
- Per-theme sound configuration
- Statistics tracking

## Usage

### Basic Usage

```rust
use sigmaos::desktop::DesktopSoundThemeManager;

let mut manager = DesktopSoundThemeManager::new();

// Get configuration
println!("Total themes: {}", manager.get_themes().len());
println!("Current theme: {:?}", manager.get_current_theme());
println!("Volume: {}", manager.get_volume());
```

### Theme Management

```rust
// Add theme
let theme = DesktopSoundTheme::new(
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

### Sound Playback

```rust
// Play sound for event
if let Some(sound) = manager.play_sound(SoundEventType::Notification) {
    println!("Playing: {}", sound);
}
```

### Volume Control

```rust
// Set volume
manager.set_volume(75);
println!("Volume: {}", manager.get_volume());
```

### Per-Theme Sound Configuration

```rust
// Add sound to theme
manager.add_sound_to_theme(
    &theme_id,
    SoundEventType::Message,
    "/custom/message.ogg".to_string(),
);

// Remove sound from theme
manager.remove_sound_from_theme(&theme_id, SoundEventType::Notification);
```

### Statistics

```rust
let stats = manager.get_statistics();
println!("Total themes: {}", stats.total_themes);
println!("Current theme set: {}", stats.current_theme_set);
println!("Volume: {}", stats.volume);
```

## Default Sound Theme

The manager includes a default sound theme with the following sounds:

- **Boot**: /usr/share/sounds/sigmaos/boot.ogg
- **Shutdown**: /usr/share/sounds/sigmaos/shutdown.ogg
- **Notification**: /usr/share/sounds/sigmaos/notification.ogg
- **Message**: /usr/share/sounds/sigmaos/message.ogg
- **Error**: /usr/share/sounds/sigmaos/error.ogg
- **Warning**: /usr/share/sounds/sigmaos/warning.ogg
- **Success**: /usr/share/sounds/sigmaos/success.ogg
- **Battery Low**: /usr/share/sounds/sigmaos/battery-low.ogg
- **Device Connect**: /usr/share/sounds/sigmaos/device-connect.ogg
- **Device Disconnect**: /usr/share/sounds/sigmaos/device-disconnect.ogg

## Default Configuration

The Sound Theme Manager includes default configuration:

- **Current Theme**: Default Sound Theme
- **Volume**: 100
- **Default Theme**: SigmaOS default sound theme

## AI Agent Maintenance Instructions

When maintaining the Sound Theme Manager:

1. **Sound System Integration**: Integrate with PulseAudio or PipeWire
2. **Sound Format Support**: Add support for OGG, WAV, FLAC, MP3
3. **Theme Import/Export**: Add theme import/export functionality
4. **Sound Preview**: Add sound preview functionality
5. **Event Subscription**: Allow applications to subscribe to sound events
6. **Per-Application Volume**: Add per-application volume control
7. **Sound Packs**: Add sound pack installation
8. **Custom Sound Recording**: Add custom sound recording
9. **Sound Fade**: Add sound fade in/out effects
10. **Mute Control**: Add mute control and mute on specific events

## Testing

Run the unit tests with:

```bash
cargo test --lib desktop::sound_theme_manager
```

## Future Enhancements

- PulseAudio or PipeWire integration
- Sound format support (OGG, WAV, FLAC, MP3)
- Theme import/export functionality
- Sound preview functionality
- Application event subscription
- Per-application volume control
- Sound pack installation
- Custom sound recording
- Sound fade in/out effects
- Mute control and event-based muting
