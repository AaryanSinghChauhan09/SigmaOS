//! SigmaOS world clock widget.
//!
//! Displays multiple time zones with notifications and alarms.

#![warn(missing_docs)]

/// A configured time zone.
#[derive(Debug, Clone)]
pub struct TimeZone {
    /// IANA timezone identifier (e.g. "Europe/London").
    pub identifier: String,
    /// Friendly display label.
    pub label: String,
    /// Offset from UTC in seconds.
    pub utc_offset: i32,
}

/// A scheduled alarm.
#[derive(Debug, Clone)]
pub struct Alarm {
    /// Name of the alarm.
    pub name: String,
    /// Unix timestamp when the alarm fires.
    pub trigger_at: u64,
    /// Whether the alarm repeats daily.
    pub repeat: bool,
}

/// Trait for the world clock service.
pub trait WorldClock {
    /// Add a time zone to the display list.
    fn add_zone(&mut self, zone: TimeZone) -> Result<(), ClockError>;
    /// Schedule an alarm.
    fn add_alarm(&mut self, alarm: Alarm) -> Result<(), ClockError>;
    /// List configured time zones.
    fn zones(&self) -> Vec<TimeZone>;
}

/// Errors returned by the clock service.
#[derive(Debug)]
pub enum ClockError {
    /// The timezone identifier was invalid.
    InvalidTimeZone,
    /// The alarm time is in the past.
    AlarmInPast,
}

impl std::fmt::Display for ClockError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ClockError::InvalidTimeZone => write!(f, "invalid timezone"),
            ClockError::AlarmInPast => write!(f, "alarm in the past"),
        }
    }
}

impl std::error::Error for ClockError {}
