# Workqueue - Asynchronous Work Execution

SigmaOS implements a workqueue subsystem inspired by Linux workqueues and BSD taskqueues, providing asynchronous work execution with priority-based scheduling.

## Overview

The workqueue subsystem provides:

- **Asynchronous work execution**: Deferrable work items
- **Priority-based scheduling**: Work prioritization
- **Worker thread pool**: Concurrent work processing
- **State tracking**: Work lifecycle management
- **Callback support**: User-defined work handlers

## Components

### WorkPriority

Work priority levels:

```rust
pub enum WorkPriority {
    Low,       // Low priority work
    Normal,    // Normal priority work
    High,      // High priority work
    Critical,  // Critical priority work
}
```

### WorkState

Work lifecycle states:

```rust
pub enum WorkState {
    Pending,   // Work is queued
    Running,   // Work is executing
    Completed, // Work completed successfully
    Failed,    // Work failed
}
```

### WorkItem

Individual work item:

```rust
pub struct WorkItem {
    pub id: WorkId,
    pub priority: WorkPriority,
    pub state: AtomicU32,
    pub callback: Option<WorkCallback>,
    pub user_data: u64,
    pub queued_at: u64,
    pub started_at: AtomicU64,
    pub completed_at: AtomicU64,
}
```

**Features:**
- Atomic state transitions
- Timestamp tracking
- User data support
- Callback execution

### Worker

Worker thread:

```rust
pub struct Worker {
    pub id: u32,
    pub is_busy: AtomicU32,
    pub work_count: AtomicU64,
}
```

**Features:**
- Busy state tracking
- Work count statistics
- Atomic operations

### WorkqueueSubsystem

Central workqueue management:

```rust
pub struct WorkqueueSubsystem {
    pending_work: VecDeque<WorkItem>,
    completed_work: VecDeque<WorkItem>,
    workers: Vec<Worker>,
    next_work_id: AtomicU64,
    next_worker_id: AtomicU64,
    current_time_ns: AtomicU64,
    max_workers: usize,
}
```

**Features:**
- Worker pool management
- Priority-based work selection
- Work queue management
- Completion tracking

## Usage

### Creating a Workqueue

```rust
let mut wq = WorkqueueSubsystem::new(4);  // 4 workers
wq.add_worker()?;
wq.add_worker()?;
```

### Submitting Work

```rust
let callback: WorkCallback = |_id, data| {
    println!("Work {} with data: {}", id, data);
    Ok(())
};

let id = wq.submit_work(WorkPriority::High, callback, 42);
```

### Processing Work

```rust
let completed = wq.process_work();
println!("Completed work items: {:?}", completed);
```

### Getting Statistics

```rust
let pending = wq.pending_count();
let completed = wq.completed_count();
let total = wq.total_work_processed();
```

## Comparison with Linux Workqueues

| Feature | Linux Workqueues | SigmaOS Workqueue |
|---------|-----------------|------------------|
| Priority | Yes | Yes (4 levels) |
| Worker Pool | Yes | Yes |
| Atomic Ops | Partial | Full |
| State Tracking | Yes | Yes |
| Timestamps | Yes | Yes |

## Implementation Details

- **Zero External Dependencies**: Uses only std:: and core:: primitives
- **Atomic Operations**: Lock-free synchronization with SeqCst ordering
- **Priority Selection**: Highest priority work selected first
- **Bounded Workers**: Configurable maximum worker count
- **Memory Safety**: Safe Rust with no unsafe code paths
- **Thread Safety**: All operations are thread-safe via atomic counters

## Testing

Comprehensive test coverage includes:

- Workqueue creation and worker addition
- Work submission and queuing
- Work processing and completion
- Priority-based work selection
- Worker pool exhaustion handling
- Statistics tracking

## Future Enhancements

- Per-CPU workqueues
- Work stealing between workers
- Delayed work support
- Work cancellation
- Work dependency chains
- CPU affinity for workers
- Real-time work priorities

## References

- Linux workqueues (kernel/workqueue.c)
- BSD taskqueues (sys/kern/subr_taskqueue.c)
- POSIX thread pools
