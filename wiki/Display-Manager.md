# Display Manager

## Overview

The Display Manager provides display and login session management inspired by Linux Mint's MDM and Omarchy's login system. It supports user authentication, session selection, display server management, and auto-login configuration.

## Features

- **Session Types**: X11, Wayland, TTY sessions
- **Desktop Sessions**: Configurable desktop session definitions
- **User Sessions**: Active user session tracking
- **Auto-Login**: Automatic login for specified users
- **Session Switching**: Switch between active sessions
- **Display Management**: Automatic display number assignment
- **Guest Login**: Optional guest session support
- **Default Session**: Configurable default session selection

## Components

### SessionType

```rust
pub enum SessionType {
    X11,      // X11/Xorg display server
    Wayland,  // Wayland display server
    Tty,      // Text-only terminal
}
```

### DesktopSession

Represents a desktop session with:
- Session name
- Command to execute
- Session type (X11/Wayland/TTY)
- Default session flag

### UserSession

Represents an active user session with:
- Username
- Session name
- Display number (e.g., :0, :1)
- Start time
- Active status

### DisplayConfig

Display manager configuration including:
- Auto-login enabled/disabled
- Auto-login username
- Default session
- Guest login allowed
- Remember last session
- Hide users from login screen

### DisplayManager

Main management interface with:
- Desktop session registration
- Default session management
- Auto-login configuration
- User session lifecycle
- Session switching
- Display number management
- Statistics tracking

## Usage

### Basic Usage

```rust
use sigmaos::desktop::DisplayManager;

let mut manager = DisplayManager::new();

// List all available sessions
let sessions = manager.list_sessions();

// Get default session
let default = manager.get_default_session();
```

### Session Management

```rust
// Add a custom session
let session = DesktopSession::new(
    "Custom Session".to_string(),
    "/usr/bin/custom-session".to_string(),
    SessionType::Wayland,
);
manager.add_session(session);

// Set default session
manager.set_default_session("Custom Session");
```

### Auto-Login Configuration

```rust
// Enable auto-login for a user
manager.enable_auto_login("username".to_string());

// Disable auto-login
manager.disable_auto_login();
```

### Starting Sessions

```rust
// Start a user session
let display = manager.start_session("username".to_string(), "SigmaOS".to_string())?;
println!("Session started on display {}", display);
```

### Session Management

```rust
// Get active sessions
let active = manager.get_active_sessions();

// Get session for specific user
let user_session = manager.get_user_session("username");

// End a session
manager.end_session(":0")?;

// Switch to another session
manager.switch_session(":1")?;
```

### Display Number Management

```rust
// Get next available display number
let next_display = manager.get_next_display();
```

### Statistics

```rust
let stats = manager.get_statistics();
println!("Total sessions: {}", stats.total_sessions);
println!("Active sessions: {}", stats.active_sessions);
println!("Default session: {}", stats.default_session);
```

## Default Sessions

The Display Manager includes these default sessions:

1. **SigmaOS** (Wayland) - Default Wayland session
2. **SigmaOS (X11)** - X11 compatibility session
3. **TTY** - Text-only terminal session

## AI Agent Maintenance Instructions

When maintaining the Display Manager:

1. **Session Safety**: Ensure session commands are validated before execution
2. **Display Uniqueness**: Maintain unique display number assignment
3. **Session Cleanup**: Ensure proper cleanup when sessions end
4. **Auto-Login Security**: Keep auto-login disabled by default for security
5. **Session Switching**: Maintain proper session activation/deactivation
6. **Configuration Persistence**: Ensure display configuration is properly saved

## Testing

Run the unit tests with:

```bash
cargo test --lib desktop::display_manager
```

## Future Enhancements

- Integration with actual display managers (GDM, LightDM, SDDM)
- User authentication integration
- Session recovery after crash
- Multi-monitor support
- Remote display support
- Session templates
- Guest session sandboxing
