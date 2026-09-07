// SigmaOS Void Linux Runit Implementation
// Implements Void Linux's runit supervision system
// Inspired by Void Linux's 3-stage process supervision

use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;

/// Service state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServiceState {
    Running,
    Stopped,
    Failed,
    Restarting,
}

/// Service
#[derive(Debug, Clone)]
pub struct RunitService {
    pub name: String,
    pub command: String,
    pub state: ServiceState,
    pub dependencies: Vec<String>,
    pub restart_count: u32,
    pub log_enabled: bool,
}

impl RunitService {
    pub fn new(name: String, command: String) -> Self {
        Self {
            name,
            command,
            state: ServiceState::Stopped,
            dependencies: Vec::new(),
            restart_count: 0,
            log_enabled: true,
        }
    }

    /// Start service
    pub fn start(&mut self) {
        self.state = ServiceState::Running;
        println!("Starting service: {}", self.name);
    }

    /// Stop service
    pub fn stop(&mut self) {
        self.state = ServiceState::Stopped;
        println!("Stopping service: {}", self.name);
    }

    /// Restart service
    pub fn restart(&mut self) {
        self.restart_count += 1;
        self.state = ServiceState::Restarting;
        println!(
            "Restarting service: {} (restart #{})",
            self.name, self.restart_count
        );
        self.state = ServiceState::Running;
    }

    /// Enable logging
    pub fn enable_logging(&mut self) {
        self.log_enabled = true;
    }

    /// Disable logging
    pub fn disable_logging(&mut self) {
        self.log_enabled = false;
    }
}

/// Runit supervisor stage
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RunitStage {
    Stage1, // One-time system initialization
    Stage2, // Concurrent process supervision
    Stage3, // Clean system shutdown
}

/// Runit supervisor
pub struct RunitSupervisor {
    pub services: BTreeMap<String, RunitService>,
    pub stage: RunitStage,
    pub current_stage_num: u32,
}

impl RunitSupervisor {
    pub fn new() -> Self {
        Self {
            services: BTreeMap::new(),
            stage: RunitStage::Stage1,
            current_stage_num: 1,
        }
    }

    /// Add service
    pub fn add_service(&mut self, service: RunitService) {
        self.services.insert(service.name.clone(), service);
    }

    /// Start stage 1 (one-time initialization)
    pub fn run_stage1(&mut self) {
        self.stage = RunitStage::Stage1;
        self.current_stage_num = 1;
        println!("Running Stage 1: One-time system initialization");

        // Run one-time initialization tasks
        println!("Mounting virtual filesystems");
        println!("Setting hostname");
        println!("Initializing devices");
    }

    /// Start stage 2 (concurrent supervision)
    pub fn run_stage2(&mut self) {
        self.stage = RunitStage::Stage2;
        self.current_stage_num = 2;
        println!("Running Stage 2: Concurrent process supervision");

        // Start all services respecting dependencies
        let mut started = Vec::new();

        for (name, service) in self.services.clone() {
            if self.can_start_service(&name, &started) {
                if let Some(s) = self.services.get_mut(&name) {
                    s.start();
                    started.push(name);
                }
            }
        }
    }

    /// Start stage 3 (clean shutdown)
    pub fn run_stage3(&mut self) {
        self.stage = RunitStage::Stage3;
        self.current_stage_num = 3;
        println!("Running Stage 3: Clean system shutdown");

        // Stop all services in reverse dependency order
        let mut stopped = Vec::new();

        for (name, service) in self.services.clone() {
            if self.can_stop_service(&name, &stopped) {
                if let Some(s) = self.services.get_mut(&name) {
                    s.stop();
                    stopped.push(name);
                }
            }
        }

        println!("Unmounting filesystems");
    }

    /// Check if service can start (dependencies satisfied)
    fn can_start_service(&self, name: &str, started: &[String]) -> bool {
        if let Some(service) = self.services.get(name) {
            for dep in &service.dependencies {
                if !started.contains(dep) {
                    return false;
                }
            }
            true
        } else {
            false
        }
    }

    /// Check if service can stop (no dependents still running)
    fn can_stop_service(&self, name: &str, stopped: &[String]) -> bool {
        for (_, service) in &self.services {
            if service.dependencies.contains(&name.to_string())
                && service.state == ServiceState::Running
            {
                return false;
            }
        }
        true
    }

    /// Get service status
    pub fn get_service_status(&self, name: &str) -> Option<&RunitService> {
        self.services.get(name)
    }

    /// Get all services
    pub fn get_all_services(&self) -> Vec<&RunitService> {
        self.services.values().collect()
    }

    /// Get services by state
    pub fn get_services_by_state(&self, state: ServiceState) -> Vec<&RunitService> {
        self.services
            .values()
            .filter(|s| s.state == state)
            .collect()
    }
}

impl Default for RunitSupervisor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_runit_service() {
        let mut service =
            RunitService::new("test-service".to_string(), "/usr/bin/test".to_string());
        service.start();
        assert_eq!(service.state, ServiceState::Running);
    }

    #[test]
    fn test_runit_supervisor() {
        let mut supervisor = RunitSupervisor::new();

        let service = RunitService::new("test".to_string(), "/usr/bin/test".to_string());
        supervisor.add_service(service);

        supervisor.run_stage2();
        assert_eq!(
            supervisor
                .get_services_by_state(ServiceState::Running)
                .len(),
            1
        );
    }

    #[test]
    fn test_service_dependencies() {
        let mut supervisor = RunitSupervisor::new();

        let mut service1 = RunitService::new("service1".to_string(), "/usr/bin/s1".to_string());
        let mut service2 = RunitService::new("service2".to_string(), "/usr/bin/s2".to_string());
        service2.dependencies = vec!["service1".to_string()];

        supervisor.add_service(service1);
        supervisor.add_service(service2);

        supervisor.run_stage2();

        // Service1 should start first
        assert_eq!(
            supervisor.get_service_status("service1").unwrap().state,
            ServiceState::Running
        );
        assert_eq!(
            supervisor.get_service_status("service2").unwrap().state,
            ServiceState::Running
        );
    }
}
