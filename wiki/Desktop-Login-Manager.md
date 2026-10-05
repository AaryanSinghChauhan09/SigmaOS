# Desktop Login Manager

## Overview

The Desktop Login Manager provides comprehensive login/display management inspired by Linux Mint's MDM (Mint Display Manager) and Omarchy's login system. It supports user session management, desktop environment selection, autologin configuration, and guest session support.

## Features

- **Session Types**: X11, Wayland, TTY
- **User Sessions**: Track and manage user login sessions
- **Session Management**: Add, remove, and activate sessions
- **Desktop Environments**: Multiple desktop environment support
- **DE Management**: Add, remove, and mark desktop environments as installed
- **Session Type Filtering**: Filter desktop environments by session type
- **Autologin**: Configure autologin user and enable/disable autologin
- **Guest Sessions**: Enable/disable guest session support
- **Session Start/Stop**: Start and stop user sessions
- **Per-User Session Tracking**: Track sessions by username
- **Statistics**: Track session counts, DE counts, and configuration status

## Components

### LoginSessionType

```rust
pub enum LoginSessionType {
    X11,     // X11 display server
    Wayland, // Wayland display server
    TTY,     // Text terminal
}
```

### LoginUserSession

User session with:
- Session ID and username
- Display number
- Session type
- Seat identifier
- Active status

### LoginDesktopEnvironment

Desktop environment with:
- DE ID and name
- Command to start session
- Session type
- Installed status

### DesktopLoginManager

Main management interface with:
- Session management (add, remove, retrieve)
- Desktop environment management
- Active session tracking
- Autologin configuration
- Guest session configuration
- Session start/stop
- Per-user session tracking
- Statistics tracking

## Usage

### Basic Usage

```rust
use sigmaos::desktop::DesktopLoginManager;

let mut manager = DesktopLoginManager::new();

// Get configuration
println!("Total DEs: {}", manager.get_desktop_environments().len());
println!("Installed DEs: {}", manager.get_installed_desktop_environments().len());
println!("Guest session enabled: {}", manager.is_guest_session_enabled());
```

### Session Management

```rust
// Add session
let session = LoginUserSession::new(
    "user".to_string(),
    ":0".to_string(),
    LoginSessionType::Wayland,
);

let id = manager.add_session(session);

// Remove session
manager.remove_session(&id);

// Set active session
manager.set_active_session(&id);
```

### Desktop Environment Management

```rust
// Add desktop environment
let de = LoginDesktopEnvironment::new(
    "custom".to_string(),
    "Custom DE".to_string(),
    "/usr/bin/custom-session".to_string(),
    LoginSessionType::Wayland,
);

let id = manager.add_desktop_environment(de);

// Mark as installed
manager.set_desktop_environment_installed(&id, true);

// Remove desktop environment
manager.remove_desktop_environment(&id);
```

### Start/Stop Sessions

```rust
// Start session
let session_id = manager.start_session("user", ":0", "de_0");
if let Some(id) = session_id {
    println!("Session started: {}", id);
}

// Stop session
manager.stop_session(&id);
```

### Autologin Configuration

```rust
// Set autologin user
manager.set_autologin_user(Some("user".to_string()));

// Enable autologin
manager.set_autologin_enabled(true);

// Get autologin status
if manager.is_autologin_enabled() {
    println!("Autologin enabled for: {:?}", manager.get_autologin_user());
}
```

### Guest Session

```rust
// Enable guest session
manager.set_guest_session_enabled(true);

// Disable guest session
manager.set_guest_session_enabled(false);
```

### Session Filtering

```rust
// Get sessions by user
let user_sessions = manager.get_sessions_by_user("user");

// Get desktop environments by type
let wayland_des = manager.get_desktop_environments_by_type(LoginSessionType::Wayland);
let x11_des = manager.get_desktop_environments_by_type(LoginSessionType::X11);
```

### Statistics

```rust
let stats = manager.get_statistics();
println!("Total sessions: {}", stats.total_sessions);
println!("Active sessions: {}", stats.active_sessions);
println!("Total DEs: {}", stats.total_desktop_environments);
println!("Installed DEs: {}", stats.installed_desktop_environments);
println!("Autologin enabled: {}", stats.autologin_enabled);
println!("Guest session enabled: {}", stats.guest_session_enabled);
```

## Default Desktop Environments

The manager includes default desktop environments:

- **Zenith**: SigmaOS native Wayland session (installed by default)
- **GNOME**: GNOME Wayland session
- **KDE Plasma**: KDE Plasma Wayland session
- **XFCE**: XFCE X11 session
- **Cinnamon**: Linux Mint Cinnamon X11 session
- **MATE**: MATE X11 session

## Default Configuration

The Login Manager includes default configuration:

- **Guest Session**: Enabled
- **Autologin**: Disabled
- **Installed DE**: Zenith (SigmaOS native)

## AI Agent Maintenance Instructions

When maintaining the Login Manager:

1. **PAM Integration**: Integrate with PAM for authentication
2. **XDG Seat Integration**: Integrate with XDG seat management
3. **Logind Integration**: Integrate with systemd-logind or equivalent
4. **Display Server**: Integrate with actual X11/Wayland display servers
5. **Session Startup**: Implement actual session startup with environment setup
6. **Session Cleanup**: Implement proper session cleanup on logout
6. **Multi-Seat**: Add multi-seat support
7. **Greeter UI**: Integrate with greeter UI for graphical login
8. **Face Recognition**: Add face recognition authentication
9. **Fingerprint**: Add fingerprint authentication
10. **Session Recovery**: Add session state persistence and recovery

## Testing

Run the unit tests with:

```bash
cargo test --lib desktop::login_manager
```

## Future Enhancements

- PAM authentication integration
- XDG seat management integration
- systemd-logind integration
- Actual X11/Wayland display server integration
- Session startup with environment setup
- Proper session cleanup on logout
- Multi-seat support
- Greeter UI integration
- Face recognition authentication
- Fingerprint authentication
- Session state persistence and recovery
