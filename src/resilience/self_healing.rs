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
        _context: &HashMap<String, String>,
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
}
