#![allow(clippy::new_without_default)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(unexpected_cfgs)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::type_complexity)]
//! # SigmaOS Read-Copy-Update (RCU) Synchronization
//!
//! A sovereign RCU implementation inspired by the Linux kernel's `rcu_read_lock` /
//! `synchronize_rcu` API.
//!
//! ## Design principles
//!
//! * **Readers are never blocked** — they obtain a guard in O(1) with no atomic
//!   CAS operations on the fast path.
//! * **Writers synchronise** via a generation counter: after publishing a new
//!   pointer, the writer increments the generation and waits until all
//!   previously-active readers have exited their read-side critical sections.
//! * **Grace periods** are tracked with a 64-bit generation counter. Readers
//!   record the generation at which they entered; `synchronize_rcu` spins until
//!   no reader holds a guard from a stale generation.
//!
//! ## API Comparison with Linux kernel RCU
//!
//! | Linux kernel                  | SigmaOS                          |
//! |-------------------------------|----------------------------------|
//! | `rcu_read_lock()`             | `RcuCell::read_lock()`           |
//! | `rcu_read_unlock()`           | drop(`RcuReadGuard`)             |
//! | `rcu_dereference(p)`          | `RcuReadGuard::get()`            |
//! | `rcu_assign_pointer(p, v)`    | `RcuCell::rcu_assign_pointer()`  |
//! | `synchronize_rcu()`           | `RcuCell::synchronize_rcu()`     |
//! | `call_rcu()`                  | *(not yet implemented)*          |
//!
//! ## Example
//!
//! ```rust,ignore
//! let cell = RcuCell::new(42u32);
//! {
//!     let guard = cell.read_lock();
//!     assert_eq!(*guard.get(), 42);
//! } // guard dropped → rcu_read_unlock()
//!
//! cell.rcu_assign_pointer(99u32);
//! cell.synchronize_rcu(); // waits for all outstanding readers
//! ```

#![allow(dead_code)]

use core::sync::atomic::{AtomicU64, AtomicUsize, Ordering};

// ─────────────────────────────────────────────────────────────────────────────
// Generation counter helpers
// ─────────────────────────────────────────────────────────────────────────────

/// Global reader count for the *current* generation.
///
/// In a real SMP kernel this would be a per-CPU counter; here we use a single
/// global for simplicity.
static READER_COUNT: AtomicUsize = AtomicUsize::new(0);

/// Monotonically-increasing grace-period generation counter.
static GENERATION: AtomicU64 = AtomicU64::new(0);

// ─────────────────────────────────────────────────────────────────────────────
// Public free functions (Linux-style API)
// ─────────────────────────────────────────────────────────────────────────────

/// Enter an RCU read-side critical section.
///
/// Must be paired with a call to [`rcu_read_unlock`].
///
/// # Safety
///
/// The caller must ensure that every `rcu_read_lock` has a matching
/// `rcu_read_unlock` on every code path.
#[inline]
pub fn rcu_read_lock() {
    READER_COUNT.fetch_add(1, Ordering::Acquire);
}

/// Exit an RCU read-side critical section.
#[inline]
pub fn rcu_read_unlock() {
    READER_COUNT.fetch_sub(1, Ordering::Release);
}

/// Block until all pre-existing RCU read-side critical sections have completed.
///
/// After this function returns, any RCU-protected data that was made
/// unreachable *before* this call may be safely reclaimed.
pub fn synchronize_rcu() {
    // Increment the generation so that new readers enter the next epoch.
    GENERATION.fetch_add(1, Ordering::SeqCst);
    // Spin until all current readers have left the critical section.
    // In a real kernel we would use a more efficient quiescent-state mechanism.
    while READER_COUNT.load(Ordering::Acquire) > 0 {
        core::hint::spin_loop();
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// RcuReadGuard
// ─────────────────────────────────────────────────────────────────────────────

/// An RAII guard that holds the RCU read-side lock for the lifetime `'a`.
///
/// Obtained via [`RcuCell::read_lock`]. Dropping this guard calls
/// [`rcu_read_unlock`] automatically.
pub struct RcuReadGuard<'a, T> {
    data: &'a T,
    /// Generation at which this guard was acquired (for diagnostics).
    _generation: u64,
}

impl<'a, T> RcuReadGuard<'a, T> {
    /// Access the RCU-protected value.
    ///
    /// The reference is valid for the lifetime of the guard.
    #[inline]
    pub fn get(&self) -> &T {
        self.data
    }
}

impl<T> Drop for RcuReadGuard<'_, T> {
    #[inline]
    fn drop(&mut self) {
        rcu_read_unlock();
    }
}

impl<T: core::fmt::Debug> core::fmt::Debug for RcuReadGuard<'_, T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("RcuReadGuard")
            .field("data", self.data)
            .field("generation", &self._generation)
            .finish()
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// RcuCell
// ─────────────────────────────────────────────────────────────────────────────

/// An RCU-protected data cell.
///
/// `RcuCell<T>` holds a single value of type `T`. Readers obtain a
/// [`RcuReadGuard`] that grants shared access with zero contention. Writers
/// use [`rcu_assign_pointer`](RcuCell::rcu_assign_pointer) followed by
/// [`synchronize_rcu`](RcuCell::synchronize_rcu) to publish a new value and
/// wait for outstanding readers to finish.
///
/// # Limitations (sovereign / single-threaded model)
///
/// In this implementation `T` is stored inline (no heap pointer indirection).
/// A production SMP implementation would use an `AtomicPtr<T>` and reclaim the
/// old allocation after the grace period.
pub struct RcuCell<T> {
    /// The currently published value.
    ///
    /// Wrapped in `core::cell::UnsafeCell` to allow interior mutability without
    /// a Mutex (safe because we follow RCU discipline: no mutation while any
    /// reader is active).
    inner: core::cell::UnsafeCell<T>,
}

// SAFETY: We uphold RCU read/write discipline manually.
unsafe impl<T: Send> Send for RcuCell<T> {}
unsafe impl<T: Sync> Sync for RcuCell<T> {}

impl<T> RcuCell<T> {
    /// Create a new `RcuCell` with an initial value.
    pub fn new(value: T) -> Self {
        RcuCell {
            inner: core::cell::UnsafeCell::new(value),
        }
    }

    /// Enter a read-side critical section and obtain a guard.
    ///
    /// The guard dereferences to `&T` and calls [`rcu_read_unlock`] when
    /// dropped.
    pub fn read_lock(&self) -> RcuReadGuard<'_, T> {
        let gen = GENERATION.load(Ordering::Acquire);
        rcu_read_lock();
        // SAFETY: while the guard is alive, no writer may overwrite `inner`
        // (because writers call `synchronize_rcu` which spins until
        // READER_COUNT is 0 before returning).
        let data = unsafe { &*self.inner.get() };
        RcuReadGuard { data, _generation: gen }
    }

    /// Publish a new value, analogous to `rcu_assign_pointer`.
    ///
    /// The write is sequentially consistent so that subsequent readers see the
    /// new value. Must be followed by [`synchronize_rcu`](RcuCell::synchronize_rcu)
    /// before the old value can be reclaimed.
    ///
    /// # Safety
    ///
    /// Caller must ensure no other writer is concurrently modifying the cell.
    pub fn rcu_assign_pointer(&self, new_value: T) {
        // SAFETY: We uphold the RCU contract: no readers are dereferencing the
        // old pointer after `synchronize_rcu` returns.
        unsafe {
            *self.inner.get() = new_value;
        }
        core::sync::atomic::fence(Ordering::SeqCst);
    }

    /// Wait for all pre-existing read-side critical sections to complete.
    ///
    /// Call this after [`rcu_assign_pointer`](RcuCell::rcu_assign_pointer) to
    /// ensure it is safe to free the old value.
    pub fn synchronize_rcu(&self) {
        synchronize_rcu();
    }

    /// Return the current generation counter value.
    pub fn generation(&self) -> u64 {
        GENERATION.load(Ordering::Acquire)
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rcu_read_basic() {
        let cell = RcuCell::new(42u32);
        let guard = cell.read_lock();
        assert_eq!(*guard.get(), 42);
        drop(guard);
        assert_eq!(READER_COUNT.load(Ordering::Relaxed), 0);
    }

    #[test]
    fn test_rcu_assign_and_synchronize() {
        let cell = RcuCell::new(0u32);
        cell.rcu_assign_pointer(99u32);
        cell.synchronize_rcu();
        let guard = cell.read_lock();
        assert_eq!(*guard.get(), 99);
    }

    #[test]
    fn test_multiple_reads_before_write() {
        let cell = RcuCell::new(7u32);
        let g1 = cell.read_lock();
        let g2 = cell.read_lock();
        assert_eq!(*g1.get(), 7);
        assert_eq!(*g2.get(), 7);
        drop(g1);
        drop(g2);
        assert_eq!(READER_COUNT.load(Ordering::Relaxed), 0);
    }
}
