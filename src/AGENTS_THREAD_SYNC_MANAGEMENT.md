# Thread Synchronization & Lock-Free Atomic Rules for AI Agents

## Core Directives

### 1. Explicit Memory Orderings
- Always use `Ordering::Release` when publishing shared state updates
- Always use `Ordering::Acquire` when reading published state
- **NEVER** use `Ordering::Relaxed` for synchronized pointer states

### 2. Futex Fast-Path Operations
- Userspace lock primitives MUST check atomic lock variables before invoking kernel futex wait syscall
- Implement exponential backoff before kernel intervention

### 3. Spinlock Backoff
- Spinlock loops MUST execute `core::hint::spin_loop()` CPU pause hints to reduce interconnect bus contention
- Implement jitter in spin loops to prevent thundering herd

## Implementation Guidelines

### Lock-Free Data Structures
```rust
use std::sync::atomic::{AtomicPtr, Ordering};

// Producer-consumer pattern with acquire/release semantics
pub struct LockFreeQueue<T> {
    head: AtomicPtr<Node<T>>,
    tail: AtomicPtr<Node<T>>,
}

impl<T> LockFreeQueue<T> {
    pub fn push(&self, value: T) {
        // Use Release ordering to publish the new node
        // before updating the tail pointer
    }
    
    pub fn pop(&self) -> Option<T> {
        // Use Acquire ordering to synchronized with the producer
    }
}
```

### RAII Lock Guards
- Use lock guards that automatically release on drop
- Implement deadlock prevention through lock ordering

### Design Patterns (OOPS)
- **Composition**: Compose thread-safe primitives into higher-level structures
- **Observer**: Notify waiting threads of state changes
- **Strategy**: Swap synchronization strategies at runtime
