# Desktop Notification Sound Manager

## Overview

The Desktop Notification Sound Manager provides comprehensive notification sound management inspired by Linux Mint's notification sounds and Omarchy's sound feedback utilities. It supports sound configuration for different notification events, volume control, and sound type selection.

## Features

- **Sound Types**: Default, Minimal, None
- **Sound Events**: Notification, Message, Email, Error, Warning, Success, Information, Custom
- **Sound Configuration**: Per-event sound file configuration
- **Volume Control**: Per-sound volume and master volume
- **Enable/Disable**: Enable or disable individual sounds
- **Sound Type Selection**: Choose between Default, Minimal, or no sounds
- **Default Sounds**: Pre-configured sounds for common events
- **Statistics**: Track total and enabled sound counts

## Components

### NotificationSoundType

```rust
pub enum NotificationSoundType {
    Default,  // Full notification sounds
    Minimal,  // Minimal notification sounds
    None,     // No sounds
}
```

### SoundEvent

```rust
pub enum SoundEvent {
    Notification,  // General notification
    Message,       // Message received
    Email,         // Email received
    Error,         // Error occurred
    Warning,       // Warning
    Success,       // Success
    Information,   // Information
    Custom,        // Custom event
}
```

### SoundConfig

Sound configuration with:
- Sound ID and event type
- Sound file path
- Volume (0-100)
- Enable/disable toggle

### NotificationSoundManager

Main management interface with:
- Sound configuration for different events
- Sound type selection (Default, Minimal, None)
- Master volume control
- Per-sound volume control
- Enable/disable individual sounds
- Sound playback lookup
- Default sounds for common events
- Statistics tracking

## Usage

### Basic Usage

```rust
use sigmaos::desktop::NotificationSoundManager;

let mut manager = NotificationSoundManager::new();

// Get configuration
println!("Sound type: {}", manager.get_sound_type().as_str());
println!("Master volume: {}", manager.get_master_volume());
```

### Sound Type Selection

```rust
// Set sound type
manager.set_sound_type(NotificationSoundType::Default);
manager.set_sound_type(NotificationSoundType::Minimal);
manager.set_sound_type(NotificationSoundType::None);
```

### Volume Control

```rust
// Set master volume
manager.set_master_volume(90);

// Set individual sound volume
manager.set_sound_volume(&sound_id, 75);
```

### Sound Management

```rust
// Add custom sound
let id = manager.add_sound(
    SoundEvent::Custom,
    "/path/to/sound.wav".to_string(),
);

// Remove sound
manager.remove_sound(&id);

// Enable/disable sound
manager.set_sound_enabled(&id, false);
```

### Sound Playback

```rust
// Play sound for event
if let Some(sound_file) = manager.play_sound(SoundEvent::Notification) {
    println!("Playing: {}", sound_file);
}
```

### Sound Filtering

```rust
// Get all sounds
let sounds = manager.get_sounds();

// Get sounds by event
let notifications = manager.get_sounds_by_event(SoundEvent::Notification);

// Get enabled sounds
let enabled = manager.get_enabled_sounds();
```

### Statistics

```rust
let stats = manager.get_statistics();
println!("Total sounds: {}", stats.total_sounds);
println!("Enabled sounds: {}", stats.enabled_sounds);
println!("Sound type: {}", stats.sound_type.as_str());
```

## Default Sounds

The manager includes default sounds for common events:

- **Notification**: General notification sound
- **Message**: Message received sound
- **Email**: Email received sound
- **Error**: Error occurred sound
- **Success**: Success sound

## Default Configuration

The Notification Sound Manager includes default configuration:

- **Sound Type**: Default
- **Master Volume**: 80
- **Default Sound Volume**: 80
- **All Sounds Enabled**: true

## Sound Type Behavior

- **Default**: Full notification sounds for all enabled events
- **Minimal**: Minimal notification sounds (shorter, quieter)
- **None**: No sounds for any events

## AI Agent Maintenance Instructions

When maintaining the Notification Sound Manager:

1. **Sound File Validation**: Ensure sound files exist before playback
2. **Audio Backend Integration**: Integrate with actual audio backend (PulseAudio, PipeWire, etc.)
3. **Sound File Formats**: Support multiple audio formats (WAV, OGG, MP3, FLAC)
4. **Custom Sound Themes**: Implement sound theme switching
5. **Per-Application Sounds**: Add per-application sound configuration
6. **Sound Duration**: Configure sound duration limits
7. **Sound Priority**: Implement sound priority for concurrent events
8. **Sound Caching**: Cache frequently used sounds
9. **Fallback Sounds**: Implement fallback sounds when files are missing
10. **Sound Preview**: Add sound preview functionality

## Testing

Run the unit tests with:

```bash
cargo test --lib desktop::notification_sound_manager
```

## Future Enhancements

- Integration with actual audio backend
- Custom sound themes
- Per-application sound configuration
- Sound duration limits
- Sound priority for concurrent events
- Sound caching
- Fallback sounds
- Sound preview functionality
- Sound recording for custom sounds
- Sound library management
