// SigmaOS Resilience and Self-Healing Modules
// Event-driven recovery and rollback snapshots

use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

/// Recovery event type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecoveryEventType {
    ProcessCrash,
    ServiceFailure,
    MemoryExhaustion,
    DiskError,
    NetworkFailure,
    SecurityViolation,
    KernelPanic,
}

/// Recovery action
#[derive(Debug, Clone)]
pub enum RecoveryAction {
    RestartProcess { pid: u64 },
    RestartService { name: String },
    ClearCache,
    RollbackSnapshot { snapshot_id: String },
    EnableSafeMode,
    NotifyAdmin { message: String },
    LogEvent { message: String },
}

/// System snapshot
#[derive(Debug, Clone)]
pub struct SystemSnapshot {
    pub id: String,
    pub timestamp: u64,
    pub system_state: HashMap<String, String>,
    pub configuration: HashMap<String, String>,
    pub description: String,
}

impl SystemSnapshot {
    pub fn new(description: String) -> Self {
        let timestamp_nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0);
        Self {
            id: format!("snap-{}", timestamp_nanos),
            timestamp: (timestamp_nanos / 1_000_000_000) as u64,
            system_state: HashMap::new(),
            configuration: HashMap::new(),
            description,
        }
    }

    pub fn with_state(mut self, key: String, value: String) -> Self {
        self.system_state.insert(key, value);
        self
    }

    pub fn with_config(mut self, key: String, value: String) -> Self {
        self.configuration.insert(key, value);
        self
    }
}

/// Recovery rule
#[derive(Debug, Clone)]
pub struct RecoveryRule {
    pub event_type: RecoveryEventType,
    pub condition: String,
    pub actions: Vec<RecoveryAction>,
    pub priority: u32,
    pub enabled: bool,
}

impl RecoveryRule {
    pub fn new(event_type: RecoveryEventType, condition: String) -> Self {
        Self {
            event_type,
            condition,
            actions: Vec::new(),
            priority: 0,
            enabled: true,
        }
    }

    pub fn with_action(mut self, action: RecoveryAction) -> Self {
        self.actions.push(action);
        self
    }

    pub fn with_priority(mut self, priority: u32) -> Self {
        self.priority = priority;
        self
    }

    pub fn matches(
        &self,
        event_type: RecoveryEventType,
        context: &HashMap<String, String>,
    ) -> bool {
        if self.event_type != event_type {
            return false;
        }

        // Simple condition matching (in production, use proper expression evaluation)
        true
    }
}

/// Self-healing module
pub struct SelfHealingModule {
    pub snapshots: Vec<SystemSnapshot>,
    pub recovery_rules: Vec<RecoveryRule>,
    pub event_log: Vec<(RecoveryEventType, u64)>,
    pub auto_recovery_enabled: bool,
    pub max_snapshots: usize,
}

impl SelfHealingModule {
    pub fn new() -> Self {
        let mut module = Self {
            snapshots: Vec::new(),
            recovery_rules: Vec::new(),
            event_log: Vec::new(),
            auto_recovery_enabled: true,
            max_snapshots: 10,
        };

        module.add_default_rules();
        module
    }

    fn add_default_rules(&mut self) {
        // Process crash recovery
        let process_crash_rule =
            RecoveryRule::new(RecoveryEventType::ProcessCrash, "process_crash".to_string())
                .with_action(RecoveryAction::RestartProcess { pid: 0 })
                .with_action(RecoveryAction::LogEvent {
                    message: "Process crashed, attempting restart".to_string(),
                })
                .with_priority(10);

        // Memory exhaustion recovery
        let memory_exhaustion_rule = RecoveryRule::new(
            RecoveryEventType::MemoryExhaustion,
            "memory_exhaustion".to_string(),
        )
        .with_action(RecoveryAction::ClearCache)
        .with_action(RecoveryAction::LogEvent {
            message: "Memory exhausted, clearing cache".to_string(),
        })
        .with_priority(15);

        // Service failure recovery
        let service_failure_rule = RecoveryRule::new(
            RecoveryEventType::ServiceFailure,
            "service_failure".to_string(),
        )
        .with_action(RecoveryAction::RestartService {
            name: String::new(),
        })
        .with_action(RecoveryAction::LogEvent {
            message: "Service failed, attempting restart".to_string(),
        })
        .with_priority(10);

        self.recovery_rules.push(process_crash_rule);
        self.recovery_rules.push(memory_exhaustion_rule);
        self.recovery_rules.push(service_failure_rule);
    }

    pub fn create_snapshot(&mut self, description: String) -> String {
        let snapshot = SystemSnapshot::new(description);
        let id = snapshot.id.clone();

        self.snapshots.push(snapshot);

        // Keep only max_snapshots
        if self.snapshots.len() > self.max_snapshots {
            self.snapshots.remove(0);
        }

        id
    }

    pub fn get_snapshot(&self, id: &str) -> Option<&SystemSnapshot> {
        self.snapshots.iter().find(|s| s.id == id)
    }

    pub fn rollback_to_snapshot(&mut self, id: &str) -> Result<(), ResilienceError> {
        let snapshot = self
            .get_snapshot(id)
            .ok_or(ResilienceError::SnapshotNotFound)?;
        if !self.snapshots.iter().any(|s| s.id == id) {
            return Err(ResilienceError::SnapshotNotFound);
        }

        let snapshot = self.get_snapshot(id).unwrap();
        let snapshot = self.get_snapshot(id).ok_or(ResilienceError::SnapshotNotFound)?;
        println!("Rolling back to snapshot: {}", snapshot.description);

        // Simulate rollback
        Ok(())
    }

    pub fn handle_event(
        &mut self,
        event_type: RecoveryEventType,
        context: HashMap<String, String>,
    ) -> Vec<RecoveryAction> {
        // Log the event
        self.event_log.push((
            event_type,
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map(|d| d.as_secs())
                .unwrap_or(0),
        ));

        if !self.auto_recovery_enabled {
            return Vec::new();
        }

        let mut actions = Vec::new();

        // Find matching rules
        for rule in &self.recovery_rules {
            if rule.matches(event_type, &context) && rule.enabled {
                actions.extend(rule.actions.clone());
            }
        }

        // Sort by priority
        actions.sort_by(|a, b| {
            let priority_a = self.get_action_priority(a);
            let priority_b = self.get_action_priority(b);
            priority_b.cmp(&priority_a)
        });

        actions
    }

    fn get_action_priority(&self, action: &RecoveryAction) -> u32 {
        match action {
            RecoveryAction::RestartProcess { .. } => 10,
            RecoveryAction::RestartService { .. } => 10,
            RecoveryAction::ClearCache => 5,
            RecoveryAction::RollbackSnapshot { .. } => 20,
            RecoveryAction::EnableSafeMode => 15,
            RecoveryAction::NotifyAdmin { .. } => 1,
            RecoveryAction::LogEvent { .. } => 0,
        }
    }

    pub fn execute_recovery_action(
        &mut self,
        action: RecoveryAction,
    ) -> Result<(), ResilienceError> {
        match action {
            RecoveryAction::RestartProcess { pid } => {
                println!("Restarting process with PID: {}", pid);
            }
            RecoveryAction::RestartService { name } => {
                println!("Restarting service: {}", name);
            }
            RecoveryAction::ClearCache => {
                println!("Clearing system cache");
            }
            RecoveryAction::RollbackSnapshot { snapshot_id } => {
                self.rollback_to_snapshot(&snapshot_id)?;
            }
            RecoveryAction::EnableSafeMode => {
                println!("Enabling safe mode");
            }
            RecoveryAction::NotifyAdmin { message } => {
                println!("Notifying admin: {}", message);
            }
            RecoveryAction::LogEvent { message } => {
                println!("Logging event: {}", message);
            }
        }
        Ok(())
    }

    pub fn add_recovery_rule(&mut self, rule: RecoveryRule) {
        self.recovery_rules.push(rule);
    }

    pub fn enable_auto_recovery(&mut self) {
        self.auto_recovery_enabled = true;
    }

    pub fn disable_auto_recovery(&mut self) {
        self.auto_recovery_enabled = false;
    }

    pub fn get_snapshots(&self) -> &[SystemSnapshot] {
        &self.snapshots
    }

    pub fn get_event_log(&self) -> &[(RecoveryEventType, u64)] {
        &self.event_log
    }
}

impl Default for SelfHealingModule {
    fn default() -> Self {
        Self::new()
    }
}

/// Tracks a registered system shard/component's heartbeat status
#[derive(Debug, Clone)]
pub struct ShardHeartbeat {
    pub name: String,
    pub last_ping_secs: u64,
    pub latency_ms: u32,
    pub is_responsive: bool,
}

/// Prevents recursive recovery cascades by tracking recovery attempts per resource.
/// If recovery fails repeatedly within a window, triggers safe mode fallback.
#[derive(Debug, Clone)]
pub struct DoubleFaultGuard {
    pub recovery_counts: HashMap<String, usize>,
    pub threshold_limit: usize,
}

impl DoubleFaultGuard {
    pub fn new(threshold_limit: usize) -> Self {
        Self {
            recovery_counts: HashMap::new(),
            threshold_limit,
        }
    }

    /// Increments recovery attempt. Returns true if recursive fault threshold is breached (Double Fault detected)
    pub fn register_attempt(&mut self, resource: &str) -> bool {
        let count = self.recovery_counts.entry(resource.to_string()).or_insert(0);
        *count += 1;
        *count >= self.threshold_limit
    }

    /// Reset recovery count for a resource upon successful restoration
    pub fn reset_attempts(&mut self, resource: &str) {
        self.recovery_counts.remove(resource);
    }
}

/// Comprehensive stability and resilience monitor (Sovereign OS Parity)
pub struct SystemStabilityMonitor {
    pub heartbeats: HashMap<String, ShardHeartbeat>,
    pub double_fault_guard: DoubleFaultGuard,
    pub system_stability_score: f64, // 0.0 to 100.0
    pub in_safe_mode: bool,
}

impl SystemStabilityMonitor {
    pub fn new() -> Self {
        let mut heartbeats = HashMap::new();
        // Register default essential kernel shards
        for shard in &["kernel", "vfs", "scheduler", "ipc"] {
            heartbeats.insert(
                shard.to_string(),
                ShardHeartbeat {
                    name: shard.to_string(),
                    last_ping_secs: 0,
                    latency_ms: 0,
                    is_responsive: true,
                },
            );
        }
        Self {
            heartbeats,
            double_fault_guard: DoubleFaultGuard::new(3), // 3 failures triggers double fault
            system_stability_score: 100.0,
            in_safe_mode: false,
        }
    }

    /// Updates shard ping. Recalculates system stability score based on responsiveness and latencies
    pub fn ping_shard(&mut self, name: &str, latency_ms: u32, is_responsive: bool) {
        if let Some(hb) = self.heartbeats.get_mut(name) {
            hb.latency_ms = latency_ms;
            hb.is_responsive = is_responsive;
            hb.last_ping_secs = 123456; // Simulated timestamp
        }

        // Calculate score
        let mut responsive_count = 0;
        let mut total_latency = 0;
        for hb in self.heartbeats.values() {
            if hb.is_responsive {
                responsive_count += 1;
                total_latency += hb.latency_ms;
            }
        }

        let responsiveness_factor = (responsive_count as f64 / self.heartbeats.len() as f64) * 70.0;
        // Average latency under 50ms is perfect. Penalize overhead.
        let avg_latency = if responsive_count > 0 {
            total_latency as f64 / responsive_count as f64
        } else {
            0.0
        };
        let latency_penalty = (avg_latency / 10.0).min(30.0);
        let stability = (responsiveness_factor + (30.0 - latency_penalty)).clamp(0.0, 100.0);
        self.system_stability_score = stability;

        // Auto safe mode degradation if stability score falls below 50%
        if self.system_stability_score < 50.0 {
            self.in_safe_mode = true;
        }
    }

    /// Registers a fault event for a component. Triggers safe mode if recursive double-fault is caught.
    pub fn trigger_recovery_for_fault(&mut self, resource: &str) -> &'static str {
        if self.double_fault_guard.register_attempt(resource) {
            self.in_safe_mode = true;
            "DOUBLE_FAULT_DETECTED: DEGRADED_TO_SAFE_MODE"
        } else {
            "ATTEMPTING_RECOVERY"
        }
    }

    /// Clear fault counts upon successful manual or automated recovery
    pub fn clear_fault(&mut self, resource: &str) {
        self.double_fault_guard.reset_attempts(resource);
    }
}

impl Default for SystemStabilityMonitor {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct ShardHeartbeat {
    pub shard_name: String,
    pub last_heartbeat_timestamp: u64,
    pub latency_ms: u32,
    pub is_responsive: bool,
}

pub struct DoubleFaultGuard {
    pub consecutive_failures: u32,
    pub max_allowed_failures: u32,
    pub safety_mode_activated: bool,
}

impl DoubleFaultGuard {
    pub fn new(max_allowed_failures: u32) -> Self {
        Self {
            consecutive_failures: 0,
            max_allowed_failures,
            safety_mode_activated: false,
        }
    }

    pub fn record_failure(&mut self) -> bool {
        self.consecutive_failures += 1;
        if self.consecutive_failures >= self.max_allowed_failures {
            self.safety_mode_activated = true;
        }
        self.safety_mode_activated
    }

    pub fn record_success(&mut self) {
        self.consecutive_failures = 0;
        self.safety_mode_activated = false;
    }
}

pub struct SystemStabilityMonitor {
    pub shards: HashMap<String, ShardHeartbeat>,
    pub fault_guard: DoubleFaultGuard,
}

impl SystemStabilityMonitor {
    pub fn new() -> Self {
        Self {
            shards: HashMap::new(),
            fault_guard: DoubleFaultGuard::new(2), // Max 2 consecutive failures triggers safety-mode
        }
    }

    pub fn report_heartbeat(&mut self, shard_name: String, timestamp: u64, latency_ms: u32) {
        let is_responsive = latency_ms < 500; // Unresponsive if latency >= 500ms
        let shard = ShardHeartbeat {
            shard_name: shard_name.clone(),
            last_heartbeat_timestamp: timestamp,
            latency_ms,
            is_responsive,
        };
        self.shards.insert(shard_name, shard);
    }

    pub fn check_overall_health(&mut self) -> u32 {
        let mut responsive_count = 0;
        let total_count = self.shards.len();
        if total_count == 0 {
            return 100;
        }

        for shard in self.shards.values() {
            if shard.is_responsive {
                responsive_count += 1;
            }
        }

        let health_percent = (responsive_count * 100) / total_count;
        if health_percent < 50 {
            self.fault_guard.record_failure();
        } else {
            self.fault_guard.record_success();
        }

        health_percent as u32
    }
}

impl Default for SystemStabilityMonitor {
    fn default() -> Self {
        Self::new()
    }
}

/// Resilience errors
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResilienceError {
    SnapshotNotFound,
    RollbackFailed,
    InvalidRule,
    RecoveryFailed,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_creation() {
        let module = SelfHealingModule::new();
        assert!(module.auto_recovery_enabled);
        assert_eq!(module.recovery_rules.len(), 3);
    }

    #[test]
    fn test_snapshot_creation() {
        let mut module = SelfHealingModule::new();
        let id = module.create_snapshot("Test snapshot".to_string());
        assert_eq!(module.snapshots.len(), 1);
        assert_eq!(module.snapshots[0].id, id);
    }

    #[test]
    fn test_event_handling() {
        let mut module = SelfHealingModule::new();
        let actions = module.handle_event(RecoveryEventType::ProcessCrash, HashMap::new());
        assert!(!actions.is_empty());
    }

    #[test]
    fn test_auto_recovery_toggle() {
        let mut module = SelfHealingModule::new();
        module.disable_auto_recovery();
        assert!(!module.auto_recovery_enabled);
        module.enable_auto_recovery();
        assert!(module.auto_recovery_enabled);
    }

    #[test]
    fn test_rollback() {
        let mut module = SelfHealingModule::new();
        let id = module.create_snapshot("Test snapshot".to_string());
        assert!(module.rollback_to_snapshot(&id).is_ok());
    }

    #[test]
    fn test_invalid_rollback() {
        let mut module = SelfHealingModule::new();
        assert!(module.rollback_to_snapshot("invalid_id").is_err());
    }

    #[test]
    fn test_system_stability_monitor_heartbeat() {
        let mut monitor = SystemStabilityMonitor::new();
        assert_eq!(monitor.system_stability_score, 100.0);
        assert!(!monitor.in_safe_mode);

        // Ping kernel shard with fast response
        monitor.ping_shard("kernel", 10, true);
        // Ping scheduler with very slow response (150ms)
        monitor.ping_shard("scheduler", 150, true);

        // Stability score should adjust, but still responsive enough to avoid safe mode
        assert!(monitor.system_stability_score < 100.0);
        assert!(!monitor.in_safe_mode);

        // Mark scheduler, vfs, and ipc as unresponsive to crash the stability score
        monitor.ping_shard("scheduler", 0, false);
        monitor.ping_shard("vfs", 0, false);
        monitor.ping_shard("ipc", 0, false);

        assert!(monitor.system_stability_score < 50.0);
        assert!(monitor.in_safe_mode);
    }

    #[test]
    fn test_double_fault_guard_trigger() {
        let mut monitor = SystemStabilityMonitor::default();
        assert!(!monitor.in_safe_mode);

        // Register first attempt
        let status1 = monitor.trigger_recovery_for_fault("filesystem_corrupt");
        assert_eq!(status1, "ATTEMPTING_RECOVERY");
        assert!(!monitor.in_safe_mode);

        // Register second attempt
        let status2 = monitor.trigger_recovery_for_fault("filesystem_corrupt");
        assert_eq!(status2, "ATTEMPTING_RECOVERY");
        assert!(!monitor.in_safe_mode);

        // Register third attempt (breaching threshold of 3)
        let status3 = monitor.trigger_recovery_for_fault("filesystem_corrupt");
        assert_eq!(status3, "DOUBLE_FAULT_DETECTED: DEGRADED_TO_SAFE_MODE");
        assert!(monitor.in_safe_mode);

        // Clear fault and confirm reset works
        monitor.clear_fault("filesystem_corrupt");
        monitor.in_safe_mode = false;
        let status_after_clear = monitor.trigger_recovery_for_fault("filesystem_corrupt");
        assert_eq!(status_after_clear, "ATTEMPTING_RECOVERY");
        assert!(!monitor.in_safe_mode);
    }

    #[test]
    fn test_double_fault_guard_and_heartbeats() {
        let mut monitor = SystemStabilityMonitor::new();
        assert_eq!(monitor.check_overall_health(), 100);

        // Report normal heartbeats
        monitor.report_heartbeat("network_shard".to_string(), 1718900000, 50);
        monitor.report_heartbeat("audio_shard".to_string(), 1718900000, 120);
        assert_eq!(monitor.check_overall_health(), 100);
        assert!(!monitor.fault_guard.safety_mode_activated);

        // Report high latency (unresponsive) on one shard
        monitor.report_heartbeat("audio_shard".to_string(), 1718900100, 600); // unresponsive
        assert_eq!(monitor.check_overall_health(), 50); // 50% responsive
        assert!(!monitor.fault_guard.safety_mode_activated);

        // Report unresponsive on both shards -> health falls below 50%
        monitor.report_heartbeat("network_shard".to_string(), 1718900100, 750); // unresponsive
        assert_eq!(monitor.check_overall_health(), 0); // 0% responsive, triggers first failure
        assert_eq!(monitor.fault_guard.consecutive_failures, 1);
        assert!(!monitor.fault_guard.safety_mode_activated);

        // Second check with 0% responsive triggers second failure -> activates safety-mode
        assert_eq!(monitor.check_overall_health(), 0);
        assert_eq!(monitor.fault_guard.consecutive_failures, 2);
        assert!(monitor.fault_guard.safety_mode_activated); // Safety mode successfully locked!

        // Back to normal responsive state clears failure counters
        monitor.report_heartbeat("network_shard".to_string(), 1718900200, 50);
        monitor.report_heartbeat("audio_shard".to_string(), 1718900200, 50);
        assert_eq!(monitor.check_overall_health(), 100);
        assert_eq!(monitor.fault_guard.consecutive_failures, 0);
        assert!(!monitor.fault_guard.safety_mode_activated);
    }
}
