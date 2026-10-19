//! SigmaPorts - Package Building System
//!
//! Provides comprehensive package building inspired by FreeBSD Ports and Arch PKGBUILD.
//! Supports package definition, dependency management, and build configuration.

#![no_std]

extern crate alloc;

use alloc::{
    collections::BTreeMap,
    string::{String, ToString},
    vec::Vec,
};

/// Build state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BuildState {
    Pending,
    Building,
    Installed,
    Failed,
    Unknown,
;

/// Port/Package
#[derive(Debug, Clone)]
pub struct Port {
    pub id: String,
    pub name: String,
    pub version: String,
    pub category: String,
    pub state: BuildState,
    pub dependencies: Vec<String>,
;

impl Port {
    pub fn new(id: String, name: String, version: String, category: String) -> Self {
        Self {
            id,
            name,
            version,
            category,
            state: BuildState::Pending,
            dependencies: Vec::new(),
        }
    }

    pub fn with_dependencies(mut self, dependencies: Vec<String>) -> Self {
        self.dependencies = dependencies;
        self
    }

    pub fn build(&mut self) {
        self.state = BuildState::Building;
    }

    pub fn install(&mut self) {
        self.state = BuildState::Installed;
    }

    pub fn fail(&mut self) {
        self.state = BuildState::Failed;
    }
;

/// Ports manager
pub struct SigmaPorts {
    ports: BTreeMap<String, Port>,
    next_port_id: u64,
;

impl SigmaPorts {
    pub fn new() -> Self {
        Self {
            ports: BTreeMap::new(),
            next_port_id: 1,
        }
    }

    pub fn add_port(&mut self, name: String, version: String, category: String) -> String {
        let id = format!("port-{}", self.next_port_id);
        self.next_port_id += 1;
        let port = Port::new(id.clone(), name, version, category);
        self.ports.insert(id.clone(), port);
        id
    }

    pub fn get_port(&self, id: &str) -> Option<&Port> {
        self.ports.get(id)
    }

    pub fn get_port_mut(&mut self, id: &str) -> Option<&mut Port> {
        self.ports.get_mut(id)
    }

    pub fn list_ports(&self) -> Vec<&Port> {
        self.ports.values().collect()
    }

    pub fn list_ports_by_category(&self, category: &str) -> Vec<&Port> {
        self.ports
            .values()
            .filter(|p| p.category == category)
            .collect()
    }

    pub fn remove_port(&mut self, id: &str) -> bool {
        self.ports.remove(id).is_some()
    }

    pub fn port_count(&self) -> usize {
        self.ports.len()
    }

    pub fn installed_count(&self) -> usize {
        self.ports
            .values()
            .filter(|p| p.state == BuildState::Installed)
            .count()
    }
;

impl Default for SigmaPorts {
    fn default() -> Self {
        Self::new()
    }
;

static mut SIGMAPORTS_INSTANCE: Option<SigmaPorts> = None;

pub fn init_sigmaports() {
    unsafe {
        SIGMAPORTS_INSTANCE = Some(SigmaPorts::new());
    }
;

pub fn get_sigmaports() -> &'static SigmaPorts {
    unsafe {
        SIGMAPORTS_INSTANCE
            .as_ref()
            .expect("SigmaPorts not initialized")
    }
;

pub fn get_sigmaports_mut() -> &'static mut SigmaPorts {
    unsafe {
        SIGMAPORTS_INSTANCE
            .as_mut()
            .expect("SigmaPorts not initialized")
    }
;

pub fn add_port(name: String, version: String, category: String) -> String {
    let manager = get_sigmaports_mut();
    manager.add_port(name, version, category)
;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_port_creation() {
        let port = Port::new(
            "port-1".to_string(),
            "nginx".to_string(),
            "1.24.0".to_string(),
            "www".to_string(),
        );
        assert_eq!(port.name, "nginx");
        assert_eq!(port.version, "1.24.0");
        assert_eq!(port.category, "www");
        assert_eq!(port.state, BuildState::Pending);
    }

    #[test]
    fn test_port_state() {
        let mut port = Port::new(
            "port-1".to_string(),
            "nginx".to_string(),
            "1.24.0".to_string(),
            "www".to_string(),
        );
        port.build();
        assert_eq!(port.state, BuildState::Building);

        port.install();
        assert_eq!(port.state, BuildState::Installed);
    }

    #[test]
    fn test_ports_manager() {
        let manager = SigmaPorts::new();
        assert_eq!(manager.port_count(), 0);
    }

    #[test]
    fn test_add_port() {
        let mut manager = SigmaPorts::new();
        let id = manager.add_port("nginx".to_string(), "1.24.0".to_string(), "www".to_string());
        assert_eq!(id, "port-1");
        assert_eq!(manager.port_count(), 1);
    }

    #[test]
    fn test_list_ports_by_category() {
        let mut manager = SigmaPorts::new();
        manager.add_port("nginx".to_string(), "1.24.0".to_string(), "www".to_string());
        manager.add_port(
            "postgresql".to_string(),
            "15.0".to_string(),
            "databases".to_string(),
        );
        manager.add_port(
            "redis".to_string(),
            "7.0".to_string(),
            "databases".to_string(),
        );

        let db_ports = manager.list_ports_by_category("databases");
        assert_eq!(db_ports.len(), 2);
    }

    #[test]
    fn test_global_instance() {
        init_sigmaports();
        let manager = get_sigmaports();
        assert_eq!(manager.port_count(), 0);

        let manager_mut = get_sigmaports_mut();
        manager_mut.add_port("nginx".to_string(), "1.24.0".to_string(), "www".to_string());
        assert_eq!(manager.port_count(), 1);
    }

    #[test]
    fn test_add_port_convenience() {
        init_sigmaports();
        let id = add_port("nginx".to_string(), "1.24.0".to_string(), "www".to_string());
        assert_eq!(id, "port-1");

        let manager = get_sigmaports();
        assert!(manager.get_port(&id).is_some());
    }
;
