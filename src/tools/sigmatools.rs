// SigmaTools - System suite for SigmaOS
// SigmaDeploy, SigmaCluster, SigmaIdentity, SigmaAccess components

#![no_std]

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SigmaToolError {
    NetworkError,
    AuthenticationFailed,
    ResourceUnavailable,
    InvalidConfiguration,
}

/// SigmaDeploy - Automated Provisioning & Netboot
pub struct SigmaDeploy {
    pub tftp_enabled: bool,
    pub dhcp_enabled: bool,
    pub kickstart_config: Option<String>,
}

impl SigmaDeploy {
    pub fn new() -> Self {
        Self {
            tftp_enabled: false,
            dhcp_enabled: false,
            kickstart_config: None,
        }
    }

    pub fn enable_tftp(&mut self) {
        self.tftp_enabled = true;
    }

    pub fn enable_dhcp(&mut self) {
        self.dhcp_enabled = true;
    }

    pub fn set_kickstart_config(&mut self, config: String) {
        self.kickstart_config = Some(config);
    }

    pub fn is_ready(&self) -> bool {
        self.tftp_enabled && self.dhcp_enabled && self.kickstart_config.is_some()
    }
}

impl Default for SigmaDeploy {
    fn default() -> Self {
        Self::new()
    }
}

/// SigmaCluster - Grid & Cluster Orchestrator
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeState {
    Idle,
    Busy,
    Offline,
    Maintenance,
}

pub struct ClusterNode {
    pub id: String,
    pub state: NodeState,
    pub load: f32,
    pub cpu_cores: u32,
}

impl ClusterNode {
    pub fn new(id: String, cpu_cores: u32) -> Self {
        Self {
            id,
            state: NodeState::Idle,
            load: 0.0,
            cpu_cores,
        }
    }

    pub fn set_state(&mut self, state: NodeState) {
        self.state = state;
    }

    pub fn update_load(&mut self, load: f32) {
        self.load = load;
    }
}

pub struct SigmaCluster {
    nodes: BTreeMap<String, ClusterNode>,
    task_queue: Vec<String>,
}

impl SigmaCluster {
    pub fn new() -> Self {
        Self {
            nodes: BTreeMap::new(),
            task_queue: Vec::new(),
        }
    }

    pub fn add_node(&mut self, node: ClusterNode) {
        let id = node.id.clone();
        self.nodes.insert(id, node);
    }

    pub fn remove_node(&mut self, id: &str) -> Result<(), SigmaToolError> {
        self.nodes
            .remove(id)
            .ok_or(SigmaToolError::ResourceUnavailable)?;
        Ok(())
    }

    pub fn queue_task(&mut self, task: String) {
        self.task_queue.push(task);
    }

    pub fn get_node(&self, id: &str) -> Option<&ClusterNode> {
        self.nodes.get(id)
    }

    pub fn get_idle_nodes(&self) -> Vec<&ClusterNode> {
        self.nodes
            .values()
            .filter(|node| node.state == NodeState::Idle)
            .collect()
    }

    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    pub fn task_count(&self) -> usize {
        self.task_queue.len()
    }
}

impl Default for SigmaCluster {
    fn default() -> Self {
        Self::new()
    }
}

/// SigmaIdentity - Enterprise Directory Integrator
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserIdentity {
    pub username: String,
    pub groups: Vec<String>,
    pub permissions: Vec<String>,
}

impl UserIdentity {
    pub fn new(username: String) -> Self {
        Self {
            username,
            groups: Vec::new(),
            permissions: Vec::new(),
        }
    }

    pub fn add_group(&mut self, group: String) {
        self.groups.push(group);
    }

    pub fn add_permission(&mut self, permission: String) {
        self.permissions.push(permission);
    }

    pub fn has_permission(&self, permission: &str) -> bool {
        self.permissions.iter().any(|p| p == permission)
    }
}

pub struct SigmaIdentity {
    users: BTreeMap<String, UserIdentity>,
    ldap_enabled: bool,
    kerberos_enabled: bool,
}

impl SigmaIdentity {
    pub fn new() -> Self {
        Self {
            users: BTreeMap::new(),
            ldap_enabled: false,
            kerberos_enabled: false,
        }
    }

    pub fn enable_ldap(&mut self) {
        self.ldap_enabled = true;
    }

    pub fn enable_kerberos(&mut self) {
        self.kerberos_enabled = true;
    }

    pub fn add_user(&mut self, user: UserIdentity) {
        let username = user.username.clone();
        self.users.insert(username, user);
    }

    pub fn authenticate(&self, username: &str, permission: &str) -> bool {
        if let Some(user) = self.users.get(username) {
            return user.has_permission(permission);
        }
        false
    }

    pub fn get_user(&self, username: &str) -> Option<&UserIdentity> {
        self.users.get(username)
    }

    pub fn user_count(&self) -> usize {
        self.users.len()
    }
}

impl Default for SigmaIdentity {
    fn default() -> Self {
        Self::new()
    }
}

/// SigmaAccess - Visual & Audio Inclusivity Toolkit
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccessibilityFeature {
    ScreenReader,
    HighContrast,
    Magnification,
    VoiceControl,
    EyeTracking,
}

pub struct SigmaAccess {
    pub enabled_features: Vec<AccessibilityFeature>,
    pub screen_reader_active: bool,
    pub high_contrast_level: u8,
}

impl SigmaAccess {
    pub fn new() -> Self {
        Self {
            enabled_features: Vec::new(),
            screen_reader_active: false,
            high_contrast_level: 0,
        }
    }

    pub fn enable_feature(&mut self, feature: AccessibilityFeature) {
        if !self.enabled_features.contains(&feature) {
            self.enabled_features.push(feature);
        }

        match feature {
            AccessibilityFeature::ScreenReader => self.screen_reader_active = true,
            AccessibilityFeature::HighContrast => self.high_contrast_level = 100,
            _ => {}
        }
    }

    pub fn disable_feature(&mut self, feature: AccessibilityFeature) {
        self.enabled_features.retain(|f| f != &feature);

        match feature {
            AccessibilityFeature::ScreenReader => self.screen_reader_active = false,
            AccessibilityFeature::HighContrast => self.high_contrast_level = 0,
            _ => {}
        }
    }

    pub fn is_feature_enabled(&self, feature: AccessibilityFeature) -> bool {
        self.enabled_features.contains(&feature)
    }

    pub fn set_contrast_level(&mut self, level: u8) {
        self.high_contrast_level = level.min(100);
    }
}

impl Default for SigmaAccess {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sigma_deploy() {
        let mut deploy = SigmaDeploy::new();
        assert!(!deploy.is_ready());

        deploy.enable_tftp();
        deploy.enable_dhcp();
        deploy.set_kickstart_config("test-config".to_string());

        assert!(deploy.is_ready());
    }

    #[test]
    fn test_sigma_cluster() {
        let mut cluster = SigmaCluster::new();
        let node = ClusterNode::new("node1".to_string(), 8);

        cluster.add_node(node);
        assert_eq!(cluster.node_count(), 1);

        let idle_nodes = cluster.get_idle_nodes();
        assert_eq!(idle_nodes.len(), 1);
    }

    #[test]
    fn test_cluster_task_queue() {
        let mut cluster = SigmaCluster::new();
        cluster.queue_task("task1".to_string());
        cluster.queue_task("task2".to_string());

        assert_eq!(cluster.task_count(), 2);
    }

    #[test]
    fn test_sigma_identity() {
        let mut identity = SigmaIdentity::new();
        let mut user = UserIdentity::new("testuser".to_string());
        user.add_permission("read".to_string());

        identity.add_user(user);
        assert!(identity.authenticate("testuser", "read"));
    }

    #[test]
    fn test_user_groups() {
        let mut user = UserIdentity::new("testuser".to_string());
        user.add_group("admin".to_string());
        user.add_group("users".to_string());

        assert_eq!(user.groups.len(), 2);
    }

    #[test]
    fn test_sigma_access() {
        let mut access = SigmaAccess::new();
        access.enable_feature(AccessibilityFeature::ScreenReader);

        assert!(access.is_feature_enabled(AccessibilityFeature::ScreenReader));
        assert!(access.screen_reader_active);
    }

    #[test]
    fn test_high_contrast() {
        let mut access = SigmaAccess::new();
        access.enable_feature(AccessibilityFeature::HighContrast);

        assert_eq!(access.high_contrast_level, 100);
    }

    #[test]
    fn test_disable_feature() {
        let mut access = SigmaAccess::new();
        access.enable_feature(AccessibilityFeature::ScreenReader);
        access.disable_feature(AccessibilityFeature::ScreenReader);

        assert!(!access.is_feature_enabled(AccessibilityFeature::ScreenReader));
    }

    #[test]
    fn test_contrast_level_clamp() {
        let mut access = SigmaAccess::new();
        access.set_contrast_level(150);

        assert_eq!(access.high_contrast_level, 100);
    }
}
