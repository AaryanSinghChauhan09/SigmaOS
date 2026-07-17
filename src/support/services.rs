// SigmaOS Support & Services Framework
// Professional support tiers, LTS maintenance guarantees, and disaster recovery configurations

use std::collections::HashMap;

/// Professional support levels
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SupportTier {
    Basic,
    Developer,
    Business,
    Enterprise,
}

/// Professional SLA / Support contract
#[derive(Debug, Clone)]
pub struct SupportContract {
    pub client_name: String,
    pub tier: SupportTier,
    pub sla_resolution_hours: u32,
    pub active_tickets: u32,
}

/// Long-Term Maintenance (LTS) release lifecycle
#[derive(Debug, Clone)]
pub struct LtsRelease {
    pub version: String,
    pub release_date: String,
    pub supported_until: String,
    pub kernel_version: String,
}

/// Disaster recovery tool mapping (such as Rescue ISO configuration)
#[derive(Debug, Clone)]
pub struct RecoveryConfig {
    pub rescue_iso_name: String,
    pub diagnostic_tools_included: Vec<String>,
    pub automount_system_drives: bool,
}

/// Support & Services Manager
pub struct SupportServicesManager {
    pub active_contracts: HashMap<String, SupportContract>,
    pub lts_releases: HashMap<String, LtsRelease>,
    pub recovery_tools: Vec<RecoveryConfig>,
}

impl SupportServicesManager {
    pub fn new() -> Self {
        Self {
            active_contracts: HashMap::new(),
            lts_releases: HashMap::new(),
            recovery_tools: Vec::new(),
        }
    }

    pub fn register_contract(&mut self, client: String, tier: SupportTier, sla: u32) {
        let contract = SupportContract {
            client_name: client.clone(),
            tier,
            sla_resolution_hours: sla,
            active_tickets: 0,
        };
        self.active_contracts.insert(client, contract);
    }

    pub fn open_support_ticket(&mut self, client: &str) -> bool {
        if let Some(contract) = self.active_contracts.get_mut(client) {
            contract.active_tickets += 1;
            true
        } else {
            false
        }
    }

    pub fn register_lts_release(&mut self, version: String, release_date: String, supported_until: String, kernel: String) {
        let release = LtsRelease {
            version: version.clone(),
            release_date,
            supported_until,
            kernel_version: kernel,
        };
        self.lts_releases.insert(version, release);
    }

    pub fn add_recovery_tool(&mut self, config: RecoveryConfig) {
        self.recovery_tools.push(config);
    }

    pub fn get_sla_limit(&self, client: &str) -> Option<u32> {
        self.active_contracts.get(client).map(|c| c.sla_resolution_hours)
    }
}

impl Default for SupportServicesManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_support_contracts_and_tickets() {
        let mut manager = SupportServicesManager::new();
        manager.register_contract("SovereignCloudCorp".to_string(), SupportTier::Enterprise, 4);

        assert_eq!(manager.get_sla_limit("SovereignCloudCorp"), Some(4));
        assert_eq!(manager.get_sla_limit("Nonexistent"), None);

        assert!(manager.open_support_ticket("SovereignCloudCorp"));
        assert_eq!(manager.active_contracts.get("SovereignCloudCorp").unwrap().active_tickets, 1);
    }

    #[test]
    fn test_lts_release_management() {
        let mut manager = SupportServicesManager::new();
        manager.register_lts_release(
            "v1.0-LTS".to_string(),
            "2025-01-15".to_string(),
            "2030-01-15".to_string(),
            "sigma-6.1-hardened".to_string(),
        );

        assert_eq!(manager.lts_releases.len(), 1);
        let release = manager.lts_releases.get("v1.0-LTS").unwrap();
        assert_eq!(release.kernel_version, "sigma-6.1-hardened");
    }

    #[test]
    fn test_recovery_tools() {
        let mut manager = SupportServicesManager::new();
        let config = RecoveryConfig {
            rescue_iso_name: "SigmaOS-Rescue-v1.0.iso".to_string(),
            diagnostic_tools_included: vec!["fsck.sigmafs".to_string(), "memtester".to_string()],
            automount_system_drives: true,
        };

        manager.add_recovery_tool(config);
        assert_eq!(manager.recovery_tools.len(), 1);
        assert_eq!(manager.recovery_tools[0].rescue_iso_name, "SigmaOS-Rescue-v1.0.iso");
        assert!(manager.recovery_tools[0].automount_system_drives);
    }
}
