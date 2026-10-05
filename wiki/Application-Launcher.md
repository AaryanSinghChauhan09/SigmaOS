# Application Launcher

## Overview

The Application Launcher provides comprehensive application launching functionality inspired by Linux Mint's menu and Omarchy's launcher utilities. It supports application search, favorites, categorization, and recent applications tracking.

## Features

- **Application Categories**: All, System, Development, Multimedia, Network, Graphics, Office, Games, Education, Utility
- **Application Management**: Add, launch applications
- **Favorites**: Add/remove favorite applications
- **Recent Applications**: Track recently launched applications (max 10)
- **Search**: Search applications by name and description
- **Category Filtering**: List applications by category
- **Application Metadata**: Name, executable, icon, description
- **Default Applications**: Pre-configured terminal, file manager, text editor, web browser, media player, settings, calculator
- **Statistics**: Track application, favorite, and recent counts

## Components

### LauncherCategory

```rust
pub enum LauncherCategory {
    All,          // All applications
    System,       // System applications
    Development,  // Development tools
    Multimedia,   // Multimedia applications
    Network,      // Network applications
    Graphics,     // Graphics applications
    Office,       // Office applications
    Games,        // Games
    Education,    // Education applications
    Utility,      // Utility applications
}
```

### LauncherApp

Represents a launcher application with:
- Unique app ID
- Application name
- Executable command
- Icon
- Category
- Description
- Favorite flag

### ApplicationLauncher

Main management interface with:
- Application addition and listing
- Category-based filtering
- Search functionality
- Launch operations
- Favorite management
- Recent applications tracking
- Statistics tracking

## Usage

### Basic Usage

```rust
use sigmaos::desktop::ApplicationLauncher;

let mut launcher = ApplicationLauncher::new();

// List all applications
let apps = launcher.list_apps();
for app in apps {
    println!("{}: {}", app.name, app.description);
}
```

### Application Management

```rust
// Add an application
let app = LauncherApp::new(
    "myapp".to_string(),
    "My App".to_string(),
    "myapp".to_string(),
    "myapp-icon".to_string(),
    LauncherCategory::Utility,
    "My application".to_string(),
);
launcher.add_app(app);

// Get an application
if let Some(app) = launcher.get_app("myapp") {
    println!("App: {}", app.name);
}

// Launch an application
launcher.launch("terminal")?;
```

### Category Filtering

```rust
// List all applications
let all = launcher.list_by_category(LauncherCategory::All);

// List system applications
let system = launcher.list_by_category(LauncherCategory::System);

// List development applications
let dev = launcher.list_by_category(LauncherCategory::Development);
```

### Search

```rust
// Search applications
let results = launcher.search("terminal");
for app in results {
    println!("{}: {}", app.name, app.description);
}
```

### Favorites

```rust
// Add to favorites
launcher.add_favorite("terminal")?;

// Remove from favorites
launcher.remove_favorite("terminal")?;

// List favorites
let favorites = launcher.list_favorites();
```

### Recent Applications

```rust
// Launch some applications
launcher.launch("terminal")?;
launcher.launch("file-manager")?;

// List recent applications
let recent = launcher.list_recent();
for app in recent {
    println!("Recently used: {}", app.name);
}
```

### Statistics

```rust
let stats = launcher.get_statistics();
println!("Total apps: {}", stats.total_apps);
println!("Favorites: {}", stats.favorite_count);
println!("Recent: {}", stats.recent_count);
```

## Default Applications

The Application Launcher includes pre-configured applications:

- **terminal**: Terminal emulator
- **file-manager**: File manager (Files)
- **text-editor**: Text editor
- **web-browser**: Web browser
- **media-player**: Media player
- **settings**: System settings
- **calculator**: Calculator

## AI Agent Maintenance Instructions

When maintaining the Application Launcher:

1. **Executable Validation**: Validate executable commands before adding
2. **Icon Validation**: Ensure icon paths are valid
3. **Category Consistency**: Ensure applications are properly categorized
4. **Recent Limit**: Maintain max 10 recent applications
5. **Duplicate Prevention**: Prevent duplicate application IDs
6. **Launch Validation**: Validate applications can be launched

## Testing

Run the unit tests with:

```bash
cargo test --lib desktop::application_launcher
```

## Future Enhancements

- Integration with actual application desktop files (.desktop)
- Application sorting options
- Application ratings
- Application suggestions
- Keyboard shortcuts
- Drag and drop support
- Custom categories
- Application pinning to dock/taskbar
- Application search by command
- Application thumbnails
- Application updates notification
