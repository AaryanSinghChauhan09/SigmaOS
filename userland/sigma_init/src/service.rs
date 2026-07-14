use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, PartialEq)]
pub enum ServiceState {
    Stopped,
    Starting,
    Running,
    Failed,
}

/// Native Rust struct representing a system service, displacing systemd .service INI files.
#[derive(Debug, Clone)]
pub struct Service {
    pub name: String,
    pub exec_start: String,
    pub requires: Vec<String>,
    pub wants: Vec<String>,
    pub socket_activation: Option<u16>, // Port for socket activation
    pub state: ServiceState,
}

impl Service {
    pub fn new(name: &str, exec_start: &str) -> Self {
        Self {
            name: name.to_string(),
            exec_start: exec_start.to_string(),
            requires: Vec::new(),
            wants: Vec::new(),
            socket_activation: None,
            state: ServiceState::Stopped,
        }
    }

    pub fn requires(mut self, dep: &str) -> Self {
        self.requires.push(dep.to_string());
        self
    }

    pub fn socket_activation(mut self, port: u16) -> Self {
        self.socket_activation = Some(port);
        self
    }
}

pub struct ServiceManager {
    services: HashMap<String, Service>,
}

impl Default for ServiceManager {
    fn default() -> Self {
        Self::new()
    }
}

impl ServiceManager {
    pub fn new() -> Self {
        Self {
            services: HashMap::new(),
        }
    }

    pub fn add_service(&mut self, service: Service) {
        self.services.insert(service.name.clone(), service);
    }

    pub fn start_service(&mut self, name: &str) -> Result<(), String> {
        if let Some(service) = self.services.get_mut(name) {
            if let Some(port) = service.socket_activation {
                // In a real system, this binds the socket and waits
                println!("Service {} configured for socket activation on port {}", name, port);
                service.state = ServiceState::Stopped; // Wait for connection
            } else {
                println!("Starting service: {}", name);
                service.state = ServiceState::Running;
            }
            Ok(())
        } else {
            Err(format!("Service {} not found", name))
        }
    }
}
