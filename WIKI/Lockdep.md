# Lockdep (Lock Dependency Detection)

SigmaOS implements Lockdep (Lock Dependency Detection), a deadlock detection and lock order validation mechanism inspired by Linux lockdep.

## Overview

Lockdep provides:

- **Deadlock detection**: Circular dependency detection in lock acquisition
- **Lock order validation**: Enforce consistent lock ordering rules
- **Lock class management**: Track different lock types
- **Lock depth tracking**: Monitor nested lock acquisitions
- **Atomic operations**: Lock-free synchronization

## Components

### LockClassId

Lock class identifier:

```rust
pub type LockClassId = u64;
```

### LockInstanceId

Lock instance identifier:

```rust
pub type LockInstanceId = u64;
```

### LockClass

Lock class with dependency tracking:

```rust
pub struct LockClass {
    pub id: LockClassId,
    pub name: String,
    pub dependencies: BTreeSet<LockClassId>,
}
```

**Features:**
- Unique ID for each lock class
- Human-readable name
- Dependency graph tracking

### LockdepError

Lockdep error types:

```rust
pub enum LockdepError {
    InvalidLockClass,   // Lock class not registered
    CircularDependency,  // Circular lock dependency detected
    AlreadyHeld,        // Lock already held by current context
    NotHeld,            // Lock not held by current context
}
```

### LockdepSubsystem

Central lockdep management:

```rust
pub struct LockdepSubsystem {
    lock_classes: BTreeMap<LockClassId, LockClass>,
    next_class_id: AtomicU64,
    held_locks: BTreeMap<LockInstanceId, LockClassId>,
    next_instance_id: AtomicU64,
    lock_depth: AtomicU32,
}
```

**Features:**
- Lock class registry
- Instance tracking
- Lock depth monitoring
- Atomic operations

## Usage

### Registering Lock Classes

```rust
let mut lockdep = LockdepSubsystem::new();

let mutex_id = lockdep.register_lock_class("mutex".to_string());
let rwlock_id = lockdep.register_lock_class("rwlock".to_string());
```

### Acquiring Locks

```rust
let instance_id = lockdep.acquire_lock(mutex_id).unwrap();
// Lock acquired
```

### Releasing Locks

```rust
lockdep.release_lock(instance_id).unwrap();
// Lock released
```

### Adding Lock Dependencies

```rust
// Enforce ordering: mutex must be acquired before rwlock
lockdep.add_dependency(mutex_id, rwlock_id).unwrap();
```

### Circular Dependency Detection

```rust
lockdep.add_dependency(mutex_id, rwlock_id).unwrap();
lockdep.add_dependency(rwlock_id, mutex_id).unwrap(); // Circular

lockdep.acquire_lock(mutex_id).unwrap();
let result = lockdep.acquire_lock(rwlock_id); // Fails with CircularDependency
```

## Implementation Details

- **Zero External Dependencies**: Uses only std:: and core:: primitives
- **Atomic Operations**: Lock-free synchronization with SeqCst ordering
- **Memory Safety**: Safe Rust with no unsafe code paths
- **Thread Safety**: All operations are thread-safe via atomic counters
- **Graph Algorithm**: DFS-based circular dependency detection

## Comparison with Linux Lockdep

| Feature | Linux Lockdep | SigmaOS Lockdep |
|---------|---------------|-----------------|
| Deadlock detection | Yes | Yes |
| Lock order validation | Yes | Yes |
| Lock classes | Yes | Yes |
| Circular detection | Yes | Yes |
| Graph analysis | Full | Basic |
| Stack trace | Yes | No |

## Testing

Comprehensive test coverage includes:

- Lock class registration
- Lock acquire/release operations
- Lock dependency addition
- Circular dependency detection
- Lock depth tracking

## Future Enhancements

- Stack trace capture on violation
- Per-CPU lockdep statistics
- Lockdep torture testing
- Lockdep deadlock recovery
- Lockdep performance profiling
- Lockdep visualization

## References

- Linux Lockdep (Documentation/locking/lockdep-design.txt)
- FreeBSD witness (sys/proc.h)
- Deadlock detection algorithms
