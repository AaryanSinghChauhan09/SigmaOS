// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// Linux-inspired time management for SigmaOS
// Zero-allocation, performance-optimized time operations

/// Clock types (Linux-style)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClockType {
    Realtime,
    Monotonic,
    Boottime,
    Tai,
    MonotonicRaw,
    MonotonicCoarse,
    BoottimeCoarse,
}

/// Time specification
#[derive(Debug, Clone, Copy)]
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
    
    pub const fn zero() -> Self {
        Self::new(0, 0)
    }
}

/// Time value
#[derive(Debug, Clone, Copy)]
pub struct Timeval {
    pub tv_sec: i64,
    pub tv_usec: i64,
}

impl Timeval {
    pub const fn new(sec: i64, usec: i64) -> Self {
        Self {
            tv_sec: sec,
            tv_usec: usec,
        }
    }
}

/// Clock trait
pub trait Clock {
    /// Get current time
    fn get_time(&self) -> Result<Timespec, TimeError>;
    
    /// Set current time
    fn set_time(&mut self, time: Timespec) -> Result<(), TimeError>;
    
    /// Get clock resolution
    fn get_resolution(&self) -> Result<Timespec, TimeError>;
    
    /// Get clock type
    fn clock_type(&self) -> ClockType;
}

/// Realtime clock
pub struct RealtimeClock {
    pub offset: Timespec,
}

impl RealtimeClock {
    pub const fn new() -> Self {
        Self {
            offset: Timespec::zero(),
        }
    }
}

impl Clock for RealtimeClock {
    fn get_time(&self) -> Result<Timespec, TimeError> {
        // Simplified - in real implementation would read hardware clock
        Ok(Timespec::zero())
    }
    
    fn set_time(&mut self, time: Timespec) -> Result<(), TimeError> {
        self.offset = time;
        Ok(())
    }
    
    fn get_resolution(&self) -> Result<Timespec, TimeError> {
        Ok(Timespec::new(0, 1))
    }
    
    fn clock_type(&self) -> ClockType {
        ClockType::Realtime
    }
}

/// Monotonic clock
pub struct MonotonicClock {
    pub start: Timespec,
}

impl MonotonicClock {
    pub const fn new() -> Self {
        Self {
            start: Timespec::zero(),
        }
    }
}

impl Clock for MonotonicClock {
    fn get_time(&self) -> Result<Timespec, TimeError> {
        // Simplified - in real implementation would read monotonic counter
        Ok(Timespec::zero())
    }
    
    fn set_time(&mut self, _time: Timespec) -> Result<(), TimeError> {
        Err(TimeError::OperationNotPermitted)
    }
    
    fn get_resolution(&self) -> Result<Timespec, TimeError> {
        Ok(Timespec::new(0, 1))
    }
    
    fn clock_type(&self) -> ClockType {
        ClockType::Monotonic
    }
}

/// Boottime clock
pub struct BoottimeClock {
    pub boot_time: Timespec,
}

impl BoottimeClock {
    pub const fn new() -> Self {
        Self {
            boot_time: Timespec::zero(),
        }
    }
}

impl Clock for BoottimeClock {
    fn get_time(&self) -> Result<Timespec, TimeError> {
        Ok(self.boot_time)
    }
    
    fn set_time(&mut self, _time: Timespec) -> Result<(), TimeError> {
        Err(TimeError::OperationNotPermitted)
    }
    
    fn get_resolution(&self) -> Result<Timespec, TimeError> {
        Ok(Timespec::new(0, 1))
    }
    
    fn clock_type(&self) -> ClockType {
        ClockType::Boottime
    }
}

/// Timer
pub struct Timer {
    pub id: u64,
    pub clock_type: ClockType,
    pub interval: Timespec,
    pub expiration: Timespec,
    pub armed: bool,
}

impl Timer {
    pub const fn new(id: u64, clock_type: ClockType) -> Self {
        Self {
            id,
            clock_type,
            interval: Timespec::zero(),
            expiration: Timespec::zero(),
            armed: false,
        }
    }
    
    pub fn arm(&mut self, expiration: Timespec) {
        self.expiration = expiration;
        self.armed = true;
    }
    
    pub fn disarm(&mut self) {
        self.armed = false;
    }
    
    pub fn is_expired(&self, current_time: Timespec) -> bool {
        self.armed && current_time.tv_sec >= self.expiration.tv_sec
    }
}

/// Timer manager
pub trait TimerManager {
    /// Create timer
    fn create_timer(&mut self, clock_type: ClockType) -> Result<u64, TimeError>;
    
    /// Delete timer
    fn delete_timer(&mut self, timer_id: u64) -> Result<(), TimeError>;
    
    /// Arm timer
    fn arm_timer(&mut self, timer_id: u64, expiration: Timespec) -> Result<(), TimeError>;
    
    /// Disarm timer
    fn disarm_timer(&mut self, timer_id: u64) -> Result<(), TimeError>;
    
    /// Get timer
    fn get_timer(&self, timer_id: u64) -> Option<&Timer>;
    
    /// Check expired timers
    fn check_expired(&mut self, current_time: Timespec) -> Vec<u64>;
}

/// Time error types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimeError {
    InvalidTime,
    ClockNotFound,
    TimerNotFound,
    OperationNotPermitted,
    TimerExpired,
    ResourceLimit,
    Other,
}

/// Timezone information
pub struct Timezone {
    pub name: String,
    pub offset: i32, // Offset from UTC in seconds
    pub dst_offset: i32, // DST offset in seconds
    pub dst_active: bool,
}

impl Timezone {
    pub const fn new(name: String, offset: i32) -> Self {
        Self {
            name,
            offset,
            dst_offset: 0,
            dst_active: false,
        }
    }
    
    pub fn utc_offset(&self) -> i32 {
        let mut offset = self.offset;
        if self.dst_active {
            offset += self.dst_offset;
        }
        offset
    }
}

/// Standard timezones
pub mod timezones {
    pub const UTC: &str = "UTC";
    pub const GMT: &str = "GMT";
    pub const EST: &str = "EST";
    pub const EDT: &str = "EDT";
    pub const CST: &str = "CST";
    pub const CDT: &str = "CDT";
    pub const MST: &str = "MST";
    pub const MDT: &str = "MDT";
    pub const PST: &str = "PST";
    pub const PDT: &str = "PDT";
}

/// NTP (Network Time Protocol) client
pub struct NtpClient {
    pub server: String,
    pub port: u16,
    pub poll_interval: u32,
    pub last_sync: Option<Timespec>,
}

impl NtpClient {
    pub const fn new(server: String) -> Self {
        Self {
            server,
            port: 123,
            poll_interval: 64,
            last_sync: None,
        }
    }
    
    pub fn sync(&mut self) -> Result<Timespec, TimeError> {
        // Simplified NTP sync - in real implementation would perform NTP protocol
        Ok(Timespec::zero())
    }
}

/// Time synchronization
pub struct TimeSync {
    pub ntp_clients: Vec<NtpClient>,
    pub current_time: Timespec,
    pub drift: i64,
}

impl TimeSync {
    pub const fn new() -> Self {
        Self {
            ntp_clients: Vec::new(),
            current_time: Timespec::zero(),
            drift: 0,
        }
    }
    
    pub fn add_ntp_server(&mut self, server: String) {
        self.ntp_clients.push(NtpClient::new(server));
    }
    
    pub fn sync_all(&mut self) -> Result<(), TimeError> {
        for client in &mut self.ntp_clients {
            if let Ok(time) = client.sync() {
                self.current_time = time;
                self.last_sync = Some(time);
            }
        }
        Ok(())
    }
}

/// Time utilities
pub mod time_utils {
    use super::Timespec;
    
    pub fn timespec_to_millis(ts: Timespec) -> i64 {
        ts.tv_sec * 1000 + ts.tv_nsec / 1_000_000
    }
    
    pub fn timespec_to_micros(ts: Timespec) -> i64 {
        ts.tv_sec * 1_000_000 + ts.tv_nsec / 1_000
    }
    
    pub fn timespec_to_nanos(ts: Timespec) -> i64 {
        ts.tv_sec * 1_000_000_000 + ts.tv_nsec
    }
    
    pub fn millis_to_timespec(millis: i64) -> Timespec {
        Timespec::new(millis / 1000, (millis % 1000) * 1_000_000)
    }
    
    pub fn micros_to_timespec(micros: i64) -> Timespec {
        Timespec::new(micros / 1_000_000, (micros % 1_000_000) * 1_000)
    }
    
    pub fn nanos_to_timespec(nanos: i64) -> Timespec {
        Timespec::new(nanos / 1_000_000_000, nanos % 1_000_000_000)
    }
}

/// Time constants
pub mod constants {
    pub const NSEC_PER_SEC: i64 = 1_000_000_000;
    pub const NSEC_PER_MSEC: i64 = 1_000_000;
    pub const NSEC_PER_USEC: i64 = 1_000;
    pub const USEC_PER_SEC: i64 = 1_000_000;
    pub const MSEC_PER_SEC: i64 = 1_000;
    pub const SEC_PER_MIN: i64 = 60;
    pub const SEC_PER_HOUR: i64 = 3600;
    pub const SEC_PER_DAY: i64 = 86400;
    pub const SEC_PER_WEEK: i64 = 604800;
}

/// Time-related file paths
pub mod paths {
    pub const ETC_LOCALTIME: &str = "/etc/localtime";
    pub const ETC_TIMEZONE: &str = "/etc/timezone";
    pub const ETC_ADJTIME: &str = "/etc/adjtime";
    pub const ETC_RTC: &str = "/etc/rtc";
    pub const DEV_RTC: &str = "/dev/rtc";
    pub const DEV_RTC0: &str = "/dev/rtc0";
    pub const SYS_CLASS_RTC: &str = "/sys/class/rtc";
}
