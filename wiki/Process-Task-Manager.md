# Process Task Manager

## Overview

The Process Task Manager provides comprehensive task management inspired by Linux Mint's task manager and Omarchy's task utilities. It supports process listing, filtering, search, and termination.

## Features

- **Process States**: Running, Sleeping, Stopped, Zombie
- **Process Entry**: PID, name, command, state, CPU usage, memory usage, user, threads
- **Process Management**: Add, get, list processes
- **State Filtering**: List processes by state
- **User Filtering**: List processes by user
- **Search**: Search processes by name and command
- **Termination**: Terminate and kill processes
- **Statistics**: Track process counts, CPU, and memory usage
- **Default Processes**: Pre-configured init, kthreadd, systemd, sigma-shell, sigma-wm

## Components

### TaskProcessState

```rust
pub enum TaskProcessState {
    Running,   // Process is running
    Sleeping,  // Process is sleeping
    Stopped,   // Process is stopped
    Zombie,    // Process is zombie
}
```

### TaskProcessEntry

Represents a process with:
- Process ID (PID)
- Process name
- Command line
- State
- CPU usage percentage
- Memory usage (MB)
- User
- Thread count

### ProcessTaskManager

Main management interface with:
- Process addition and listing
- State-based filtering
- User-based filtering
- Search functionality
- Process termination
- Statistics tracking

## Usage

### Basic Usage

```rust
use sigmaos::system::ProcessTaskManager;

let mut manager = ProcessTaskManager::new();

// List all processes
let processes = manager.list_processes();
for proc in processes {
    println!("{}: {} ({})", proc.pid, proc.name, proc.state.as_str());
}
```

### Process Management

```rust
// Add a process
let proc = TaskProcessEntry::new(
    999,
    "myapp".to_string(),
    "/usr/bin/myapp".to_string(),
);
manager.add_process(proc);

// Get a process
if let Some(proc) = manager.get_process(999) {
    println!("Process: {}", proc.name);
}

// Terminate a process
manager.terminate(999)?;
```

### Filtering

```rust
// List by state
let running = manager.list_by_state(TaskProcessState::Running);
let sleeping = manager.list_by_state(TaskProcessState::Sleeping);

// List by user
let user = manager.list_by_user("user");
let root = manager.list_by_user("root");
```

### Search

```rust
// Search processes
let results = manager.search("sigma");
for proc in results {
    println!("{}: {}", proc.name, proc.command);
}
```

### Process Metadata

```rust
let mut proc = TaskProcessEntry::new(
    1,
    "test".to_string(),
    "/bin/test".to_string(),
);

// Set state
proc.set_state(TaskProcessState::Running);

// Set CPU usage
proc.set_cpu(15.5);

// Set memory usage
proc.set_memory(100.0);
```

### Statistics

```rust
let stats = manager.get_statistics();
println!("Total processes: {}", stats.total_processes);
println!("Running: {}", stats.running_count);
println!("Sleeping: {}", stats.sleeping_count);
println!("Zombie: {}", stats.zombie_count);
println!("Total CPU: {}%", stats.total_cpu);
println!("Total Memory: {} MB", stats.total_memory);
```

## Default Processes

The Process Task Manager includes pre-configured processes:

- **PID 1**: init (/sbin/init)
- **PID 2**: kthreadd ([kthreadd])
- **PID 100**: systemd (/usr/lib/systemd/systemd)
- **PID 200**: sigma-shell (/usr/bin/sigma-shell)
- **PID 300**: sigma-wm (/usr/bin/sigma-wm)

## AI Agent Maintenance Instructions

When maintaining the Process Task Manager:

1. **PID Validation**: Ensure PIDs are unique and positive
2. **Init Protection**: Prevent termination of init process (PID 1)
3. **State Consistency**: Ensure process states are accurate
4. **CPU Validation**: Ensure CPU percentages are between 0-100
5. **Memory Validation**: Ensure memory values are reasonable
6. **User Validation**: Validate user names before filtering

## Testing

Run the unit tests with:

```bash
cargo test --lib system::task_manager
```

## Future Enhancements

- Integration with actual process monitoring (procfs)
- Real-time process updates
- Process tree display
- Process priority management
- Process affinity control
- I/O statistics per process
- Network connections per process
- Open files per process
- Process resource limits
- Process search by executable path
- Process grouping by application
