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
    #[allow(clippy::new_without_default)]
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

/// Represents a registered shard's heartbeat state
#[derive(Debug, Clone)]
pub struct ShardHeartbeat {
    pub shard_name: String,
    pub last_heartbeat_timestamp: u64,
    pub max_allowed_latency_secs: u64,
}

/// Double Fault Guard & Cascade Protection
/// Tracks sequential recovery failures within small observation windows
#[derive(Debug, Clone)]
pub struct DoubleFaultGuard {
    pub recovery_timestamps: Vec<u64>,
    pub max_allowed_failures: u32,
    pub observation_window_secs: u64,
    pub is_isolated: bool,
}

impl DoubleFaultGuard {
    pub fn new(max_failures: u32, window_secs: u64) -> Self {
        Self {
            recovery_timestamps: Vec::new(),
            max_allowed_failures: max_failures,
            observation_window_secs: window_secs,
            is_isolated: false,
        }
    }

    /// Records a recovery attempt. Returns an Error if cascading failure threshold is breached.
    pub fn record_attempt(&mut self, timestamp: u64) -> Result<(), &'static str> {
        if self.is_isolated {
            return Err("Double Fault Guard Active: Shard is isolated. Restart blocked to prevent cascading crash loops.");
        }

        self.recovery_timestamps.push(timestamp);

        // Remove timestamps older than the observation window
        let window_start = timestamp.saturating_sub(self.observation_window_secs);
        self.recovery_timestamps.retain(|&t| t >= window_start);

        if self.recovery_timestamps.len() > self.max_allowed_failures as usize {
            self.is_isolated = true;
            return Err("Double Fault Guard Triggered: Cascading failures detected! Shard isolated to protect microkernel stability.");
        }

        Ok(())
    }
}

/// Highly robust System Stability and Fault Tolerance Monitor
pub struct SystemStabilityMonitor {
    pub heartbeats: HashMap<String, ShardHeartbeat>,
    pub fault_guards: HashMap<String, DoubleFaultGuard>,
    pub system_health_score: u32, // 0 to 100
    pub is_degraded_safety_mode: bool,
}

impl SystemStabilityMonitor {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            heartbeats: HashMap::new(),
            fault_guards: HashMap::new(),
            system_health_score: 100,
            is_degraded_safety_mode: false,
        }
    }

    pub fn register_shard(&mut self, shard_name: &str, max_latency_secs: u64) {
        let heartbeat = ShardHeartbeat {
            shard_name: shard_name.to_string(),
            last_heartbeat_timestamp: 0,
            max_allowed_latency_secs: max_latency_secs,
        };
        self.heartbeats.insert(shard_name.to_string(), heartbeat);
        // Default guard rule: max 3 crashes within 10 seconds before isolating
        self.fault_guards
            .insert(shard_name.to_string(), DoubleFaultGuard::new(3, 10));
    }

    pub fn send_heartbeat(&mut self, shard_name: &str, timestamp: u64) -> Result<(), &'static str> {
        let heartbeat = self
            .heartbeats
            .get_mut(shard_name)
            .ok_or("Shard not registered")?;
        heartbeat.last_heartbeat_timestamp = timestamp;
        Ok(())
    }

    /// Checks the health of all registered shards. Returns lists of dead/unresponsive shard names.
    pub fn check_shards_health(&mut self, current_time_secs: u64) -> Vec<String> {
        let mut dead_shards = Vec::new();
        for heartbeat in self.heartbeats.values() {
            // Heartbeat of 0 represents never booted yet
            if heartbeat.last_heartbeat_timestamp == 0 {
                continue;
            }
            let latency = current_time_secs.saturating_sub(heartbeat.last_heartbeat_timestamp);
            if latency > heartbeat.max_allowed_latency_secs {
                dead_shards.push(heartbeat.shard_name.clone());
            }
        }

        // Deduct 15 points of health for each unresponsive dead shard
        let total_deductions = (dead_shards.len() as u32) * 15;
        self.system_health_score = 100u32.saturating_sub(total_deductions);

        // Degraded safety mode triggers automatically if health falls below 50%
        if self.system_health_score < 50 {
            self.is_degraded_safety_mode = true;
        } else {
            self.is_degraded_safety_mode = false;
        }

        dead_shards
    }

    /// Tracks recovery crashes. Prevents endless restart cascades.
    pub fn record_recovery_attempt(
        &mut self,
        shard_name: &str,
        timestamp: u64,
    ) -> Result<(), &'static str> {
        let guard = self
            .fault_guards
            .get_mut(shard_name)
            .ok_or("Fault guard not registered for shard")?;
        guard.record_attempt(timestamp)
    }
}

impl Default for SystemStabilityMonitor {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for SelfHealingModule {
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
    fn test_shard_heartbeats_and_degraded_safety_mode() {
        let mut monitor = SystemStabilityMonitor::new();
        assert_eq!(monitor.system_health_score, 100);
        assert!(!monitor.is_degraded_safety_mode);

        monitor.register_shard("S-MM", 5); // 5 seconds max allowed latency
        monitor.register_shard("S-NET", 5);

        // Send heartbeats
        assert!(monitor.send_heartbeat("S-MM", 1000).is_ok());
        assert!(monitor.send_heartbeat("S-NET", 1000).is_ok());

        // Check health at t=1002 (all healthy)
        let dead = monitor.check_shards_health(1002);
        assert!(dead.is_empty());
        assert_eq!(monitor.system_health_score, 100);
        assert!(!monitor.is_degraded_safety_mode);

        // Check health at t=1010 (both are timed out since last heartbeat was at 1000, 10s > 5s max latency)
        let dead_now = monitor.check_shards_health(1010);
        assert_eq!(dead_now.len(), 2);
        assert!(dead_now.contains(&"S-MM".to_string()));
        assert!(dead_now.contains(&"S-NET".to_string()));

        // Deductions should drop health below 50 (100 - 30 = 70. Wait, 2 dead shards * 15 = 30 deduction. Health = 70.
        // Let's add more shards to trigger Degraded Safety Mode below 50%)
        monitor.register_shard("S-FS", 5);
        monitor.register_shard("S-SCHED", 5);
        monitor.send_heartbeat("S-FS", 1000).unwrap();
        monitor.send_heartbeat("S-SCHED", 1000).unwrap();

        let dead_four = monitor.check_shards_health(1010);
        assert_eq!(dead_four.len(), 4); // 4 dead shards * 15 deduction = 60. Health = 40%
        assert_eq!(monitor.system_health_score, 40);
        assert!(monitor.is_degraded_safety_mode); // Activated because health fell below 50%!
    }

    #[test]
    fn test_double_fault_guard_cascades() {
        let mut monitor = SystemStabilityMonitor::new();
        monitor.register_shard("S-MM", 5);

        // Simulate successful recovery attempts separated by time
        assert!(monitor.record_recovery_attempt("S-MM", 1000).is_ok());
        assert!(monitor.record_recovery_attempt("S-MM", 1005).is_ok());
        assert!(monitor.record_recovery_attempt("S-MM", 1012).is_ok());

        // Simulate rapid crashing cascade: 4 crashes within 10 seconds (observation window)
        let mut guard = DoubleFaultGuard::new(3, 10);
        assert!(guard.record_attempt(1000).is_ok());
        assert!(guard.record_attempt(1001).is_ok());
        assert!(guard.record_attempt(1002).is_ok());

        // 4th crash within 10 seconds of 1000 (at 1003) -> threshold (3) exceeded, triggers isolation!
        let crash_err = guard.record_attempt(1003);
        assert!(crash_err.is_err());
        assert!(guard.is_isolated);

        // Successive restarts are blocked to protect microkernel stability
        assert!(guard.record_attempt(1004).is_err());
    }
}
