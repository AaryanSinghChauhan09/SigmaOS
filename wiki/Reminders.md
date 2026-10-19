# SigmaOS Reminders Components

## Overview
SigmaOS reminders system providing interactive reminder setup flow with desktop notification reminders, competitive with Omarchy reminders.

## Features
- **Time-based reminders** (at specific timestamp)
- **Duration-based reminders** (in N seconds)
- **Recurring reminders** (every N seconds)
- **Priority levels** (Low, Normal, High, Urgent)
- **Status tracking** (Pending, Active, Completed, Dismissed, Overdue)
- **Due reminder detection**
- **Completion and dismissal**

## Structures

| Structure | Description |
|-----------|-------------|
| **ReminderManager** | Reminder manager with create/complete/dismiss |
| **Reminder** | Individual reminder entry |
| **ReminderPriority** | Low, Normal, High, Urgent |
| **ReminderStatus** | Pending, Active, Completed, Dismissed, Overdue |
| **ReminderTrigger** | AtTime, InSeconds, Recurring |

## Usage Examples

```rust
// Create reminder manager
let mut mgr = ReminderManager::new();

// Create a time-based reminder
let now = now_secs();
mgr.create_at_time(
    "Meeting".to_string(),
    Some("Team meeting".to_string()),
    now + 3600,
    ReminderPriority::Normal,
).unwrap();

// Create a duration-based reminder
mgr.create_in_seconds(
    "Break".to_string(),
    Some("Take a break".to_string()),
    300,
    ReminderPriority::Low,
).unwrap();

// Create recurring reminder
mgr.create_recurring(
    "Daily standup".to_string(),
    None,
    86400,
    ReminderPriority::High,
).unwrap();

// Get due reminders
let due = mgr.get_due(now);
```

## Status
- **Implementation**: Complete
- **Dependencies**: None (stdlib only)
- **Roadmap Phase**: Desktop & UX (P1)

## Related Files
- `src/reminders/manager.rs` - Reminder manager implementation
- `wiki/Reminders.md` - Wiki documentation

## Principles Followed
SINGLE_RESPONSIBILITY, COMPONENT_ISOLATION, MODULARITY, LAYERED DESIGN,
KEEP_IT_SIMPLE_STUPID, DISCOVERABILITY, DEFENSIVE_PROGRAMMING, FAIL_FAST_SYSTEM,
SECURE_DEFAULTS, DETERMINISTIC_TESTING, MEMORY_SAFETY, USER_FREEDOM_AND_CONTROL,
COMMAND_QUERY_SEPARATION