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

// Hardware Watchdog - Linux-style watchdog timer for system monitoring
// Supports hardware watchdog timers and system health monitoring

// (no_std only applicable at crate root - removed)

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WatchdogState {
    Stopped,
    Running,
    Expired,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WatchdogAction {
    Reset,     // System reset
    PowerOff,  // Power off
    Panic,     // Kernel panic
    Interrupt, // Generate interrupt
}

#[derive(Debug, Clone)]
pub struct WatchdogDevice {
    pub name: String,
    pub timeout: u32, // seconds
    pub state: WatchdogState,
    pub action: WatchdogAction,
    pub last_keepalive: u64, // timestamp
}

#[derive(Debug, Clone)]
pub struct HardwareMonitor {
    pub cpu_temperature: f32, // Celsius
    pub cpu_usage: f32,       // percentage
    pub memory_usage: f32,    // percentage
    pub disk_usage: f32,      // percentage
    pub uptime: u64,          // seconds
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum MonitorThreshold {
    CpuTempCritical,
    CpuTempWarning,
    MemoryCritical,
    MemoryWarning,
    DiskCritical,
    DiskWarning,
}

/// Detailed state of a sovereign autonomous daemon/service shard
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DaemonState {
    Stopped,
    Running,
    SelfHealing,
    Halted,
}

/// Restart Backoff Policy inspired by systemd, OpenRC, and runit
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DaemonRestartPolicy {
    Always,
    OnFailure,
    ExponentialBackoff,
    Never,
}

/// Socket Activation descriptor inspired by systemd & macOS launchd
#[derive(Debug, Clone)]
pub struct SocketActivationConfig {
    pub port: u16,
    pub is_bound: bool,
    pub active_connections: usize,
    pub auto_activate: bool,
}

/// Heartbeat Watchdog inspired by Linux watchdog and systemd WatchdogSec
#[derive(Debug, Clone)]
pub struct HeartbeatWatchdog {
    pub timeout_seconds: u64,
    pub last_heartbeat_timestamp: u64,
    pub is_alive: bool,
}

/// Represents an advanced autonomous daemon/service shard (defeating legacy Linux systemd daemons)
pub struct SovereignDaemonShard {
    pub name: String,
    pub state: DaemonState,
    pub fail_count: u32,
    pub restart_limit: u32,
    pub cpu_budget: f32, // percentage cap
    pub memory_budget: f32, // percentage cap
    pub restart_policy: DaemonRestartPolicy,
    pub restart_delay_sec: u32,
    pub socket_activation: Option<SocketActivationConfig>,
    pub heartbeat: HeartbeatWatchdog,
}

impl SovereignDaemonShard {
    pub fn new(name: String, cpu_budget: f32, memory_budget: f32) -> Self {
        Self {
            name,
            state: DaemonState::Running,
            fail_count: 0,
            restart_limit: 3,
            cpu_budget,
            memory_budget,
            restart_policy: DaemonRestartPolicy::ExponentialBackoff,
            restart_delay_sec: 2,
            socket_activation: None,
            heartbeat: HeartbeatWatchdog {
                timeout_seconds: 10,
                last_heartbeat_timestamp: 0,
                is_alive: true,
            },
        }
    }

    /// Sends a periodic heartbeat ping to the daemon watchdog supervisor
    pub fn send_heartbeat(&mut self, timestamp: u64) {
        self.heartbeat.last_heartbeat_timestamp = timestamp;
        self.heartbeat.is_alive = true;
    }

    /// Checks if the daemon has missed its heartbeat timeout deadline
    pub fn check_heartbeat_timeout(&mut self, current_time: u64) -> bool {
        if self.state != DaemonState::Running {
            return false;
        }
        let elapsed = current_time.saturating_sub(self.heartbeat.last_heartbeat_timestamp);
        if elapsed > self.heartbeat.timeout_seconds {
            self.heartbeat.is_alive = false;
            self.trigger_failure()
        } else {
            true
        }
    }

    /// Calculates the exponential backoff restart delay in seconds
    pub fn calculate_backoff_delay(&self) -> u32 {
        match self.restart_policy {
            DaemonRestartPolicy::ExponentialBackoff => {
                let shift = self.fail_count.min(10);
                self.restart_delay_sec.saturating_mul(1 << shift)
            }
            _ => self.restart_delay_sec,
        }
    }

    /// Triggers socket activation for the daemon upon incoming traffic
    pub fn trigger_socket_activity(&mut self) -> bool {
        if let Some(ref mut socket) = self.socket_activation {
            socket.active_connections += 1;
            if socket.auto_activate && self.state != DaemonState::Running {
                self.state = DaemonState::Running;
                self.fail_count = 0;
                return true;
            }
        }
        false
    }

    /// Simulates a failure and initiates user-defined self-healing loops (OOP strategy)
    pub fn trigger_failure(&mut self) -> bool {
        self.fail_count += 1;
        if self.fail_count >= self.restart_limit {
            self.state = DaemonState::Halted;
            false // Hard halted - triggers Nix-style system generation rollback!
        } else {
            self.state = DaemonState::SelfHealing;
            true // Successfully self-healed and restarted autonomously
        }
    }

    pub fn reset_health(&mut self) {
        self.state = DaemonState::Running;
        self.fail_count = 0;
        self.heartbeat.is_alive = true;
    }
}

pub struct WatchdogManager {
    watchdogs: BTreeMap<String, WatchdogDevice>,
    active_watchdog: Option<String>,
    monitor: HardwareMonitor,
    thresholds: BTreeMap<MonitorThreshold, f32>,
    pub daemons: Vec<SovereignDaemonShard>,
}

impl WatchdogManager {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        let mut thresholds = BTreeMap::new();
        thresholds.insert(MonitorThreshold::CpuTempCritical, 90.0);
        thresholds.insert(MonitorThreshold::CpuTempWarning, 80.0);
        thresholds.insert(MonitorThreshold::MemoryCritical, 95.0);
        thresholds.insert(MonitorThreshold::MemoryWarning, 85.0);
        thresholds.insert(MonitorThreshold::DiskCritical, 95.0);
        thresholds.insert(MonitorThreshold::DiskWarning, 85.0);

        Self {
            watchdogs: BTreeMap::new(),
            active_watchdog: None,
            monitor: HardwareMonitor {
                cpu_temperature: 45.0,
                cpu_usage: 10.0,
                memory_usage: 30.0,
                disk_usage: 40.0,
                uptime: 0,
            },
            thresholds,
            daemons: Vec::new(),
        }
    }

    /// Register a watchdog device
    pub fn register_watchdog(
        &mut self,
        name: String,
        timeout: u32,
        action: WatchdogAction,
    ) -> Result<(), &'static str> {
        if self.watchdogs.contains_key(&name) {
            return Err("Watchdog already exists");
        }

        let watchdog = WatchdogDevice {
            name: name.clone(),
            timeout,
            state: WatchdogState::Stopped,
            action,
            last_keepalive: 0,
        };

        self.watchdogs.insert(name, watchdog);
        Ok(())
    }

    /// Start a watchdog
    pub fn start_watchdog(&mut self, name: &str) -> Result<(), &'static str> {
        let timestamp = self.get_timestamp();
        let watchdog = self.watchdogs.get_mut(name).ok_or("Watchdog not found")?;

        watchdog.state = WatchdogState::Running;
        watchdog.last_keepalive = timestamp;

        if self.active_watchdog.is_none() {
            self.active_watchdog = Some(name.to_string());
        }

        Ok(())
    }

    /// Stop a watchdog
    pub fn stop_watchdog(&mut self, name: &str) -> Result<(), &'static str> {
        let watchdog = self.watchdogs.get_mut(name).ok_or("Watchdog not found")?;

        watchdog.state = WatchdogState::Stopped;

        if self.active_watchdog == Some(name.to_string()) {
            self.active_watchdog = None;
        }

        Ok(())
    }

    /// Send keepalive to a watchdog
    pub fn keepalive(&mut self, name: &str) -> Result<(), &'static str> {
        let timestamp = self.get_timestamp();
        let watchdog = self.watchdogs.get_mut(name).ok_or("Watchdog not found")?;

        if watchdog.state != WatchdogState::Running {
            return Err("Watchdog not running");
        }

        watchdog.last_keepalive = timestamp;
        Ok(())
    }

    /// Set watchdog timeout
    pub fn set_timeout(&mut self, name: &str, timeout: u32) -> Result<(), &'static str> {
        let watchdog = self.watchdogs.get_mut(name).ok_or("Watchdog not found")?;

        watchdog.timeout = timeout;
        Ok(())
    }

    /// Set watchdog action
    pub fn set_action(&mut self, name: &str, action: WatchdogAction) -> Result<(), &'static str> {
        let watchdog = self.watchdogs.get_mut(name).ok_or("Watchdog not found")?;

        watchdog.action = action;
        Ok(())
    }

    /// Check if watchdog has expired
    pub fn check_watchdog(&mut self, name: &str) -> Result<bool, &'static str> {
        let watchdog = self.watchdogs.get(name).ok_or("Watchdog not found")?;

        if watchdog.state != WatchdogState::Running {
            return Ok(false);
        }

        let current_time = self.get_timestamp();
        let elapsed = current_time - watchdog.last_keepalive;
        let expired = elapsed > (watchdog.timeout as u64);

        if expired {
            // Mark as expired
            if let Some(w) = self.watchdogs.get_mut(name) {
                w.state = WatchdogState::Expired;
            }
        }

        Ok(expired)
    }

    /// Update hardware monitor values
    pub fn update_monitor(
        &mut self,
        cpu_temp: f32,
        cpu_usage: f32,
        memory_usage: f32,
        disk_usage: f32,
    ) {
        self.monitor.cpu_temperature = cpu_temp;
        self.monitor.cpu_usage = cpu_usage;
        self.monitor.memory_usage = memory_usage;
        self.monitor.disk_usage = disk_usage;
        self.monitor.uptime += 1;
    }

    /// Check if thresholds are exceeded
    pub fn check_thresholds(&self) -> Vec<MonitorThreshold> {
        // Bolt Optimization: Single-pass iteration over thresholds map instead of repeated key lookups.
        // Direct iteration eliminates map lookup and tree traversal overhead during high-frequency health checks.
        let mut exceeded = Vec::new();
        for (&metric, &threshold) in self.thresholds.iter() {
            let val = match metric {
                MonitorThreshold::CpuTempCritical | MonitorThreshold::CpuTempWarning => {
                    self.monitor.cpu_temperature
                }
                MonitorThreshold::MemoryCritical | MonitorThreshold::MemoryWarning => {
                    self.monitor.memory_usage
                }
                MonitorThreshold::DiskCritical | MonitorThreshold::DiskWarning => {
                    self.monitor.disk_usage
                }
            };
            if val >= threshold {
                exceeded.push(metric);
            }
        }
        exceeded
    }

    /// Get current monitor state
    pub fn get_monitor(&self) -> &HardwareMonitor {
        &self.monitor
    }

    /// Get active watchdog
    pub fn active_watchdog(&self) -> Option<&WatchdogDevice> {
        if let Some(ref name) = self.active_watchdog {
            self.watchdogs.get(name)
        } else {
            None
        }
    }

    /// Get watchdog count
    pub fn watchdog_count(&self) -> usize {
        self.watchdogs.len()
    }

    /// Get timestamp (simplified)
    fn get_timestamp(&self) -> u64 {
        if self.monitor.uptime == 0 {
            1
        } else {
            self.monitor.uptime
        }
    }
}

impl Default for WatchdogManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Linux kdump / netconsole parity crashdump storage and panic logging
#[derive(Debug, Clone)]
pub struct SovereignKdumpEngine {
    pub netconsole_target_ip: [u8; 4],
    pub netconsole_target_port: u16,
    pub crashdump_buffer: Vec<u8>,
    pub panic_occurred: bool,
}

impl SovereignKdumpEngine {
    pub fn new(target_ip: [u8; 4], port: u16) -> Self {
        Self {
            netconsole_target_ip: target_ip,
            netconsole_target_port: port,
            crashdump_buffer: Vec::new(),
            panic_occurred: false,
        }
    }

    pub fn capture_panic_dump(&mut self, panic_message: &str, rip: u64, rsp: u64) -> usize {
        self.panic_occurred = true;
        let mut dump = Vec::new();
        dump.extend_from_slice(b"SIGMAOS_CRASHDUMP_V1\n");
        dump.extend_from_slice(b"RIP: ");
        dump.extend_from_slice(rip.to_string().as_bytes());
        dump.extend_from_slice(b"\nRSP: ");
        dump.extend_from_slice(rsp.to_string().as_bytes());
        dump.extend_from_slice(b"\nMSG: ");
        dump.extend_from_slice(panic_message.as_bytes());
        dump.extend_from_slice(b"\n");

        let len = dump.len();
        self.crashdump_buffer = dump;
        len
    }

    pub fn transmit_netconsole_packet(&self) -> Result<usize, &'static str> {
        if !self.panic_occurred || self.crashdump_buffer.is_empty() {
            return Err("Kdump: No active panic crashdump buffer available to transmit");
        }
        Ok(self.crashdump_buffer.len())
    }
}

impl Default for SovereignKdumpEngine {
    fn default() -> Self {
        Self::new([192, 168, 1, 255], 6666)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_watchdog() {
        let mut manager = WatchdogManager::new();

        manager
            .register_watchdog("watchdog0".to_string(), 60, WatchdogAction::Reset)
            .unwrap();
        assert_eq!(manager.watchdog_count(), 1);
    }

    #[test]
    fn test_start_watchdog() {
        let mut manager = WatchdogManager::new();

        manager
            .register_watchdog("watchdog0".to_string(), 60, WatchdogAction::Reset)
            .unwrap();
        manager.start_watchdog("watchdog0").unwrap();

        let watchdog = manager.watchdogs.get("watchdog0").unwrap();
        assert_eq!(watchdog.state, WatchdogState::Running);
    }

    #[test]
    fn test_stop_watchdog() {
        let mut manager = WatchdogManager::new();

        manager
            .register_watchdog("watchdog0".to_string(), 60, WatchdogAction::Reset)
            .unwrap();
        manager.start_watchdog("watchdog0").unwrap();
        manager.stop_watchdog("watchdog0").unwrap();

        let watchdog = manager.watchdogs.get("watchdog0").unwrap();
        assert_eq!(watchdog.state, WatchdogState::Stopped);
    }

    #[test]
    fn test_keepalive() {
        let mut manager = WatchdogManager::new();

        manager
            .register_watchdog("watchdog0".to_string(), 60, WatchdogAction::Reset)
            .unwrap();
        manager.start_watchdog("watchdog0").unwrap();
        manager.keepalive("watchdog0").unwrap();

        let watchdog = manager.watchdogs.get("watchdog0").unwrap();
        assert!(watchdog.last_keepalive > 0);
    }

    #[test]
    fn test_set_timeout() {
        let mut manager = WatchdogManager::new();

        manager
            .register_watchdog("watchdog0".to_string(), 60, WatchdogAction::Reset)
            .unwrap();
        manager.set_timeout("watchdog0", 120).unwrap();

        let watchdog = manager.watchdogs.get("watchdog0").unwrap();
        assert_eq!(watchdog.timeout, 120);
    }

    #[test]
    fn test_check_thresholds() {
        let mut manager = WatchdogManager::new();

        manager.update_monitor(95.0, 50.0, 96.0, 50.0);
        let exceeded = manager.check_thresholds();

        assert!(exceeded.contains(&MonitorThreshold::CpuTempCritical));
        assert!(exceeded.contains(&MonitorThreshold::MemoryCritical));
    }

    #[test]
    fn test_active_watchdog() {
        let mut manager = WatchdogManager::new();

        manager
            .register_watchdog("watchdog0".to_string(), 60, WatchdogAction::Reset)
            .unwrap();
        manager.start_watchdog("watchdog0").unwrap();

        let active = manager.active_watchdog();
        assert!(active.is_some());
        assert_eq!(active.unwrap().name, "watchdog0");
    }

    #[test]
    fn test_watchdog_expiration() {
        let mut manager = WatchdogManager::new();

        manager
            .register_watchdog("watchdog0".to_string(), 5, WatchdogAction::Reset)
            .unwrap();
        manager.start_watchdog("watchdog0").unwrap();

        // Simulate time passing
        manager.monitor.uptime = 10;

        let expired = manager.check_watchdog("watchdog0").unwrap();
        assert!(expired);
    }

    #[test]
    fn test_sovereign_daemon_self_healing_vs_systemd() {
        let mut daemon = SovereignDaemonShard::new("network_shard".to_string(), 10.0, 512.0);

        assert_eq!(daemon.state, DaemonState::Running);

        // Trigger first failure (triggers autonomous OOP self-healing)
        let healed1 = daemon.trigger_failure();
        assert!(healed1);
        assert_eq!(daemon.state, DaemonState::SelfHealing);

        // Trigger second failure
        let healed2 = daemon.trigger_failure();
        assert!(healed2);

        // Trigger third failure (reaches limit - hard halts and signals Nix rollback)
        let healed3 = daemon.trigger_failure();
        assert!(!healed3);
        assert_eq!(daemon.state, DaemonState::Halted);
    }

    #[test]
    fn test_daemon_heartbeat_timeout() {
        let mut daemon = SovereignDaemonShard::new("audit_daemon".to_string(), 5.0, 256.0);
        daemon.send_heartbeat(100);

        // Check before timeout (elapsed 5s <= 10s)
        assert!(daemon.check_heartbeat_timeout(105));
        assert!(daemon.heartbeat.is_alive);

        // Check after timeout (elapsed 12s > 10s)
        let healed = daemon.check_heartbeat_timeout(117);
        assert!(healed); // Self-healing triggered
        assert_eq!(daemon.state, DaemonState::SelfHealing);
        assert!(!daemon.heartbeat.is_alive);
    }

    #[test]
    fn test_daemon_exponential_backoff_and_socket_activation() {
        let mut daemon = SovereignDaemonShard::new("httpd_shard".to_string(), 15.0, 1024.0);
        daemon.state = DaemonState::Stopped;

        daemon.socket_activation = Some(SocketActivationConfig {
            port: 80,
            is_bound: true,
            active_connections: 0,
            auto_activate: true,
        });

        // Exponential backoff verification
        assert_eq!(daemon.calculate_backoff_delay(), 2); // 2 * 2^0
        daemon.fail_count = 2;
        assert_eq!(daemon.calculate_backoff_delay(), 8); // 2 * 2^2

        // Socket activation auto-starts daemon
        let activated = daemon.trigger_socket_activity();
        assert!(activated);
        assert_eq!(daemon.state, DaemonState::Running);
        assert_eq!(daemon.socket_activation.as_ref().unwrap().active_connections, 1);
    }

    #[test]
    fn test_sovereign_kdump_engine() {
        let mut kdump = SovereignKdumpEngine::default();
        assert!(kdump.transmit_netconsole_packet().is_err());

        let dump_len = kdump.capture_panic_dump("Kernel Panic: Page Fault", 0xFFFFFFFF81001000, 0x7FFFFFFF0000);
        assert!(dump_len > 0);
        assert!(kdump.panic_occurred);

        let tx_len = kdump.transmit_netconsole_packet().unwrap();
        assert_eq!(tx_len, dump_len);
    }
}
