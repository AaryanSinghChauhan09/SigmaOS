//! # Hierarchical Timer Wheel
//!
//! Multi-level timer wheel inspired by Linux kernel timer implementation.
//! Provides efficient O(1) timer scheduling and expiration.

#![no_std]

extern crate alloc;
use alloc::boxed::Box;
use alloc::vec::Vec;
use core::cmp::Ordering;

/// Timer wheel configuration (inspired by Linux HZ=1000 jiffies)
pub const HZ: usize = 1000; // Ticks per second
pub const WHEEL_SIZE: usize = 256; // Slots per wheel level
pub const NUM_LEVELS: usize = 4; // 4-level hierarchical wheel

/// Timer callback type
pub type TimerCallback = Box<dyn FnMut() + Send>;

/// Timer entry
pub struct Timer {
    /// Unique timer ID
    pub id: u64,
    /// Absolute expiration time (in ticks)
    pub expires: u64,
    /// Timer callback
    pub callback: Option<TimerCallback>,
    /// Periodic timer interval (0 = one-shot)
    pub interval: u64,
    /// Next timer in bucket
    pub next: Option<Box<Timer>>,
}

impl Timer {
    pub fn new(id: u64, expires: u64, callback: TimerCallback) -> Self {
        Self {
            id,
            expires,
            callback: Some(callback),
            interval: 0,
            next: None,
        }
    }

    pub fn new_periodic(id: u64, expires: u64, interval: u64, callback: TimerCallback) -> Self {
        Self {
            id,
            expires,
            callback: Some(callback),
            interval,
            next: None,
        }
    }

    /// Check if timer is periodic
    pub const fn is_periodic(&self) -> bool {
        self.interval > 0
    }

    /// Fire timer callback
    pub fn fire(&mut self) {
        if let Some(mut callback) = self.callback.take() {
            callback();
            // Restore callback if periodic
            if self.is_periodic() {
                self.callback = Some(callback);
            }
        }
    }
}

/// Timer bucket (linked list of timers)
pub struct TimerBucket {
    head: Option<Box<Timer>>,
}

impl TimerBucket {
    pub const fn new() -> Self {
        Self { head: None }
    }

    /// Add timer to bucket
    pub fn add(&mut self, timer: Box<Timer>) {
        let mut timer = timer;
        timer.next = self.head.take();
        self.head = Some(timer);
    }

    /// Remove and return all timers with expiration <= now
    pub fn collect_expired(&mut self, now: u64) -> Vec<Box<Timer>> {
        let mut expired = Vec::new();
        let mut remaining = None;
        let mut current = self.head.take();

        while let Some(mut timer) = current {
            current = timer.next.take();
            if timer.expires <= now {
                expired.push(timer);
            } else {
                timer.next = remaining;
                remaining = Some(timer);
            }
        }

        self.head = remaining;
        expired
    }

    /// Check if bucket is empty
    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }
}

/// Timer wheel level
pub struct TimerWheelLevel {
    /// Buckets for this level
    buckets: [TimerBucket; WHEEL_SIZE],
    /// Granularity (ticks per bucket)
    granularity: u64,
    /// Current index in this level
    index: usize,
}

impl TimerWheelLevel {
    pub fn new(granularity: u64) -> Self {
        const BUCKET: TimerBucket = TimerBucket::new();
        Self {
            buckets: [BUCKET; WHEEL_SIZE],
            granularity,
            index: 0,
        }
    }

    /// Calculate bucket index for expiration time
    fn bucket_index(&self, expires: u64, now: u64) -> usize {
        let delta = expires.saturating_sub(now);
        let scaled = delta / self.granularity;
        (scaled as usize % WHEEL_SIZE)
    }

    /// Add timer to appropriate bucket
    pub fn add_timer(&mut self, timer: Box<Timer>, now: u64) {
        let index = self.bucket_index(timer.expires, now);
        self.buckets[index].add(timer);
    }

    /// Advance level and collect expired timers
    pub fn advance(&mut self, now: u64) -> Vec<Box<Timer>> {
        let expired = self.buckets[self.index].collect_expired(now);
        self.index = (self.index + 1) % WHEEL_SIZE;
        expired
    }

    /// Check if level needs cascading
    pub fn needs_cascade(&self) -> bool {
        self.index == 0
    }
}

/// Hierarchical timer wheel
pub struct TimerWheel {
    /// Current time (in ticks)
    current_tick: u64,
    /// Next timer ID
    next_id: u64,
    /// Timer levels (4 levels with increasing granularity)
    levels: [TimerWheelLevel; NUM_LEVELS],
}

impl TimerWheel {
    pub fn new() -> Self {
        Self {
            current_tick: 0,
            next_id: 1,
            levels: [
                TimerWheelLevel::new(1),    // Level 0: 1 tick granularity (0-255 ticks)
                TimerWheelLevel::new(256),  // Level 1: 256 tick granularity (256-65K ticks)
                TimerWheelLevel::new(65536), // Level 2: 64K tick granularity (64K-16M ticks)
                TimerWheelLevel::new(16777216), // Level 3: 16M tick granularity (16M+ ticks)
            ],
        }
    }

    /// Allocate new timer ID
    fn allocate_id(&mut self) -> u64 {
        let id = self.next_id;
        self.next_id = self.next_id.wrapping_add(1);
        id
    }

    /// Determine which level should handle this timer
    fn select_level(&self, expires: u64) -> usize {
        let delta = expires.saturating_sub(self.current_tick);
        if delta < 256 {
            0
        } else if delta < 65536 {
            1
        } else if delta < 16777216 {
            2
        } else {
            3
        }
    }

    /// Add a one-shot timer
    pub fn add_timer(&mut self, expires_in: u64, callback: TimerCallback) -> u64 {
        let id = self.allocate_id();
        let expires = self.current_tick + expires_in;
        let timer = Timer::new(id, expires, callback);
        let level = self.select_level(expires);
        self.levels[level].add_timer(Box::new(timer), self.current_tick);
        id
    }

    /// Add a periodic timer
    pub fn add_periodic_timer(&mut self, interval: u64, callback: TimerCallback) -> u64 {
        let id = self.allocate_id();
        let expires = self.current_tick + interval;
        let timer = Timer::new_periodic(id, expires, interval, callback);
        let level = self.select_level(expires);
        self.levels[level].add_timer(Box::new(timer), self.current_tick);
        id
    }

    /// Advance wheel by one tick and fire expired timers
    pub fn tick(&mut self) {
        self.current_tick += 1;

        // Collect expired timers from level 0
        let mut expired = self.levels[0].advance(self.current_tick);

        // Cascade from higher levels if needed
        for level_idx in 0..NUM_LEVELS - 1 {
            if self.levels[level_idx].needs_cascade() {
                let cascaded = self.levels[level_idx + 1].advance(self.current_tick);
                // Re-insert cascaded timers into appropriate levels
                for timer in cascaded {
                    let level = self.select_level(timer.expires);
                    self.levels[level].add_timer(timer, self.current_tick);
                }
            } else {
                break;
            }
        }

        // Fire expired timers
        for mut timer in expired {
            timer.fire();

            // Re-schedule periodic timers
            if timer.is_periodic() {
                timer.expires = self.current_tick + timer.interval;
                let level = self.select_level(timer.expires);
                self.levels[level].add_timer(timer, self.current_tick);
            }
        }
    }

    /// Get current tick count
    pub fn current_tick(&self) -> u64 {
        self.current_tick
    }

    /// Convert ticks to milliseconds (assuming HZ=1000)
    pub fn ticks_to_ms(ticks: u64) -> u64 {
        ticks
    }

    /// Convert milliseconds to ticks
    pub fn ms_to_ticks(ms: u64) -> u64 {
        ms
    }
}

/// Timer manager with sorted list (fallback for very long timers)
pub struct TimerManager {
    wheel: TimerWheel,
}

impl TimerManager {
    pub const fn new() -> Self {
        Self {
            wheel: TimerWheel {
                current_tick: 0,
                next_id: 1,
                levels: [
                    TimerWheelLevel {
                        buckets: [TimerBucket { head: None }; WHEEL_SIZE],
                        granularity: 1,
                        index: 0,
                    },
                    TimerWheelLevel {
                        buckets: [TimerBucket { head: None }; WHEEL_SIZE],
                        granularity: 256,
                        index: 0,
                    },
                    TimerWheelLevel {
                        buckets: [TimerBucket { head: None }; WHEEL_SIZE],
                        granularity: 65536,
                        index: 0,
                    },
                    TimerWheelLevel {
                        buckets: [TimerBucket { head: None }; WHEEL_SIZE],
                        granularity: 16777216,
                        index: 0,
                    },
                ],
            },
        }
    }

    /// Schedule one-shot timer (relative time in ticks)
    pub fn schedule(&mut self, delay_ticks: u64, callback: TimerCallback) -> u64 {
        self.wheel.add_timer(delay_ticks, callback)
    }

    /// Schedule periodic timer
    pub fn schedule_periodic(&mut self, interval_ticks: u64, callback: TimerCallback) -> u64 {
        self.wheel.add_periodic_timer(interval_ticks, callback)
    }

    /// Process one timer tick
    pub fn tick(&mut self) {
        self.wheel.tick();
    }

    /// Get current time
    pub fn now(&self) -> u64 {
        self.wheel.current_tick()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timer_wheel_creation() {
        let wheel = TimerWheel::new();
        assert_eq!(wheel.current_tick(), 0);
    }

    #[test]
    fn test_timer_level_selection() {
        let wheel = TimerWheel::new();
        assert_eq!(wheel.select_level(100), 0); // < 256
        assert_eq!(wheel.select_level(300), 1); // 256-65K
        assert_eq!(wheel.select_level(70000), 2); // 64K-16M
        assert_eq!(wheel.select_level(20000000), 3); // > 16M
    }

    #[test]
    fn test_timer_bucket() {
        let mut bucket = TimerBucket::new();
        let timer = Timer::new(1, 100, Box::new(|| {}));
        bucket.add(Box::new(timer));
        assert!(!bucket.is_empty());

        let expired = bucket.collect_expired(100);
        assert_eq!(expired.len(), 1);
    }

    #[test]
    fn test_periodic_timer() {
        let timer = Timer::new_periodic(1, 100, 50, Box::new(|| {}));
        assert!(timer.is_periodic());
        assert_eq!(timer.interval, 50);
    }
}
