# Kernel Timers - High-Resolution Timer Management

SigmaOS implements a high-resolution kernel timer subsystem inspired by Linux hrtimers and BSD callouts, providing nanosecond-precision timing for kernel and user-space applications.

## Overview

The kernel timer subsystem provides:

- **High-resolution timers**: Nanosecond-precision timing
- **Multiple timer modes**: One-shot, periodic, and absolute deadlines
- **Atomic operations**: Lock-free timer management
- **Callback support**: User-defined timer callbacks
- **State tracking**: Timer lifecycle management

## Components

### TimerMode

Timer execution modes:

```rust
pub enum TimerMode {
    OneShot,   // Fires once
    Periodic,  // Fires repeatedly
    Absolute,  // Absolute deadline
}
```

### TimerState

Timer lifecycle states:

```rust
pub enum TimerState {
    Idle,      // Timer is idle
    Armed,     // Timer is scheduled
    Executing, // Timer is running callback
    Expired,   // Timer has expired
}
```

### TimerDescriptor

Individual timer instance:

```rust
pub struct TimerDescriptor {
    pub id: TimerId,
    pub mode: TimerMode,
    pub interval_ns: u64,
    pub expires_at: u64,
    pub state: AtomicU32,
    pub fire_count: AtomicU64,
    pub callback: Option<TimerCallback>,
    pub user_data: u64,
}
```

**Features:**
- Atomic state transitions
- Fire count tracking
- User data support
- Callback execution

### KernelTimerSubsystem

Central timer management:

```rust
pub struct KernelTimerSubsystem {
    timers: BTreeMap<TimerId, TimerDescriptor>,
    next_id: AtomicU64,
    current_time_ns: AtomicU64,
}
```

**Features:**
- Timer creation and deletion
- Timer arm/disarm operations
- Time advancement and expiration processing
- Active timer counting

## Usage

### Creating a One-Shot Timer

```rust
let mut subsystem = KernelTimerSubsystem::new();

let callback: TimerCallback = |_id, data| {
    println!("Timer fired with data: {}", data);
    Ok(())
};

let id = subsystem.create_timer(TimerMode::OneShot, 1000, callback, 42);
subsystem.arm_timer(id)?;
```

### Creating a Periodic Timer

```rust
let id = subsystem.create_timer(TimerMode::Periodic, 100, callback, 42);
subsystem.arm_timer(id)?;

// Process timer ticks
subsystem.tick(150);  // Fire once
subsystem.tick(100);  // Fire again (periodic)
```

### Getting Timer Statistics

```rust
let fire_count = subsystem.get_fire_count(id)?;
let active_count = subsystem.active_timer_count();
```

## Comparison with Linux hrtimers

| Feature | Linux hrtimers | SigmaOS Timers |
|---------|---------------|----------------|
| Resolution | Nanosecond | Nanosecond |
| Modes | HRTIMER_MODE_ABS/REL | OneShot/Periodic/Absolute |
| Callbacks | Yes | Yes |
| Lock-free | Partial | Full |
| State tracking | Yes | Yes |

## Implementation Details

- **Zero External Dependencies**: Uses only std:: and core:: primitives
- **Atomic Operations**: Lock-free synchronization with SeqCst ordering
- **Bounded Resources**: No explicit bounds (BTreeMap grows dynamically)
- **Memory Safety**: Safe Rust with no unsafe code paths
- **Thread Safety**: All operations are thread-safe via atomic counters

## Testing

Comprehensive test coverage includes:

- Timer creation and deletion
- Timer arm/disarm operations
- One-shot timer expiration
- Periodic timer repeated firing
- Fire count tracking
- Active timer counting

## Future Enhancements

- Timer slack mode
- Timer coalescing
- Per-CPU timer wheels
- High-resolution clocksources
- Timer drift compensation
- Timer priority inheritance

## References

- Linux hrtimers (kernel/time/hrtimer.c)
- BSD callouts (sys/kern/kern_timeout.c)
- POSIX timers (timer_create, timer_settime)
