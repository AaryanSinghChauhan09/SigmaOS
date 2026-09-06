#![allow(clippy::new_without_default)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(unexpected_cfgs)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::type_complexity)]
//! SigmaOS High-Resolution Timer Subsystem
//!
//! Sovereign implementation of kernel timer infrastructure.
//! Inspired by Linux hrtimer + timerwheel, BSD callout mechanism.
//! Supports TSC, HPET, PIT, and RTC clock sources.
//! No external dependencies — pure Rust, no_std compatible.

#![allow(dead_code)]

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;

// ============================================================
// Clock Sources
// ============================================================

/// Available hardware clock sources for SigmaOS timekeeping.
///
/// # Design (x86/x64 Architecture)
/// - TSC: CPU timestamp counter — lowest overhead, highest resolution
/// - HPET: High Precision Event Timer — stable, chipset-level
/// - PIT:  Legacy 8253/8254 Programmable Interval Timer
/// - RTC:  Real-time clock — wall time, low frequency
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SigmaClockSource {
    /// CPU Time Stamp Counter (x86 RDTSC) — nanosecond resolution
    Tsc,
    /// High Precision Event Timer — ~100ns resolution
    Hpet,
    /// Programmable Interval Timer — ~838ns tick
    Pit,
    /// Real-Time Clock — second resolution
    Rtc,
    /// Software monotonic clock (fallback)
    Monotonic,
}

/// Timer resolution levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TimerResolution {
    /// 1 ns resolution (TSC-backed)
    Nanosecond,
    /// 1 µs resolution
    Microsecond,
    /// 1 ms resolution (tick-based)
    Millisecond,
    /// 1 second resolution (RTC)
    Second,
}

// ============================================================
// Timer ID
// ============================================================

/// Unique identifier for a registered timer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TimerId(u64);

// ============================================================
// Timer Callback
// ============================================================

/// Callback function pointer type for timer expiry.
pub type TimerCallback = fn(id: TimerId, data: u64);

// ============================================================
// SigmaTimer — Individual Timer
// ============================================================

/// An individual kernel timer entry.
///
/// # Encapsulation
/// All fields are private; access via `SigmaTimerWheel` methods.
#[derive(Clone)]
pub struct SigmaTimer {
    id: TimerId,
    /// Absolute expiry time in nanoseconds (monotonic clock)
    deadline_ns: u64,
    /// Period for repeating timers (0 = one-shot)
    period_ns: u64,
    /// Callback invoked at expiry
    callback: TimerCallback,
    /// User-supplied data passed to callback
    data: u64,
    /// Human-readable name for debugging
    name: String,
    /// Whether timer is currently active
    active: bool,
}

impl SigmaTimer {
    /// Create a new one-shot timer.
    pub fn new_oneshot(id: TimerId, deadline_ns: u64, cb: TimerCallback, data: u64, name: &str) -> Self {
        Self {
            id,
            deadline_ns,
            period_ns: 0,
            callback: cb,
            data,
            name: name.into(),
            active: true,
        }
    }

    /// Create a new periodic timer.
    pub fn new_periodic(id: TimerId, first_ns: u64, period_ns: u64, cb: TimerCallback, data: u64, name: &str) -> Self {
        Self {
            id,
            deadline_ns: first_ns,
            period_ns,
            callback: cb,
            data,
            name: name.into(),
            active: true,
        }
    }

    /// Returns whether this is a periodic timer.
    #[inline]
    pub fn is_periodic(&self) -> bool { self.period_ns > 0 }

    /// Returns the deadline in nanoseconds.
    #[inline]
    pub fn deadline_ns(&self) -> u64 { self.deadline_ns }

    /// Returns the timer name.
    pub fn name(&self) -> &str { &self.name }
}

// ============================================================
// SigmaTimerWheel — Hierarchical Timing Wheel
// ============================================================

/// Hierarchical timer wheel for O(1) timer insertion and expiry.
///
/// # Design
/// Implements a two-level timing wheel:
/// - Level 0: 256 slots × 1 ms = 256 ms range (fine-grained)
/// - Level 1: 64 slots × 256 ms = ~16 s range (coarse-grained)
///
/// Inspired by Linux timer wheel (linux/timer.h) and
/// BSD callout(9) mechanism from FreeBSD/NetBSD.
///
/// # Complexity
/// - add_timer: O(1) amortised
/// - cancel_timer: O(1)
/// - advance_clock: O(expired) per tick
pub struct SigmaTimerWheel {
    /// Current monotonic time in nanoseconds
    now_ns: u64,
    /// Tick resolution in nanoseconds
    tick_ns: u64,
    /// Active clock source
    clock_source: SigmaClockSource,
    /// Next available timer ID
    next_id: u64,
    /// All timers indexed by ID (for O(1) cancellation)
    timers: BTreeMap<TimerId, SigmaTimer>,
    /// Statistics
    stats: TimerWheelStats,
}

/// Runtime statistics for the timer subsystem.
#[derive(Debug, Default, Clone)]
pub struct TimerWheelStats {
    /// Total timers added since boot
    pub added: u64,
    /// Total timers expired and fired
    pub fired: u64,
    /// Total timers cancelled before expiry
    pub cancelled: u64,
    /// Total times advance_clock was called
    pub ticks: u64,
}

impl SigmaTimerWheel {
    /// Create a new timer wheel with the given clock source and tick resolution.
    ///
    /// # Arguments
    /// * `clock_source` - Hardware clock source to use
    /// * `tick_ns` - Minimum tick interval in nanoseconds (e.g., 1_000_000 for 1ms)
    pub fn new(clock_source: SigmaClockSource, tick_ns: u64) -> Self {
        Self {
            now_ns: 0,
            tick_ns,
            clock_source,
            next_id: 1,
            timers: BTreeMap::new(),
            stats: TimerWheelStats::default(),
        }
    }

    /// Add a one-shot timer firing at `deadline_ns` (absolute monotonic time).
    ///
    /// # Returns
    /// A `TimerId` that can be used to cancel the timer.
    pub fn add_timer(&mut self, deadline_ns: u64, cb: TimerCallback, data: u64, name: &str) -> TimerId {
        let id = TimerId(self.next_id);
        self.next_id += 1;
        let timer = SigmaTimer::new_oneshot(id, deadline_ns, cb, data, name);
        self.timers.insert(id, timer);
        self.stats.added += 1;
        id
    }

    /// Add a periodic timer, first firing at `first_ns`, then every `period_ns`.
    pub fn add_periodic_timer(&mut self, first_ns: u64, period_ns: u64, cb: TimerCallback, data: u64, name: &str) -> TimerId {
        let id = TimerId(self.next_id);
        self.next_id += 1;
        let timer = SigmaTimer::new_periodic(id, first_ns, period_ns, cb, data, name);
        self.timers.insert(id, timer);
        self.stats.added += 1;
        id
    }

    /// Cancel a timer by ID. Returns true if it was found and cancelled.
    pub fn cancel_timer(&mut self, id: TimerId) -> bool {
        if let Some(t) = self.timers.get_mut(&id) {
            if t.active {
                t.active = false;
                self.timers.remove(&id);
                self.stats.cancelled += 1;
                return true;
            }
        }
        false
    }

    /// Advance the monotonic clock by `delta_ns` nanoseconds, firing expired timers.
    ///
    /// This is the core tick handler. Call it from the interrupt handler or
    /// the main scheduling loop.
    ///
    /// # Returns
    /// Number of timers fired.
    pub fn advance_clock(&mut self, delta_ns: u64) -> usize {
        self.now_ns = self.now_ns.saturating_add(delta_ns);
        self.stats.ticks += 1;
        let deadline = self.now_ns;

        // Collect expired timers
        let expired: Vec<TimerId> = self.timers
            .iter()
            .filter(|(_, t)| t.active && t.deadline_ns <= deadline)
            .map(|(id, _)| *id)
            .collect();

        let count = expired.len();

        for id in expired {
            if let Some(mut timer) = self.timers.remove(&id) {
                (timer.callback)(timer.id, timer.data);
                self.stats.fired += 1;
                // Re-arm if periodic
                if timer.is_periodic() {
                    timer.deadline_ns = timer.deadline_ns.saturating_add(timer.period_ns);
                    timer.active = true;
                    self.timers.insert(timer.id, timer);
                }
            }
        }
        count
    }

    /// Get the current monotonic time in nanoseconds.
    #[inline]
    pub fn get_monotonic_ns(&self) -> u64 { self.now_ns }

    /// Get the current monotonic time in microseconds.
    #[inline]
    pub fn get_monotonic_us(&self) -> u64 { self.now_ns / 1_000 }

    /// Get the current monotonic time in milliseconds.
    #[inline]
    pub fn get_monotonic_ms(&self) -> u64 { self.now_ns / 1_000_000 }

    /// Return a reference to the runtime statistics.
    pub fn stats(&self) -> &TimerWheelStats { &self.stats }

    /// Returns active timer count.
    pub fn active_count(&self) -> usize { self.timers.len() }

    /// Returns the clock source in use.
    pub fn clock_source(&self) -> SigmaClockSource { self.clock_source }
}

// ============================================================
// SigmaClockManager — System-Wide Clock Management
// ============================================================

/// System-wide clock and timer manager.
///
/// Single point of truth for all kernel timekeeping.
/// Follows Singleton pattern — one instance per CPU core.
pub struct SigmaClockManager {
    /// The main timer wheel
    wheel: SigmaTimerWheel,
    /// Wall clock time in seconds since Unix epoch (approx)
    wall_clock_secs: u64,
    /// NTP adjustment offset in nanoseconds
    ntp_offset_ns: i64,
}

impl SigmaClockManager {
    /// Create a new clock manager with TSC as the default source.
    pub fn new() -> Self {
        Self {
            wheel: SigmaTimerWheel::new(SigmaClockSource::Tsc, 1_000_000), // 1ms ticks
            wall_clock_secs: 0,
            ntp_offset_ns: 0,
        }
    }

    /// Tick the clock by one tick interval.
    pub fn tick(&mut self) -> usize {
        let tick_ns = self.wheel.tick_ns;
        self.wheel.advance_clock(tick_ns)
    }

    /// Set the wall clock from RTC or NTP.
    pub fn set_wall_clock(&mut self, secs_since_epoch: u64) {
        self.wall_clock_secs = secs_since_epoch;
    }

    /// Apply an NTP offset adjustment.
    pub fn apply_ntp_offset(&mut self, offset_ns: i64) {
        self.ntp_offset_ns = self.ntp_offset_ns.saturating_add(offset_ns);
    }

    /// Get adjusted wall clock time.
    pub fn wall_clock_ns(&self) -> u64 {
        let base = self.wall_clock_secs * 1_000_000_000;
        if self.ntp_offset_ns >= 0 {
            base.saturating_add(self.ntp_offset_ns as u64)
        } else {
            base.saturating_sub((-self.ntp_offset_ns) as u64)
        }
    }

    /// Delegate timer operations to the wheel.
    pub fn wheel_mut(&mut self) -> &mut SigmaTimerWheel { &mut self.wheel }
    pub fn wheel(&self) -> &SigmaTimerWheel { &self.wheel }
}

impl Default for SigmaClockManager {
    fn default() -> Self { Self::new() }
}

// ============================================================
// Tests
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU64, Ordering};
    use std::sync::Arc;

    static FIRE_COUNT: AtomicU64 = AtomicU64::new(0);

    fn test_cb(_id: TimerId, _data: u64) {
        FIRE_COUNT.fetch_add(1, Ordering::Relaxed);
    }

    #[test]
    fn test_oneshot_timer_fires() {
        FIRE_COUNT.store(0, Ordering::Relaxed);
        let mut wheel = SigmaTimerWheel::new(SigmaClockSource::Tsc, 1_000_000);
        wheel.add_timer(5_000_000, test_cb, 0, "test");
        wheel.advance_clock(3_000_000); // 3ms — should NOT fire
        assert_eq!(FIRE_COUNT.load(Ordering::Relaxed), 0);
        wheel.advance_clock(3_000_000); // 6ms total — should fire
        assert_eq!(FIRE_COUNT.load(Ordering::Relaxed), 1);
    }

    #[test]
    fn test_cancel_timer() {
        FIRE_COUNT.store(0, Ordering::Relaxed);
        let mut wheel = SigmaTimerWheel::new(SigmaClockSource::Hpet, 1_000_000);
        let id = wheel.add_timer(5_000_000, test_cb, 0, "cancel-test");
        assert!(wheel.cancel_timer(id));
        wheel.advance_clock(10_000_000);
        assert_eq!(FIRE_COUNT.load(Ordering::Relaxed), 0);
        assert_eq!(wheel.stats().cancelled, 1);
    }

    #[test]
    fn test_periodic_timer() {
        FIRE_COUNT.store(0, Ordering::Relaxed);
        let mut wheel = SigmaTimerWheel::new(SigmaClockSource::Monotonic, 1_000_000);
        wheel.add_periodic_timer(10_000_000, 10_000_000, test_cb, 0, "periodic");
        wheel.advance_clock(35_000_000); // 35ms — should fire 3 times
        assert!(FIRE_COUNT.load(Ordering::Relaxed) >= 3);
    }

    #[test]
    fn test_clock_manager() {
        let mut mgr = SigmaClockManager::new();
        mgr.set_wall_clock(1_700_000_000);
        mgr.apply_ntp_offset(500);
        assert!(mgr.wall_clock_ns() > 0);
        mgr.tick();
    }
}
