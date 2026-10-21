#![no_std]
extern crate alloc;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// Integrity manager for file integrity checking
/// Inspired by Linux AIDE, tripwire, and integrity verification tools
#[derive(Debug, Clone)]
pub struct SigmaIntegrityManager2 {
    checks: Vec<IntegrityCheck>,
}

/// An integrity check
#[derive(Debug, Clone)]
pub struct IntegrityCheck {
    id: String,
    path: String,
    hash: String,
    algorithm: IntegrityManager2HashAlgorithm,
    state: CheckState,
    last_checked: u64,
}

/// Hash algorithm
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntegrityManager2HashAlgorithm {
    Sha256,
    Sha512,
    Md5,
    Blake2b,
}

/// Check state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CheckState {
    Valid,
    Invalid,
    Pending,
    Error,
}

impl SigmaIntegrityManager2 {
    pub fn new() -> Self {
        Self {
            checks: Vec::new(),
        }
    }

    /// Add an integrity check
    pub fn add_check(&mut self, id: String, path: String, hash: String, algorithm: IntegrityManager2HashAlgorithm) -> Result<String, String> {
        if self.checks.iter().any(|c| c.id == id) {
            return Err(format!("Check with id {} already exists", id));
        }
        
        let check = IntegrityCheck {
            id: id.clone(),
            path,
            hash,
            algorithm,
            state: CheckState::Pending,
            last_checked: 0,
        };
        self.checks.push(check);
        Ok(id)
    }

    /// Remove a check
    pub fn remove_check(&mut self, id: &str) -> Result<(), String> {
        let pos = self.checks.iter().position(|c| c.id == id)
            .ok_or_else(|| format!("Check with id {} not found", id))?;
        self.checks.remove(pos);
        Ok(())
    }

    /// Verify an integrity check
    pub fn verify(&mut self, id: &str, current_hash: String) -> Result<(), String> {
        let check = self.checks.iter_mut().find(|c| c.id == id)
            .ok_or_else(|| format!("Check with id {} not found", id))?;
        
        if check.hash == current_hash {
            check.state = CheckState::Valid;
        } else {
            check.state = CheckState::Invalid;
        }
        check.last_checked = 0; // Would be set to current time
        Ok(())
    }

    /// Update hash for a check
    pub fn update_hash(&mut self, id: &str, new_hash: String) -> Result<(), String> {
        let check = self.checks.iter_mut().find(|c| c.id == id)
            .ok_or_else(|| format!("Check with id {} not found", id))?;
        check.hash = new_hash;
        check.state = CheckState::Pending;
        Ok(())
    }

    /// Get all checks
    pub fn get_checks(&self) -> &[IntegrityCheck] {
        &self.checks
    }

    /// Get valid checks
    pub fn get_valid_checks(&self) -> Vec<&IntegrityCheck> {
        self.checks.iter().filter(|c| c.state == CheckState::Valid).collect()
    }

    /// Get invalid checks
    pub fn get_invalid_checks(&self) -> Vec<&IntegrityCheck> {
        self.checks.iter().filter(|c| c.state == CheckState::Invalid).collect()
    }

    /// Get check by ID
    pub fn get_check(&self, id: &str) -> Option<&IntegrityCheck> {
        self.checks.iter().find(|c| c.id == id)
    }
}

impl Default for SigmaIntegrityManager2 {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_check() {
        let mut manager = SigmaIntegrityManager2::new();
        let result = manager.add_check("check1".to_string(), "/bin/bash".to_string(), "abc123".to_string(), IntegrityManager2HashAlgorithm::Sha256);
        assert!(result.is_ok());
        assert_eq!(manager.get_checks().len(), 1);
    }

    #[test]
    fn test_duplicate_check() {
        let mut manager = SigmaIntegrityManager2::new();
        manager.add_check("check1".to_string(), "/bin/bash".to_string(), "abc123".to_string(), IntegrityManager2HashAlgorithm::Sha256).unwrap();
        let result = manager.add_check("check1".to_string(), "/bin/sh".to_string(), "def456".to_string(), IntegrityManager2HashAlgorithm::Sha256);
        assert!(result.is_err());
    }

    #[test]
    fn test_verify_valid() {
        let mut manager = SigmaIntegrityManager2::new();
        manager.add_check("check1".to_string(), "/bin/bash".to_string(), "abc123".to_string(), IntegrityManager2HashAlgorithm::Sha256).unwrap();
        
        manager.verify("check1", "abc123".to_string()).unwrap();
        assert_eq!(manager.get_check("check1").unwrap().state, CheckState::Valid);
    }

    #[test]
    fn test_verify_invalid() {
        let mut manager = SigmaIntegrityManager2::new();
        manager.add_check("check1".to_string(), "/bin/bash".to_string(), "abc123".to_string(), IntegrityManager2HashAlgorithm::Sha256).unwrap();
        
        manager.verify("check1", "def456".to_string()).unwrap();
        assert_eq!(manager.get_check("check1").unwrap().state, CheckState::Invalid);
    }
}
