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

// SigmaOS Security Evolution Bridge (SecurityBridge)
// Maps legacy security models (basic Unix DAC, AppArmor profile contexts, legacy SELinux labels) to modern zero-trust capability gates

use crate::klib::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LegacySecurityType {
    UnixDac,
    SELinux,
    AppArmor,
}

pub struct SecurityBridge {
    pub security_type: LegacySecurityType,
    pub legacy_rules: HashMap<String, String>,
}

impl SecurityBridge {
    pub fn new(sec_type: LegacySecurityType) -> Self {
        let mut rules = HashMap::new();
        match sec_type {
            LegacySecurityType::UnixDac => {
                rules.insert("owner_read".to_string(), "sigma_pledge_read".to_string());
                rules.insert("owner_write".to_string(), "sigma_pledge_write".to_string());
            }
            LegacySecurityType::SELinux => {
                rules.insert("unconfined_t".to_string(), "sigma_unconfined_sandbox".to_string());
            }
            LegacySecurityType::AppArmor => {
                rules.insert("/bin/ping".to_string(), "sigma_network_pledge".to_string());
            }
        }
        SecurityBridge {
            security_type: sec_type,
            legacy_rules: rules,
        }
    }

    pub fn map_legacy_rule_to_modern(&self, legacy_rule: &str) -> Option<&String> {
        self.legacy_rules.get(legacy_rule)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_security_bridge_mapping() {
        let bridge = SecurityBridge::new(LegacySecurityType::UnixDac);
        let mapped = bridge.map_legacy_rule_to_modern("owner_read").unwrap();
        assert_eq!(mapped, "sigma_pledge_read");

        let missing = bridge.map_legacy_rule_to_modern("group_exec");
        assert!(missing.is_none());
    }

    #[test]
    fn test_security_bridge_apparmor() {
        let bridge = SecurityBridge::new(LegacySecurityType::AppArmor);
        let mapped = bridge.map_legacy_rule_to_modern("/bin/ping").unwrap();
        assert_eq!(mapped, "sigma_network_pledge");
    }
}
