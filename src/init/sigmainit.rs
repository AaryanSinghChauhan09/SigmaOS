// SigmaInit - Modern Init System
// Inspired by OpenRC, runit, s6 (systemd alternatives)


use std::string::String;
use std::vec::Vec;
use std::collections::BTreeMap;
use std::boxed::Box;
use core::sync::atomic::{AtomicBool, AtomicU32, Ordering};

/// Service restart policy
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RestartPolicy {
    Always,
    OnFailure,
    Never,
}

/// Service state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServiceState {
    Stopped,
    Starting,
    Running,
    Stopping,
    Failed,
}

/// System target (Systemd targets & SysVInit / OpenRC runlevel parity)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SystemTarget {
    Emergency,      // Emergency mode: Minimal read-only rootfs recovery shell
    Rescue,         // Runlevel 1 / Single-user mode: Single-user maintenance
    MultiUserNoNet, // Runlevel 2: Multi-user text console without networking
    MultiUser,      // Runlevel 3: Standard multi-user text console with networking
    Graphical,      // Runlevel 5: Multi-user graphical desktop environment
    Cloud,          // Headless cloud-init server profile
    Realtime,       // Hard real-time audio/HPC workload profile
    Reboot,         // Runlevel 6: System reboot
    Poweroff,       // Runlevel 0: System shutdown / poweroff
}

impl SystemTarget {
    pub fn to_runlevel_number(&self) -> u8 {
        match self {
            SystemTarget::Poweroff => 0,
            SystemTarget::Emergency => 1,
            SystemTarget::Rescue => 1,
            SystemTarget::MultiUserNoNet => 2,
            SystemTarget::MultiUser => 3,
            SystemTarget::Cloud => 3,
            SystemTarget::Realtime => 4,
            SystemTarget::Graphical => 5,
            SystemTarget::Reboot => 6,
        }
    }

    pub fn to_target_name(&self) -> &'static str {
        match self {
            SystemTarget::Emergency => "emergency.target",
            SystemTarget::Rescue => "rescue.target",
            SystemTarget::MultiUserNoNet => "multi-user-nonet.target",
            SystemTarget::MultiUser => "multi-user.target",
            SystemTarget::Graphical => "graphical.target",
            SystemTarget::Cloud => "cloud.target",
            SystemTarget::Realtime => "realtime.target",
            SystemTarget::Reboot => "reboot.target",
            SystemTarget::Poweroff => "poweroff.target",
        }
    }
}

/// Service definition
#[derive(Debug, Clone)]
pub struct Service {
    pub name: String,
    pub description: String,
    pub depends: Vec<String>,
    pub command: Vec<String>,
    pub working_dir: Option<String>,
    pub environment: BTreeMap<String, String>,
    pub capabilities: Vec<String>,
    pub restart_policy: RestartPolicy,
    pub state: ServiceState,
    pub pid: Option<u32>,
    pub socket: Option<String>,
    pub timer: Option<String>,
}

impl Service {
    pub fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
            description: String::new(),
            depends: Vec::new(),
            command: Vec::new(),
            working_dir: None,
            environment: BTreeMap::new(),
            capabilities: Vec::new(),
            restart_policy: RestartPolicy::OnFailure,
            state: ServiceState::Stopped,
            pid: None,
            socket: None,
            timer: None,
        }
    }
    
    pub fn with_description(mut self, desc: &str) -> Self {
        self.description = String::from(desc);
        self
    }
    
    pub fn with_command(mut self, command: Vec<String>) -> Self {
        self.command = command;
        self
    }
    
    pub fn with_depends(mut self, depends: Vec<String>) -> Self {
        self.depends = depends;
        self
    }
    
    pub fn with_restart_policy(mut self, policy: RestartPolicy) -> Self {
        self.restart_policy = policy;
        self
    }
}

/// Dependency graph for service management
pub struct DependencyGraph {
    services: BTreeMap<String, Vec<String>>, // service -> dependencies
    reverse_deps: BTreeMap<String, Vec<String>>, // service -> dependents
}

impl DependencyGraph {
    pub fn new() -> Self {
        Self {
            services: BTreeMap::new(),
            reverse_deps: BTreeMap::new(),
        }
    }
    
    pub fn add_service(&mut self, name: &str, dependencies: Vec<String>) {
        self.services.insert(String::from(name), dependencies.clone());
        
        for dep in &dependencies {
            self.reverse_deps
                .entry(dep.clone())
                .or_insert_with(Vec::new)
                .push(String::from(name));
        }
    }
    
    pub fn get_dependencies(&self, name: &str) -> Option<&Vec<String>> {
        self.services.get(name)
    }
    
    pub fn get_dependents(&self, name: &str) -> Option<&Vec<String>> {
        self.reverse_deps.get(name)
    }
    
    pub fn topological_sort(&self) -> Result<Vec<String>, DependencyError> {
        let mut visited = BTreeMap::new();
        let mut temp_mark = BTreeMap::new();
        let mut result = Vec::new();
        
        for name in self.services.keys() {
            if !visited.contains_key(name) {
                self.visit(name, &mut visited, &mut temp_mark, &mut result)?;
            }
        }
        
        Ok(result)
    }
    
    fn visit(
        &self,
        name: &str,
        visited: &mut BTreeMap<String, bool>,
        temp_mark: &mut BTreeMap<String, bool>,
        result: &mut Vec<String>,
    ) -> Result<(), DependencyError> {
        if temp_mark.contains_key(name) {
            return Err(DependencyError::Cycle);
        }
        
        if visited.contains_key(name) {
            return Ok(());
        }
        
        temp_mark.insert(String::from(name), true);
        
        if let Some(deps) = self.services.get(name) {
            for dep in deps {
                self.visit(dep, visited, temp_mark, result)?;
            }
        }
        
        temp_mark.remove(name);
        visited.insert(String::from(name), true);
        result.push(String::from(name));
        
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DependencyError {
    Cycle,
    NotFound,
}

/// Process supervisor (runit/s6-style)
pub struct Supervisor {
    services: BTreeMap<String, Service>,
    dependency_graph: DependencyGraph,
}

impl Supervisor {
    pub fn new() -> Self {
        Self {
            services: BTreeMap::new(),
            dependency_graph: DependencyGraph::new(),
        }
    }
    
    pub fn add_service(&mut self, service: Service) {
        let name = service.name.clone();
        let deps = service.depends.clone();
        self.dependency_graph.add_service(&name, deps);
        self.services.insert(name, service);
    }
    
    pub fn start_service(&mut self, name: &str) -> Result<(), ServiceError> {
        // Start dependencies first
        let deps = self.dependency_graph.get_dependencies(name).cloned();
        if let Some(deps) = deps {
            for dep in &deps {
                self.start_service(dep)?;
            }
        }

        if let Some(service) = self.services.get_mut(name) {
            service.state = ServiceState::Starting;
            // In a real implementation, this would fork and execute
            service.state = ServiceState::Running;
            service.pid = Some(42); // Mock PID
            Ok(())
        } else {
            Err(ServiceError::NotFound)
        }
    }

    pub fn stop_service(&mut self, name: &str) -> Result<(), ServiceError> {
        // Stop dependents first
        let dependents = self.dependency_graph.get_dependents(name).cloned();
        if let Some(dependents) = dependents {
            for dep in &dependents {
                self.stop_service(dep)?;
            }
        }
        
        if let Some(service) = self.services.get_mut(name) {
            service.state = ServiceState::Stopping;
            // In a real implementation, this would send SIGTERM
            service.state = ServiceState::Stopped;
            service.pid = None;
            Ok(())
        } else {
            Err(ServiceError::NotFound)
        }
    }
    
    pub fn restart_service(&mut self, name: &str) -> Result<(), ServiceError> {
        self.stop_service(name)?;
        self.start_service(name)
    }
    
    pub fn get_service_state(&self, name: &str) -> Option<ServiceState> {
        self.services.get(name).map(|s| s.state)
    }
    
    pub fn start_service_if_exists(&mut self, name: &str) -> Result<(), ServiceError> {
        if self.services.contains_key(name) {
            self.start_service(name)
        } else {
            Ok(())
        }
    }

    pub fn stop_all_services(&mut self) -> Result<(), ServiceError> {
        let order_opt = self.dependency_graph.topological_sort().ok();
        if let Some(order) = order_opt {
            for service_name in order.into_iter().rev() {
                let _ = self.stop_service(&service_name);
            }
        }
        Ok(())
    }

    pub fn start_target(&mut self, target: SystemTarget) -> Result<(), ServiceError> {
        match target {
            SystemTarget::Emergency => {
                self.start_service_if_exists("emergency-shell")?;
            }
            SystemTarget::Rescue => {
                self.start_service_if_exists("syslog")?;
                self.start_service_if_exists("rescue-shell")?;
            }
            SystemTarget::MultiUserNoNet => {
                self.start_service_if_exists("syslog")?;
                self.start_service_if_exists("dbus")?;
                self.start_service_if_exists("local-console")?;
            }
            SystemTarget::MultiUser => {
                self.start_service_if_exists("syslog")?;
                self.start_service_if_exists("network")?;
                self.start_service_if_exists("sshd")?;
                self.start_service_if_exists("cron")?;
            }
            SystemTarget::Graphical => {
                self.start_target(SystemTarget::MultiUser)?;
                self.start_service_if_exists("display-manager")?;
                self.start_service_if_exists("desktop-environment")?;
            }
            SystemTarget::Cloud => {
                self.start_service_if_exists("syslog")?;
                self.start_service_if_exists("network")?;
                self.start_service_if_exists("cloud-init")?;
                self.start_service_if_exists("sshd")?;
            }
            SystemTarget::Realtime => {
                self.start_service_if_exists("syslog")?;
                self.start_service_if_exists("realtime-scheduler")?;
            }
            SystemTarget::Reboot | SystemTarget::Poweroff => {
                self.stop_all_services()?;
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServiceError {
    NotFound,
    AlreadyRunning,
    AlreadyStopped,
    DependencyFailed,
    StartFailed,
    StopFailed,
    AuthenticationRequired,
    TargetTransitionFailed,
}

/// Main SigmaInit manager
pub struct SigmaInit {
    pub supervisor: Supervisor,
    pub current_target: SystemTarget,
    pub boot_complete: AtomicBool,
    pub rescue_authenticated: bool,
}

impl SigmaInit {
    pub fn new() -> Self {
        Self {
            supervisor: Supervisor::new(),
            current_target: SystemTarget::MultiUser,
            boot_complete: AtomicBool::new(false),
            rescue_authenticated: false,
        }
    }

    pub fn authenticate_rescue(&mut self, provided_secret: &str, expected_secret: &str) -> bool {
        if provided_secret == expected_secret {
            self.rescue_authenticated = true;
            true
        } else {
            false
        }
    }
    
    pub fn load_services(&mut self, services: Vec<Service>) {
        for service in services {
            self.supervisor.add_service(service);
        }
    }
    
    pub fn boot(&mut self, target: SystemTarget) -> Result<(), ServiceError> {
        self.current_target = target;
        
        // Perform topological sort and start services in order
        let order = self.supervisor.dependency_graph.topological_sort()
            .map_err(|_| ServiceError::DependencyFailed)?;
        
        for service_name in order {
            if let Err(e) = self.supervisor.start_service(&service_name) {
                return Err(e);
            }
        }
        
        self.boot_complete.store(true, Ordering::SeqCst);
        Ok(())
    }
    
    pub fn shutdown(&mut self) -> Result<(), ServiceError> {
        // Stop services in reverse dependency order
        let order = self.supervisor.dependency_graph.topological_sort()
            .map_err(|_| ServiceError::DependencyFailed)?;
        
        for service_name in order.into_iter().rev() {
            let _ = self.supervisor.stop_service(&service_name);
        }
        
        self.boot_complete.store(false, Ordering::SeqCst);
        Ok(())
    }
    
    pub fn is_boot_complete(&self) -> bool {
        self.boot_complete.load(Ordering::SeqCst)
    }
    
    pub fn switch_target(&mut self, target: SystemTarget) -> Result<(), ServiceError> {
        if target == SystemTarget::Rescue || target == SystemTarget::Emergency {
            if !self.rescue_authenticated {
                return Err(ServiceError::AuthenticationRequired);
            }
        }

        let previous_target = self.current_target;
        if let Err(e) = self.supervisor.start_target(target) {
            // Target transition failed! Rollback to previous target
            let _ = self.supervisor.start_target(previous_target);
            self.current_target = previous_target;
            return Err(ServiceError::TargetTransitionFailed);
        }

        self.current_target = target;
        Ok(())
    }

    /// Isolates a system target (`systemctl isolate` parity), stopping non-target units first
    pub fn isolate_target(&mut self, target: SystemTarget) -> Result<(), ServiceError> {
        if target == SystemTarget::Rescue || target == SystemTarget::Emergency {
            if !self.rescue_authenticated {
                return Err(ServiceError::AuthenticationRequired);
            }
        }

        self.supervisor.stop_all_services()?;
        self.switch_target(target)
    }
}

impl Default for SigmaInit {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

    #[test]
    fn test_service_creation() {
        let service = Service::new("test-service")
            .with_description("Test service")
            .with_command(vec![String::from("/bin/test")])
            .with_restart_policy(RestartPolicy::Always);
        
        assert_eq!(service.name, "test-service");
        assert_eq!(service.restart_policy, RestartPolicy::Always);
    }

    #[test]
    fn test_dependency_graph() {
        let mut graph = DependencyGraph::new();
        graph.add_service("network", vec![]);
        graph.add_service("sshd", vec![String::from("network")]);
        
        let deps = graph.get_dependencies("sshd");
        assert!(deps.is_some());
        assert_eq!(deps.unwrap(), &vec![String::from("network")]);
    }

    #[test]
    fn test_topological_sort() {
        let mut graph = DependencyGraph::new();
        graph.add_service("network", vec![]);
        graph.add_service("syslog", vec![]);
        graph.add_service("sshd", vec![String::from("network"), String::from("syslog")]);
        
        let order = graph.topological_sort().unwrap();
        assert!(order.len() == 3);
        // network and syslog should come before sshd
        let sshd_idx = order.iter().position(|x| x == "sshd").unwrap();
        let network_idx = order.iter().position(|x| x == "network").unwrap();
        let syslog_idx = order.iter().position(|x| x == "syslog").unwrap();
        assert!(sshd_idx > network_idx);
        assert!(sshd_idx > syslog_idx);
    }

    #[test]
    fn test_supervisor() {
        let mut supervisor = Supervisor::new();
        
        let network = Service::new("network")
            .with_command(vec![String::from("/bin/network")]);
        
        let sshd = Service::new("sshd")
            .with_command(vec![String::from("/bin/sshd")])
            .with_depends(vec![String::from("network")]);
        
        supervisor.add_service(network);
        supervisor.add_service(sshd);
        
        assert!(supervisor.start_service("sshd").is_ok());
        assert_eq!(supervisor.get_service_state("network"), Some(ServiceState::Running));
        assert_eq!(supervisor.get_service_state("sshd"), Some(ServiceState::Running));
    }

    #[test]
    fn test_sigmainit_boot() {
        let mut init = SigmaInit::new();
        
        let syslog = Service::new("syslog")
            .with_command(vec![String::from("/bin/syslog")]);
        
        let network = Service::new("network")
            .with_command(vec![String::from("/bin/network")])
            .with_depends(vec![String::from("syslog")]);
        
        init.load_services(vec![syslog, network]);
        
        assert!(init.boot(SystemTarget::MultiUser).is_ok());
        assert!(init.is_boot_complete());
    }

    #[test]
    fn test_system_target_runlevels_and_names() {
        assert_eq!(SystemTarget::Poweroff.to_runlevel_number(), 0);
        assert_eq!(SystemTarget::Rescue.to_runlevel_number(), 1);
        assert_eq!(SystemTarget::MultiUserNoNet.to_runlevel_number(), 2);
        assert_eq!(SystemTarget::MultiUser.to_runlevel_number(), 3);
        assert_eq!(SystemTarget::Graphical.to_runlevel_number(), 5);
        assert_eq!(SystemTarget::Reboot.to_runlevel_number(), 6);

        assert_eq!(SystemTarget::Graphical.to_target_name(), "graphical.target");
        assert_eq!(SystemTarget::Rescue.to_target_name(), "rescue.target");
    }

    #[test]
    fn test_rescue_mode_authentication_gate() {
        let mut init = SigmaInit::new();
        assert_eq!(init.switch_target(SystemTarget::Rescue), Err(ServiceError::AuthenticationRequired));

        assert!(init.authenticate_rescue("secret_pass", "secret_pass"));
        assert!(init.switch_target(SystemTarget::Rescue).is_ok());
        assert_eq!(init.current_target, SystemTarget::Rescue);
    }

    #[test]
    fn test_target_isolation_and_switching() {
        let mut init = SigmaInit::new();

        let syslog = Service::new("syslog").with_command(vec![String::from("/bin/syslog")]);
        let network = Service::new("network").with_command(vec![String::from("/bin/network")]);
        let dm = Service::new("display-manager").with_command(vec![String::from("/bin/dm")]);

        init.load_services(vec![syslog, network, dm]);

        // Boot into MultiUser
        assert!(init.switch_target(SystemTarget::MultiUser).is_ok());
        assert_eq!(init.supervisor.get_service_state("syslog"), Some(ServiceState::Running));

        // Isolate Graphical target
        assert!(init.isolate_target(SystemTarget::Graphical).is_ok());
        assert_eq!(init.current_target, SystemTarget::Graphical);
        assert_eq!(init.supervisor.get_service_state("display-manager"), Some(ServiceState::Running));
    }
}
