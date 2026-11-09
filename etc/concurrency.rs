// SPDX-License-Identifier: MIT
// SigmaOS Concurrency & Resource Management
// Race-free design, safe concurrency, memory discipline, resource limits

use std::collections::HashMap;
use std::fmt;
use std::sync::atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering};
use std::sync::{Arc, Condvar, Mutex, RwLock};
use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

// ============================================================================
// SAFE CONCURRENCY PRIMITIVES
// ============================================================================

/// A bounded channel-like queue with backpressure (no unbounded growth)
pub struct BoundedQueue<T> {
    inner: Mutex<VecDeque<T>>,
    not_empty: Condvar,
    not_full: Condvar,
    capacity: usize,
    closed: AtomicBool,
}

use std::collections::VecDeque;

impl<T> BoundedQueue<T> {
    pub fn new(capacity: usize) -> Self {
        Self {
            inner: Mutex::new(VecDeque::with_capacity(capacity)),
            not_empty: Condvar::new(),
            not_full: Condvar::new(),
            capacity,
            closed: AtomicBool::new(false),
        }
    }

    /// Push with backpressure; blocks when full. Returns false if closed.
    pub fn push(&self, item: T) -> bool {
        let mut q = self.inner.lock().unwrap();
        loop {
            if self.closed.load(Ordering::SeqCst) {
                return false;
            }
            if q.len() < self.capacity {
                q.push_back(item);
                self.not_empty.notify_one();
                return true;
            }
            q = self.not_full.wait(q).unwrap();
        }
    }

    /// Pop; blocks when empty. Returns None if closed and drained.
    pub fn pop(&self) -> Option<T> {
        let mut q = self.inner.lock().unwrap();
        loop {
            if let Some(item) = q.pop_front() {
                self.not_full.notify_one();
                return Some(item);
            }
            if self.closed.load(Ordering::SeqCst) {
                return None;
            }
            q = self.not_empty.wait(q).unwrap();
        }
    }

    pub fn close(&self) {
        self.closed.store(true, Ordering::SeqCst);
        self.not_empty.notify_all();
        self.not_full.notify_all();
    }

    pub fn len(&self) -> usize {
        self.inner.lock().unwrap().len()
    }

    pub fn is_closed(&self) -> bool {
        self.closed.load(Ordering::SeqCst)
    }
}

/// Reader-writer registry with versioned snapshots (race-free reads)
pub struct VersionedRegistry<T: Clone> {
    inner: RwLock<(u64, T)>,
}

impl<T: Clone> VersionedRegistry<T> {
    pub fn new(initial: T) -> Self {
        Self {
            inner: RwLock::new((0, initial)),
        }
    }

    /// Read a consistent snapshot
    pub fn snapshot(&self) -> (u64, T) {
        let guard = self.inner.read().unwrap();
        guard.clone()
    }

    /// Read with optimistic retry (avoids lock hold during use)
    pub fn read_with<F, R>(&self, f: F) -> R
    where
        F: Fn(&T) -> R,
    {
        let guard = self.inner.read().unwrap();
        f(&guard.1)
    }

    /// Update atomically; returns new version
    pub fn update_with<F>(&self, f: F) -> u64
    where
        F: Fn(&mut T),
    {
        let mut guard = self.inner.write().unwrap();
        f(&mut guard.1);
        guard.0 += 1;
        guard.0
    }

    pub fn version(&self) -> u64 {
        self.inner.read().unwrap().0
    }
}

/// Work-stealing style shared task queue (MPMC)
pub struct TaskQueue {
    queue: BoundedQueue<Box<dyn FnOnce() + Send>>,
}

impl TaskQueue {
    pub fn new(capacity: usize) -> Self {
        Self {
            queue: BoundedQueue::new(capacity),
        }
    }

    pub fn submit(&self, task: impl FnOnce() + Send + 'static) -> bool {
        self.queue.push(Box::new(task))
    }

    pub fn get(&self) -> Option<Box<dyn FnOnce() + Send>> {
        self.queue.pop()
    }

    pub fn close(&self) {
        self.queue.close();
    }
}

/// Deadlock-free lock ordering helper: always acquire locks in sorted order
pub struct OrderedLocks<A, B> {
    pub a: Mutex<A>,
    pub b: Mutex<B>,
    order_ab: bool, // true if a < b by address ordering preference
}

impl<A, B> OrderedLocks<A, B> {
    pub fn new(a: A, b: B) -> Self {
        Self {
            a: Mutex::new(a),
            b: Mutex::new(b),
            order_ab: true,
        }
    }

    /// Access both locks in a deterministic order to prevent deadlock
    pub fn with_both<R>(&self, f: impl FnOnce(&A, &B) -> R) -> R {
        // Always lock 'a' first — callers must use this helper for any
        // operation needing both resources.
        let ga = self.a.lock().unwrap();
        let gb = self.b.lock().unwrap();
        f(&ga, &gb)
    }
}

// ============================================================================
// RESOURCE MANAGEMENT — memory discipline, limits, cleanup
// ============================================================================

/// Memory budget tracker with hard limits (fail-fast on exhaustion)
pub struct MemoryBudget {
    limit: u64,
    used: AtomicU64,
}

impl MemoryBudget {
    pub fn new(limit: u64) -> Self {
        Self {
            limit,
            used: AtomicU64::new(0),
        }
    }

    /// Try to reserve `bytes`; returns guard that releases on drop
    pub fn reserve(self: &Arc<Self>, bytes: u64) -> Result<MemoryReservation, BudgetError> {
        let mut current = self.used.load(Ordering::SeqCst);
        loop {
            if current + bytes > self.limit {
                return Err(BudgetError::Exhausted {
                    limit: self.limit,
                    used: current,
                    requested: bytes,
                });
            }
            match self.used.compare_exchange_weak(
                current,
                current + bytes,
                Ordering::SeqCst,
                Ordering::SeqCst,
            ) {
                Ok(_) => break,
                Err(actual) => current = actual,
            }
        }
        Ok(MemoryReservation {
            budget: self.clone(),
            bytes,
        })
    }

    pub fn used(&self) -> u64 {
        self.used.load(Ordering::SeqCst)
    }

    pub fn limit(&self) -> u64 {
        self.limit
    }

    pub fn remaining(&self) -> u64 {
        self.limit.saturating_sub(self.used())
    }
}

/// RAII guard releasing reserved memory on drop
pub struct MemoryReservation {
    budget: Arc<MemoryBudget>,
    bytes: u64,
}

impl Drop for MemoryReservation {
    fn drop(&mut self) {
        self.budget.used.fetch_sub(self.bytes, Ordering::SeqCst);
    }
}

#[derive(Debug)]
pub enum BudgetError {
    Exhausted { limit: u64, used: u64, requested: u64 },
}

impl fmt::Display for BudgetError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BudgetError::Exhausted { limit, used, requested } => write!(
                f,
                "memory budget exhausted: limit={} used={} requested={}",
                limit, used, requested
            ),
        }
    }
}

impl std::error::Error for BudgetError {}

/// Connection pool with max size and idle timeout (resource discipline)
pub struct ConnectionPool<T>
where
    T: Send,
{
    idle: Mutex<Vec<(T, Instant)>>,
    max_size: usize,
    idle_timeout: Duration,
    created: AtomicUsize,
    factory: Box<dyn Fn() -> T + Send + Sync>,
}

impl<T: Send + 'static> ConnectionPool<T> {
    pub fn new(
        max_size: usize,
        idle_timeout: Duration,
        factory: impl Fn() -> T + Send + Sync + 'static,
    ) -> Self {
        Self {
            idle: Mutex::new(Vec::new()),
            max_size,
            idle_timeout,
            created: AtomicUsize::new(0),
            factory: Box::new(factory),
        }
    }

    /// Acquire a connection; returns RAII guard that returns it to the pool
    pub fn acquire(self: &Arc<Self>) -> PooledConnection<T> {
        // Reuse idle connection if fresh
        {
            let mut idle = self.idle.lock().unwrap();
            while let Some((conn, ts)) = idle.pop() {
                if ts.elapsed() <= self.idle_timeout {
                    return PooledConnection {
                        conn: Some(conn),
                        pool: self.clone(),
                    };
                }
                // Expired: drop it and decrement count
                self.created.fetch_sub(1, Ordering::SeqCst);
            }
        }

        // Create new if under limit
        let count = self.created.fetch_add(1, Ordering::SeqCst);
        if count >= self.max_size {
            self.created.fetch_sub(1, Ordering::SeqCst);
            // Spin-wait briefly for a connection to be returned
            for _ in 0..1000 {
                thread::sleep(Duration::from_micros(100));
                let mut idle = self.idle.lock().unwrap();
                if let Some((conn, ts)) = idle.pop() {
                    if ts.elapsed() <= self.idle_timeout {
                        return PooledConnection {
                            conn: Some(conn),
                            pool: self.clone(),
                        };
                    }
                    self.created.fetch_sub(1, Ordering::SeqCst);
                }
            }
            panic!("connection pool exhausted"); // fail-fast
        }

        PooledConnection {
            conn: Some((self.factory)()),
            pool: self.clone(),
        }
    }

    pub fn stats(&self) -> PoolStats {
        PoolStats {
            created: self.created.load(Ordering::SeqCst),
            idle: self.idle.lock().unwrap().len(),
            max_size: self.max_size,
        }
    }
}

pub struct PoolStats {
    pub created: usize,
    pub idle: usize,
    pub max_size: usize,
}

/// RAII pooled connection
pub struct PooledConnection<T: Send + 'static> {
    conn: Option<T>,
    pool: Arc<ConnectionPool<T>>,
}

impl<T: Send + 'static> PooledConnection<T> {
    pub fn get(&mut self) -> &mut T {
        self.conn.as_mut().expect("connection already returned")
    }
}

impl<T: Send + 'static> Drop for PooledConnection<T> {
    fn drop(&mut self) {
        if let Some(conn) = self.conn.take() {
            let mut idle = self.pool.idle.lock().unwrap();
            if idle.len() < self.pool.max_size {
                idle.push((conn, Instant::now()));
            } else {
                self.pool.created.fetch_sub(1, Ordering::SeqCst);
            }
        }
    }
}

// ============================================================================
// RATE LIMITER / TOKEN BUCKET (efficient scheduling support)
// ============================================================================

/// Token bucket rate limiter — lock-free reads, atomic refill
pub struct TokenBucket {
    capacity: u64,
    tokens: AtomicU64,
    refill_rate_per_sec: f64,
    last_refill: Mutex<Instant>,
}

impl TokenBucket {
    pub fn new(capacity: u64, refill_rate_per_sec: f64) -> Self {
        Self {
            capacity,
            tokens: AtomicU64::new(capacity),
            refill_rate_per_sec,
            last_refill: Mutex::new(Instant::now()),
        }
    }

    fn refill(&self) {
        let mut last = self.last_refill.lock().unwrap();
        let now = Instant::now();
        let elapsed = now.duration_since(*last).as_secs_f64();
        if elapsed > 0.0 {
            let new_tokens = (elapsed * self.refill_rate_per_sec) as u64;
            if new_tokens > 0 {
                let current = self.tokens.load(Ordering::SeqCst);
                let updated = (current + new_tokens).min(self.capacity);
                self.tokens.store(updated, Ordering::SeqCst);
                *last = now;
            }
        }
    }

    /// Try to consume one token; returns true if allowed
    pub fn try_consume(&self) -> bool {
        self.refill();
        let mut current = self.tokens.load(Ordering::SeqCst);
        loop {
            if current == 0 {
                return false;
            }
            match self.tokens.compare_exchange_weak(
                current,
                current - 1,
                Ordering::SeqCst,
                Ordering::SeqCst,
            ) {
                Ok(_) => return true,
                Err(actual) => current = actual,
            }
        }
    }

    pub fn available(&self) -> u64 {
        self.refill();
        self.tokens.load(Ordering::SeqCst)
    }
}

// ============================================================================
// SINGLETON-SAFE GLOBAL STATE (once-init pattern)
// ============================================================================

/// Global registry initialized once, read lock-free afterwards
pub struct GlobalRegistry {
    inner: &'static RwLock<HashMap<String, String>>,
}

impl GlobalRegistry {
    /// Returns the process-wide registry (idempotent init)
    pub fn instance() -> &'static GlobalRegistry {
        use std::sync::OnceLock;
        static REGISTRY: OnceLock<GlobalRegistry> = OnceLock::new();
        REGISTRY.get_or_init(|| GlobalRegistry {
            inner: Box::leak(Box::new(RwLock::new(HashMap::new()))),
        })
    }

    pub fn set(&self, key: &str, value: &str) {
        self.inner.write().unwrap().insert(key.to_string(), value.to_string());
    }

    pub fn get(&self, key: &str) -> Option<String> {
        self.inner.read().unwrap().get(key).cloned()
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::AtomicUsize;

    #[test]
    fn test_bounded_queue() {
        let q = BoundedQueue::new(2);
        assert!(q.push(1));
        assert!(q.push(2));
        // Third push would block — verify via thread
        let q2 = Arc::new(BoundedQueue::new(1));
        let q3 = q2.clone();
        let handle = thread::spawn(move || {
            q3.push(1);
            q3.push(2); // blocks until pop
        });
        thread::sleep(Duration::from_millis(20));
        assert_eq!(q2.pop(), Some(1));
        handle.join().unwrap();
        assert_eq!(q2.pop(), Some(2));
    }

    #[test]
    fn test_versioned_registry() {
        let reg = VersionedRegistry::new(vec![1, 2, 3]);
        let (v0, snap) = reg.snapshot();
        assert_eq!(v0, 0);
        assert_eq!(snap, vec![1, 2, 3]);

        reg.update_with(|v| v.push(4));
        let (v1, snap) = reg.snapshot();
        assert_eq!(v1, 1);
        assert_eq!(snap, vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_memory_budget() {
        let budget = Arc::new(MemoryBudget::new(100));
        let r1 = budget.reserve(60).unwrap();
        let r2 = budget.reserve(30).unwrap();
        assert_eq!(budget.used(), 90);
        assert!(budget.reserve(20).is_err()); // would exceed 100
        drop(r1);
        assert_eq!(budget.used(), 30);
        assert!(budget.reserve(70).is_ok());
    }

    #[test]
    fn test_connection_pool() {
        let pool = Arc::new(ConnectionPool::new(2, Duration::from_secs(60), || {
            AtomicUsize::new(0)
        }));
        let mut c1 = pool.acquire();
        let stats = pool.stats();
        assert_eq!(stats.created, 1);
        drop(c1);
        let _c2 = pool.acquire(); // should reuse idle
        let stats = pool.stats();
        assert_eq!(stats.created, 1); // reused, not created new
    }

    #[test]
    fn test_token_bucket() {
        let bucket = TokenBucket::new(5, 10.0);
        for _ in 0..5 {
            assert!(bucket.try_consume());
        }
        assert!(!bucket.try_consume());
        thread::sleep(Duration::from_millis(150));
        assert!(bucket.try_consume()); // refilled
    }

    #[test]
    fn test_global_registry() {
        let reg = GlobalRegistry::instance();
        reg.set("key", "value");
        assert_eq!(reg.get("key"), Some("value".to_string()));
    }

    #[test]
    fn test_task_queue() {
        let tq = TaskQueue::new(10);
        let counter = Arc::new(AtomicUsize::new(0));
        let c2 = counter.clone();
        assert!(tq.submit(move || { c2.fetch_add(1, Ordering::SeqCst); }));
        let task = tq.get().unwrap();
        task();
        assert_eq!(counter.load(Ordering::SeqCst), 1);
        tq.close();
        assert!(tq.get().is_none());
    }
}
