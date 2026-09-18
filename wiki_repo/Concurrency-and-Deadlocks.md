# Concurrency and Deadlocks

SigmaOS implements comprehensive concurrency management with Linux and BSD-inspired features including threads, synchronization primitives, deadlock detection, and lock-free algorithms.

## Overview

Concurrency management provides:
- Thread creation and management with pthread-inspired API
- Synchronization primitives (mutexes, rwlocks, semaphores, condition variables)
- Atomic operations and memory ordering
- Lock-free data structures and algorithms
- Deadlock detection and prevention
- Thread scheduling and priority inheritance
- RCU (Read-Copy-Update) for read-mostly data
- Work queues and thread pools

## Implementation

### Thread Management
```rust
// src/kernel/thread.rs
pub struct ThreadManager {
    pub threads: BTreeMap<ThreadId, Thread>,
    pub ready_queue: VecDeque<ThreadId>,
    pub sleeping_threads: BTreeMap<Duration, Vec<ThreadId>>,
    pub current_thread: Option<ThreadId>,
}

#[derive(Debug, Clone)]
pub struct Thread {
    pub id: ThreadId,
    pub state: ThreadState,
    pub priority: ThreadPriority,
    pub stack: Vec<u8>,
    pub context: ThreadContext,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThreadState {
    Ready,
    Running,
    Sleeping,
    Blocked,
    Terminated,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ThreadPriority {
    Idle = 0,
    Normal = 10,
    High = 20,
    Realtime = 30,
}

impl ThreadManager {
    pub fn new() -> Self {
        ThreadManager {
            threads: BTreeMap::new(),
            ready_queue: VecDeque::new(),
            sleeping_threads: BTreeMap::new(),
            current_thread: None,
        }
    }

    pub fn create_thread(&mut self, entry: fn() -> i32, priority: ThreadPriority) -> Result<ThreadId, ThreadError> {
        let thread_id = self.generate_thread_id();
        let stack_size = 8192; // 8KB stack
        let stack = vec![0u8; stack_size];
        
        let thread = Thread {
            id: thread_id,
            state: ThreadState::Ready,
            priority,
            stack,
            context: ThreadContext::new(entry),
        };
        
        self.threads.insert(thread_id, thread);
        self.ready_queue.push_back(thread_id);
        
        Ok(thread_id)
    }

    pub fn schedule(&mut self) -> Option<ThreadId> {
        if let Some(thread_id) = self.ready_queue.pop_front() {
            self.current_thread = Some(thread_id);
            if let Some(thread) = self.threads.get_mut(&thread_id) {
                thread.state = ThreadState::Running;
            }
            Some(thread_id)
        } else {
            None
        }
    }

    pub fn yield_thread(&mut self) {
        if let Some(current_id) = self.current_thread {
            if let Some(thread) = self.threads.get_mut(&current_id) {
                thread.state = ThreadState::Ready;
                self.ready_queue.push_back(current_id);
            }
            self.current_thread = None;
        }
    }

    fn generate_thread_id(&self) -> ThreadId {
        self.threads.len() as ThreadId + 1
    }
}
```

### Synchronization Primitives
```rust
// src/kernel/sync.rs
pub struct Mutex {
    pub locked: AtomicBool,
    pub owner: AtomicUsize,
}

impl Mutex {
    pub fn new() -> Self {
        Mutex {
            locked: AtomicBool::new(false),
            owner: AtomicUsize::new(0),
        }
    }

    pub fn lock(&self) {
        while self.locked.compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed).is_err() {
            // Spin wait
            core::hint::spin_loop();
        }
        self.owner.store(thread_id(), Ordering::Relaxed);
    }

    pub fn unlock(&self) {
        self.owner.store(0, Ordering::Relaxed);
        self.locked.store(false, Ordering::Release);
    }
}

pub struct RwLock {
    pub readers: AtomicUsize,
    pub writer: AtomicBool,
}

impl RwLock {
    pub fn new() -> Self {
        RwLock {
            readers: AtomicUsize::new(0),
            writer: AtomicBool::new(false),
        }
    }

    pub fn read_lock(&self) {
        while self.writer.load(Ordering::Acquire) {
            core::hint::spin_loop();
        }
        self.readers.fetch_add(1, Ordering::Acquire);
    }

    pub fn read_unlock(&self) {
        self.readers.fetch_sub(1, Ordering::Release);
    }

    pub fn write_lock(&self) {
        while self.writer.compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed).is_err() {
            core::hint::spin_loop();
        }
        while self.readers.load(Ordering::Acquire) > 0 {
            core::hint::spin_loop();
        }
    }

    pub fn write_unlock(&self) {
        self.writer.store(false, Ordering::Release);
    }
}

pub struct Semaphore {
    pub count: AtomicUsize,
}

impl Semaphore {
    pub fn new(initial: usize) -> Self {
        Semaphore {
            count: AtomicUsize::new(initial),
        }
    }

    pub fn wait(&self) {
        while self.count.load(Ordering::Acquire) == 0 {
            core::hint::spin_loop();
        }
        self.count.fetch_sub(1, Ordering::Acquire);
    }

    pub fn signal(&self) {
        self.count.fetch_add(1, Ordering::Release);
    }
}
```

### Deadlock Detection
```rust
// src/kernel/deadlock.rs
pub struct DeadlockDetector {
    pub lock_graph: BTreeMap<ThreadId, Vec<LockId>>,
    pub wait_graph: BTreeMap<ThreadId, ThreadId>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LockId(u64);

impl DeadlockDetector {
    pub fn new() -> Self {
        DeadlockDetector {
            lock_graph: BTreeMap::new(),
            wait_graph: BTreeMap::new(),
        }
    }

    pub fn acquire_lock(&mut self, thread_id: ThreadId, lock_id: LockId) {
        self.lock_graph.entry(thread_id).or_insert_with(Vec::new).push(lock_id);
    }

    pub fn release_lock(&mut self, thread_id: ThreadId, lock_id: LockId) {
        if let Some(locks) = self.lock_graph.get_mut(&thread_id) {
            if let Some(pos) = locks.iter().position(|&l| l == lock_id) {
                locks.remove(pos);
            }
        }
    }

    pub fn wait_for_lock(&mut self, waiter: ThreadId, holder: ThreadId) {
        self.wait_graph.insert(waiter, holder);
    }

    pub fn detect_deadlock(&self) -> Option<Vec<ThreadId>> {
        // Detect cycles in wait graph
        for thread in self.wait_graph.keys() {
            if let Some(cycle) = self.find_cycle(*thread) {
                return Some(cycle);
            }
        }
        None
    }

    fn find_cycle(&self, start: ThreadId) -> Option<Vec<ThreadId>> {
        let mut visited = BTreeSet::new();
        let mut path = Vec::new();
        self.dfs_find_cycle(start, &mut visited, &mut path)
    }

    fn dfs_find_cycle(&self, current: ThreadId, visited: &mut BTreeSet<ThreadId>, path: &mut Vec<ThreadId>) -> Option<Vec<ThreadId>> {
        if visited.contains(&current) {
            if let Some(pos) = path.iter().position(|&t| t == current) {
                return Some(path[pos..].to_vec());
            }
            return None;
        }

        visited.insert(current);
        path.push(current);

        if let Some(&holder) = self.wait_graph.get(&current) {
            if let Some(cycle) = self.dfs_find_cycle(holder, visited, path) {
                return Some(cycle);
            }
        }

        path.pop();
        None
    }
}
```

### Lock-Free Data Structures
```rust
// src/kernel/lockfree.rs
pub struct LockFreeQueue<T> {
    pub head: AtomicPtr<Node<T>>,
    pub tail: AtomicPtr<Node<T>>,
}

struct Node<T> {
    data: T,
    next: AtomicPtr<Node<T>>,
}

impl<T> LockFreeQueue<T> {
    pub fn new() -> Self {
        let dummy = Box::into_raw(Box::new(Node {
            data: unsafe { core::mem::zeroed() },
            next: AtomicPtr::new(ptr::null_mut()),
        }));
        
        LockFreeQueue {
            head: AtomicPtr::new(dummy),
            tail: AtomicPtr::new(dummy),
        }
    }

    pub fn enqueue(&self, item: T) {
        let new_node = Box::into_raw(Box::new(Node {
            data: item,
            next: AtomicPtr::new(ptr::null_mut()),
        }));

        loop {
            let tail = self.tail.load(Ordering::Acquire);
            let next = unsafe { (*tail).next.load(Ordering::Acquire) };

            if !next.is_null() {
                // Tail is not pointing to the last node, advance it
                let _ = self.tail.compare_exchange_weak(tail, next, Ordering::Release, Ordering::Relaxed);
            } else {
                // Tail is pointing to the last node, try to link new node
                if unsafe { (*tail).next.compare_exchange_weak(next, new_node, Ordering::Release, Ordering::Relaxed).is_ok() } {
                    // Successfully linked, now try to advance tail
                    let _ = self.tail.compare_exchange_weak(tail, new_node, Ordering::Release, Ordering::Relaxed);
                    break;
                }
            }
        }
    }

    pub fn dequeue(&self) -> Option<T> {
        loop {
            let head = self.head.load(Ordering::Acquire);
            let tail = self.tail.load(Ordering::Acquire);
            let next = unsafe { (*head).next.load(Ordering::Acquire) };

            if head == tail {
                if next.is_null() {
                    // Queue is empty
                    return None;
                }
                // Tail is lagging, help advance it
                let _ = self.tail.compare_exchange_weak(tail, next, Ordering::Release, Ordering::Relaxed);
            } else {
                // Try to dequeue
                if self.head.compare_exchange_weak(head, next, Ordering::Release, Ordering::Relaxed).is_ok() {
                    // Successfully dequeued
                    let data = unsafe { ptr::read(&(*next).data) };
                    // Deallocate the old head node
                    unsafe { Box::from_raw(head) };
                    return Some(data);
                }
            }
        }
    }
}
```

## Configuration

### Concurrency Configuration
```toml
# /etc/sigmaos/concurrency.toml
[threads]
# Thread settings
default_stack_size = 8192
max_threads = 4096
default_priority = "normal"

[synchronization]
# Synchronization settings
mutex_spin_limit = 1000
rwlock_readers_limit = 64
semaphore_initial = 10

[deadlock]
# Deadlock detection settings
enabled = true
detection_interval_seconds = 5
prevention_enabled = true

[lockfree]
# Lock-free settings
enabled = true
queue_size = 1024
rcu_enabled = true
```

### Runtime Control
```bash
# Show thread statistics
sigthread stats

# Create thread
sigthread create <function> --priority high

# Set thread priority
sigthread set-priority <thread_id> high

# Show lock status
siglock show-locks

# Detect deadlock
siglock detect-deadlock

# Enable deadlock detection
siglock enable-deadlock-detection

# Show RCU statistics
sigrcu stats
```

## Performance Optimization

### Thread Tuning
Optimize threads for performance:
```bash
# Set thread stack size
sigthread set-stack-size 16384

# Enable thread priority inheritance
sigthread enable-priority-inheritance

# Set time slice
sigthread set-time-slice 10

# Enable thread affinity
sigthread enable-affinity
```

### Synchronization Optimization
Optimize synchronization for performance:
```bash
# Set mutex spin limit
siglock set-mutex-spin-limit 100

# Enable adaptive spinning
siglock enable-adaptive-spin

# Enable lock elision
siglock enable-lock-elision

# Enable rwlock optimization
siglock enable-rwlock-optimization
```

### Lock-Free Optimization
Optimize lock-free structures for performance:
```bash
# Enable RCU
sigrcu enable

# Set RCU grace period
sigrcu set-grace-period 100

# Enable lock-free queue
siglockfree enable-queue

# Set queue size
siglockfree set-queue-size 2048
```

## Troubleshooting

### Deadlock Detected
If deadlock is detected:
1. Check deadlock report: `siglock detect-deadlock`
2. Identify threads in deadlock
3. Review lock acquisition order
4. Consider lock ordering
5. Kill deadlocked threads if necessary

### High Lock Contention
If lock contention is high:
1. Check lock statistics: `siglock show-locks`
2. Identify contended locks
3. Consider lock splitting
4. Use lock-free alternatives
5. Adjust lock granularity

### Thread Starvation
If thread starvation occurs:
1. Check thread priorities: `sigthread stats`
2. Adjust thread priorities
3. Enable priority inheritance
4. Check scheduler settings
5. Use fair scheduling

### Performance Degradation
If performance degrades:
1. Check thread count: `sigthread stats`
2. Check lock contention
3. Check context switches
4. Optimize critical sections
5. Use lock-free alternatives

---

**[Concurrency](Category-Concurrency)** | **[Synchronization](Category-Synchronization)** | **[Deadlocks](Category-Deadlocks)**
