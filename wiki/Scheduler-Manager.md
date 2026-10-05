# Scheduler Manager

## Overview

The Scheduler Manager provides comprehensive task scheduling inspired by Linux Mint's scheduler and Omarchy's scheduler utilities. It supports task scheduling, cron jobs, and automation with configurable frequencies.

## Features

- **Schedule Frequencies**: Once, Hourly, Daily, Weekly, Monthly
- **Schedule Status**: Pending, Running, Completed, Failed, Disabled
- **Scheduled Task**: Name, command, frequency, status, next run, last run
- **Task Management**: Add, create, enable, disable, remove tasks
- **Frequency Filtering**: List tasks by frequency
- **Status Filtering**: List tasks by status
- **Task Execution**: Run scheduled tasks on demand
- **Statistics**: Track task counts by status
- **Default Tasks**: Pre-configured backup, update, cleanup, log rotation

## Components

### ScheduleFrequency

```rust
pub enum ScheduleFrequency {
    Once,     // Run once
    Hourly,   // Run every hour
    Daily,    // Run every day
    Weekly,   // Run every week
    Monthly,  // Run every month
}
```

### ScheduleStatus

```rust
pub enum ScheduleStatus {
    Pending,   // Task is pending
    Running,   // Task is running
    Completed, // Task completed successfully
    Failed,    // Task failed
    Disabled,  // Task is disabled
}
```

### ScheduledTask

Represents a scheduled task with:
- Unique task ID
- Task name
- Command to execute
- Frequency
- Status
- Next run timestamp
- Last run timestamp
- Enabled flag

### SchedulerManager

Main management interface with:
- Task addition and creation
- Enable/disable operations
- Task execution
- Frequency-based filtering
- Status-based filtering
- Statistics tracking

## Usage

### Basic Usage

```rust
use sigmaos::system::SchedulerManager;

let mut manager = SchedulerManager::new();

// List all tasks
let tasks = manager.list_tasks();
for task in tasks {
    println!("{}: {} ({})", task.name, task.command, task.frequency.as_str());
}
```

### Task Management

```rust
// Add a task
let task = ScheduledTask::new(
    "mytask".to_string(),
    "My Task".to_string(),
    "/usr/bin/mytask".to_string(),
    ScheduleFrequency::Daily,
);
manager.add_task(task);

// Create a task
let id = manager.create_task(
    "New Task".to_string(),
    "/usr/bin/newtask".to_string(),
    ScheduleFrequency::Hourly,
);

// Get a task
if let Some(task) = manager.get_task(&id) {
    println!("Task: {}", task.name);
}

// Remove a task
manager.remove_task(&id)?;
```

### Enable/Disable

```rust
// Enable a task
manager.enable_task("backup")?;

// Disable a task
manager.disable_task("backup")?;
```

### Task Execution

```rust
// Run a task
manager.run_task("backup")?;

// Check status
if let Some(task) = manager.get_task("backup") {
    println!("Status: {}", task.status.as_str());
    if let Some(last_run) = task.last_run {
        println!("Last run: {}", last_run);
    }
}
```

### Filtering

```rust
// List by frequency
let daily = manager.list_by_frequency(ScheduleFrequency::Daily);
let weekly = manager.list_by_frequency(ScheduleFrequency::Weekly);

// List by status
let pending = manager.list_by_status(ScheduleStatus::Pending);
let completed = manager.list_by_status(ScheduleStatus::Completed);
let failed = manager.list_by_status(ScheduleStatus::Failed);
```

### Statistics

```rust
let stats = manager.get_statistics();
println!("Total tasks: {}", stats.total_tasks);
println!("Enabled: {}", stats.enabled_count);
println!("Pending: {}", stats.pending_count);
println!("Failed: {}", stats.failed_count);
```

## Default Tasks

The Scheduler Manager includes pre-configured tasks:

- **backup**: System Backup (Daily, /usr/bin/sigma-backup)
- **update**: System Update (Weekly, /usr/bin/sigma-update)
- **cleanup**: System Cleanup (Weekly, /usr/bin/sigma-cleanup)
- **logrotate**: Log Rotation (Daily, /usr/sbin/logrotate)

## AI Agent Maintenance Instructions

When maintaining the Scheduler Manager:

1. **Command Validation**: Validate commands before adding tasks
2. **Frequency Validation**: Ensure frequencies are valid
3. **Time Calculation**: Accurately calculate next run times
4. **Task Idempotency**: Prevent duplicate task execution
5. **Error Handling**: Capture and report task execution errors
6. **Task Limits**: Implement reasonable task execution limits

## Testing

Run the unit tests with:

```bash
cargo test --lib system::scheduler_manager
```

## Future Enhancements

- Integration with actual cron system
- Time zone support
- Complex cron expressions (crontab format)
- Task dependencies
- Task chains
- Task notifications
- Task history and logs
- Task resource limits
- Parallel task execution
- Task retry policies
- Task priority levels
