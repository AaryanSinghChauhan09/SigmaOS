// SPDX-License-Identifier: MIT
// Copyright (c) 2026 SigmaOS Contributors

//! SigmaTime - Time Synchronization Service
//!
//! SigmaTime provides time synchronization for SigmaOS, inspired by NTP and systemd-timesyncd.

#![no_std]
extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::fmt;

/// NTP server configuration
#[derive(Debug, Clone)]
pub struct NtpServer {
    /// Server hostname or IP address
    pub address: String,
    /// Server port (default 123)
    pub port: u16,
    /// Server priority (lower is higher priority)
    pub priority: u32,
    /// Enabled flag
    pub enabled: bool,
}

impl NtpServer {
    /// Create a new NTP server
    pub fn new(address: String) -> Self {
        Self {
            address,
            port: 123,
            priority: 100,
            enabled: true,
        }
    }

    /// Set port
    pub fn with_port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

    /// Set priority
    pub fn with_priority(mut self, priority: u32) -> Self {
        self.priority = priority;
        self
    }

    /// Set enabled
    pub fn with_enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }
}

/// Time synchronization status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SyncStatus {
    /// Not synchronized
    NotSynced,
    /// Synchronizing
    Syncing,
    /// Synchronized
    Synced,
    /// Error
    Error,
}

impl fmt::Display for SyncStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SyncStatus::NotSynced => write!(f, "not_synced"),
            SyncStatus::Syncing => write!(f, "syncing"),
            SyncStatus::Synced => write!(f, "synced"),
            SyncStatus::Error => write!(f, "error"),
        }
    }
}

/// Time source
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimeSource {
    /// Hardware clock (RTC)
    HardwareClock,
    /// NTP server
    NtpServer,
    /// Manual setting
    Manual,
    /// Unknown
    Unknown,
}

impl fmt::Display for TimeSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TimeSource::HardwareClock => write!(f, "hardware_clock"),
            TimeSource::NtpServer => write!(f, "ntp_server"),
            TimeSource::Manual => write!(f, "manual"),
            TimeSource::Unknown => write!(f, "unknown"),
        }
    }
}

/// Time synchronization statistics
#[derive(Debug, Clone)]
pub struct SyncStats {
    /// Last successful sync timestamp
    pub last_sync: Option<u64>,
    /// Last sync server
    pub last_server: Option<String>,
    /// Sync attempts
    pub sync_attempts: u32,
    /// Successful syncs
    pub successful_syncs: u32,
    /// Failed syncs
    pub failed_syncs: u32,
    /// Current offset (milliseconds)
    pub current_offset: i64,
    /// Root delay (milliseconds)
    pub root_delay: u64,
    /// Root dispersion (milliseconds)
    pub root_dispersion: u64,
}

impl SyncStats {
    /// Create new sync statistics
    pub fn new() -> Self {
        Self {
            last_sync: None,
            last_server: None,
            sync_attempts: 0,
            successful_syncs: 0,
            failed_syncs: 0,
            current_offset: 0,
            root_delay: 0,
            root_dispersion: 0,
        }
    }

    /// Update last sync
    pub fn update_last_sync(&mut self, timestamp: u64, server: String) {
        self.last_sync = Some(timestamp);
        self.last_server = Some(server);
    }

    /// Increment sync attempts
    pub fn increment_attempts(&mut self) {
        self.sync_attempts += 1;
    }

    /// Increment successful syncs
    pub fn increment_success(&mut self) {
        self.successful_syncs += 1;
    }

    /// Increment failed syncs
    pub fn increment_failure(&mut self) {
        self.failed_syncs += 1;
    }

    /// Set current offset
    pub fn set_offset(&mut self, offset: i64) {
        self.current_offset = offset;
    }

    /// Set root delay
    pub fn set_root_delay(&mut self, delay: u64) {
        self.root_delay = delay;
    }

    /// Set root dispersion
    pub fn set_root_dispersion(&mut self, dispersion: u64) {
        self.root_dispersion = dispersion;
    }
}

impl Default for SyncStats {
    fn default() -> Self {
        Self::new()
    }
}

/// Time synchronization configuration
#[derive(Debug, Clone)]
pub struct TimeConfig {
    /// Sync interval in seconds
    pub sync_interval: u64,
    /// Maximum sync attempts
    pub max_sync_attempts: u32,
    /// Minimum sync interval (backoff)
    pub min_sync_interval: u64,
    /// Maximum sync interval
    pub max_sync_interval: u64,
    /// Enable RTC synchronization
    pub sync_rtc: bool,
    /// Enable drift correction
    pub enable_drift_correction: bool,
    /// Maximum drift threshold (milliseconds)
    pub max_drift_threshold: i64,
}

impl TimeConfig {
    /// Create new time configuration
    pub fn new() -> Self {
        Self {
            sync_interval: 3600, // 1 hour
            max_sync_attempts: 5,
            min_sync_interval: 64, // 64 seconds
            max_sync_interval: 86400, // 24 hours
            sync_rtc: true,
            enable_drift_correction: true,
            max_drift_threshold: 1000, // 1 second
        }
    }

    /// Set sync interval
    pub fn with_sync_interval(mut self, interval: u64) -> Self {
        self.sync_interval = interval;
        self
    }

    /// Set max sync attempts
    pub fn with_max_sync_attempts(mut self, attempts: u32) -> Self {
        self.max_sync_attempts = attempts;
        self
    }

    /// Set sync RTC
    pub fn with_sync_rtc(mut self, sync: bool) -> Self {
        self.sync_rtc = sync;
        self
    }

    /// Set drift correction
    pub fn with_drift_correction(mut self, enable: bool) -> Self {
        self.enable_drift_correction = enable;
        self
    }
}

impl Default for TimeConfig {
    fn default() -> Self {
        Self::new()
    }
}

/// SigmaTime - Time Synchronization Service
pub struct SigmaTime {
    /// Time configuration
    config: TimeConfig,
    /// NTP servers
    ntp_servers: Vec<NtpServer>,
    /// Synchronization status
    sync_status: SyncStatus,
    /// Current time source
    time_source: TimeSource,
    /// Synchronization statistics
    stats: SyncStats,
    /// Running state
    running: bool,
    /// Current system time (seconds since epoch)
    current_time: u64,
}

impl SigmaTime {
    /// Create a new SigmaTime instance
    pub fn new(config: TimeConfig) -> Self {
        Self {
            config,
            ntp_servers: Vec::new(),
            sync_status: SyncStatus::NotSynced,
            time_source: TimeSource::Unknown,
            stats: SyncStats::new(),
            running: false,
            current_time: 0,
        }
    }

    /// Create SigmaTime with default configuration
    pub fn with_default_config() -> Self {
        Self::new(TimeConfig::default())
    }

    /// Start the time synchronization service
    pub fn start(&mut self) {
        self.running = true;
        self.sync_status = SyncStatus::Syncing;
    }

    /// Stop the time synchronization service
    pub fn stop(&mut self) {
        self.running = false;
        self.sync_status = SyncStatus::NotSynced;
    }

    /// Check if the service is running
    pub fn is_running(&self) -> bool {
        self.running
    }

    /// Add an NTP server
    pub fn add_ntp_server(&mut self, server: NtpServer) {
        self.ntp_servers.push(server);
        // Sort by priority (lower is higher priority)
        self.ntp_servers.sort_by(|a, b| a.priority.cmp(&b.priority));
    }

    /// Remove an NTP server
    pub fn remove_ntp_server(&mut self, address: &str) {
        self.ntp_servers.retain(|s| s.address != address);
    }

    /// Get NTP servers
    pub fn ntp_servers(&self) -> &[NtpServer] {
        &self.ntp_servers
    }

    /// Get enabled NTP servers
    pub fn enabled_ntp_servers(&self) -> Vec<&NtpServer> {
        self.ntp_servers.iter().filter(|s| s.enabled).collect()
    }

    /// Get synchronization status
    pub fn sync_status(&self) -> SyncStatus {
        self.sync_status
    }

    /// Get time source
    pub fn time_source(&self) -> TimeSource {
        self.time_source
    }

    /// Get synchronization statistics
    pub fn stats(&self) -> &SyncStats {
        &self.stats
    }

    /// Get current time
    pub fn current_time(&self) -> u64 {
        self.current_time
    }

    /// Set current time manually
    pub fn set_time(&mut self, time: u64) {
        self.current_time = time;
        self.time_source = TimeSource::Manual;
        self.sync_status = SyncStatus::Synced;
    }

    /// Get configuration
    pub fn config(&self) -> &TimeConfig {
        &self.config
    }

    /// Update configuration
    pub fn update_config(&mut self, config: TimeConfig) {
        self.config = config;
    }

    /// Synchronize time with NTP servers
    pub fn sync_time(&mut self, current_timestamp: u64) -> Result<String, String> {
        if !self.running {
            return Err("SigmaTime not running".to_string());
        }

        self.sync_status = SyncStatus::Syncing;
        self.stats.increment_attempts();

        // Try enabled NTP servers in priority order
        for server in self.enabled_ntp_servers() {
            // In a real implementation, this would perform actual NTP synchronization
            // For now, we simulate a successful sync
            let server_address = server.address.clone();
            
            // Simulate successful sync
            self.stats.increment_success();
            self.stats.update_last_sync(current_timestamp, server_address.clone());
            self.time_source = TimeSource::NtpServer;
            self.sync_status = SyncStatus::Synced;
            
            // Simulate time adjustment (small drift correction)
            let simulated_offset = 50; // 50ms offset
            self.stats.set_offset(simulated_offset);
            self.stats.set_root_delay(100);
            self.stats.set_root_dispersion(50);
            
            // Update current time (in real implementation, this would be from NTP)
            self.current_time = current_timestamp;
            
            return Ok(format!("Synced with {}", server_address));
        }

        // No enabled servers
        self.stats.increment_failure();
        self.sync_status = SyncStatus::Error;
        Err("No enabled NTP servers available".to_string())
    }

    /// Synchronize RTC with system time
    pub fn sync_rtc(&mut self) -> Result<(), String> {
        if !self.config.sync_rtc {
            return Err("RTC synchronization disabled".to_string());
        }

        if !self.running {
            return Err("SigmaTime not running".to_string());
        }

        // In a real implementation, this would write to the hardware RTC
        // For now, we just return success
        Ok(())
    }

    /// Check if synchronization is needed
    pub fn needs_sync(&self, current_time: u64) -> bool {
        match self.stats.last_sync {
            Some(last) => current_time - last > self.config.sync_interval,
            None => true,
        }
    }

    /// Get time as formatted string
    pub fn formatted_time(&self) -> String {
        // In a real implementation, this would format the time properly
        format!("{}s since epoch", self.current_time)
    }

    /// Get server statistics
    pub fn server_stats(&self) -> BTreeMap<String, String> {
        let mut stats = BTreeMap::new();
        stats.insert("status".to_string(), self.sync_status.to_string());
        stats.insert("time_source".to_string(), self.time_source.to_string());
        stats.insert("sync_attempts".to_string(), self.stats.sync_attempts.to_string());
        stats.insert("successful_syncs".to_string(), self.stats.successful_syncs.to_string());
        stats.insert("failed_syncs".to_string(), self.stats.failed_syncs.to_string());
        stats.insert("current_offset_ms".to_string(), self.stats.current_offset.to_string());
        stats.insert("ntp_servers".to_string(), self.ntp_servers.len().to_string());
        if let Some(last) = self.stats.last_sync {
            stats.insert("last_sync".to_string(), last.to_string());
        }
        if let Some(server) = &self.stats.last_server {
            stats.insert("last_server".to_string(), server.clone());
        }
        stats
    }
}

impl Default for SigmaTime {
    fn default() -> Self {
        Self::with_default_config()
    }
}

/// Global SigmaTime instance (placeholder for actual global state)
static mut SIGMATIME_INSTANCE: Option<SigmaTime> = None;

/// Initialize the global SigmaTime instance
pub fn init_sigmatime(config: TimeConfig) {
    unsafe {
        SIGMATIME_INSTANCE = Some(SigmaTime::new(config));
    }
}

/// Initialize SigmaTime with default configuration
pub fn init_sigmatime_default() {
    init_sigmatime(TimeConfig::default());
}

/// Get the global SigmaTime instance
pub fn get_sigmatime() -> Option<&'static mut SigmaTime> {
    unsafe { SIGMATIME_INSTANCE.as_mut() }
}

/// Convenience function to add an NTP server
pub fn add_ntp_server(address: String, port: u16, priority: u32) {
    if let Some(time) = get_sigmatime() {
        let server = NtpServer::new(address)
            .with_port(port)
            .with_priority(priority);
        time.add_ntp_server(server);
    }
}

/// Convenience function to synchronize time
pub fn sync_time(current_timestamp: u64) -> Result<String, String> {
    if let Some(time) = get_sigmatime() {
        time.sync_time(current_timestamp)
    } else {
        Err("SigmaTime not initialized".to_string())
    }
}

/// Convenience function to get current time
pub fn get_current_time() -> Option<u64> {
    get_sigmatime().map(|t| t.current_time())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ntp_server() {
        let server = NtpServer::new("pool.ntp.org".to_string())
            .with_port(123)
            .with_priority(50)
            .with_enabled(true);

        assert_eq!(server.address, "pool.ntp.org");
        assert_eq!(server.port, 123);
        assert_eq!(server.priority, 50);
        assert!(server.enabled);
    }

    #[test]
    fn test_sync_status_display() {
        assert_eq!(SyncStatus::Synced.to_string(), "synced");
        assert_eq!(SyncStatus::Syncing.to_string(), "syncing");
    }

    #[test]
    fn test_time_source_display() {
        assert_eq!(TimeSource::NtpServer.to_string(), "ntp_server");
        assert_eq!(TimeSource::Manual.to_string(), "manual");
    }

    #[test]
    fn test_sync_stats() {
        let mut stats = SyncStats::new();
        
        stats.increment_attempts();
        stats.increment_success();
        stats.update_last_sync(1000, "pool.ntp.org".to_string());
        stats.set_offset(50);
        
        assert_eq!(stats.sync_attempts, 1);
        assert_eq!(stats.successful_syncs, 1);
        assert_eq!(stats.last_sync, Some(1000));
        assert_eq!(stats.current_offset, 50);
    }

    #[test]
    fn test_time_config() {
        let config = TimeConfig::new()
            .with_sync_interval(7200)
            .with_max_sync_attempts(10)
            .with_sync_rtc(true)
            .with_drift_correction(true);

        assert_eq!(config.sync_interval, 7200);
        assert_eq!(config.max_sync_attempts, 10);
        assert!(config.sync_rtc);
        assert!(config.enable_drift_correction);
    }

    #[test]
    fn test_sigmatime() {
        let mut time = SigmaTime::with_default_config();
        assert!(!time.is_running());
        time.start();
        assert!(time.is_running());
        assert_eq!(time.sync_status(), SyncStatus::Syncing);
        time.stop();
        assert!(!time.is_running());
    }

    #[test]
    fn test_sigmatime_add_server() {
        let mut time = SigmaTime::with_default_config();
        
        let server = NtpServer::new("pool.ntp.org".to_string());
        time.add_ntp_server(server);
        
        assert_eq!(time.ntp_servers().len(), 1);
    }

    #[test]
    fn test_sigmatime_server_priority() {
        let mut time = SigmaTime::with_default_config();
        
        let server1 = NtpServer::new("server1.org".to_string()).with_priority(100);
        let server2 = NtpServer::new("server2.org".to_string()).with_priority(50);
        
        time.add_ntp_server(server1);
        time.add_ntp_server(server2);
        
        // Should be sorted by priority (lower first)
        assert_eq!(time.ntp_servers()[0].address, "server2.org");
        assert_eq!(time.ntp_servers()[1].address, "server1.org");
    }

    #[test]
    fn test_sigmatime_set_time() {
        let mut time = SigmaTime::with_default_config();
        
        time.set_time(1000);
        
        assert_eq!(time.current_time(), 1000);
        assert_eq!(time.time_source(), TimeSource::Manual);
        assert_eq!(time.sync_status(), SyncStatus::Synced);
    }

    #[test]
    fn test_sigmatime_sync_time() {
        let mut time = SigmaTime::with_default_config();
        time.start();
        
        let server = NtpServer::new("pool.ntp.org".to_string());
        time.add_ntp_server(server);
        
        let result = time.sync_time(1000);
        
        assert!(result.is_ok());
        assert_eq!(time.sync_status(), SyncStatus::Synced);
        assert_eq!(time.time_source(), TimeSource::NtpServer);
    }

    #[test]
    fn test_sigmatime_needs_sync() {
        let mut time = SigmaTime::with_default_config();
        
        // Should need sync initially
        assert!(time.needs_sync(1000));
        
        // After sync, should not need sync immediately
        time.stats.update_last_sync(1000, "pool.ntp.org".to_string());
        assert!(!time.needs_sync(1500)); // 500 seconds later
        
        // Should need sync after interval
        assert!(time.needs_sync(5000)); // 4000 seconds later
    }

    #[test]
    fn test_sigmatime_server_stats() {
        let mut time = SigmaTime::with_default_config();
        time.start();
        
        let server = NtpServer::new("pool.ntp.org".to_string());
        time.add_ntp_server(server);
        
        time.sync_time(1000).unwrap();
        
        let stats = time.server_stats();
        assert_eq!(stats.get("status"), Some(&"synced".to_string()));
        assert_eq!(stats.get("time_source"), Some(&"ntp_server".to_string()));
    }

    #[test]
    fn test_add_ntp_server_convenience() {
        init_sigmatime_default();
        
        add_ntp_server("pool.ntp.org".to_string(), 123, 50);
        
        let time = get_sigmatime().unwrap();
        assert_eq!(time.ntp_servers().len(), 1);
    }

    #[test]
    fn test_sync_time_convenience() {
        init_sigmatime_default();
        
        let mut time = get_sigmatime().unwrap();
        time.start();
        
        let server = NtpServer::new("pool.ntp.org".to_string());
        time.add_ntp_server(server);
        
        let result = sync_time(1000);
        assert!(result.is_ok());
    }

    #[test]
    fn test_get_current_time_convenience() {
        init_sigmatime_default();
        
        let mut time = get_sigmatime().unwrap();
        time.set_time(1000);
        
        let current = get_current_time();
        assert_eq!(current, Some(1000));
    }
}
