#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]

// SigmaOS Custom Time Library
// Reduces dependency on predefined time functions

// (no_std only applicable at crate root - removed)

/// Simple time structure
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Time {
    pub hours: u8,
    pub minutes: u8,
    pub seconds: u8,
}

impl Time {
    pub fn new(hours: u8, minutes: u8, seconds: u8) -> Self {
        Time {
            hours: hours % 24,
            minutes: minutes % 60,
            seconds: seconds % 60,
        }
    }

    pub fn to_seconds(&self) -> u32 {
        (self.hours as u32 * 3600) + (self.minutes as u32 * 60) + self.seconds as u32
    }

    pub fn from_seconds(total_seconds: u32) -> Self {
        let hours = (total_seconds / 3600) as u8 % 24;
        let minutes = ((total_seconds % 3600) / 60) as u8;
        let seconds = (total_seconds % 60) as u8;
        Time::new(hours, minutes, seconds)
    }

    pub fn add_seconds(&self, seconds: u32) -> Self {
        let total = self.to_seconds() + seconds;
        Self::from_seconds(total)
    }
}

/// Simple date structure
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Date {
    pub year: u16,
    pub month: u8,
    pub day: u8,
}

impl Date {
    pub fn new(year: u16, month: u8, day: u8) -> Self {
        Date {
            year,
            month: month.min(12),
            day: day.min(31),
        }
    }

    pub fn days_in_month(month: u8, year: u16) -> u8 {
        match month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 => {
                if Self::is_leap_year(year) {
                    29
                } else {
                    28
                }
            }
            _ => 30,
        }
    }

    pub fn is_leap_year(year: u16) -> bool {
        (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
    }

    pub fn add_days(&self, days: u32) -> Self {
        let mut day = self.day as u32 + days;
        let mut month = self.month;
        let mut year = self.year;

        while day > Self::days_in_month(month, year) as u32 {
            day -= Self::days_in_month(month, year) as u32;
            month += 1;
            if month > 12 {
                month = 1;
                year += 1;
            }
        }

        Date::new(year, month, day as u8)
    }
}

/// Simple timestamp structure
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Timestamp {
    pub date: Date,
    pub time: Time,
}

impl Timestamp {
    pub fn new(date: Date, time: Time) -> Self {
        Timestamp { date, time }
    }

    pub fn now() -> Self {
        // In a real implementation, this would get the actual time
        Timestamp::new(Date::new(2024, 8, 1), Time::new(0, 0, 0))
    }
}

/// Custom sleep function (placeholder)
pub fn sleep_ms(milliseconds: u32) {
    // In a real implementation, this would use CPU sleep instructions
    // For now, this is a placeholder
    let _ = milliseconds;
}

/// Custom uptime counter
pub fn uptime_ms() -> u64 {
    // In a real implementation, this would return actual uptime
    0
}

/// Custom monotonic clock
pub fn monotonic_ms() -> u64 {
    // In a real implementation, this would return monotonic time
    0
}

/// Custom Duration - replaces std::time::Duration
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Duration {
    pub secs: u64,
    pub nanos: u32,
}

impl Duration {
    pub const fn new(secs: u64, nanos: u32) -> Self {
        Duration { secs, nanos: nanos % 1_000_000_000 }
    }

    pub const fn from_secs(secs: u64) -> Self {
        Duration { secs, nanos: 0 }
    }

    pub const fn from_millis(millis: u64) -> Self {
        Duration {
            secs: millis / 1_000,
            nanos: ((millis % 1_000) * 1_000_000) as u32,
        }
    }

    pub const fn from_micros(micros: u64) -> Self {
        Duration {
            secs: micros / 1_000_000,
            nanos: ((micros % 1_000_000) * 1_000) as u32,
        }

    }

    pub const fn from_nanos(nanos: u64) -> Self {
        Duration {
            secs: nanos / 1_000_000_000,
            nanos: (nanos % 1_000_000_000) as u32,
        }
    }

    pub const fn as_secs(&self) -> u64 {
        self.secs
    }

    pub const fn as_millis(&self) -> u64 {
        self.secs * 1_000 + (self.nanos / 1_000_000) as u64
    }

    pub const fn as_micros(&self) -> u64 {
        self.secs * 1_000_000 + (self.nanos / 1_000) as u64
    }

    pub const fn as_nanos(&self) -> u64 {
        self.secs * 1_000_000_000 + self.nanos as u64
    }

    pub const fn zero() -> Self {
        Duration { secs: 0, nanos: 0 }
    }

    pub fn checked_add(self, rhs: Duration) -> Option<Duration> {
        let mut secs = self.secs.checked_add(rhs.secs)?;
        let mut nanos = self.nanos + rhs.nanos;
        if nanos >= 1_000_000_000 {
            nanos -= 1_000_000_000;
            secs = secs.checked_add(1)?;
        }
        Some(Duration { secs, nanos })
    }

    pub fn checked_sub(self, rhs: Duration) -> Option<Duration> {
        let mut secs = self.secs.checked_sub(rhs.secs)?;
        let nanos = if self.nanos >= rhs.nanos {
            self.nanos - rhs.nanos
        } else {
            secs = secs.checked_sub(1)?;
            self.nanos + 1_000_000_000 - rhs.nanos
        };
        Some(Duration { secs, nanos })
    }
}

impl Default for Duration {
    fn default() -> Self {
        Duration::zero()
    }
}

/// Custom Instant - replaces std::time::Instant
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Instant {
    pub nanos_since_epoch: u64,
}

impl Instant {
    pub fn now() -> Self {
        // In a real implementation, this would use hardware timers
        // For now, use a placeholder based on uptime
        Instant {
            nanos_since_epoch: monotonic_ms() * 1_000_000,
        }
    }

    pub fn duration_since(&self, earlier: Instant) -> Duration {
        Duration::from_nanos(self.nanos_since_epoch.saturating_sub(earlier.nanos_since_epoch))
    }

    pub fn elapsed(&self) -> Duration {
        Self::now().duration_since(*self)
    }

    pub fn checked_add(&self, duration: Duration) -> Option<Instant> {
        let new_nanos = self.nanos_since_epoch.checked_add(duration.as_nanos())?;
        Some(Instant {
            nanos_since_epoch: new_nanos,
        })
    }

    pub fn checked_sub(&self, duration: Duration) -> Option<Instant> {
        let new_nanos = self.nanos_since_epoch.checked_sub(duration.as_nanos())?;
        Some(Instant {
            nanos_since_epoch: new_nanos,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_time() {
        let time = Time::new(12, 30, 45);
        assert_eq!(time.to_seconds(), 45045);

        let time2 = Time::from_seconds(45045);
        assert_eq!(time2.hours, 12);
        assert_eq!(time2.minutes, 30);
        assert_eq!(time2.seconds, 45);
    }

    #[test]
    fn test_time_add_seconds() {
        let time = Time::new(23, 59, 30);
        let time2 = time.add_seconds(45);
        assert_eq!(time2.hours, 0);
        assert_eq!(time2.minutes, 0);
        assert_eq!(time2.seconds, 15);
    }

    #[test]
    fn test_date() {
        let date = Date::new(2024, 8, 1);
        assert_eq!(date.year, 2024);
        assert_eq!(date.month, 8);
        assert_eq!(date.day, 1);
    }

    #[test]
    fn test_leap_year() {
        assert!(Date::is_leap_year(2024));
        assert!(!Date::is_leap_year(2023));
        assert!(Date::is_leap_year(2000));
        assert!(!Date::is_leap_year(1900));
    }

    #[test]
    fn test_days_in_month() {
        assert_eq!(Date::days_in_month(1, 2024), 31);
        assert_eq!(Date::days_in_month(2, 2024), 29);
        assert_eq!(Date::days_in_month(2, 2023), 28);
        assert_eq!(Date::days_in_month(4, 2024), 30);
    }

    #[test]
    fn test_date_add_days() {
        let date = Date::new(2024, 1, 31);
        let date2 = date.add_days(1);
        assert_eq!(date2.month, 2);
        assert_eq!(date2.day, 1);
    }

    #[test]
    fn test_duration_creation() {
        let dur = Duration::new(5, 500_000_000);
        assert_eq!(dur.secs, 5);
        assert_eq!(dur.nanos, 500_000_000);

        let dur_secs = Duration::from_secs(10);
        assert_eq!(dur_secs.as_secs(), 10);

        let dur_millis = Duration::from_millis(1500);
        assert_eq!(dur_millis.as_secs(), 1);
        assert_eq!(dur_millis.as_millis(), 1500);
    }

    #[test]
    fn test_duration_arithmetic() {
        let dur1 = Duration::from_secs(5);
        let dur2 = Duration::from_secs(3);
        let sum = dur1.checked_add(dur2).unwrap();
        assert_eq!(sum.as_secs(), 8);

        let diff = dur1.checked_sub(dur2).unwrap();
        assert_eq!(diff.as_secs(), 2);
    }

    #[test]
    fn test_instant() {
        let now = Instant::now();
        let later = now.checked_add(Duration::from_secs(1)).unwrap();
        let elapsed = later.duration_since(now);
        assert_eq!(elapsed.as_secs(), 1);
    }

    #[test]
    fn test_duration_conversions() {
        let dur = Duration::from_nanos(1_500_000_000);
        assert_eq!(dur.as_secs(), 1);
        assert_eq!(dur.as_millis(), 1500);
        assert_eq!(dur.as_micros(), 1_500_000);
        assert_eq!(dur.as_nanos(), 1_500_000_000);
    }
}
