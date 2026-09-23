#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]

use std::boxed::Box;
use std::collections::BTreeMap;
use std::vec::Vec;
use core::sync::atomic::{AtomicI64, AtomicU64, AtomicUsize, Ordering};

pub type TimerID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClockSource {
    Rtc = 0,
    Tsc = 1,
    Hpet = 2,
    AcpiPm = 3,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClockId {
    Realtime = 0,
    Monotonic = 1,
    ProcessCpuTime = 2,
    ThreadCpuTime = 3,
    MonotonicRaw = 4,
    RealtimeCoarse = 5,
    MonotonicCoarse = 6,
    Boottime = 7,
    RealtimeAlarm = 8,
    BoottimeAlarm = 9,
    Tai = 11,
    UptimeBsd = 12,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimerError {
    Success = 0,
    NotFound = 1,
    InvalidTime = 2,
    ClockUnsupported = 3,
    TimerLimitExceeded = 4,
}

pub trait SystemClock {
    fn get_timestamp(&self) -> u64;
    fn get_nanoseconds(&self) -> u64;
    fn set_time(&mut self, timestamp: u64) -> Result<(), TimerError>;
}

#[repr(C)]
pub struct SimpleSystemClock {
    pub timestamp: AtomicUsize,
    pub source: AtomicUsize,
}

impl SimpleSystemClock {
    pub fn new(source: ClockSource) -> Self {
        SimpleSystemClock {
            timestamp: AtomicUsize::new(0),
            source: AtomicUsize::new(source as usize),
        }
    }
}

impl SystemClock for SimpleSystemClock {
    fn get_timestamp(&self) -> u64 {
        self.timestamp.load(Ordering::SeqCst) as u64
    }

    fn get_nanoseconds(&self) -> u64 {
        let base = self.timestamp.load(Ordering::SeqCst) as u64;
        base * 1_000_000_000
    }

    fn set_time(&mut self, timestamp: u64) -> Result<(), TimerError> {
        self.timestamp.store(timestamp as usize, Ordering::SeqCst);
        Ok(())
    }
}

pub trait Timer {
    fn id(&self) -> TimerID;
    fn is_expired(&self) -> bool;
    fn remaining_ms(&self) -> u64;
    fn reset(&mut self);
}

#[repr(C)]
pub struct SimpleTimer {
    pub id: TimerID,
    pub expiry: AtomicUsize,
    pub duration: AtomicUsize,
    pub created: AtomicUsize,
}

impl SimpleTimer {
    pub fn new(id: TimerID, duration_ms: u64) -> Self {
        let current = 1000000u64;
        SimpleTimer {
            id,
            expiry: AtomicUsize::new((current + duration_ms) as usize),
            duration: AtomicUsize::new(duration_ms as usize),
            created: AtomicUsize::new(current as usize),
        }
    }
}

impl Timer for SimpleTimer {
    fn id(&self) -> TimerID {
        self.id
    }

    fn is_expired(&self) -> bool {
        let current = 1000000usize;
        current >= self.expiry.load(Ordering::SeqCst)
    }

    fn remaining_ms(&self) -> u64 {
        let current = 1000000usize;
        let expiry = self.expiry.load(Ordering::SeqCst);
        if current >= expiry {
            0
        } else {
            (expiry - current) as u64
        }
    }

    fn reset(&mut self) {
        let current = 1000000usize;
        let duration = self.duration.load(Ordering::SeqCst);
        self.expiry.store(current + duration, Ordering::SeqCst);
        self.created.store(current, Ordering::SeqCst);
    }
}

pub trait TimerManager {
    fn create_timer(&mut self, duration_ms: u64) -> Result<TimerID, TimerError>;
    fn cancel_timer(&mut self, id: TimerID) -> Result<(), TimerError>;
    fn get_expired_timers(&self) -> Vec<TimerID>;
    fn get_timer(&self, id: TimerID) -> Option<&dyn Timer>;
}

pub struct SimpleTimerManager {
    pub timers: Vec<Option<Box<dyn Timer>>>,
    pub next_id: AtomicUsize,
}

impl SimpleTimerManager {
    pub fn new() -> Self {
        SimpleTimerManager {
            timers: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl TimerManager for SimpleTimerManager {
    fn create_timer(&mut self, duration_ms: u64) -> Result<TimerID, TimerError> {
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        let timer = SimpleTimer::new(id, duration_ms);
        self.timers.push(Some(Box::new(timer)));
        Ok(id)
    }

    fn cancel_timer(&mut self, id: TimerID) -> Result<(), TimerError> {
        for timer_option in &mut self.timers {
            if let Some(ref timer) = *timer_option {
                if timer.id() == id {
                    *timer_option = None;
                    return Ok(());
                }
            }
        }
        Err(TimerError::NotFound)
    }

    fn get_expired_timers(&self) -> Vec<TimerID> {
        let mut expired = Vec::new();
        for timer_option in &self.timers {
            if let Some(ref timer) = *timer_option {
                if timer.is_expired() {
                    expired.push(timer.id());
                }
            }
        }
        expired
    }

    fn get_timer(&self, id: TimerID) -> Option<&dyn Timer> {
        for timer_option in &self.timers {
            if let Some(ref timer) = *timer_option {
                if timer.id() == id {
                    return Some(timer.as_ref());
                }
            }
        }
        None
    }
}

pub trait Alarm {
    fn set_alarm(&mut self, timestamp: u64, callback: fn()) -> Result<TimerID, TimerError>;
    fn cancel_alarm(&mut self, id: TimerID) -> Result<(), TimerError>;
    fn check_alarms(&mut self) -> Vec<fn()>;
}

pub struct SimpleAlarm {
    pub alarms: Vec<(TimerID, u64, fn())>,
    pub next_id: AtomicUsize,
}

impl SimpleAlarm {
    pub fn new() -> Self {
        SimpleAlarm {
            alarms: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl Alarm for SimpleAlarm {
    fn set_alarm(&mut self, timestamp: u64, callback: fn()) -> Result<TimerID, TimerError> {
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        self.alarms.push((id, timestamp, callback));
        Ok(id)
    }

    fn cancel_alarm(&mut self, id: TimerID) -> Result<(), TimerError> {
        if let Some(pos) = self.alarms.iter().position(|a| a.0 == id) {
            self.alarms.remove(pos);
            Ok(())
        } else {
            Err(TimerError::NotFound)
        }
    }

    fn check_alarms(&mut self) -> Vec<fn()> {
        let mut triggered = Vec::new();
        let current = 1000000u64;

        let mut i = 0;
        while i < self.alarms.len() {
            if self.alarms[i].1 <= current {
                triggered.push(self.alarms[i].2);
                self.alarms.remove(i);
            } else {
                i += 1;
            }
        }

        triggered
    }
}

// ============================================================================
// LINUX & BSD INSPIRED ADVANCED CLOCKS & TIMERS ENGINE
// ============================================================================

/// High-resolution timespec representation matching POSIX struct timespec
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Timespec {
    pub tv_sec: i64,
    pub tv_nsec: i64,
}

impl Timespec {
    pub const fn new(sec: i64, nsec: i64) -> Self {
        Self {
            tv_sec: sec,
            tv_nsec: nsec,
        }
    }

    pub fn to_nanos(&self) -> u64 {
        (self.tv_sec as u64)
            .wrapping_mul(1_000_000_000)
            .wrapping_add(self.tv_nsec as u64)
    }

    pub fn from_nanos(nanos: u64) -> Self {
        Self {
            tv_sec: (nanos / 1_000_000_000) as i64,
            tv_nsec: (nanos % 1_000_000_000) as i64,
        }
    }
}

/// Linux/BSD high-resolution timer wheel (hrtimer / FreeBSD callwheel)
#[derive(Debug, Clone)]
pub struct HrtimerEntry {
    pub timer_id: u64,
    pub clock_id: ClockId,
    pub expire_nanos: u64,
    pub interval_nanos: u64,
    pub is_periodic: bool,
    pub callback_id: u32,
}

pub struct SovereignHrtimerWheel {
    pub current_nanos: AtomicU64,
    pub active_timers: BTreeMap<u64, Vec<HrtimerEntry>>, // expire_nanos -> entries
    pub next_timer_id: AtomicU64,
}

impl SovereignHrtimerWheel {
    pub fn new(initial_nanos: u64) -> Self {
        Self {
            current_nanos: AtomicU64::new(initial_nanos),
            active_timers: BTreeMap::new(),
            next_timer_id: AtomicU64::new(1),
        }
    }

    pub fn add_timer(
        &mut self,
        clock_id: ClockId,
        delay_nanos: u64,
        interval_nanos: u64,
        callback_id: u32,
    ) -> u64 {
        let id = self.next_timer_id.fetch_add(1, Ordering::SeqCst);
        let now = self.current_nanos.load(Ordering::SeqCst);
        let expire_nanos = now + delay_nanos;

        let entry = HrtimerEntry {
            timer_id: id,
            clock_id,
            expire_nanos,
            interval_nanos,
            is_periodic: interval_nanos > 0,
            callback_id,
        };

        self.active_timers
            .entry(expire_nanos)
            .or_insert_with(Vec::new)
            .push(entry);

        id
    }

    pub fn cancel_timer(&mut self, timer_id: u64) -> bool {
        let mut found = false;
        for entries in self.active_timers.values_mut() {
            if let Some(pos) = entries.iter().position(|e| e.timer_id == timer_id) {
                entries.remove(pos);
                found = true;
                break;
            }
        }
        self.active_timers.retain(|_, entries| !entries.is_empty());
        found
    }

    pub fn advance_time(&mut self, advance_nanos: u64) -> Vec<HrtimerEntry> {
        let new_now = self.current_nanos.fetch_add(advance_nanos, Ordering::SeqCst) + advance_nanos;
        let mut expired = Vec::new();

        // Split off all keys <= new_now
        let expired_keys: Vec<u64> = self
            .active_timers
            .keys()
            .cloned()
            .take_while(|k| *k <= new_now)
            .collect();

        for k in expired_keys {
            if let Some(entries) = self.active_timers.remove(&k) {
                for entry in entries {
                    expired.push(entry.clone());
                    // Re-arm if periodic
                    if entry.is_periodic {
                        let next_expire = k + entry.interval_nanos;
                        let mut rearmed = entry;
                        rearmed.expire_nanos = next_expire;
                        self.active_timers
                            .entry(next_expire)
                            .or_insert_with(Vec::new)
                            .push(rearmed);
                    }
                }
            }
        }

        expired
    }
}

/// NTP / PTP Precision Clock Synchronization & Drift Engine
pub struct NtpPtpTimeSyncEngine {
    pub base_realtime_nanos: AtomicU64,
    pub drift_parts_per_billion: AtomicI64, // Frequency adjustment (+/- ppb)
    pub last_sync_nanos: AtomicU64,
}

impl NtpPtpTimeSyncEngine {
    pub fn new(initial_realtime_nanos: u64) -> Self {
        Self {
            base_realtime_nanos: AtomicU64::new(initial_realtime_nanos),
            drift_parts_per_billion: AtomicI64::new(0),
            last_sync_nanos: AtomicU64::new(initial_realtime_nanos),
        }
    }

    pub fn adjust_frequency_ppb(&self, ppb: i64) {
        self.drift_parts_per_billion.store(ppb, Ordering::SeqCst);
    }

    pub fn get_time_with_drift(&self, elapsed_monotonic_nanos: u64) -> u64 {
        let base = self.base_realtime_nanos.load(Ordering::SeqCst);
        let ppb = self.drift_parts_per_billion.load(Ordering::SeqCst);
        let drift_offset = (elapsed_monotonic_nanos as i128 * ppb as i128 / 1_000_000_000i128) as i64;
        (base as i64 + elapsed_monotonic_nanos as i64 + drift_offset) as u64
    }

    pub fn update_realtime_offset(&self, offset_nanos: i64) {
        let current = self.base_realtime_nanos.load(Ordering::SeqCst);
        let new_val = (current as i64 + offset_nanos).max(0) as u64;
        self.base_realtime_nanos.store(new_val, Ordering::SeqCst);
    }
}

// ============================================================================
// UNIT TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_system_clock() {
        let mut clock = SimpleSystemClock::new(ClockSource::Tsc);
        assert_eq!(clock.get_timestamp(), 0);
        assert!(clock.set_time(500).is_ok());
        assert_eq!(clock.get_timestamp(), 500);
        assert_eq!(clock.get_nanoseconds(), 500_000_000_000);
    }

    #[test]
    fn test_simple_timer_manager() {
        let mut manager = SimpleTimerManager::new();
        let t1 = manager.create_timer(100).unwrap();
        assert!(manager.get_timer(t1).is_some());
        assert!(manager.cancel_timer(t1).is_ok());
        assert!(manager.get_timer(t1).is_none());
    }

    #[test]
    fn test_hrtimer_wheel_expiration_and_periodic() {
        let mut wheel = SovereignHrtimerWheel::new(1000);
        let t_oneshot = wheel.add_timer(ClockId::Monotonic, 500, 0, 101);
        let _t_periodic = wheel.add_timer(ClockId::Monotonic, 200, 200, 102);

        // Advance 250ns -> periodic timer triggers
        let expired1 = wheel.advance_time(250);
        assert_eq!(expired1.len(), 1);
        assert_eq!(expired1[0].callback_id, 102);

        // Advance another 300ns (total 1550ns) -> oneshot and periodic trigger
        let expired2 = wheel.advance_time(300);
        assert_eq!(expired2.len(), 2);

        // Oneshot timer cancelled
        assert!(!wheel.cancel_timer(t_oneshot));
    }

    #[test]
    fn test_ntp_ptp_drift_compensation() {
        let sync_engine = NtpPtpTimeSyncEngine::new(1_000_000_000);
        sync_engine.adjust_frequency_ppb(100_000_000); // +10% frequency drift

        let adjusted = sync_engine.get_time_with_drift(1_000_000_000);
        assert_eq!(adjusted, 2_100_000_000); // 1s base + 1s monotonic + 0.1s drift
    }
}
