// SPDX-License-Identifier: MIT
// SigmaOS - Timekeeping Component
// Inspired by Linux timekeeping (kernel/time) and BSD timekeeping
// Provides system time, timers, NTP synchronization, and clock sources

#![no_std]

extern crate alloc;
use alloc::collections::BTreeMap;
use core::sync::atomic::{AtomicU64, AtomicBool, Ordering};

/// Clock source type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClockSource {
    TSC, // Time Stamp Counter
    HPET, // High Precision Event Timer
    ACPI_PM, // ACPI Power Management Timer
    PIT, // Programmable Interval Timer
}

/// Clock quality
#[derive(Debug, Clone, Copy)]
pub struct ClockQuality {
    pub resolution: u64, // nanoseconds
    pub accuracy: u64, // parts per billion
    pub stability: u64, // parts per billion
}

/// Timekeeper state
pub struct Timekeeper {
    pub clock_source: ClockSource,
    pub clock_quality: ClockQuality,
    pub nanoseconds_since_boot: AtomicU64,
    pub wall_clock_offset: AtomicU64,
    pub ntp_enabled: AtomicBool,
    pub ntp_servers: Vec<String>,
    pub ntp_sync_interval: u64,
    pub last_ntp_sync: AtomicU64,
}

impl Timekeeper {
    pub fn new() -> Self {
        Self {
            clock_source: ClockSource::TSC,
            clock_quality: ClockQuality {
                resolution: 1, // 1 nanosecond
                accuracy: 1000, // 1000 ppb
                stability: 1000, // 1000 ppb
            },
            nanoseconds_since_boot: AtomicU64::new(0),
            wall_clock_offset: AtomicU64::new(0),
            ntp_enabled: AtomicBool::new(false),
            ntp_servers: Vec::new(),
            ntp_sync_interval: 1000, // 1000 seconds
            last_ntp_sync: AtomicU64::new(0),
        }
    }
    
    /// Get current time in nanoseconds since boot
    pub fn get_time(&self) -> u64 {
        self.nanoseconds_since_boot.load(Ordering::Relaxed)
    }
    
    /// Get wall clock time (Unix timestamp in nanoseconds)
    pub fn get_wall_clock(&self) -> u64 {
        let boot_time = self.nanoseconds_since_boot.load(Ordering::Relaxed);
        let offset = self.wall_clock_offset.load(Ordering::Relaxed);
        boot_time + offset
    }
    
    /// Set wall clock offset
    pub fn set_wall_clock_offset(&self, offset: u64) {
        self.wall_clock_offset.store(offset, Ordering::Relaxed);
    }
    
    /// Add time (called by interrupt handler)
    pub fn add_time(&self, nanoseconds: u64) {
        self.nanoseconds_since_boot.fetch_add(nanoseconds, Ordering::Relaxed);
    }
    
    /// Set clock source
    pub fn set_clock_source(&mut self, source: ClockSource) {
        self.clock_source = source;
    }
    
    /// Enable NTP
    pub fn enable_ntp(&mut self) {
        self.ntp_enabled.store(true, Ordering::Relaxed);
    }
    
    /// Disable NTP
    pub fn disable_ntp(&mut self) {
        self.ntp_enabled.store(false, Ordering::Relaxed);
    }
    
    /// Add NTP server
    pub fn add_ntp_server(&mut self, server: String) {
        self.ntp_servers.push(server);
    }
    
    /// Sync with NTP server
    pub fn sync_ntp(&mut self) -> Result<(), &'static str> {
        if !self.ntp_enabled.load(Ordering::Relaxed) {
            return Err("NTP not enabled");
        }
        
        if self.ntp_servers.is_empty() {
            return Err("No NTP servers configured");
        }
        
        // In real implementation, would query NTP server
        // For now, simulate sync
        let current_time = self.get_wall_clock();
        let offset = 0; // Would be calculated from NTP response
        self.set_wall_clock_offset(offset);
        self.last_ntp_sync.store(current_time, Ordering::Relaxed);
        
        Ok(())
    }
    
    /// Get clock quality
    pub fn get_clock_quality(&self) -> ClockQuality {
        self.clock_quality
    }
    
    /// Set clock quality
    pub fn set_clock_quality(&mut self, quality: ClockQuality) {
        self.clock_quality = quality;
    }
}

impl Default for Timekeeper {
    fn default() -> Self {
        Self::new()
    }
}

/// High-resolution timer
pub struct HighResTimer {
    pub id: u64,
    pub interval: u64, // nanoseconds
    pub callback: Option<fn()>,
    pub armed: AtomicBool,
    pub next_expiration: AtomicU64,
}

impl HighResTimer {
    pub fn new(id: u64, interval: u64) -> Self {
        Self {
            id,
            interval,
            callback: None,
            armed: AtomicBool::new(false),
            next_expiration: AtomicU64::new(0),
        }
    }
    
    pub fn arm(&self, current_time: u64) {
        self.next_expiration.store(current_time + self.interval, Ordering::Relaxed);
        self.armed.store(true, Ordering::Relaxed);
    }
    
    pub fn disarm(&self) {
        self.armed.store(false, Ordering::Relaxed);
    }
    
    pub fn is_armed(&self) -> bool {
        self.armed.load(Ordering::Relaxed)
    }
    
    pub fn check_expiration(&self, current_time: u64) -> bool {
        if !self.is_armed() {
            return false;
        }
        
        let expiration = self.next_expiration.load(Ordering::Relaxed);
        if current_time >= expiration {
            self.disarm();
            if let Some(callback) = self.callback {
                callback();
            }
            true
        } else {
            false
        }
    }
    
    pub fn set_callback(&mut self, callback: fn()) {
        self.callback = Some(callback);
    }
}

/// Timer manager
pub struct TimerManager {
    pub timers: BTreeMap<u64, HighResTimer>,
    pub next_timer_id: AtomicU64,
}

impl TimerManager {
    pub fn new() -> Self {
        Self {
            timers: BTreeMap::new(),
            next_timer_id: AtomicU64::new(1),
        }
    }
    
    pub fn create_timer(&mut self, interval: u64) -> u64 {
        let id = self.next_timer_id.fetch_add(1, Ordering::Relaxed);
        let timer = HighResTimer::new(id, interval);
        self.timers.insert(id, timer);
        id
    }
    
    pub fn arm_timer(&mut self, id: u64, current_time: u64) -> Result<(), &'static str> {
        let timer = self.timers.get(&id).ok_or("Timer not found")?;
        timer.arm(current_time);
        Ok(())
    }
    
    pub fn disarm_timer(&mut self, id: u64) -> Result<(), &'static str> {
        let timer = self.timers.get(&id).ok_or("Timer not found")?;
        timer.disarm();
        Ok(())
    }
    
    pub fn delete_timer(&mut self, id: u64) -> Result<(), &'static str> {
        self.timers.remove(&id).ok_or("Timer not found")?;
        Ok(())
    }
    
    pub fn check_timers(&mut self, current_time: u64) {
        for timer in self.timers.values() {
            timer.check_expiration(current_time);
        }
    }
    
    pub fn set_timer_callback(&mut self, id: u64, callback: fn()) -> Result<(), &'static str> {
        let timer = self.timers.get_mut(&id).ok_or("Timer not found")?;
        timer.set_callback(callback);
        Ok(())
    }
}

impl Default for TimerManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Global timekeeper
static mut TIMEKEEPER: Option<Timekeeper> = None;

/// Global timer manager
static mut TIMER_MANAGER: Option<TimerManager> = None;

/// Initialize timekeeping
pub fn init_timekeeping() {
    unsafe {
        TIMEKEEPER = Some(Timekeeper::new());
        TIMER_MANAGER = Some(TimerManager::new());
    }
}

/// Get global timekeeper
pub fn get_timekeeper() -> Option<&'static mut Timekeeper> {
    unsafe {
        TIMEKEEPER.as_mut()
    }
}

/// Get global timer manager
pub fn get_timer_manager() -> Option<&'static mut TimerManager> {
    unsafe {
        TIMER_MANAGER.as_mut()
    }
}

/// Get current time (convenience function)
pub fn get_current_time() -> u64 {
    if let Some(tk) = get_timekeeper() {
        tk.get_wall_clock()
    } else {
        0
    }
}

/// Get current time in seconds (convenience function)
pub fn get_current_time_seconds() -> u64 {
    get_current_time() / 1_000_000_000
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_timekeeper_init() {
        let tk = Timekeeper::new();
        assert_eq!(tk.get_time(), 0);
        assert_eq!(tk.get_wall_clock(), 0);
    }
    
    #[test]
    fn test_add_time() {
        let tk = Timekeeper::new();
        tk.add_time(1000);
        assert_eq!(tk.get_time(), 1000);
    }
    
    #[test]
    fn test_wall_clock_offset() {
        let tk = Timekeeper::new();
        tk.set_wall_clock_offset(1_000_000_000); // 1 second
        assert_eq!(tk.get_wall_clock(), 1_000_000_000);
    }
    
    #[test]
    fn test_ntp_enable_disable() {
        let mut tk = Timekeeper::new();
        tk.enable_ntp();
        assert!(tk.ntp_enabled.load(Ordering::Relaxed));
        tk.disable_ntp();
        assert!(!tk.ntp_enabled.load(Ordering::Relaxed));
    }
    
    #[test]
    fn test_ntp_add_server() {
        let mut tk = Timekeeper::new();
        tk.add_ntp_server("pool.ntp.org".to_string());
        assert_eq!(tk.ntp_servers.len(), 1);
    }
    
    #[test]
    fn test_ntp_sync() {
        let mut tk = Timekeeper::new();
        tk.enable_ntp();
        tk.add_ntp_server("pool.ntp.org".to_string());
        assert!(tk.sync_ntp().is_ok());
    }
    
    #[test]
    fn test_high_res_timer() {
        let timer = HighResTimer::new(1, 1_000_000); // 1ms
        assert!(!timer.is_armed());
        timer.arm(0);
        assert!(timer.is_armed());
        timer.disarm();
        assert!(!timer.is_armed());
    }
    
    #[test]
    fn test_timer_expiration() {
        let timer = HighResTimer::new(1, 1_000_000); // 1ms
        timer.arm(0);
        assert!(!timer.check_expiration(500_000)); // Not expired
        assert!(timer.check_expiration(1_500_000)); // Expired
        assert!(!timer.is_armed());
    }
    
    #[test]
    fn test_timer_manager() {
        let mut tm = TimerManager::new();
        let id = tm.create_timer(1_000_000);
        assert!(tm.arm_timer(id, 0).is_ok());
        assert!(tm.disarm_timer(id).is_ok());
        assert!(tm.delete_timer(id).is_ok());
    }
    
    #[test]
    fn test_timer_callback() {
        let mut tm = TimerManager::new();
        let id = tm.create_timer(1_000_000);
        static mut CALLED: bool = false;
        extern "C" fn test_callback() {
            unsafe { CALLED = true; }
        }
        assert!(tm.set_timer_callback(id, test_callback).is_ok());
    }
}
