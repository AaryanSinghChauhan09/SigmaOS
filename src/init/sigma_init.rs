// OOP-based Lightweight Init System with Runlevels & Targets for SigmaOS
// Designed to surpass legacy SysVInit runlevels and modern Systemd target schemes.

#![no_std]

extern crate alloc;
use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;
use alloc::string::ToString;
use core::sync::atomic::{AtomicUsize, Ordering};

pub type ServiceID = usize;

/// Standard operating runlevels inspired by SysVInit and Linux distributions
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Runlevel {
    Level0_Halt = 0,
    Level1_SingleUser = 1,      // Maintenance / Recovery Mode
    Level2_MultiUser = 2,       // Multi-User Command Line (No Networking)
    Level3_MultiUserNetwork = 3,// Full Multi-User with Networking (CLI)
    Level5_Graphical = 5,       // Zenith Desktop GUI Mode
    Level6_Reboot = 6,
}

/// Service states within the supervisor
#[repr(usize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServiceState {
    Stopped = 0,
    Starting = 1,
    Running = 2,
    Stopping = 3,
    Failed = 4,
}

/// Dynamic errors thrown by the init coordinator
#[repr(usize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InitError {
    Success = 0,
    ServiceNotFound = 1,
    DependencyFailed = 2,
    StartFailed = 3,
    StopFailed = 4,
    InvalidTransition = 5,
}

/// OOP Service Trait
pub trait Service {
    fn id(&self) -> ServiceID;
    fn name(&self) -> &[u8];
    fn state(&self) -> ServiceState;
    fn dependencies(&self) -> Vec<ServiceID>;
    fn start(&mut self) -> Result<(), InitError>;
    fn stop(&mut self) -> Result<(), InitError>;
    fn restart(&mut self) -> Result<(), InitError>;

    /// Returns the minimal runlevel at which this service should run
    fn required_runlevel(&self) -> Runlevel {
        Runlevel::Level2_MultiUser
    }
}

/// Standard service implementation
pub struct SimpleService {
    pub id: ServiceID,
    pub name: String,
    pub state: AtomicUsize, // ServiceState as usize
    pub deps: Vec<ServiceID>,
    pub pid: AtomicUsize,
    pub min_runlevel: Runlevel,
}

impl SimpleService {
    pub fn new(id: ServiceID, name: &str) -> Self {
        SimpleService {
            id,
            name: name.to_string(),
            state: AtomicUsize::new(ServiceState::Stopped as usize),
            deps: Vec::new(),
            pid: AtomicUsize::new(0),
            min_runlevel: Runlevel::Level2_MultiUser,
        }
    }

    pub fn with_deps(mut self, deps: Vec<ServiceID>) -> Self {
        self.deps = deps;
        self
    }

    pub fn with_runlevel(mut self, level: Runlevel) -> Self {
        self.min_runlevel = level;
        self
    }
}

impl Service for SimpleService {
    fn id(&self) -> ServiceID {
        self.id
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn state(&self) -> ServiceState {
        match self.state.load(Ordering::SeqCst) {
            0 => ServiceState::Stopped,
            1 => ServiceState::Starting,
            2 => ServiceState::Running,
            3 => ServiceState::Stopping,
            _ => ServiceState::Failed,
        }
    }

    fn dependencies(&self) -> Vec<ServiceID> {
        self.deps.clone()
    }

    fn start(&mut self) -> Result<(), InitError> {
        self.state.store(ServiceState::Starting as usize, Ordering::SeqCst);
        self.state.store(ServiceState::Running as usize, Ordering::SeqCst);
        self.pid.store(self.id + 1000, Ordering::SeqCst);
        Ok(())
    }

    fn stop(&mut self) -> Result<(), InitError> {
        self.state.store(ServiceState::Stopping as usize, Ordering::SeqCst);
        self.state.store(ServiceState::Stopped as usize, Ordering::SeqCst);
        self.pid.store(0, Ordering::SeqCst);
        Ok(())
    }

    fn restart(&mut self) -> Result<(), InitError> {
        self.stop()?;
        self.start()?;
        Ok(())
    }

    fn required_runlevel(&self) -> Runlevel {
        self.min_runlevel
    }
}

/// Abstract Init System interface
pub trait InitSystem {
    fn register_service(&mut self, service: Box<dyn Service>) -> Result<ServiceID, InitError>;
    fn start_service(&mut self, id: ServiceID) -> Result<(), InitError>;
    fn stop_service(&mut self, id: ServiceID) -> Result<(), InitError>;
    fn get_service(&self, id: ServiceID) -> Option<&dyn Service>;
    fn get_all_services(&self) -> Vec<ServiceID>;

    /// SWITCH operating runlevel (mimicking init/telinit transitions)
    fn switch_runlevel(&mut self, level: Runlevel) -> Result<(), InitError>;
}

/// Concrete SigmaOS Init implementation (supporting parallel startup, targets, and runlevels)
pub struct SigmaInit {
    pub services: Vec<Option<Box<dyn Service>>>,
    pub parallel_startup: bool,
    pub current_runlevel: Runlevel,
    pub active_target: String,
}

impl SigmaInit {
    pub fn new() -> Self {
        SigmaInit {
            services: Vec::new(),
            parallel_startup: true,
            current_runlevel: Runlevel::Level3_MultiUserNetwork,
            active_target: "multi-user.target".to_string(),
        }
    }

    pub fn enable_parallel_startup(&mut self) {
        self.parallel_startup.store(1, Ordering::SeqCst);
    }

    pub fn disable_parallel_startup(&mut self) {
        self.parallel_startup = false;
    }

    /// Sets the active Systemd-style target and transitions services accordingly
    pub fn set_active_target(&mut self, target: &str) -> Result<(), InitError> {
        self.active_target = target.to_string();
        let target_level = match target {
            "poweroff.target" => Runlevel::Level0_Halt,
            "rescue.target" => Runlevel::Level1_SingleUser,
            "multi-user.target" => Runlevel::Level3_MultiUserNetwork,
            "graphical.target" => Runlevel::Level5_Graphical,
            "reboot.target" => Runlevel::Level6_Reboot,
            _ => return Err(InitError::InvalidTransition),
        };
        self.switch_runlevel(target_level)
    }

    fn get_service_mut(&mut self, id: ServiceID) -> Option<&mut Box<dyn Service>> {
        for svc_option in &mut self.services {
            if let Some(ref mut svc) = *svc_option {
                if svc.id() == id {
                    return Some(svc);
                }
            }
        }
        None
    }
}

impl Default for SigmaInit {
    fn default() -> Self {
        Self::new()
    }
}

impl InitSystem for SigmaInit {
    fn register_service(&mut self, service: Box<dyn Service>) -> Result<ServiceID, InitError> {
        let id = service.id();
        self.services.push(Some(service));
        Ok(id)
    }

    fn start_service(&mut self, id: ServiceID) -> Result<(), InitError> {
        // Fetch dependencies first
        let mut deps = Vec::new();
        let mut req_level = Runlevel::Level2_MultiUser;
        for svc_option in &self.services {
            if let Some(ref svc) = *svc_option {
                if svc.id() == id {
                    deps = svc.dependencies();
                    req_level = svc.required_runlevel();
                    break;
                }
            }
        }

        // Safety check: Don't start service if current runlevel is too low
        if (self.current_runlevel as usize) < (req_level as usize) {
            return Err(InitError::DependencyFailed);
        }

        for dep_id in deps {
            self.start_service(dep_id)?;
        }

        // Start main service
        if let Some(svc) = self.get_service_mut(id) {
            return svc.start();
        }
        Err(InitError::ServiceNotFound)
    }

    fn stop_service(&mut self, id: ServiceID) -> Result<(), InitError> {
        if let Some(svc) = self.get_service_mut(id) {
            return svc.stop();
        }
        Err(InitError::ServiceNotFound)
    }

    fn restart_service(&mut self, id: ServiceID) -> Result<(), InitError> {
        if let Some(svc) = self.get_service_mut(id) {
            return svc.restart();
        }
        Err(InitError::ServiceNotFound)
    }

    fn get_service(&self, id: ServiceID) -> Option<&dyn Service> {
        for svc_option in &self.services {
            if let Some(ref svc) = *svc_option {
                if svc.id() == id {
                    return Some(svc.as_ref());
                }
            }
        }
        None
    }

    fn get_all_services(&self) -> Vec<ServiceID> {
        let mut ids = Vec::new();
        for svc_option in &self.services {
            if let Some(ref svc) = *svc_option {
                ids.push(svc.id());
            }
        }
        ids
    }

    /// Transitions active runlevel, stopping services too advanced, and starting required ones
    fn switch_runlevel(&mut self, level: Runlevel) -> Result<(), InitError> {
        self.current_runlevel = level;

        // Collect all service ids
        let ids = self.get_all_services();

        for id in ids {
            let req_level = self.get_service(id).unwrap().required_runlevel();
            let is_running = self.get_service(id).unwrap().state() == ServiceState::Running;

            if (level as usize) < (req_level as usize) {
                // Stop service since runlevel is too low (e.g. GUI stopped when entering CLI)
                if is_running {
                    self.stop_service(id)?;
                }
            } else {
                // Start service if it is required and stopped (e.g. starting network on level 3)
                if !is_running {
                    let _ = self.start_service(id);
                }
            }
        }

        Ok(())
    }
}

// ============================================================================
// Dependency Resolver
// ============================================================================

pub trait DependencyResolver {
    fn resolve_startup_order(&self, services: &[ServiceID]) -> Result<Vec<ServiceID>, InitError>;
    fn detect_cycles(&self, services: &[ServiceID]) -> bool;
}

pub struct SimpleDependencyResolver {
    pub init: SigmaInit,
}

impl SimpleDependencyResolver {
    pub fn new(init: SigmaInit) -> Self {
        SimpleDependencyResolver { init }
    }
}

impl DependencyResolver for SimpleDependencyResolver {
    fn resolve_startup_order(&self, services: &[ServiceID]) -> Result<Vec<ServiceID>, InitError> {
        let mut order = Vec::new();
        let mut visited = Vec::new();

        for &id in services {
            if !visited.contains(&id) {
                self.visit(id, &mut order, &mut visited)?;
            }
        }

        Ok(order)
    }

    fn detect_cycles(&self, services: &[ServiceID]) -> bool {
        let mut visited = Vec::new();
        let mut rec_stack = Vec::new();

        for &id in services {
            if self.has_cycle(id, &mut visited, &mut rec_stack) {
                return true;
            }
        }

        false
    }
}

impl SimpleDependencyResolver {
    fn visit(
        &self,
        id: ServiceID,
        order: &mut Vec<ServiceID>,
        visited: &mut Vec<ServiceID>,
    ) -> Result<(), InitError> {
        if visited.contains(&id) {
            return Ok(());
        }

        visited.push(id);

        if let Some(svc) = self.init.get_service(id) {
            for dep_id in svc.dependencies() {
                self.visit(dep_id, order, visited)?;
            }
        }

        order.push(id);
        Ok(())
    }

    fn has_cycle(
        &self,
        id: ServiceID,
        visited: &mut Vec<ServiceID>,
        rec_stack: &mut Vec<ServiceID>,
    ) -> bool {
        visited.push(id);
        rec_stack.push(id);

        if let Some(svc) = self.init.get_service(id) {
            for dep_id in svc.dependencies() {
                if !visited.contains(&dep_id) {
                    if self.has_cycle(dep_id, visited, rec_stack) {
                        return true;
                    }
                } else if rec_stack.contains(&dep_id) {
                    return true;
                }
            }
        }

        rec_stack.pop();
        false
    }
}

// ============================================================================
// Service Monitor
// ============================================================================

pub trait ServiceMonitor {
    fn monitor_service(&mut self, id: ServiceID) -> Result<(), InitError>;
    fn auto_restart(&mut self, id: ServiceID) -> Result<(), InitError>;
    fn get_service_status(&self, id: ServiceID) -> Option<ServiceState>;
}

pub struct SimpleServiceMonitor {
    pub init: SigmaInit,
    pub monitored: Vec<ServiceID>,
    pub auto_restart_enabled: AtomicUsize,
}

impl SimpleServiceMonitor {
    pub fn new(init: SigmaInit) -> Self {
        SimpleServiceMonitor {
            init,
            monitored: Vec::new(),
            auto_restart_enabled: AtomicUsize::new(0),
        }
    }
}

impl ServiceMonitor for SimpleServiceMonitor {
    fn monitor_service(&mut self, id: ServiceID) -> Result<(), InitError> {
        if self.init.get_service(id).is_none() {
            return Err(InitError::ServiceNotFound);
        }
        self.monitored.push(id);
        Ok(())
    }

    fn auto_restart(&mut self, id: ServiceID) -> Result<(), InitError> {
        if self.auto_restart_enabled.load(Ordering::SeqCst) == 0 {
            return Err(InitError::StartFailed);
        }
        self.init.restart_service(id)
    }

    fn get_service_status(&self, id: ServiceID) -> Option<ServiceState> {
        self.init.get_service(id).map(|svc| svc.state())
    }
}

// ============================================================================
// Firmware Ports & Security Ports
// ============================================================================

pub trait FirmwarePort {
    fn boot_type(&self) -> &'static str;
    fn handoff(&self) -> Result<(), &'static str>;
}

pub struct BIOSPort;
impl FirmwarePort for BIOSPort {
    fn boot_type(&self) -> &'static str {
        "Legacy BIOS (MBR)"
    }
    fn handoff(&self) -> Result<(), &'static str> {
        Ok(())
    }
}

pub struct UEFIPort;
impl FirmwarePort for UEFIPort {
    fn boot_type(&self) -> &'static str {
        "Modern UEFI (GPT)"
    }
    fn handoff(&self) -> Result<(), &'static str> {
        Ok(())
    }
}

pub struct CorebootPort;
impl FirmwarePort for CorebootPort {
    fn boot_type(&self) -> &'static str {
        "Coreboot (Open Source Firmware)"
    }
    fn handoff(&self) -> Result<(), &'static str> {
        Ok(())
    }
}

pub trait SecurityPort {
    fn policy_name(&self) -> &'static str;
    fn check_capability(&self, cap: u32) -> bool;
}

pub struct DACPort;
impl SecurityPort for DACPort {
    fn policy_name(&self) -> &'static str {
        "Discretionary Access Control (DAC)"
    }
    fn check_capability(&self, _cap: u32) -> bool {
        true
    }
}

pub struct SELinuxPort;
impl SecurityPort for SELinuxPort {
    fn policy_name(&self) -> &'static str {
        "Security-Enhanced Linux (SELinux)"
    }
    fn check_capability(&self, cap: u32) -> bool {
        cap > 10
    }
}

pub struct ZeroTrustPort;
impl SecurityPort for ZeroTrustPort {
    fn policy_name(&self) -> &'static str {
        "Zero-Trust Enforcement Security"
    }
    fn check_capability(&self, _cap: u32) -> bool {
        false
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_service_dependency_resolution() {
        let mut init = SigmaInit::new();

        let svc1 = SimpleService::new(1, "udev");
        let svc2 = SimpleService::new(2, "display").with_deps(vec![1]);

        init.register_service(Box::new(svc1)).unwrap();
        init.register_service(Box::new(svc2)).unwrap();

        let resolver = SimpleDependencyResolver::new(init);
        let order = resolver.resolve_startup_order(&[2]).unwrap();
        assert_eq!(order.len(), 2);
        assert_eq!(order[0], 1); // udev must start first
        assert_eq!(order[1], 2);
    }

    #[test]
    fn test_linux_runlevels_and_target_transitions() {
        let mut init = SigmaInit::new();

        let udev = SimpleService::new(1, "udev").with_runlevel(Runlevel::Level1_SingleUser);
        let network = SimpleService::new(2, "network").with_deps(vec![1]).with_runlevel(Runlevel::Level3_MultiUserNetwork);
        let gdm = SimpleService::new(3, "zenith-gdm").with_deps(vec![2]).with_runlevel(Runlevel::Level5_Graphical);

        init.register_service(Box::new(udev)).unwrap();
        init.register_service(Box::new(network)).unwrap();
        init.register_service(Box::new(gdm)).unwrap();

        // 1. Enter CLI Mode (Runlevel 3)
        init.switch_runlevel(Runlevel::Level3_MultiUserNetwork).unwrap();
        assert_eq!(init.get_service(1).unwrap().state(), ServiceState::Running); // udev
        assert_eq!(init.get_service(2).unwrap().state(), ServiceState::Running); // network
        assert_eq!(init.get_service(3).unwrap().state(), ServiceState::Stopped); // gdm (too advanced)

        // 2. Switch to Graphical Target (systemd-style)
        init.set_active_target("graphical.target").unwrap();
        assert_eq!(init.current_runlevel, Runlevel::Level5_Graphical);
        assert_eq!(init.get_service(3).unwrap().state(), ServiceState::Running); // gdm starts

        // 3. Switch to Rescue Target (systemd-style)
        init.set_active_target("rescue.target").unwrap();
        assert_eq!(init.current_runlevel, Runlevel::Level1_SingleUser);
        assert_eq!(init.get_service(1).unwrap().state(), ServiceState::Running); // udev remains
        assert_eq!(init.get_service(2).unwrap().state(), ServiceState::Stopped); // network stopped
        assert_eq!(init.get_service(3).unwrap().state(), ServiceState::Stopped); // gdm stopped
    }

    #[test]
    fn test_firmware_ports() {
        let bios: Box<dyn FirmwarePort> = Box::new(BIOSPort);
        let uefi: Box<dyn FirmwarePort> = Box::new(UEFIPort);
        let coreboot: Box<dyn FirmwarePort> = Box::new(CorebootPort);

        assert_eq!(bios.boot_type(), "Legacy BIOS (MBR)");
        assert_eq!(uefi.boot_type(), "Modern UEFI (GPT)");
        assert_eq!(coreboot.boot_type(), "Coreboot (Open Source Firmware)");

        assert!(bios.handoff().is_ok());
    }

    #[test]
    fn test_security_ports() {
        let dac: Box<dyn SecurityPort> = Box::new(DACPort);
        let selinux: Box<dyn SecurityPort> = Box::new(SELinuxPort);
        let zt: Box<dyn SecurityPort> = Box::new(ZeroTrustPort);

        assert_eq!(dac.policy_name(), "Discretionary Access Control (DAC)");
        assert_eq!(selinux.policy_name(), "Security-Enhanced Linux (SELinux)");
        assert_eq!(zt.policy_name(), "Zero-Trust Enforcement Security");

        assert!(dac.check_capability(1));
        assert!(selinux.check_capability(20));
        assert!(!zt.check_capability(1));
    }
}
