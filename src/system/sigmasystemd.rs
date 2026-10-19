//! SigmaSystemd - Init System and Service Manager
//!
//! Provides comprehensive init system and service management inspired by systemd.
//! Supports service units, targets, dependencies, and system state management.

#![no_std]

extern crate alloc;

use alloc::{
    collections::BTreeMap,
    string::{String, ToString},
    vec::Vec,
};

/// Unit type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UnitType {
    Service,
    Target,
    Mount,
    Socket,
    Timer,
    Path,
    Device,
    Unknown,
;

/// Unit state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnitState {
    Inactive,
    Activating,
    Active,
    Deactivating,
    Failed,
    Unknown,
;

/// Dependency type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DependencyType {
    Requires,
    Wants,
    Requisite,
    Conflicts,
    Before,
    After,
    Unknown,
;

/// Service restart policy
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RestartPolicy {
    No,
    OnSuccess,
    OnFailure,
    Always,
    Unknown,
;

/// Unit
#[derive(Debug, Clone)]
pub struct Unit {
    pub id: String,
    pub name: String,
    pub unit_type: UnitType,
    pub state: UnitState,
    pub description: String,
    pub dependencies: Vec<(String, DependencyType)>,
    pub enabled: bool,
    pub restart_policy: RestartPolicy,
    pub exec_start: Option<String>,
    pub exec_stop: Option<String>,
    pub pid: Option<u32>,
;

impl Unit {
    pub fn new(id: String, name: String, unit_type: UnitType) -> Self {
        Self {
            id,
            name,
            unit_type,
            state: UnitState::Inactive,
            description: String::new(),
            dependencies: Vec::new(),
            enabled: false,
            restart_policy: RestartPolicy::No,
            exec_start: None,
            exec_stop: None,
            pid: None,
        }
    }

    pub fn with_description(mut self, description: String) -> Self {
        self.description = description;
        self
    }

    pub fn with_exec_start(mut self, command: String) -> Self {
        self.exec_start = Some(command);
        self
    }

    pub fn with_exec_stop(mut self, command: String) -> Self {
        self.exec_stop = Some(command);
        self
    }

    pub fn with_restart_policy(mut self, policy: RestartPolicy) -> Self {
        self.restart_policy = policy;
        self
    }

    pub fn add_dependency(&mut self, unit_id: String, dep_type: DependencyType) {
        self.dependencies.push((unit_id, dep_type));
    }

    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn disable(&mut self) {
        self.enabled = false;
    }

    pub fn start(&mut self) {
        self.state = UnitState::Activating;
    }

    pub fn activate(&mut self) {
        self.state = UnitState::Active;
    }

    pub fn stop(&mut self) {
        self.state = UnitState::Deactivating;
    }

    pub fn deactivate(&mut self) {
        self.state = UnitState::Inactive;
    }

    pub fn fail(&mut self) {
        self.state = UnitState::Failed;
    }
;

/// System target
#[derive(Debug, Clone)]
pub struct Target {
    pub id: String,
    pub name: String,
    pub state: UnitState,
    pub units: Vec<String>,
;

impl Target {
    pub fn new(id: String, name: String) -> Self {
        Self {
            id,
            name,
            state: UnitState::Inactive,
            units: Vec::new(),
        }
    }

    pub fn add_unit(&mut self, unit_id: String) {
        self.units.push(unit_id);
    }

    pub fn activate(&mut self) {
        self.state = UnitState::Active;
    }

    pub fn deactivate(&mut self) {
        self.state = UnitState::Inactive;
    }
;

/// Systemd manager
pub struct SigmaSystemd {
    units: BTreeMap<String, Unit>,
    targets: BTreeMap<String, Target>,
    next_unit_id: u64,
    next_target_id: u64,
    default_target: Option<String>,
;

impl SigmaSystemd {
    pub fn new() -> Self {
        Self {
            units: BTreeMap::new(),
            targets: BTreeMap::new(),
            next_unit_id: 1,
            next_target_id: 1,
            default_target: None,
        }
    }

    pub fn create_unit(&mut self, name: String, unit_type: UnitType) -> String {
        let id = format!("unit-{}", self.next_unit_id);
        self.next_unit_id += 1;
        let unit = Unit::new(id.clone(), name, unit_type);
        self.units.insert(id.clone(), unit);
        id
    }

    pub fn get_unit(&self, id: &str) -> Option<&Unit> {
        self.units.get(id)
    }

    pub fn get_unit_mut(&mut self, id: &str) -> Option<&mut Unit> {
        self.units.get_mut(id)
    }

    pub fn list_units(&self) -> Vec<&Unit> {
        self.units.values().collect()
    }

    pub fn list_units_by_type(&self, unit_type: UnitType) -> Vec<&Unit> {
        self.units
            .values()
            .filter(|u| u.unit_type == unit_type)
            .collect()
    }

    pub fn list_units_by_state(&self, state: UnitState) -> Vec<&Unit> {
        self.units.values().filter(|u| u.state == state).collect()
    }

    pub fn remove_unit(&mut self, id: &str) -> bool {
        self.units.remove(id).is_some()
    }

    pub fn enable_unit(&mut self, id: &str) -> bool {
        if let Some(unit) = self.units.get_mut(id) {
            unit.enable();
            true
        } else {
            false
        }
    }

    pub fn disable_unit(&mut self, id: &str) -> bool {
        if let Some(unit) = self.units.get_mut(id) {
            unit.disable();
            true
        } else {
            false
        }
    }

    pub fn start_unit(&mut self, id: &str) -> bool {
        if let Some(unit) = self.units.get_mut(id) {
            unit.start();
            unit.activate();
            true
        } else {
            false
        }
    }

    pub fn stop_unit(&mut self, id: &str) -> bool {
        if let Some(unit) = self.units.get_mut(id) {
            unit.stop();
            unit.deactivate();
            true
        } else {
            false
        }
    }

    pub fn fail_unit(&mut self, id: &str) -> bool {
        if let Some(unit) = self.units.get_mut(id) {
            unit.fail();
            true
        } else {
            false
        }
    }

    pub fn add_dependency(
        &mut self,
        unit_id: &str,
        dep_id: String,
        dep_type: DependencyType,
    ) -> bool {
        if let Some(unit) = self.units.get_mut(unit_id) {
            unit.add_dependency(dep_id, dep_type);
            true
        } else {
            false
        }
    }

    pub fn create_target(&mut self, name: String) -> String {
        let id = format!("target-{}", self.next_target_id);
        self.next_target_id += 1;
        let target = Target::new(id.clone(), name);
        self.targets.insert(id.clone(), target);
        id
    }

    pub fn get_target(&self, id: &str) -> Option<&Target> {
        self.targets.get(id)
    }

    pub fn get_target_mut(&mut self, id: &str) -> Option<&mut Target> {
        self.targets.get_mut(id)
    }

    pub fn list_targets(&self) -> Vec<&Target> {
        self.targets.values().collect()
    }

    pub fn add_unit_to_target(&mut self, target_id: &str, unit_id: String) -> bool {
        if let Some(target) = self.targets.get_mut(target_id) {
            target.add_unit(unit_id);
            true
        } else {
            false
        }
    }

    pub fn activate_target(&mut self, id: &str) -> bool {
        if let Some(target) = self.targets.get_mut(id) {
            target.activate();
            true
        } else {
            false
        }
    }

    pub fn deactivate_target(&mut self, id: &str) -> bool {
        if let Some(target) = self.targets.get_mut(id) {
            target.deactivate();
            true
        } else {
            false
        }
    }

    pub fn set_default_target(&mut self, target_id: String) {
        self.default_target = Some(target_id);
    }

    pub fn get_default_target(&self) -> Option<&String> {
        self.default_target.as_ref()
    }

    pub fn unit_count(&self) -> usize {
        self.units.len()
    }

    pub fn active_unit_count(&self) -> usize {
        self.units
            .values()
            .filter(|u| u.state == UnitState::Active)
            .count()
    }

    pub fn failed_unit_count(&self) -> usize {
        self.units
            .values()
            .filter(|u| u.state == UnitState::Failed)
            .count()
    }
;

impl Default for SigmaSystemd {
    fn default() -> Self {
        Self::new()
    }
;

static mut SIGMASYSTEMD_INSTANCE: Option<SigmaSystemd> = None;

pub fn init_sigmasystemd() {
    unsafe {
        SIGMASYSTEMD_INSTANCE = Some(SigmaSystemd::new());
    }
;

pub fn get_sigmasystemd() -> &'static SigmaSystemd {
    unsafe {
        SIGMASYSTEMD_INSTANCE
            .as_ref()
            .expect("SigmaSystemd not initialized")
    }
;

pub fn get_sigmasystemd_mut() -> &'static mut SigmaSystemd {
    unsafe {
        SIGMASYSTEMD_INSTANCE
            .as_mut()
            .expect("SigmaSystemd not initialized")
    }
;

pub fn create_unit(name: String, unit_type: UnitType) -> String {
    let manager = get_sigmasystemd_mut();
    manager.create_unit(name, unit_type)
;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unit_creation() {
        let unit = Unit::new("unit-1".to_string(), "nginx".to_string(), UnitType::Service);
        assert_eq!(unit.id, "unit-1");
        assert_eq!(unit.name, "nginx");
        assert_eq!(unit.unit_type, UnitType::Service);
        assert_eq!(unit.state, UnitState::Inactive);
    }

    #[test]
    fn test_unit_builder() {
        let unit = Unit::new("unit-1".to_string(), "nginx".to_string(), UnitType::Service)
            .with_description("Web server".to_string())
            .with_exec_start("/usr/bin/nginx".to_string())
            .with_exec_stop("/usr/bin/nginx -s stop".to_string())
            .with_restart_policy(RestartPolicy::Always);

        assert_eq!(unit.description, "Web server");
        assert_eq!(unit.exec_start, Some("/usr/bin/nginx".to_string()));
        assert_eq!(unit.exec_stop, Some("/usr/bin/nginx -s stop".to_string()));
        assert_eq!(unit.restart_policy, RestartPolicy::Always);
    }

    #[test]
    fn test_unit_state_transitions() {
        let mut unit = Unit::new("unit-1".to_string(), "nginx".to_string(), UnitType::Service);
        assert_eq!(unit.state, UnitState::Inactive);

        unit.start();
        assert_eq!(unit.state, UnitState::Activating);

        unit.activate();
        assert_eq!(unit.state, UnitState::Active);

        unit.stop();
        assert_eq!(unit.state, UnitState::Deactivating);

        unit.deactivate();
        assert_eq!(unit.state, UnitState::Inactive);

        unit.fail();
        assert_eq!(unit.state, UnitState::Failed);
    }

    #[test]
    fn test_enable_disable_unit() {
        let mut unit = Unit::new("unit-1".to_string(), "nginx".to_string(), UnitType::Service);
        assert!(!unit.enabled);

        unit.enable();
        assert!(unit.enabled);

        unit.disable();
        assert!(!unit.enabled);
    }

    #[test]
    fn test_unit_dependencies() {
        let mut unit = Unit::new("unit-1".to_string(), "nginx".to_string(), UnitType::Service);
        unit.add_dependency("network".to_string(), DependencyType::Requires);
        unit.add_dependency("syslog".to_string(), DependencyType::Wants);

        assert_eq!(unit.dependencies.len(), 2);
    }

    #[test]
    fn test_target_creation() {
        let target = Target::new("target-1".to_string(), "multi-user".to_string());
        assert_eq!(target.id, "target-1");
        assert_eq!(target.name, "multi-user");
        assert_eq!(target.state, UnitState::Inactive);
    }

    #[test]
    fn test_target_unit_management() {
        let mut target = Target::new("target-1".to_string(), "multi-user".to_string());
        target.add_unit("unit-1".to_string());
        target.add_unit("unit-2".to_string());

        assert_eq!(target.units.len(), 2);
    }

    #[test]
    fn test_systemd_manager() {
        let manager = SigmaSystemd::new();
        assert_eq!(manager.unit_count(), 0);
    }

    #[test]
    fn test_create_unit() {
        let mut manager = SigmaSystemd::new();
        let id = manager.create_unit("nginx".to_string(), UnitType::Service);
        assert_eq!(id, "unit-1");
        assert_eq!(manager.unit_count(), 1);
    }

    #[test]
    fn test_start_stop_unit() {
        let mut manager = SigmaSystemd::new();
        let id = manager.create_unit("nginx".to_string(), UnitType::Service);

        assert!(manager.start_unit(&id));
        assert_eq!(manager.get_unit(&id).unwrap().state, UnitState::Active);

        assert!(manager.stop_unit(&id));
        assert_eq!(manager.get_unit(&id).unwrap().state, UnitState::Inactive);
    }

    #[test]
    fn test_enable_disable_unit_manager() {
        let mut manager = SigmaSystemd::new();
        let id = manager.create_unit("nginx".to_string(), UnitType::Service);

        assert!(manager.enable_unit(&id));
        assert!(manager.get_unit(&id).unwrap().enabled);

        assert!(manager.disable_unit(&id));
        assert!(!manager.get_unit(&id).unwrap().enabled);
    }

    #[test]
    fn test_add_dependency_manager() {
        let mut manager = SigmaSystemd::new();
        let id1 = manager.create_unit("nginx".to_string(), UnitType::Service);
        let id2 = manager.create_unit("network".to_string(), UnitType::Target);

        assert!(manager.add_dependency(&id1, id2.clone(), DependencyType::Requires));
        assert_eq!(manager.get_unit(&id1).unwrap().dependencies.len(), 1);
    }

    #[test]
    fn test_create_target() {
        let mut manager = SigmaSystemd::new();
        let id = manager.create_target("multi-user".to_string());
        assert_eq!(id, "target-1");
    }

    #[test]
    fn test_add_unit_to_target() {
        let mut manager = SigmaSystemd::new();
        let target_id = manager.create_target("multi-user".to_string());
        let unit_id = manager.create_unit("nginx".to_string(), UnitType::Service);

        assert!(manager.add_unit_to_target(&target_id, unit_id));
        assert_eq!(manager.get_target(&target_id).unwrap().units.len(), 1);
    }

    #[test]
    fn test_activate_deactivate_target() {
        let mut manager = SigmaSystemd::new();
        let id = manager.create_target("multi-user".to_string());

        assert!(manager.activate_target(&id));
        assert_eq!(manager.get_target(&id).unwrap().state, UnitState::Active);

        assert!(manager.deactivate_target(&id));
        assert_eq!(manager.get_target(&id).unwrap().state, UnitState::Inactive);
    }

    #[test]
    fn test_default_target() {
        let mut manager = SigmaSystemd::new();
        let target_id = manager.create_target("multi-user".to_string());
        manager.set_default_target(target_id.clone());

        assert_eq!(manager.get_default_target(), Some(&target_id));
    }

    #[test]
    fn test_unit_counts() {
        let mut manager = SigmaSystemd::new();
        let id1 = manager.create_unit("nginx".to_string(), UnitType::Service);
        let id2 = manager.create_unit("apache".to_string(), UnitType::Service);
        let id3 = manager.create_unit("mysql".to_string(), UnitType::Service);
        manager.start_unit(&id1);
        manager.start_unit(&id2);
        manager.fail_unit(&id3);

        assert_eq!(manager.unit_count(), 3);
        assert_eq!(manager.active_unit_count(), 2);
        assert_eq!(manager.failed_unit_count(), 1);
    }

    #[test]
    fn test_list_units_by_type() {
        let mut manager = SigmaSystemd::new();
        manager.create_unit("nginx".to_string(), UnitType::Service);
        manager.create_unit("network".to_string(), UnitType::Target);
        manager.create_unit("apache".to_string(), UnitType::Service);

        let services = manager.list_units_by_type(UnitType::Service);
        assert_eq!(services.len(), 2);

        let targets = manager.list_units_by_type(UnitType::Target);
        assert_eq!(targets.len(), 1);
    }

    #[test]
    fn test_global_instance() {
        init_sigmasystemd();
        let manager = get_sigmasystemd();
        assert_eq!(manager.unit_count(), 0);

        let manager_mut = get_sigmasystemd_mut();
        manager_mut.create_unit("nginx".to_string(), UnitType::Service);
        assert_eq!(manager.unit_count(), 1);
    }

    #[test]
    fn test_create_unit_convenience() {
        init_sigmasystemd();
        let id = create_unit("nginx".to_string(), UnitType::Service);
        assert_eq!(id, "unit-1");

        let manager = get_sigmasystemd();
        assert!(manager.get_unit(&id).is_some());
    }
;
