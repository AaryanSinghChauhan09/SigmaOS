// AppArmor Implementation
// Inspired by Linux AppArmor security module
// Provides mandatory access control for SigmaOS

#![no_std]

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;

/// AppArmor profile name
pub type ProfileName = String;

/// AppArmor profile status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProfileStatus {
    Enforcing,
    Complain,
    Deactivated,
}

/// AppArmor rule element
#[derive(Debug, Clone)]
pub struct RuleElement {
    pub kind: RuleElementKind,
    pub value: String,
}

/// AppArmor rule element kind
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuleElementKind {
    File,
    Capability,
    Network,
    Signal,
    NetworkFamily,
    Tcp,
    Udp,
    Abil,
    Kill,
    Owner,
    Group,
    Tty,
    Time,
    Profile,
    Includes,
    Abstraction,
    Networking,
    Signal2,
    Privilege,
    QualityOfService,
    Ordering,
    ProfileVersion,
    IncludesVersion,
    AbstractionVersion,
}

/// AppArmor profile
#[derive(Debug, Clone)]
pub struct AppArmorProfile {
    pub name: ProfileName,
    pub status: ProfileStatus,
    pub subtypes: Vec<String>,
    pub file_rules: Vec<RuleElement>,
    pub cap_rules: Vec<RuleElement>,
    pub network_rules: Vec<RuleElement>,
}

impl AppArmorProfile {
    pub fn new(name: &str, status: ProfileStatus) -> Self {
        AppArmorProfile {
            name: name.to_string(),
            status,
            subtypes: Vec::new(),
            file_rules: Vec::new(),
            cap_rules: Vec::new(),
            network_rules: Vec::new(),
        }
    }

    pub fn add_file_rule(&mut self, rule: RuleElement) {
        self.file_rules.push(rule);
    }

    pub fn add_cap_rule(&mut self, rule: RuleElement) {
        self.cap_rules.push(rule);
    }

    pub fn add_network_rule(&mut self, rule: RuleElement) {
        self.network_rules.push(rule);
    }

    pub fn set_status(&mut self, status: ProfileStatus) {
        self.status = status;
    }

    pub fn add_subtype(&mut self, subtype: &str) {
        self.subtypes.push(subtype.to_string());
    }
}

/// AppArmor error
#[derive(Debug)]
pub enum AppArmorError {
    InvalidProfile,
    InvalidRule,
    ProfileNotFound,
    ParsingError,
}

impl core::fmt::Display for AppArmorError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            AppArmorError::InvalidProfile => write!(f, "Invalid AppArmor profile"),
            AppArmorError::InvalidRule => write!(f, "Invalid AppArmor rule"),
            AppArmorError::ProfileNotFound => write!(f, "AppArmor profile not found"),
            AppArmorError::ParsingError => write!(f, "AppArmor profile parsing error"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_profile_creation() {
        let mut profile = AppArmorProfile::new("test_profile", ProfileStatus::Enforcing);
        assert_eq!(profile.name, "test_profile");
        assert_eq!(profile.status, ProfileStatus::Enforcing);
    }

    #[test]
    fn test_file_rule_adding() {
        let mut profile = AppArmorProfile::new("test_profile", ProfileStatus::Enforcing);
        let rule = RuleElement {
            kind: RuleElementKind::File,
            value: "/bin/*".to_string(),
        };
        profile.add_file_rule(rule);
        assert_eq!(profile.file_rules.len(), 1);
    }

    #[test]
    fn test_network_rule_adding() {
        let mut profile = AppArmorProfile::new("test_profile", ProfileStatus::Enforcing);
        let rule = RuleElement {
            kind: RuleElementKind::Network,
            value: "allow tcp * *".to_string(),
        };
        profile.add_network_rule(rule);
        assert_eq!(profile.network_rules.len(), 1);
    }
}
