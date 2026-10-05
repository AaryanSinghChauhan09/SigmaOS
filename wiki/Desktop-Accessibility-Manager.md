# Desktop Accessibility Manager

## Overview

The Desktop Accessibility Manager provides comprehensive accessibility features inspired by Linux Mint's accessibility tools and Omarchy's accessibility utilities. It supports screen reading, high contrast modes, text scaling, cursor sizing, keyboard accessibility features, and customizable accessibility profiles.

## Features

- **Screen Reader Modes**: Off, On, On with Magnification
- **High Contrast Modes**: Off, On, High Contrast Black, High Contrast White
- **Text Scaling**: Normal, Large, Larger, Extra Large with scale factors
- **Cursor Sizes**: Default, Medium, Large, Extra Large with scale factors
- **Keyboard Repeat**: Off, Slow, Medium, Fast with configurable delay and rate
- **Sticky Keys**: Enable sticky modifier keys for easier typing
- **Slow Keys**: Configure slow key acceptance delay
- **Bounce Keys**: Configure bounce key rejection
- **Accessibility Profiles**: Pre-configured profiles for different impairments
- **Profile Management**: Create, switch, and customize accessibility profiles

## Components

### ScreenReaderMode

```rust
pub enum ScreenReaderMode {
    Off,                  // Screen reader disabled
    On,                   // Screen reader enabled
    OnWithMagnification,  // Screen reader with magnification
}
```

### HighContrastMode

```rust
pub enum HighContrastMode {
    Off,                 // High contrast disabled
    On,                  // High contrast enabled
    HighContrastBlack,   // High contrast black theme
    HighContrastWhite,   // High contrast white theme
}
```

### TextScaling

```rust
pub enum TextScaling {
    Normal,      // 1.0x scale
    Large,       // 1.25x scale
    Larger,      // 1.5x scale
    ExtraLarge,  // 2.0x scale
}
```

### A11yCursorSize

```rust
pub enum A11yCursorSize {
    Default,     // 1.0x scale
    Medium,      // 1.5x scale
    Large,       // 2.0x scale
    ExtraLarge,  // 2.5x scale
}
```

### KeyboardRepeat

```rust
pub enum KeyboardRepeat {
    Off,    // No repeat
    Slow,   // 600ms delay, 100ms rate
    Medium, // 400ms delay, 50ms rate
    Fast,   // 200ms delay, 30ms rate
}
```

### A11yProfile

Accessibility profile with:
- Profile ID and name
- Screen reader mode
- High contrast mode
- Text scaling
- Cursor size
- Keyboard repeat settings
- Sticky keys toggle
- Slow keys toggle
- Bounce keys toggle

### DesktopA11yManager

Main management interface with:
- Profile creation and management
- Profile switching
- Individual accessibility setting updates
- Default profiles for common impairments
- Statistics tracking

## Usage

### Basic Usage

```rust
use sigmaos::desktop::DesktopA11yManager;

let mut manager = DesktopA11yManager::new();

// Get current profile
if let Some(profile) = manager.get_current_profile() {
    println!("Current profile: {}", profile.name);
}
```

### Profile Management

```rust
// Create custom profile
let id = manager.add_profile("My Settings".to_string());

// Switch to profile
manager.set_current_profile(&id);

// Remove custom profile
manager.remove_profile(&id);
```

### Update Accessibility Settings

```rust
// Update screen reader
manager.update_profile_screen_reader(&id, ScreenReaderMode::On);

// Update high contrast
manager.update_profile_high_contrast(&id, HighContrastMode::HighContrastBlack);

// Update text scaling
manager.update_profile_text_scaling(&id, TextScaling::Larger);

// Update sticky keys
manager.update_profile_sticky_keys(&id, true);
```

### Default Profiles

The manager includes default profiles:

- **Default**: Standard accessibility settings
- **Visual Impairment**: Screen reader, high contrast, larger text, larger cursor
- **Motor Impairment**: Sticky keys, slow keys, bounce keys, slow keyboard repeat
- **High Contrast**: High contrast mode with larger text

### Profile Customization

```rust
// Get profile and modify settings
if let Some(profile) = manager.get_profile_mut(&id) {
    profile.set_screen_reader(ScreenReaderMode::OnWithMagnification);
    profile.set_high_contrast(HighContrastMode::HighContrastWhite);
    profile.set_text_scaling(TextScaling::ExtraLarge);
    profile.set_cursor_size(A11yCursorSize::ExtraLarge);
    profile.set_keyboard_repeat(KeyboardRepeat::Slow);
    profile.set_sticky_keys(true);
    profile.set_slow_keys(true);
    profile.set_bounce_keys(true);
}
```

### Statistics

```rust
let stats = manager.get_statistics();
println!("Total profiles: {}", stats.total_profiles);
println!("Current profile set: {}", stats.current_profile);
```

## Scale Factors

### Text Scaling
- Normal: 1.0x
- Large: 1.25x
- Larger: 1.5x
- Extra Large: 2.0x

### Cursor Size
- Default: 1.0x
- Medium: 1.5x
- Large: 2.0x
- Extra Large: 2.5x

## Keyboard Repeat Timing

| Mode | Delay (ms) | Rate (ms) |
|------|------------|-----------|
| Off | 0 | 0 |
| Slow | 600 | 100 |
| Medium | 400 | 50 |
| Fast | 200 | 30 |

## Default Configuration

The Accessibility Manager includes default configuration:

- **Default Profile**: All accessibility features disabled
- **Visual Impairment Profile**: Screen reader enabled, high contrast black, larger text, large cursor
- **Motor Impairment Profile**: Sticky keys, slow keys, bounce keys, slow keyboard repeat
- **High Contrast Profile**: High contrast black, large text

## AI Agent Maintenance Instructions

When maintaining the Accessibility Manager:

1. **Screen Reader Integration**: Integrate with actual screen reader backend (Orca, NVDA, etc.)
2. **High Contrast**: Integrate with compositor for high contrast themes
3. **Text Scaling**: Apply scaling to all UI elements and applications
4. **Cursor Scaling**: Integrate with cursor manager for cursor size changes
5. **Keyboard Accessibility**: Integrate with input device manager for keyboard features
6. **Profile Persistence**: Implement persistent storage for accessibility profiles
7. **Per-Application Settings**: Add per-application accessibility overrides
8. **Magnification**: Implement screen magnification integration
9. **Braille Display**: Add braille display support
10. **Voice Control**: Add voice control integration

## Testing

Run the unit tests with:

```bash
cargo test --lib desktop::a11y_manager
```

## Future Enhancements

- Integration with actual screen reader backends
- Per-application accessibility settings
- Screen magnification integration
- Braille display support
- Voice control integration
- Color blindness filters
- Focus tracking and visual indicators
- Sound themes for accessibility
- Touch screen accessibility
- Gesture-based navigation
