// src/reminder/reminder_store.rs
// Bounded, deterministic reminder store for SigmaOS, inspired by Omarchy 4.0.5
// (`omancy-reminder`: set/show/clear lightweight desktop notification reminders
// with a delay and a message; Omancy-Manual-09 Reminders / Omancy-Manual-10 Notices).
//
// This subsystem owns the reminder state machine only — delivery (notification
// bus, shell indicator, DBus/portal) is the responsibility of callers, keeping
// a strict separation of mechanism (this store) and policy (when/how to alert).
//
// SigmaOS engineering canon applied throughout:
//   * Single responsibility: manage reminders only (add / list / due / fire / cancel / clear).
//   * Clear abstraction & encapsulation: internal buffer and clock are private.
//   * Command-query separation: commands mutate and report an outcome;
//     queries observe without mutating.
//   * Composition over inheritance: plain data + free functions, no class hierarchy.
//   * Memory safety: #![no_std] + alloc, bounds-checked, no `unsafe`.
//   * Fail-fast / defensive: validates input, clamps capacity, never panics.
//   * Deterministic: sequence IDs + logical tick clock (no wall clock) so tests are
//     FAST, ISOLATED, REPEATABLE, SELF-VALIDATING and TIMELY.
//   * Cache-conscious / locality of reference: flat Vec.
//   * Zero-copy observation: list_active() borrows; due() borrows references (no payload clone).
//   * Zero-one-infinity: zero reminders valid, one typical, infinity bounded by capacity.
//   * Secure defaults: empty messages rejected; active reminders are never silently dropped.

#![no_std]

extern crate alloc;

use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::fmt;

/// Default maximum number of reminders retained.
pub const DEFAULT_REMINDER_CAPACITY: usize = 200;

/// Lifecycle status of a reminder.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReminderStatus {
    Active,    // pending, not yet delivered
    Fired,     // delivered at least once
    Cancelled, // dismissed before firing
}

impl fmt::Display for ReminderStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Active => write!(f, "active"),
            Self::Fired => write!(f, "fired"),
            Self::Cancelled => write!(f, "cancelled"),
        }
    }
}

/// A single reminder.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Reminder {
    /// Monotonically increasing identifier (deterministic).
    pub id: u64,
    /// User-facing reminder message.
    pub message: String,
    /// Logical tick at which the reminder becomes due.
    pub due_at: u64,
    /// Logical tick at which the reminder was created.
    pub created_at: u64,
    /// Current lifecycle status.
    pub status: ReminderStatus,
}

impl Reminder {
    /// Whether the reminder is pending delivery (active and not yet due).
    pub fn is_pending(&self, tick: u64) -> bool {
        self.status == ReminderStatus::Active && self.due_at > tick
    }

    /// Whether the reminder is due for delivery at `tick`.
    pub fn is_due(&self, tick: u64) -> bool {
        self.status == ReminderStatus::Active && self.due_at <= tick
    }
}

/// Bounded, deterministic reminder store.
///
/// Time is expressed as a logical `u64` tick injected by the caller (mapped to
/// the kernel's monotonic clock in production). This keeps the subsystem
/// deterministic and testable without wall-clock dependence.
pub struct ReminderStore {
    items: Vec<Reminder>,
    next_id: u64,
    tick: u64,
    capacity: usize,
    firings: u64,
}

impl ReminderStore {
    /// Create a new reminder store with the given capacity.
    /// A capacity of `0` is defensively clamped to `1`.
    pub fn new(capacity: usize) -> Self {
        Self {
            items: Vec::with_capacity(capacity.max(1)),
            next_id: 1,
            tick: 0,
            capacity: capacity.max(1),
            firings: 0,
        }
    }

    /// Default-capacity store.
    pub fn default_capacity() -> Self {
        Self::new(DEFAULT_REMINDER_CAPACITY)
    }

    /// Inject the current logical tick (deterministic clock source). (Command)
    pub fn set_tick(&mut self, tick: u64) {
        self.tick = tick;
    }

    /// Advance the logical tick by `delta`. (Command)
    pub fn advance_tick(&mut self, delta: u64) {
        self.tick = self.tick.saturating_add(delta);
    }

    /// Current logical tick. (Query)
    pub fn tick(&self) -> u64 {
        self.tick
    }

    /// Add a reminder that becomes due after `ttl_ticks` ticks, returning its id.
    /// Returns `None` for an empty message or when the store is full of active
    /// reminders (secure default: active reminders are never silently dropped).
    /// (Command)
    pub fn add(&mut self, message: String, ttl_ticks: u64) -> Option<u64> {
        if message.trim().is_empty() {
            return None;
        }
        let id = self.next_id;
        self.next_id += 1;
        let item = Reminder {
            id,
            message,
            due_at: self.tick.saturating_add(ttl_ticks),
            created_at: self.tick,
            status: ReminderStatus::Active,
        };
        if self.items.len() >= self.capacity {
            // Make room by evicting the oldest non-active reminder.
            if let Some(pos) = self
                .items
                .iter()
                .position(|r| r.status != ReminderStatus::Active)
            {
                self.items.remove(pos);
            } else {
                // All active: refuse rather than silently drop.
                return None;
            }
        }
        self.items.push(item);
        Some(id)
    }

    /// Borrow all reminders (active, fired and cancelled), oldest-first. (Query, zero-copy)
    pub fn list(&self) -> &[Reminder] {
        &self.items
    }

    /// Borrow references to reminders due for delivery at the current tick.
    /// (Query, zero-copy of payloads)
    pub fn due(&self) -> Vec<&Reminder> {
        self.items
            .iter()
            .filter(|r| r.is_due(self.tick))
            .collect()
    }

    /// Mark the reminder with `id` as fired, returning `true` when found and active.
    /// (Command)
    pub fn mark_fired(&mut self, id: u64) -> bool {
        if let Some(r) = self
            .items
            .iter_mut()
            .find(|r| r.id == id && r.status == ReminderStatus::Active)
        {
            r.status = ReminderStatus::Fired;
            self.firings += 1;
            true
        } else {
            false
        }
    }

    /// Cancel the reminder with `id`, returning `true` when found and active.
    /// (Command)
    pub fn cancel(&mut self, id: u64) -> bool {
        if let Some(r) = self
            .items
            .iter_mut()
            .find(|r| r.id == id && r.status == ReminderStatus::Active)
        {
            r.status = ReminderStatus::Cancelled;
            true
        } else {
            false
        }
    }

    /// Remove every reminder. (Command)
    pub fn clear(&mut self) {
        self.items.clear();
    }

    /// Number of reminders currently stored. (Query)
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Whether the store is empty. (Query)
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Configured capacity. (Query)
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// Number of reminders fired (instrumentation / observability). (Query)
    pub fn firings(&self) -> u64 {
        self.firings
    }
}

impl Default for ReminderStore {
    fn default() -> Self {
        Self::default_capacity()
    }
}

impl fmt::Display for ReminderStore {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ReminderStore {{ len={}, capacity={}, tick={}, firings={} }}",
            self.items.len(),
            self.capacity,
            self.tick,
            self.firings
        )
    }
}

// ---- Deterministic unit tests (ARRANGE-ACT-ASSERT,
// FAST-ISOLATED-REPEATABLE-SELF-VALIDATING-TIMELY) ----

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_and_due_round_trip() {
        // Arrange
        let mut store = ReminderStore::new(10);
        store.set_tick(100);
        // Act
        let id = store.add("check the oven".to_string(), 30).expect("reminder added");
        // Assert
        assert_eq!(store.tick(), 100);
        assert_eq!(store.len(), 1);
        assert!(store.due().is_empty()); // not due at tick 100
        assert_eq!(store.pending_count(), 1);
        store.advance_tick(30);
        // Act
        let due: Vec<u64> = store.due().iter().map(|r| r.id).collect();
        // Assert
        assert_eq!(due, vec![id]);
        assert_eq!(store.pending_count(), 0);
    }

    #[test]
    fn empty_message_is_rejected() {
        // Arrange
        let mut store = ReminderStore::new(10);
        store.set_tick(0);
        // Act
        let id = store.add("".to_string(), 10);
        let id_ws = store.add("   ".to_string(), 10);
        // Assert
        assert!(id.is_none());
        assert!(id_ws.is_none());
        assert!(store.is_empty());
    }

    #[test]
    fn mark_fired_transitions_status() {
        // Arrange
        let mut store = ReminderStore::new(10);
        store.set_tick(0);
        let id = store.add("ping".to_string(), 5).expect("added");
        store.advance_tick(5);
        // Act
        let fired = store.mark_fired(id);
        let again = store.mark_fired(id);
        // Assert
        assert!(fired);
        assert!(!again); // already fired, not active
        assert_eq!(store.firings(), 1);
        assert!(store.due().is_empty());
    }

    #[test]
    fn cancel_removes_from_due() {
        // Arrange
        let mut store = ReminderStore::new(10);
        store.set_tick(0);
        let id = store.add("ping".to_string(), 5).expect("added");
        store.advance_tick(5);
        assert_eq!(store.due().len(), 1);
        // Act
        let cancelled = store.cancel(id);
        // Assert
        assert!(cancelled);
        assert!(store.due().is_empty());
        assert_eq!(store.firings(), 0);
    }

    #[test]
    fn unknown_id_operations_return_false() {
        // Arrange
        let mut store = ReminderStore::new(10);
        // Act / Assert
        assert!(!store.mark_fired(999));
        assert!(!store.cancel(999));
    }

    #[test]
    fn capacity_evicts_oldest_non_active_before_refusing() {
        // Arrange
        let mut store = ReminderStore::new(2);
        store.set_tick(0);
        let first = store.add("first".to_string(), 100).expect("added");
        let second = store.add("second".to_string(), 100).expect("added");
        assert_eq!(store.len(), 2);
        // Fire the first so it becomes evictable.
        store.mark_fired(first);
        // Act
        let third = store.add("third".to_string(), 100).expect("added");
        // Assert
        assert_eq!(store.len(), 2);
        assert_eq!(store.firings(), 1);
        // `first` was evicted (it was non-active), `second` and `third` remain.
        assert!(store
            .list()
            .iter()
            .any(|r| r.id == second));
        assert!(store
            .list()
            .iter()
            .any(|r| r.id == third));
    }

    #[test]
    fn full_of_active_reminders_refuses_instead_of_dropping() {
        // Arrange
        let mut store = ReminderStore::new(1);
        store.set_tick(0);
        store.add("only".to_string(), 100).expect("added");
        // Act
        let extra = store.add("extra".to_string(), 100);
        // Assert
        assert!(extra.is_none());
        assert_eq!(store.len(), 1);
    }

    #[test]
    fn clear_empties_store() {
        // Arrange
        let mut store = ReminderStore::new(10);
        store.set_tick(0);
        store.add("a".to_string(), 10).expect("added");
        store.add("b".to_string(), 10).expect("added");
        // Act
        store.clear();
        // Assert
        assert!(store.is_empty());
        assert_eq!(store.len(), 0);
    }

    #[test]
    fn ids_are_monotonic_and_deterministic() {
        // Arrange
        let mut store = ReminderStore::new(100);
        store.set_tick(0);
        // Act
        let a = store.add("a".to_string(), 10).expect("added");
        let b = store.add("b".to_string(), 10).expect("added");
        // Assert
        assert!(b > a);
    }

    #[test]
    fn zero_capacity_is_clamped_instead_of_panicking() {
        // Arrange
        let store = ReminderStore::new(0);
        // Assert
        assert_eq!(store.capacity(), 1);
    }

    // Helper used by tests only.
    trait PendingCount {
        fn pending_count(&self) -> usize;
    }
    impl PendingCount for ReminderStore {
        fn pending_count(&self) -> usize {
            self.items
                .iter()
                .filter(|r| r.status == ReminderStatus::Active && r.due_at > self.tick)
                .count()
        }
    }
}
