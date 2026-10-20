// SPDX-License-Identifier: MIT
// SigmaOS - Privilege Manager
// Privilege escalation management inspired by Omarchy privilege handling and Linux sudo/pkexec

#![no_std]
extern crate alloc;

use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// Privilege level
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrivilegeLevel {
    User,
    Admin,
    Root,
}

/// Privilege request
#[derive(Debug, Clone)]
pub struct PrivilegeRequest {
    pub id: String,
    pub command: String,
    pub required_level: PrivilegeLevel,
    pub current_level: PrivilegeLevel,
}

/// Privilege manager
pub struct SigmaPrivilegeManager {
    requests: Vec<PrivilegeRequest>,
}

impl SigmaPrivilegeManager {
    pub fn new() -> Self {
        Self {
            requests: Vec::new(),
        }
    }

    pub fn check_privilege(
        &self,
        required_level: PrivilegeLevel,
        current_level: PrivilegeLevel,
    ) -> bool {
        match required_level {
            PrivilegeLevel::User => true,
            PrivilegeLevel::Admin => {
                current_level == PrivilegeLevel::Admin || current_level == PrivilegeLevel::Root
            }
            PrivilegeLevel::Root => current_level == PrivilegeLevel::Root,
        }
    }

    pub fn request_privilege(
        &mut self,
        command: String,
        required_level: PrivilegeLevel,
        current_level: PrivilegeLevel,
    ) -> PrivilegeRequest {
        let request = PrivilegeRequest {
            id: alloc::format!("req_{}", self.requests.len()),
            command,
            required_level,
            current_level,
        };
        self.requests.push(request.clone());
        request
    }

    pub fn get_requests(&self) -> &[PrivilegeRequest] {
        &self.requests
    }
}

impl Default for SigmaPrivilegeManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_privilege_manager_creation() {
        let manager = SigmaPrivilegeManager::new();
        assert_eq!(manager.get_requests().len(), 0);
    }

    #[test]
    fn test_check_privilege() {
        let manager = SigmaPrivilegeManager::new();
        assert!(manager.check_privilege(PrivilegeLevel::User, PrivilegeLevel::User));
        assert!(manager.check_privilege(PrivilegeLevel::Admin, PrivilegeLevel::Root));
        assert!(!manager.check_privilege(PrivilegeLevel::Root, PrivilegeLevel::Admin));
    }

    #[test]
    fn test_request_privilege() {
        let mut manager = SigmaPrivilegeManager::new();
        manager.request_privilege(
            "test".to_string(),
            PrivilegeLevel::Admin,
            PrivilegeLevel::User,
        );
        assert_eq!(manager.get_requests().len(), 1);
    }
}
