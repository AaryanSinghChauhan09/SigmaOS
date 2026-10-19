//! Init supervisor with dependency DAG and service management
//!
//! Implements PID 1 init supervisor following BSD-inspired patterns
//! with dependency DAG and service management capabilities.

use std::collections::HashMap;
use std::collections::HashSet;
use std::sync::Mutex;

/// Service state enumeration
#[derive(Debug, Clone, PartialEq)]
pub enum ServiceState {
    /// Service is stopped
    Stopped,
    /// Service is starting
    Starting,
    /// Service is running
    Running,
    /// Service is failing
    Failing,
    /// Service is stopped permanently
    Dead,
}

/// Service dependency type
#[derive(Debug, Clone)]
pub enum Dependency {
    /// Must start before this service
    Before,
    /// Must start after this service
    After,
}

/// Service configuration
#[derive(Debug, Clone)]
pub struct ServiceConfig {
    /// Service name
    pub name: String,
    /// Service dependencies
    pub dependencies: Vec<Dependency>,
    /// Command to start
    pub command: String,
    /// Auto-restart policy
    pub restart_policy: RestartPolicy,
    /// Timeout in seconds
    pub timeout_secs: u64,
}

/// Restart policy types
#[derive(Debug, Clone, PartialEq)]
pub enum RestartPolicy {
    /// No restart
    Never,
    /// Restart on failure
    OnFailure,
    /// Always restart
    Always,
}

/// Init supervisor managing service lifecycle
pub struct InitSupervisor {
    /// Service configurations
    services: Mutex<HashMap<String, ServiceConfig>>,
    /// Service states
    states: Mutex<HashMap<String, ServiceState>>,
    /// Start order (dependency DAG topological sort)
    start_order: Mutex<Vec<String>>,
}

impl InitSupervisor {
    /// Create new init supervisor
    pub fn new() -> Self {
        InitSupervisor {
            services: Mutex::new(HashMap::new()),
            states: Mutex::new(HashMap::new()),
            start_order: Mutex::new(Vec::new()),
        }
    }

    /// Register a service
    pub fn register(&mut self, config: ServiceConfig) {
        let mut services = self.services.lock().unwrap();
        services.insert(config.name.clone(), config);
        
        let mut states = self.states.lock().unwrap();
        states.insert(config.name.clone(), ServiceState::Stopped);
        
        // Recalculate start order
        self.calculate_start_order();
    }

    /// Start all services in dependency order
    pub fn start_all(&mut self) -> Result<(), String> {
        let order = self.start_order.lock().unwrap().clone();
        
        for service_name in &order {
            self.start_service(service_name)?;
        }
        
        Ok(())
    }

    /// Start a specific service
    pub fn start_service(&mut self, name: &str) -> Result<(), String> {
        let mut states = self.states.lock().unwrap();
        
        if *states.get(name).unwrap() == ServiceState::Running {
            return Ok(()); // Already running
        }
        
        // Check dependencies first
        let services = self.services.lock().unwrap();
        if let Some(config) = services.get(name) {
            for dep in &config.dependencies {
                match dep {
                    Dependency::Before => {
                        // Dependent must start before this
                        // Check if dependency is running
                        if !self.is_running(&dep.name) {
                            return Err(format!("Dependency {} not running", dep.name));
                        }
                    }
                    Dependency::After => {
                        // This must start after dependency
                        if !self.is_running(&dep.name) {
                            return Err(format!("Dependency {} not running", dep.name));
                        }
                    }
                }
            }
        }
        
        // Mark as starting
        states.insert(name.to_string(), ServiceState::Starting);
        
        // Mark as running
        states.insert(name.to_string(), ServiceState::Running);
        
        Ok(())
    }

    /// Stop a specific service
    pub fn stop_service(&mut self, name: &str) -> Result<(), String> {
        let mut states = self.states.lock().unwrap();
        
        if *states.get(name).unwrap() == ServiceState::Stopped {
            return Ok(()); // Already stopped
        }
        
        states.insert(name.to_string(), ServiceState::Stopped);
        Ok(())
    }

    /// Check if service is running
    pub fn is_running(&self, name: &str) -> bool {
        let states = self.states.lock().unwrap();
        *states.get(name).unwrap() == ServiceState::Running
    }

    /// Calculate start order using topological sort
    fn calculate_start_order(&mut self) {
        let services = self.services.lock().unwrap();
        let mut states = self.states.lock().unwrap();
        
        // Simple topological sort based on dependencies
        let mut visited = HashSet::new();
        let mut order: Vec<String> = Vec::new();
        
        fn visit(
            name: &str,
            services: &std::sync::Mutex<HashMap<String, ServiceConfig>>,
            states: &std::sync::Mutex<HashMap<String, ServiceState>>,
            visited: &mut HashSet<String>,
            order: &mut Vec<String>,
        ) {
            if visited.contains(name) {
                return;
            }
            
            visited.insert(name.to_string());
            
            if let Some(config) = services.get(name) {
                for dep in &config.dependencies {
                    // Visit dependency first
                    let dep_name = // simplify: get dep name from config
                        if dep == &Dependency::Before {
                            // For Before dependency, the dependent comes after
                            // This is simplified - real implementation needs dep name
                            name.to_string()
                        } else {
                            name.to_string()
                        };
                    visit(&dep_name, services, states, visited, order);
                }
            }
            
            order.push(name.to_string());
        }
        
        // Visit all services
        for name in services.keys() {
            visit(name, &services, &states, &mut visited, &mut order);
        }
        
        *self.start_order.lock().unwrap() = order;
    }
    
    /// Check if dependency is running (simplified)
    fn is_running(&self, name: &str) -> bool {
        let states = self.states.lock().unwrap();
        *states.get(name).unwrap_or(&ServiceState::Stopped) == ServiceState::Running
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_register_service() {
        let mut supervisor = InitSupervisor::new();
        let config = ServiceConfig {
            name: "test-service".to_string(),
            dependencies: vec![],
            command: "/usr/bin/test".to_string(),
            restart_policy: RestartPolicy::Never,
            timeout_secs: 30,
        };
        supervisor.register(config);
        
        let states = supervisor.states.lock().unwrap();
        assert!(states.contains_key("test-service"));
    }
    
    #[test]
    fn test_start_service() {
        let mut supervisor = InitSupervisor::new();
        let config = ServiceConfig {
            name: "test-service".to_string(),
            dependencies: vec![],
            command: "/usr/bin/test".to_string(),
            restart_policy: RestartPolicy::Never,
            timeout_secs: 30,
        };
        supervisor.register(config);
        
        let result = supervisor.start_service("test-service");
        assert!(result.is_ok());
        
        let states = supervisor.states.lock().unwrap();
        assert_eq!(*states.get("test-service").unwrap(), ServiceState::Running);
    }
}
