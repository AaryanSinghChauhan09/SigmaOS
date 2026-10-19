//! SigmaOS Version Manager
//!
//! Inspired by: Omarchy version (omancy-version, omancy-version-branch, omancy-version-channel, omancy-version-pkgs)
//! Following principles: SINGLE_RESPONSIBILITY, COMPONENT_ISOLATION, MODULARITY,
//! LAYERED DESIGN, KEEP_IT_SIMPLE_STUPID, DISCOVERABILITY, VERSION_CONTROL_PRACTICES,
/// STABLE_APIS, TRANSPARENCY, USER_CONTROL.

/// Version info
#[derive(Debug, Clone)]
pub struct VersionInfo {
    pub version: String,
    pub branch: String,
    pub channel: String,
    pub package_count: u32,
    pub build_date: u64,
    pub commit_hash: Option<String>,
}

/// Version manager
pub struct VersionManager {
    pub info: VersionInfo,
}

impl VersionManager {
    pub fn new() -> Self {
        Self {
            info: VersionInfo {
                version: env!("CARGO_PKG_VERSION").to_string(),
                branch: "main".to_string(),
                channel: "stable".to_string(),
                package_count: 0,
                build_date: now_secs(),
                commit_hash: None,
            },
        }
    }

    /// Get version
    pub fn get_version(&self) -> &str {
        &self.info.version
    }

    /// Get branch
    pub fn get_branch(&self) -> &str {
        &self.info.branch
    }

    /// Get channel
    pub fn get_channel(&self) -> &str {
        &self.info.channel
    }

    /// Set branch
    pub fn set_branch(&mut self, branch: String) {
        self.info.branch = branch;
    }

    /// Set channel
    pub fn set_channel(&mut self, channel: String) {
        self.info.channel = channel;
    }

    /// Set package count
    pub fn set_package_count(&mut self, count: u32) {
        self.info.package_count = count;
    }

    /// Set commit hash
    pub fn set_commit_hash(&mut self, hash: String) {
        self.info.commit_hash = Some(hash);
    }

    /// Get full version string
    pub fn get_full_version(&self) -> String {
        format!(
            "SigmaOS v{} (branch: {}, channel: {})",
            self.info.version, self.info.branch, self.info.channel
        )
    }

    /// Get package count
    pub fn get_package_count(&self) -> u32 {
        self.info.package_count
    }
}

impl Default for VersionManager {
    fn default() -> Self {
        Self::new()
    }
}

fn now_secs() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_version() {
        let mgr = VersionManager::new();
        assert!(!mgr.get_version().is_empty());
    }

    #[test]
    fn test_get_branch() {
        let mgr = VersionManager::new();
        assert_eq!(mgr.get_branch(), "main");
    }

    #[test]
    fn test_get_channel() {
        let mgr = VersionManager::new();
        assert_eq!(mgr.get_channel(), "stable");
    }

    #[test]
    fn test_set_branch() {
        let mut mgr = VersionManager::new();
        mgr.set_branch("dev".to_string());
        assert_eq!(mgr.get_branch(), "dev");
    }

    #[test]
    fn test_set_channel() {
        let mut mgr = VersionManager::new();
        mgr.set_channel("edge".to_string());
        assert_eq!(mgr.get_channel(), "edge");
    }

    #[test]
    fn test_full_version() {
        let mut mgr = VersionManager::new();
        mgr.set_package_count(42);
        mgr.set_commit_hash("abc123".to_string());
        let full = mgr.get_full_version();
        assert!(full.contains("SigmaOS"));
        assert!(full.contains("main"));
        assert!(full.contains("stable"));
    }
}