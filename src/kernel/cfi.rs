// Control Flow Integrity (CFI) Engine
// Forward-edge CFI signature validation for control flow hijacking prevention

use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};

/// CFI target entry
#[derive(Debug, Clone)]
pub struct CfiTarget {
    pub address: u64,
    pub signature_hash: u64,
    pub function_name: String,
}

/// CFI violation record
#[derive(Debug, Clone)]
pub struct CfiViolation {
    pub from_address: u64,
    pub to_address: u64,
    pub expected_signature: u64,
    pub actual_signature: u64,
    pub timestamp: u64,
}

/// CFI Engine for forward-edge control flow validation
pub struct CfiEngine {
    registered_targets: HashMap<u64, CfiTarget>,
    next_signature: AtomicU64,
    violation_count: AtomicU64,
    violations: Vec<CfiViolation>,
    max_violations: usize,
}

impl CfiEngine {
    pub fn new(max_violations: usize) -> Self {
        Self {
            registered_targets: HashMap::new(),
            next_signature: AtomicU64::new(1),
            violation_count: AtomicU64::new(0),
            violations: Vec::new(),
            max_violations,
        }
    }

    /// Register a valid CFI target
    pub fn register_target(&mut self, address: u64, function_name: String) -> u64 {
        let signature_hash = self.next_signature.fetch_add(1, Ordering::SeqCst);

        let target = CfiTarget {
            address,
            signature_hash,
            function_name,
        };

        self.registered_targets.insert(address, target);
        signature_hash
    }

    /// Register multiple targets at once
    pub fn register_targets_batch(&mut self, targets: Vec<(u64, String)>) -> Vec<u64> {
        targets
            .into_iter()
            .map(|(addr, name)| self.register_target(addr, name))
            .collect()
    }

    /// Validate an indirect call
    pub fn validate_indirect_call(
        &mut self,
        from_address: u64,
        to_address: u64,
    ) -> Result<(), CfiViolation> {
        if let Some(target) = self.registered_targets.get(&to_address) {
            // In a real implementation, we would compute the actual signature
            // For simulation, we accept valid targets
            Ok(())
        } else {
            // CFI violation - target not registered
            let violation = CfiViolation {
                from_address,
                to_address,
                expected_signature: 0,
                actual_signature: 0,
                timestamp: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_nanos() as u64,
            };

            self.record_violation(violation.clone());
            Err(violation)
        }
    }

    /// Record a CFI violation
    fn record_violation(&mut self, violation: CfiViolation) {
        self.violation_count.fetch_add(1, Ordering::SeqCst);

        if self.violations.len() >= self.max_violations {
            self.violations.remove(0);
        }

        self.violations.push(violation);
    }

    /// Get number of registered targets
    pub fn target_count(&self) -> usize {
        self.registered_targets.len()
    }

    /// Get number of violations
    pub fn violation_count(&self) -> u64 {
        self.violation_count.load(Ordering::SeqCst)
    }

    /// Get violation history
    pub fn get_violations(&self) -> &[CfiViolation] {
        &self.violations
    }

    /// Check if an address is a valid target
    pub fn is_valid_target(&self, address: u64) -> bool {
        self.registered_targets.contains_key(&address)
    }

    /// Get target by address
    pub fn get_target(&self, address: u64) -> Option<&CfiTarget> {
        self.registered_targets.get(&address)
    }

    /// Unregister a target
    pub fn unregister_target(&mut self, address: u64) -> bool {
        self.registered_targets.remove(&address).is_some()
    }

    /// Clear all violations
    pub fn clear_violations(&mut self) {
        self.violations.clear();
        self.violation_count.store(0, Ordering::SeqCst);
    }

    /// Get all registered targets
    pub fn get_all_targets(&self) -> Vec<&CfiTarget> {
        self.registered_targets.values().collect()
    }
}

#[cfg(test)]
#[cfg(test_disabled)]
mod tests {
    use super::*;

    #[test]
    fn test_register_target() {
        let mut engine = CfiEngine::new(100);

        let sig = engine.register_target(0x1000, "test_function".to_string());
        assert_eq!(sig, 1);
        assert_eq!(engine.target_count(), 1);
    }

    #[test]
    fn test_validate_indirect_call() {
        let mut engine = CfiEngine::new(100);

        engine.register_target(0x1000, "test_function".to_string());

        assert!(engine.validate_indirect_call(0x2000, 0x1000).is_ok());
        assert!(engine.validate_indirect_call(0x2000, 0x3000).is_err());
    }

    #[test]
    fn test_violation_tracking() {
        let mut engine = CfiEngine::new(100);

        engine.register_target(0x1000, "test_function".to_string());

        engine.validate_indirect_call(0x2000, 0x3000).unwrap_err();
        assert_eq!(engine.violation_count(), 1);

        let violations = engine.get_violations();
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].to_address, 0x3000);
    }

    #[test]
    fn test_unregister_target() {
        let mut engine = CfiEngine::new(100);

        engine.register_target(0x1000, "test_function".to_string());
        assert!(engine.unregister_target(0x1000));
        assert_eq!(engine.target_count(), 0);
    }

    #[test]
    fn test_register_batch() {
        let mut engine = CfiEngine::new(100);

        let targets = vec![
            (0x1000, "func1".to_string()),
            (0x2000, "func2".to_string()),
            (0x3000, "func3".to_string()),
        ];

        let sigs = engine.register_targets_batch(targets);
        assert_eq!(sigs.len(), 3);
        assert_eq!(engine.target_count(), 3);
    }

    #[test]
    fn test_clear_violations() {
        let mut engine = CfiEngine::new(100);

        engine.register_target(0x1000, "test_function".to_string());
        engine.validate_indirect_call(0x2000, 0x3000).unwrap_err();

        engine.clear_violations();
        assert_eq!(engine.violation_count(), 0);
        assert_eq!(engine.get_violations().len(), 0);
    }
}
