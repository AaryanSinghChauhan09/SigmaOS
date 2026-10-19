# SigmaOS Notification System Components

## Overview
SigmaOS notification system providing popups, DND, deduping, and replayable history, competitive with Omarchy's notification daemon.

## Features
- **Notification popups** with urgency levels
- **Do Not Disturb mode** (blocks non-critical notifications)
- **Deduplication** (prevents duplicate notifications within 5 seconds)
- **Replayable history** (replay last N notifications)
- **Urgent bypass** (critical notifications bypass DND)
- **Dismiss support** (dismiss individual notifications)
- **Clear all** functionality

## Urgency Levels

| Level | Description | DND Bypass |
|-------|-------------|------------|
| **Low** | Low priority | No |
| **Normal** | Normal priority | No |
| **High** | High priority | No |
| **Critical** | Critical priority | Yes |

## Structures

| Structure | Description |
|-----------|-------------|
| **NotificationManager** | Notification manager with history |
| **Notification** | Individual notification |
| **NotificationHistory** | Replayable notification history |
| **Urgency** | Low, Normal, High, Critical |

## Usage Examples

```rust
// Create notification manager
let mut mgr = NotificationManager::new();

// Send a notification
mgr.send("system".to_string(), "Update ready".to_string(), "New packages available".to_string(), Urgency::Normal).unwrap();

// Toggle DND
mgr.toggle_dnd();

// Replay last 10 notifications
let recent = mgr.replay(10);

// List active notifications
let active = mgr.list_active();
```

## Status
- **Implementation**: Complete
- **Dependencies**: None (stdlib only)
- **Roadmap Phase**: Desktop & UX (P1)

## Related Files
- `src/notification/manager.rs` - Notification manager implementation
- `wiki/Notification-System.md` - Wiki documentation

## Principles Followed
SINGLE_RESPONSIBILITY, COMPONENT_ISOLATION, MODULARITY, LAYERED DESIGN,
KEEP_IT_SIMPLE_STUPID, DISCOVERABILITY, DEFENSIVE_PROGRAMMING, FAIL_FAST_SYSTEM,
SECURE_DEFAULTS, RACE_FREE DESIGN, SAFE_CONCURRENCY, DETERMINISTIC TESTING,
ZERO_COPY, MEMORY_SAFETY, MEMORY_DISCIPLINE