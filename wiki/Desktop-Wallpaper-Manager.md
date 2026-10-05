# Desktop Wallpaper Manager

## Overview

The Desktop Wallpaper Manager provides comprehensive wallpaper management inspired by Linux Mint's wallpaper settings and Omarchy's wallpaper utilities. It supports multiple wallpaper sources, profiles, display modes, and automatic wallpaper cycling.

## Features

- **Wallpaper Modes**: Stretch, Fit, Fill, Center, Tile, Span
- **Wallpaper Sources**: Multiple wallpaper sources with thumbnails
- **Wallpaper Profiles**: Pre-configured wallpaper profiles with modes
- **Default Source**: Set default wallpaper source
- **Current Wallpaper**: Track and set current wallpaper
- **Profile Management**: Create and manage wallpaper profiles
- **Auto-Change**: Automatic wallpaper cycling with configurable interval
- **Cycle Wallpaper**: Manually cycle through wallpapers
- **Dark Mode**: Profile dark mode support
- **Statistics**: Track source and profile counts

## Components

### WallpaperMode

```rust
pub enum WallpaperMode {
    Stretch,  // Stretch to fill screen
    Fit,      // Fit to screen with letterboxing
    Fill,     // Fill screen (may crop)
    Center,   // Center on screen
    Tile,     // Tile across screen
    Span,     // Span across multiple monitors
}
```

### WallpaperSource

Wallpaper source structure with:
- Source ID and name
- Image path
- Thumbnail path (optional)
- Default flag

### WallpaperProfile

Wallpaper profile structure with:
- Profile ID and name
- Wallpaper source reference
- Display mode
- Dark mode flag

### WallpaperManager

Main management interface with:
- Source registration and management
- Profile creation and management
- Default source configuration
- Current wallpaper and profile tracking
- Auto-change configuration
- Wallpaper cycling
- Statistics tracking

## Usage

### Basic Usage

```rust
use sigmaos::desktop::WallpaperManager;
use std::path::PathBuf;

let mut manager = WallpaperManager::new();

// Get current wallpaper
if let Some(wallpaper) = manager.get_current_wallpaper() {
    println!("Current wallpaper: {}", wallpaper.name);
    println!("Path: {:?}", wallpaper.path);
}
```

### Source Management

```rust
// Add a new wallpaper source
let id = manager.add_source(
    "Nature".to_string(),
    PathBuf::from("/path/to/nature.jpg"),
);

// Get source by ID
if let Some(source) = manager.get_source(&id) {
    println!("Source: {}", source.name);
}

// Set default source
manager.set_default_source(&id);

// Remove source
manager.remove_source(&id);
```

### Profile Management

```rust
// Add a new profile
let source_id = manager.get_sources()[0].id.clone();
let id = manager.add_profile(
    "My Profile".to_string(),
    source_id,
    WallpaperMode::Fit,
);

// Get profile by ID
if let Some(profile) = manager.get_profile(&id) {
    println!("Profile: {}", profile.name);
    println!("Mode: {}", profile.mode.as_str());
}

// Remove profile
manager.remove_profile(&id);
```

### Wallpaper Selection

```rust
// Set wallpaper directly
manager.set_wallpaper(&source_id);

// Set profile (sets wallpaper and mode)
manager.set_profile(&profile_id);

// Get current profile
if let Some(profile) = manager.get_current_profile() {
    println!("Current profile: {}", profile.name);
}
```

### Auto-Change

```rust
// Enable auto-change
manager.set_auto_change_enabled(true);

// Set interval (in minutes)
manager.set_auto_change_interval(30);

// Check status
if manager.is_auto_change_enabled() {
    println!("Auto-change interval: {} minutes", manager.get_auto_change_interval());
}
```

### Wallpaper Cycling

```rust
// Cycle to next wallpaper
manager.cycle_wallpaper();

// This cycles through all available sources
```

### Statistics

```rust
let stats = manager.get_statistics();
println!("Sources: {}", stats.source_count);
println!("Profiles: {}", stats.profile_count);
println!("Auto-change: {}", stats.auto_change_enabled);
println!("Interval: {} minutes", stats.auto_change_interval);
```

## Default Configuration

The Wallpaper Manager includes default sources:

- **Sigma Blue**: Default wallpaper (Sigma Blue)
- **Sigma Dark**: Dark wallpaper
- **Nature**: Nature wallpaper

Default profile:
- **Default**: Uses Sigma Blue with Fill mode

Default settings:
- **Auto-Change**: Disabled
- **Auto-Change Interval**: 30 minutes

## AI Agent Maintenance Instructions

When maintaining the Wallpaper Manager:

1. **Path Validation**: Ensure wallpaper paths exist before setting
2. **Default Protection**: Prevent removal of default source without replacement
3. **Profile Validation**: Ensure profile wallpaper IDs reference valid sources
4. **Auto-Change Logic**: Implement timer-based auto-change in production
5. **Thumbnail Generation**: Generate thumbnails for new sources
6. **Multi-Monitor**: Implement per-monitor wallpaper support
7. **Backend Integration**: Integrate with actual wallpaper backend (GNOME, KDE, Sway)

## Testing

Run the unit tests with:

```bash
cargo test --lib desktop::wallpaper_manager
```

## Future Enhancements

- Integration with actual wallpaper backend (GNOME, KDE, Sway, XFCE)
- Online wallpaper sources
- Wallpaper fetching from URLs
- Color extraction for theme matching
- Per-monitor wallpaper support
- Wallpaper transitions and animations
- Wallpaper scheduling (time-based)
- Wallpaper from photo gallery
- Custom wallpaper directories
- Wallpaper metadata (author, license)
- Wallpaper rating system
