// OpenRC-Inspired Service Management System
// Dependency-based init system with runlevels, service supervision, and parallel startup

#![no_std]

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// OpenRC-inspired service states
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServiceState {
    Stopped,
    Starting,
    Started,
    Stopping,
    Crashed,
    Unknown,
}

/// OpenRC-inspired service dependency types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DependencyType {
    Need,       // Hard dependency - must be running
    Want,       // Soft dependency - try to start but continue if fails
    Use,        // Optional dependency - start if available
    Before,     // Ordering only - start before specified service
    After,      // Ordering only - start after specified service
    Provide,    // Virtual service this service provides
}

/// OpenRC-inspired service dependency
#[derive(Debug, Clone)]
pub struct ServiceDependency {
    pub target: String,
    pub dep_type: DependencyType,
}

impl ServiceDependency {
    pub fn new(target: String, dep_type: DependencyType) -> Self {
        Self { target, dep_type }
    }
}

/// OpenRC-inspired service configuration
#[derive(Debug, Clone)]
pub struct ServiceConfig {
    pub name: String,
    pub description: String,
    pub command: String,
    pub arguments: Vec<String>,
    pub working_dir: Option<String>,
    pub environment: Vec<(String, String)>,
    pub dependencies: Vec<ServiceDependency>,
    pub provides: Vec<String>, // Virtual services this provides
    pub auto_start: bool,
    pub restart_on_failure: bool,
    pub max_restarts: usize,
    pub start_timeout: u64, // milliseconds
    pub stop_timeout: u64,  // milliseconds
    pub pid_file: Option<String>,
    pub log_file: Option<String>,
}

impl ServiceConfig {
    pub fn new(name: String, command: String) -> Self {
        Self {
            name,
            description: String::new(),
            command,
            arguments: Vec::new(),
            working_dir: None,
            environment: Vec::new(),
            dependencies: Vec::new(),
            provides: Vec::new(),
            auto_start: false,
            restart_on_failure: false,
            max_restarts: 3,
            start_timeout: 30000,
            stop_timeout: 10000,
            pid_file: None,
            log_file: None,
        }
    }

    pub fn with_description(mut self, description: String) -> Self {
        self.description = description;
        self
    }

    pub fn with_dependency(mut self, target: String, dep_type: DependencyType) -> Self {
        self.dependencies.push(ServiceDependency::new(target, dep_type));
        self
    }

    pub fn with_provides(mut self, virtual_service: String) -> Self {
        self.provides.push(virtual_service);
        self
    }

    pub fn with_environment(mut self, key: String, value: String) -> Self {
        self.environment.push((key, value));
        self
    }
}

/// OpenRC-inspired service instance
#[derive(Debug, Clone)]
pub struct ServiceInstance {
    pub config: ServiceConfig,
    pub state: ServiceState,
    pub pid: Option<u32>,
    pub restart_count: usize,
    pub start_time: Option<u64>,
    pub last_state_change: u64,
}

impl ServiceInstance {
    pub fn new(config: ServiceConfig) -> Self {
        Self {
            config,
            state: ServiceState::Stopped,
            pid: None,
            restart_count: 0,
            start_time: None,
            last_state_change: 0,
        }
    }

    pub fn set_state(&mut self, new_state: ServiceState, timestamp: u64) {
        self.state = new_state;
        self.last_state_change = timestamp;

        if new_state == ServiceState::Started {
            self.start_time = Some(timestamp);
            self.restart_count = 0;
        } else if new_state == ServiceState::Crashed {
            self.restart_count += 1;
        }
    }

    pub fn is_running(&self) -> bool {
        self.state == ServiceState::Started || self.state == ServiceState::Starting
    }

    pub fn should_restart(&self) -> bool {
        self.config.restart_on_failure && 
        self.state == ServiceState::Crashed && 
        self.restart_count < self.config.max_restarts
    }
}

/// OpenRC-inspired runlevel
#[derive(Debug, Clone)]
pub struct Runlevel {
    pub name: String,
    pub services: Vec<String>, // Service names in this runlevel
    pub is_default: bool,
}

impl Runlevel {
    pub fn new(name: String, is_default: bool) -> Self {
        Self {
            name,
            services: Vec::new(),
            is_default,
        }
    }

    pub fn add_service(&mut self, service_name: String) {
        if !self.services.contains(&service_name) {
            self.services.push(service_name);
        }
    }

    pub fn remove_service(&mut self, service_name: &str) {
        self.services.retain(|s| s != service_name);
    }
}

/// OpenRC-inspired service manager
pub struct OpenRCManager {
    services: BTreeMap<String, ServiceInstance>,
    runlevels: BTreeMap<String, Runlevel>,
    current_runlevel: String,
    virtual_services: BTreeMap<String, String>, // virtual -> actual service mapping
}

impl OpenRCManager {
    pub fn new() -> Self {
        let mut manager = Self {
            services: BTreeMap::new(),
            runlevels: BTreeMap::new(),
            current_runlevel: "default".to_string(),
            virtual_services: BTreeMap::new(),
        };

        // Initialize standard runlevels
        manager.runlevels.insert("boot".to_string(), Runlevel::new("boot".to_string(), false));
        manager.runlevels.insert("default".to_string(), Runlevel::new("default".to_string(), true));
        manager.runlevels.insert("nonetwork".to_string(), Runlevel::new("nonetwork".to_string(), false));
        manager.runlevels.insert("single".to_string(), Runlevel::new("single".to_string(), false));
        manager.runlevels.insert("shutdown".to_string(), Runlevel::new("shutdown".to_string(), false));

        manager
    }

    pub fn add_service(&mut self, config: ServiceConfig) -> Result<(), &'static str> {
        if self.services.contains_key(&config.name) {
            return Err("Service already exists");
        }

        let instance = ServiceInstance::new(config.clone());
        self.services.insert(config.name.clone(), instance);

        // Register virtual services
        for virtual_service in &config.provides {
            self.virtual_services.insert(virtual_service.clone(), config.name.clone());
        }

        Ok(())
    }

    pub fn remove_service(&mut self, name: &str) -> Result<(), &'static str> {
        if !self.services.contains_key(name) {
            return Err("Service does not exist");
        }

        let service = self.services.get(name).unwrap();
        
        // Remove virtual service mappings
        for virtual_service in &service.config.provides {
            self.virtual_services.remove(virtual_service);
        }

        self.services.remove(name);
        Ok(())
    }

    pub fn get_service(&self, name: &str) -> Option<&ServiceInstance> {
        self.services.get(name)
    }

    pub fn get_service_mut(&mut self, name: &str) -> Option<&mut ServiceInstance> {
        self.services.get_mut(name)
    }

    pub fn resolve_virtual_service(&self, virtual_name: &str) -> Option<&str> {
        self.virtual_services.get(virtual_name).map(|s| s.as_str())
    }

    pub fn start_service(&mut self, name: &str, timestamp: u64) -> Result<(), &'static str> {
        let actual_name = self.resolve_virtual_service(name).unwrap_or(name);

        if !self.services.contains_key(actual_name) {
            return Err("Service does not exist");
        }

        let service = self.services.get_mut(actual_name).unwrap();

        if service.is_running() {
            return Err("Service is already running");
        }

        // Check dependencies
        for dep in &service.config.dependencies {
            match dep.dep_type {
                DependencyType::Need => {
                    let dep_name = self.resolve_virtual_service(&dep.target).unwrap_or(&dep.target);
                    if let Some(dep_service) = self.services.get(dep_name) {
                        if !dep_service.is_running() {
                            return Err("Required dependency not running");
                        }
                    } else {
                        return Err("Required dependency not found");
                    }
                }
                DependencyType::Want => {
                    let dep_name = self.resolve_virtual_service(&dep.target).unwrap_or(&dep.target);
                    if let Some(dep_service) self.services.get(dep_name) {
                        if !dep_service.is_running() {
                            // Try to start it
                            let _ = self.start_service(dep_name, timestamp);
                        }
                    }
                }
                _ => {} // Handle before/after in dependency resolution
            }
        }

        service.set_state(ServiceState::Starting, timestamp);
        // In a real implementation, this would execute the command
        service.set_state(ServiceState::Started, timestamp);
        Ok(())
    }

    pub fn stop_service(&mut self, name: &str, timestamp: u64) -> Result<(), &'static str> {
        let actual_name = self.resolve_virtual_service(name).unwrap_or(name);

        if !self.services.contains_key(actual_name) {
            return Err("Service does not exist");
        }

        let service = self.services.get_mut(actual_name).unwrap();

        if !service.is_running() {
            return Err("Service is not running");
        }

        // Check if other services depend on this one
        for other_service in self.services.values() {
            for dep in &other_service.config.dependencies {
                if dep.dep_type == DependencyType::Need {
                    let dep_name = self.resolve_virtual_service(&dep.target).unwrap_or(&dep.target);
                    if dep_name == actual_name && other_service.is_running() {
                        return Err("Service is needed by other running services");
                    }
                }
            }
        }

        service.set_state(ServiceState::Stopping, timestamp);
        // In a real implementation, this would send signal to process
        service.set_state(ServiceState::Stopped, timestamp);
        Ok(())
    }

    pub fn restart_service(&mut self, name: &str, timestamp: u64) -> Result<(), &'static str> {
        self.stop_service(name, timestamp)?;
        self.start_service(name, timestamp)?;
        Ok(())
    }

    pub fn add_runlevel(&mut self, name: String, is_default: bool) -> Result<(), &'static str> {
        if self.runlevels.contains_key(&name) {
            return Err("Runlevel already exists");
        }

        if is_default {
            // Remove default from other runlevels
            for runlevel in self.runlevels.values_mut() {
                runlevel.is_default = false;
            }
        }

        self.runlevels.insert(name.clone(), Runlevel::new(name, is_default));
        Ok(())
    }

    pub fn add_service_to_runlevel(&mut self, runlevel_name: &str, service_name: String) -> Result<(), &'static str> {
        if !self.runlevels.contains_key(runlevel_name) {
            return Err("Runlevel does not exist");
        }

        if !self.services.contains_key(&service_name) {
            return Err("Service does not exist");
        }

        let runlevel = self.runlevels.get_mut(runlevel_name).unwrap();
        runlevel.add_service(service_name);
        Ok(())
    }

    pub fn switch_runlevel(&mut self, new_runlevel: &str, timestamp: u64) -> Result<(), &'static str> {
        if !self.runlevels.contains_key(new_runlevel) {
            return Err("Runlevel does not exist");
        }

        let old_runlevel = self.current_runlevel.clone();
        
        // Stop services not in new runlevel
        if let Some(old_level) = self.runlevels.get(&old_runlevel) {
            for service_name in &old_level.services {
                if let Some(new_level) = self.runlevels.get(new_runlevel) {
                    if !new_level.services.contains(service_name) {
                        let _ = self.stop_service(service_name, timestamp);
                    }
                }
            }
        }

        // Start services in new runlevel
        if let Some(new_level) = self.runlevels.get(new_runlevel) {
            for service_name in &new_level.services {
                let _ = self.start_service(service_name, timestamp);
            }
        }

        self.current_runlevel = new_runlevel.to_string();
        Ok(())
    }

    pub fn get_current_runlevel(&self) -> &str {
        &self.current_runlevel
    }

    pub fn get_runlevel(&self, name: &str) -> Option<&Runlevel> {
        self.runlevels.get(name)
    }

    pub fn list_services(&self) -> Vec<&ServiceInstance> {
        self.services.values().collect()
    }

    pub fn list_runlevels(&self) -> Vec<&Runlevel> {
        self.runlevels.values().collect()
    }

    /// Dependency resolution for service startup order
    pub fn resolve_startup_order(&self, service_names: &[String]) -> Result<Vec<String>, &'static str> {
        let mut order = Vec::new();
        let mut visited = alloc::collections::BTreeSet::new();

        for name in service_names {
            self.visit_dependency(name, &mut order, &mut visited)?;
        }

        Ok(order)
    }

    fn visit_dependency(
        &self,
        name: &str,
        order: &mut Vec<String>,
        visited: &mut alloc::collections::BTreeSet<String>,
    ) -> Result<(), &'static str> {
        let actual_name = self.resolve_virtual_service(name).unwrap_or(name);

        if visited.contains(actual_name) {
            return Ok(());
        }

        visited.insert(actual_name.to_string());

        if let Some(service) = self.services.get(actual_name) {
            for dep in &service.config.dependencies {
                match dep.dep_type {
                    DependencyType::Need | DependencyType::Want => {
                        let dep_name = self.resolve_virtual_service(&dep.target).unwrap_or(&dep.target);
                        self.visit_dependency(dep_name, order, visited)?;
                    }
                    _ => {}
                }
            }
        }

        order.push(actual_name.to_string());
        Ok(())
    }

    /// Get services that should be auto-started in current runlevel
    pub fn get_auto_start_services(&self) -> Vec<String> {
        let mut services = Vec::new();

        if let Some(runlevel) = self.runlevels.get(&self.current_runlevel) {
            for service_name in &runlevel.services {
                if let Some(service) = self.services.get(service_name) {
                    if service.config.auto_start {
                        services.push(service_name.clone());
                    }
                }
            }
        }

        services
    }

    /// Check service health and restart if needed
    pub fn check_service_health(&mut self, timestamp: u64) -> Vec<String> {
        let mut restarted = Vec::new();

        for (name, service) in self.services.iter_mut() {
            if service.should_restart() {
                let _ = self.restart_service(name, timestamp);
                restarted.push(name.clone());
            }
        }

        restarted
    }
}

/// OpenRC-inspired service supervisor
pub struct ServiceSupervisor {
    manager: OpenRCManager,
    check_interval: u64, // milliseconds
    last_check: u64,
}

impl ServiceSupervisor {
    pub fn new(manager: OpenRCManager, check_interval: u64) -> Self {
        Self {
            manager,
            check_interval,
            last_check: 0,
        }
    }

    pub fn manager(&mut self) -> &mut OpenRCManager {
        &mut self.manager
    }

    pub fn tick(&mut self, current_time: u64) -> Vec<String> {
        if current_time - self.last_check >= self.check_interval {
            self.last_check = current_time;
            self.manager.check_service_health(current_time)
        } else {
            Vec::new()
        }
    }

    pub fn start_runlevel_services(&mut self, timestamp: u64) -> Result<(), &'static str> {
        let auto_services = self.manager.get_auto_start_services();
        let startup_order = self.manager.resolve_startup_order(&auto_services)?;

        for service_name in startup_order {
            let _ = self.manager.start_service(&service_name, timestamp);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_service_config() {
        let config = ServiceConfig::new("test".to_string(), "/bin/test".to_string())
            .with_description("Test service".to_string())
            .with_dependency("network".to_string(), DependencyType::Need);

        assert_eq!(config.name, "test");
        assert_eq!(config.dependencies.len(), 1);
    }

    #[test]
    fn test_openrc_manager() {
        let mut manager = OpenRCManager::new();
        let config = ServiceConfig::new("test".to_string(), "/bin/test".to_string());
        manager.add_service(config).unwrap();
        assert!(manager.get_service("test").is_some());
    }

    #[test]
    fn test_runlevel_management() {
        let mut manager = OpenRCManager::new();
        manager.add_service_to_runlevel("default", "test".to_string()).unwrap_err(); // Service doesn't exist
        
        let config = ServiceConfig::new("test".to_string(), "/bin/test".to_string());
        manager.add_service(config).unwrap();
        manager.add_service_to_runlevel("default", "test".to_string()).unwrap();
        
        let runlevel = manager.get_runlevel("default").unwrap();
        assert!(runlevel.services.contains(&"test".to_string()));
    }

    #[test]
    fn test_dependency_resolution() {
        let mut manager = OpenRCManager::new();
        
        let network = ServiceConfig::new("network".to_string(), "/bin/network".to_string());
        manager.add_service(network).unwrap();
        
        let web = ServiceConfig::new("web".to_string(), "/bin/web".to_string())
            .with_dependency("network".to_string(), DependencyType::Need);
        manager.add_service(web).unwrap();
        
        let order = manager.resolve_startup_order(&vec!["web".to_string(), "network".to_string()]).unwrap();
        assert_eq!(order[0], "network");
        assert_eq!(order[1], "web");
    }
}