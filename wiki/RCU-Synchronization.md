# RCU (Read-Copy-Update) Synchronization

SigmaOS implements RCU (Read-Copy-Update) synchronization, a scalable read-mostly data structure synchronization mechanism inspired by Linux RCU and BSD UMA.

## Overview

RCU provides:

- **Scalable reads**: Lock-free read-side critical sections
- **Grace periods**: Deferred callback execution
- **Epoch-based synchronization**: State tracking via epochs
- **Callback registration**: Deferred work after grace period
- **Atomic operations**: Lock-free synchronization

## Components

### RcuEpoch

Epoch type for tracking RCU state:

```rust
pub type RcuEpoch = u64;
```

### RcuState

RCU subsystem state:

```rust
pub enum RcuState {
    Idle,           // RCU is idle
    InGracePeriod,  // RCU is in grace period
    Processing,     // RCU is processing callbacks
}
```

### RcuCallbackDescriptor

Callback registration:

```rust
pub struct RcuCallbackDescriptor {
    pub id: u64,
    pub callback: RcuCallback,
    pub user_data: u64,
    pub registered_at: RcuEpoch,
}
```

### RcuSubsystem

Central RCU management:

```rust
pub struct RcuSubsystem {
    pub current_epoch: AtomicU64,
    pub grace_period_start: AtomicU64,
    pub state: AtomicU32,
    pub pending_callbacks: VecDeque<RcuCallbackDescriptor>,
    pub next_callback_id: AtomicU64,
    pub readers_count: AtomicU32,
    pub grace_period_id: AtomicU64,
}
```

**Features:**
- Epoch-based state tracking
- Reader count tracking
- Grace period management
- Callback queue management
- Atomic operations throughout

## Usage

### Read-Side Critical Section

```rust
let rcu = RcuSubsystem::new();

// Enter read-side critical section
let epoch = rcu.read_lock();
// Read data here (no locks)
rcu.read_unlock(epoch);
```

### Registering Callbacks

```rust
let callback: RcuCallback = |_data| {
    println!("Callback executed");
    Ok(())
};

let id = rcu.register_callback(callback, 42);
```

### Synchronizing RCU

```rust
// Begin grace period
let gp_id = rcu.synchronize_rcu();

// Wait for grace period to end
while !rcu.grace_period_ended() {
    // Process callbacks
    rcu.process_callbacks();
}
```

### Advancing Epoch

```rust
rcu.advance_epoch();
```

## Comparison with Linux RCU

| Feature | Linux RCU | SigmaOS RCU |
|---------|-----------|-------------|
| Lock-free reads | Yes | Yes |
| Grace periods | Yes | Yes |
| Epoch-based | Yes | Yes |
| Callbacks | Yes | Yes |
| Atomic ops | Partial | Full |

## Implementation Details

- **Zero External Dependencies**: Uses only std:: and core:: primitives
- **Atomic Operations**: Lock-free synchronization with SeqCst ordering
- **Memory Safety**: Safe Rust with no unsafe code paths
- **Thread Safety**: All operations are thread-safe via atomic counters
- **Scalability**: Reads are completely lock-free

## Testing

Comprehensive test coverage includes:

- Read lock/unlock operations
- Callback registration
- Grace period management
- Grace period detection
- Callback processing
- Epoch advancement

## Future Enhancements

- Per-CPU RCU
- SRCU (Sleepable RCU)
- RCU priority boosting
- RCU torture testing
- RCU deadlock detection
- RCU memory barriers

## References

- Linux RCU (Documentation/RCU/whatisRCU.txt)
- BSD UMA (sys/uma.h)
- RCU papers and documentation
