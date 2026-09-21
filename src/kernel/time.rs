// Linux-inspired Time Management
// Provides system time management with timezones and clock sources

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Clock source type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClockSource {
    TSC,      // Time Stamp Counter
    HPET,     // High Precision Event Timer
    AcpiPm,   // ACPI Power Management Timer
    RTC,      // Real Time Clock
}

/// Clock source statistics
#[derive(Debug, Clone, Copy)]
pub struct ClockSourceStats {
    pub clock_id: u32,
    pub clock_source: ClockSource,
    pub resolution_ns: u64,
    pub accuracy_ppm: i32,
    pub registered: bool,
}

impl ClockSourceStats {
    pub fn new(clock_id: u32, clock_source: ClockSource, resolution_ns: u64) -> Self {
        Self {
            clock_id,
            clock_source,
            resolution_ns,
            accuracy_ppm: 0,
            registered: false,
        }
    }

    pub fn with_accuracy(mut self, accuracy_ppm: i32) -> Self {
        self.accuracy_ppm = accuracy_ppm;
        self
    }

    pub fn with_registered(mut self, registered: bool) -> Self {
        self.registered = registered;
        self
    }
}

/// Timezone information
#[derive(Debug, Clone)]
pub struct Timezone {
    pub name: String,
    pub offset_minutes: i32,
    pub dst_active: bool,
}

impl Timezone {
    pub fn new(name: String, offset_minutes: i32) -> Self {
        Self {
            name,
            offset_minutes,
            dst_active: false,
        }
    }

    pub fn with_dst(mut self, dst_active: bool) -> Self {
        self.dst_active = dst_active;
        self
    }

    /// Get timezone offset in hours
    pub fn offset_hours(&self) -> f64 {
        self.offset_minutes as f64 / 60.0
    }
}

/// System time
#[derive(Debug, Clone, Copy)]
pub struct SystemTime {
    pub seconds: u64,
    pub nanoseconds: u32,
}

impl SystemTime {
    pub fn new(seconds: u64, nanoseconds: u32) -> Self {
        Self {
            seconds,
            nanoseconds,
        }
    }

    /// Convert to milliseconds
    pub fn as_millis(&self) -> u64 {
        self.seconds * 1000 + (self.nanoseconds / 1_000_000) as u64
    }

    /// Convert to nanoseconds
    pub fn as_nanos(&self) -> u128 {
        (self.seconds as u128) * 1_000_000_000 + self.nanoseconds as u128
    }
}

/// Time manager for system-wide time management
pub struct TimeManager {
    clock_sources: Arc<Mutex<HashMap<u32, ClockSourceStats>>>,
    next_clock_id: Arc<Mutex<u32>>,
    current_clock_id: Arc<Mutex<Option<u32>>>,
    system_time: Arc<Mutex<SystemTime>>,
    timezone: Arc<Mutex<Timezone>>,
    timezones: Arc<Mutex<HashMap<String, Timezone>>>,
}

impl TimeManager {
    pub fn new() -> Self {
        let mut timezones = HashMap::new();
        
        // Common timezones
        timezones.insert("UTC".to_string(), Timezone::new("UTC".to_string(), 0));
        timezones.insert("America/New_York".to_string(), Timezone::new("EST".to_string(), -300));
        timezones.insert("America/Los_Angeles".to_string(), Timezone::new("PST".to_string(), -480));
        timezones.insert("Europe/London".to_string(), Timezone::new("GMT".to_string(), 0));
        timezones.insert("Europe/Paris".to_string(), Timezone::new("CET".to_string(), 60));
        timezones.insert("Asia/Tokyo".to_string(), Timezone::new("JST".to_string(), 540));

        Self {
            clock_sources: Arc::new(Mutex::new(HashMap::new())),
            next_clock_id: Arc::new(Mutex::new(1)),
            current_clock_id: Arc::new(Mutex::new(None)),
            system_time: Arc::new(Mutex::new(SystemTime::new(0, 0))),
            timezone: Arc::new(Mutex::new(Timezone::new("UTC".to_string(), 0))),
            timezones: Arc::new(Mutex::new(timezones)),
        }
    }

    /// Register a clock source
    pub fn register_clock_source(&self, clock_source: ClockSource, resolution_ns: u64) -> u32 {
        let mut next_id = self.next_clock_id.lock().unwrap();
        let clock_id = *next_id;
        *next_id += 1;
        drop(next_id);

        let stats = ClockSourceStats::new(clock_id, clock_source, resolution_ns).with_registered(true);
        let mut clock_sources = self.clock_sources.lock().unwrap();
        clock_sources.insert(clock_id, stats);

        clock_id
    }

    /// Get a clock source by ID
    pub fn get_clock_source(&self, clock_id: u32) -> Option<ClockSourceStats> {
        let clock_sources = self.clock_sources.lock().unwrap();
        clock_sources.get(&clock_id).cloned()
    }

    /// Select a clock source
    pub fn select_clock_source(&self, clock_id: u32) -> Result<(), String> {
        let clock_sources = self.clock_sources.lock().unwrap();
        if !clock_sources.contains_key(&clock_id) {
            return Err(format!("Clock source {} not found", clock_id));
        }
        drop(clock_sources);

        let mut current_clock_id = self.current_clock_id.lock().unwrap();
        *current_clock_id = Some(clock_id);
        Ok(())
    }

    /// Get current clock source
    pub fn get_current_clock_source(&self) -> Option<ClockSourceStats> {
        let current_clock_id = self.current_clock_id.lock().unwrap();
        match *current_clock_id {
            Some(id) => {
                let clock_sources = self.clock_sources.lock().unwrap();
                clock_sources.get(&id).cloned()
            }
            None => None,
        }
    }

    /// Set system time
    pub fn set_system_time(&self, time: SystemTime) {
        let mut system_time = self.system_time.lock().unwrap();
        *system_time = time;
    }

    /// Get system time
    pub fn get_system_time(&self) -> SystemTime {
        let system_time = self.system_time.lock().unwrap();
        *system_time
    }

    /// Set timezone
    pub fn set_timezone(&self, name: String) -> Result<(), String> {
        let timezones = self.timezones.lock().unwrap();
        match timezones.get(&name).cloned() {
            Some(tz) => {
                drop(timezones);
                let mut timezone = self.timezone.lock().unwrap();
                *timezone = tz;
                Ok(())
            }
            None => Err(format!("Timezone {} not found", name)),
        }
    }

    /// Get current timezone
    pub fn get_timezone(&self) -> Timezone {
        let timezone = self.timezone.lock().unwrap();
        timezone.clone()
    }

    /// Add a timezone
    pub fn add_timezone(&self, timezone: Timezone) {
        let mut timezones = self.timezones.lock().unwrap();
        timezones.insert(timezone.name.clone(), timezone);
    }

    /// Get all timezones
    pub fn get_all_timezones(&self) -> HashMap<String, Timezone> {
        let timezones = self.timezones.lock().unwrap();
        timezones.clone()
    }

    /// Get clock source count
    pub fn clock_source_count(&self) -> usize {
        let clock_sources = self.clock_sources.lock().unwrap();
        clock_sources.len()
    }
}

impl Default for TimeManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clock_source_stats() {
        let stats = ClockSourceStats::new(1, ClockSource::TSC, 1);
        assert_eq!(stats.clock_id, 1);
        assert_eq!(stats.clock_source, ClockSource::TSC);
        assert_eq!(stats.resolution_ns, 1);
        assert!(!stats.registered);
    }

    #[test]
    fn test_clock_source_stats_builder() {
        let stats = ClockSourceStats::new(1, ClockSource::TSC, 1)
            .with_accuracy(100)
            .with_registered(true);

        assert_eq!(stats.accuracy_ppm, 100);
        assert!(stats.registered);
    }

    #[test]
    fn test_timezone() {
        let tz = Timezone::new("EST".to_string(), -300);
        assert_eq!(tz.name, "EST");
        assert_eq!(tz.offset_minutes, -300);
        assert_eq!(tz.offset_hours(), -5.0);
        assert!(!tz.dst_active);
    }

    #[test]
    fn test_timezone_with_dst() {
        let tz = Timezone::new("EDT".to_string(), -240).with_dst(true);
        assert!(tz.dst_active);
    }

    #[test]
    fn test_system_time() {
        let time = SystemTime::new(1000, 500_000_000);
        assert_eq!(time.seconds, 1000);
        assert_eq!(time.nanoseconds, 500_000_000);
        assert_eq!(time.as_millis(), 1000500);
    }

    #[test]
    fn test_system_time_nanos() {
        let time = SystemTime::new(1, 1_000_000_000);
        assert_eq!(time.as_nanos(), 2_000_000_000);
    }

    #[test]
    fn test_time_manager() {
        let manager = TimeManager::new();

        let clock_id = manager.register_clock_source(ClockSource::TSC, 1);
        assert_eq!(clock_id, 1);

        manager.select_clock_source(clock_id).unwrap();
        let current = manager.get_current_clock_source().unwrap();
        assert_eq!(current.clock_id, clock_id);
    }

    #[test]
    fn test_time_manager_system_time() {
        let manager = TimeManager::new();

        let time = SystemTime::new(1000, 0);
        manager.set_system_time(time);

        let retrieved = manager.get_system_time();
        assert_eq!(retrieved.seconds, 1000);
    }

    #[test]
    fn test_time_manager_timezone() {
        let manager = TimeManager::new();

        manager.set_timezone("America/New_York".to_string()).unwrap();
        let tz = manager.get_timezone();
        assert_eq!(tz.name, "EST");
        assert_eq!(tz.offset_minutes, -300);
    }

    #[test]
    fn test_time_manager_invalid_timezone() {
        let manager = TimeManager::new();
        assert!(manager.set_timezone("Invalid/Timezone".to_string()).is_err());
    }

    #[test]
    fn test_time_manager_add_timezone() {
        let manager = TimeManager::new();

        let tz = Timezone::new("Custom".to_string(), 120);
        manager.add_timezone(tz);

        manager.set_timezone("Custom".to_string()).unwrap();
        let current = manager.get_timezone();
        assert_eq!(current.name, "Custom");
    }

    #[test]
    fn test_time_manager_multiple_clocks() {
        let manager = TimeManager::new();

        let clock_id1 = manager.register_clock_source(ClockSource::TSC, 1);
        let _clock_id2 = manager.register_clock_source(ClockSource::HPET, 100);

        assert_eq!(manager.clock_source_count(), 2);

        manager.select_clock_source(clock_id1).unwrap();
        let current = manager.get_current_clock_source().unwrap();
        assert_eq!(current.clock_source, ClockSource::TSC);
    }

    #[test]
    fn test_time_manager_select_invalid() {
        let manager = TimeManager::new();
        assert!(manager.select_clock_source(999).is_err());
    }
}
