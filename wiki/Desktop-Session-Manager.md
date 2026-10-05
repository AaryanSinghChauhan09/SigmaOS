# Desktop Session Manager

## Overview

The Desktop Session Manager provides comprehensive session management inspired by Linux Mint's session handling and Omarchy's session utilities. It supports X11, Wayland, TTY, and remote sessions with lifecycle management and process tracking.

## Features

- **Session Types**: X11, Wayland, TTY, Remote
- **Session Status**: Active, Inactive, Failed, Terminated
- **Session Management**: Create, remove, terminate sessions
- **Active Session**: Track and switch active session
- **User Sessions**: Track sessions per user
- **Display Management**: Display number assignment
- **TTY Tracking**: TTY device tracking
- **Remote Host**: Remote host tracking for remote sessions
- **Process Tracking**: Track processes within each session
- **Session Filtering**: List sessions by user or type
- **Statistics**: Track session counts by type and status

## Components

### DesktopSessionType

```rust
pub enum DesktopSessionType {
    X11,      // X11 display server session
    Wayland,  // Wayland compositor session
    TTY,      // Text terminal session
    Remote,   // Remote login session
}
```

### DesktopSessionStatus

```rust
pub enum DesktopSessionStatus {
    Active,      // Session is currently active
    Inactive,    // Session is inactive
    Failed,      // Session failed to start
    Terminated,  // Session was terminated
}
```

### DesktopUserSession

User session structure with:
- Session ID and user
- Session type and status
- Display number
- TTY device (optional)
- Remote host (optional)
- Start time
- Process list

### DesktopSessionManager

Main management interface with:
- Session creation and removal
- Active session management
- Session termination
- Process tracking per session
- Session filtering by user and type
- Statistics tracking

## Usage

### Basic Usage

```rust
use sigmaos::desktop::DesktopSessionManager;

let mut manager = DesktopSessionManager::new();

// Create a session
let id = manager.create_session(
    "user1".to_string(),
    DesktopSessionType::Wayland,
    ":0".to_string(),
);

// Get session
if let Some(session) = manager.get_session(&id) {
    println!("Session: {}", session.id);
    println!("User: {}", session.user);
}
```

### Session Management

```rust
// Create sessions
let id1 = manager.create_session(
    "user1".to_string(),
    DesktopSessionType::Wayland,
    ":0".to_string(),
);

let id2 = manager.create_session(
    "user2".to_string(),
    DesktopSessionType::X11,
    ":1".to_string(),
);

// Remove session (must be terminated first)
manager.terminate_session(&id1);
manager.remove_session(&id1);
```

### Active Session

```rust
// Get active session
if let Some(session) = manager.get_active_session() {
    println!("Active session: {}", session.id);
}

// Set active session
manager.set_active_session(&id2);
```

### Session Termination

```rust
// Terminate a session
manager.terminate_session(&id);

// Check status
if let Some(session) = manager.get_session(&id) {
    println!("Status: {}", session.status.as_str());
}
```

### Process Tracking

```rust
// Add process to session
manager.add_process_to_session(&id, "1234".to_string());
manager.add_process_to_session(&id, "5678".to_string());

// Remove process from session
manager.remove_process_from_session(&id, "1234");

// Get process count
if let Some(session) = manager.get_session(&id) {
    println!("Processes: {}", session.processes.len());
}
```

### Session Filtering

```rust
// Get sessions by user
let user_sessions = manager.get_sessions_by_user("user1");
println!("User sessions: {}", user_sessions.len());

// Get sessions by type
let wayland_sessions = manager.get_sessions_by_type(DesktopSessionType::Wayland);
println!("Wayland sessions: {}", wayland_sessions.len());
```

### Statistics

```rust
let stats = manager.get_statistics();
println!("Total sessions: {}", stats.total_sessions);
println!("Active sessions: {}", stats.active_sessions);
println!("X11 sessions: {}", stats.x11_sessions);
println!("Wayland sessions: {}", stats.wayland_sessions);
println!("TTY sessions: {}", stats.tty_sessions);
println!("Remote sessions: {}", stats.remote_sessions);
```

## Default Configuration

The Desktop Session Manager starts with no sessions. Sessions are created on demand.

## AI Agent Maintenance Instructions

When maintaining the Desktop Session Manager:

1. **Session Validation**: Ensure session types and displays are valid before creating
2. **Active Protection**: Prevent removal of active sessions without termination
3. **Process Cleanup**: Remove processes when sessions terminate
4. **Resource Cleanup**: Clean up session resources on termination
5. **Session Limits**: Implement session limits per user if needed
6. **Display Assignment**: Ensure display numbers are unique
7. **Backend Integration**: Integrate with actual session backend (systemd-logind, elogind)

## Testing

Run the unit tests with:

```bash
cargo test --lib desktop::session_manager
```

## Future Enhancements

- Integration with actual session backend (systemd-logind, elogind)
- Session recovery after crash
- Session migration between displays
- Per-session resource limits
- Session audit logging
- Multi-seat support
- Session locking and unlocking
- Session templates
- Session restoration on login
- Session policy enforcement
