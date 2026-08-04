// OOP-based Lightweight Init System with Runlevels & Targets for SigmaOS
// Designed to surpass legacy SysVInit runlevels and modern Systemd target schemes.

#![no_std]
||||||| 43be3a7e8
#![no_main]

/// OOP-based Lightweight Init System for SigmaOS
/// Based on Ideas-999-Structured: Core System Item 5
/// Implements minimal init system with service management, dependency resolution, parallel startup
#![allow(warnings)]
#![allow(clippy::all)]
||||||| 0ddf2eac7
#![allow(warnings)]
#![allow(clippy::all)]

extern crate alloc;
use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;
||||||| 52d783ca0
use alloc::vec::Vec;
use crate::klib::Vec;

extern crate alloc;
use alloc::boxed::Box;
use alloc::string::String;
use alloc::string::ToString;
||||||| 0ddf2eac7

extern crate alloc;
use alloc::boxed::Box;
use alloc::vec;
use alloc::vec::Vec;
use alloc::vec;
use alloc::string::ToString;
||||||| 165ded71c
use alloc::vec;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicUsize, Ordering};

pub type ServiceID = usize;

/// Standard operating runlevels inspired by SysVInit and Linux distributions
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Runlevel {
    Level0_Halt = 0,
    Level1_SingleUser = 1,       // Maintenance / Recovery Mode
    Level2_MultiUser = 2,        // Multi-User Command Line (No Networking)
    Level3_MultiUserNetwork = 3, // Full Multi-User with Networking (CLI)
    Level5_Graphical = 5,        // Zenith Desktop GUI Mode
    Level6_Reboot = 6,
}
||||||| 43be3a7e8
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum ServiceState { Stopped = 0, Starting = 1, Running = 2, Stopping = 3, Failed = 4 }
||||||| 0ddf2eac7
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
||||||| 43be3a7e8
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum InitError { Success = 0, ServiceNotFound = 1, DependencyFailed = 2, StartFailed = 3, StopFailed = 4 }
||||||| 0ddf2eac7
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
||||||| 0ddf2eac7
/// OOP Service Trait
pub trait Service {
    fn id(&self) -> ServiceID;
    fn name(&self) -> &str;
    fn state(&self) -> ServiceState;
    fn dependencies(&self) -> Vec<ServiceID>;
    fn start(&mut self) -> Result<(), InitError>;
    fn stop(&mut self) -> Result<(), InitError>;
    fn restart(&mut self) -> Result<(), InitError>;

    /// Returns the minimal runlevel at which this service should run
    fn required_runlevel(&self) -> Runlevel {
        Runlevel::Level2_MultiUser
    }
||||||| 52d783ca0

    // Timing metrics
    fn start_time_ms(&self) -> u64 {
        0
    }
    fn end_time_ms(&self) -> u64 {
        0
    }
}

/// Standard service implementation
||||||| 43be3a7e8
#[repr(C)]
||||||| 0ddf2eac7
/// Standard service implementation
pub struct SimpleService {
    pub id: ServiceID,
    pub name: String,
    pub state: AtomicUsize, // ServiceState as usize
    pub deps: Vec<ServiceID>,
    pub pid: AtomicUsize,
    pub min_runlevel: Runlevel,
||||||| 52d783ca0
    pub start_time_ms: u64,
    pub end_time_ms: u64,
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
||||||| 52d783ca0
            start_time_ms: 0,
            end_time_ms: 0,
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
||||||| 43be3a7e8
    fn id(&self) -> ServiceID { self.id }
    fn name(&self) -> &[u8] {
        let len = self.name.iter().position(|&b| b == 0).unwrap_or(64);
        &self.name[..len]
    fn id(&self) -> ServiceID {
        self.id
    }
    fn name(&self) -> &[u8] {
        let len = self.name.iter().position(|&b| b == 0).unwrap_or(64);
        &self.name[..len]
||||||| 0ddf2eac7
    fn name(&self) -> &[u8] {
        let len = self.name.iter().position(|&b| b == 0).unwrap_or(64);
        &self.name[..len]

    fn name(&self) -> &str {
        &self.name
    }
||||||| 43be3a7e8
    fn state(&self) -> ServiceState { unsafe { core::mem::transmute(self.state.load(Ordering::SeqCst)) } }
    fn dependencies(&self) -> Vec<ServiceID> { self.deps.clone() }
||||||| 0ddf2eac7

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
        self.state
            .store(ServiceState::Starting as usize, Ordering::SeqCst);
        self.start_time_ms = 50; // simulated start timestamp
        self.state
            .store(ServiceState::Running as usize, Ordering::SeqCst);
        self.end_time_ms = 180;  // simulated end timestamp (130ms duration)
        self.pid.store(self.id + 1000, Ordering::SeqCst);
        Ok(())
    }

    fn stop(&mut self) -> Result<(), InitError> {
        self.state
            .store(ServiceState::Stopping as usize, Ordering::SeqCst);
        self.state
            .store(ServiceState::Stopped as usize, Ordering::SeqCst);
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
||||||| 52d783ca0

    fn start_time_ms(&self) -> u64 {
        self.start_time_ms
    }

    fn end_time_ms(&self) -> u64 {
        self.end_time_ms
    }
}

/// Abstract Init System interface
pub trait InitSystem {
    fn register_service(&mut self, service: Box<dyn Service>) -> Result<ServiceID, InitError>;
    fn start_service(&mut self, id: ServiceID) -> Result<(), InitError>;
    fn stop_service(&mut self, id: ServiceID) -> Result<(), InitError>;
    fn restart_service(&mut self, id: ServiceID) -> Result<(), InitError>;
    fn get_service(&self, id: ServiceID) -> Option<&dyn Service>;
    fn get_all_services(&self) -> Vec<ServiceID>;

    /// SWITCH operating runlevel (mimicking init/telinit transitions)
    fn switch_runlevel(&mut self, level: Runlevel) -> Result<(), InitError>;
}

/// Concrete SigmaOS Init implementation (supporting parallel startup, targets, and runlevels)
||||||| 43be3a7e8
#[repr(C)]
||||||| 0ddf2eac7
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
        self.parallel_startup = true;
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

    pub fn restart_service(&mut self, id: ServiceID) -> Result<(), InitError> {
        for i in 0..self.services.len() {
            if let Some(ref mut svc) = self.services[i] {
                if svc.id() == id {
                    return svc.restart();
                }
            }
        }
        Err(InitError::ServiceNotFound)
||||||| 0ddf2eac7
        Err(InitError::ServiceNotFound)
    }
}

impl Default for SigmaInit {
    fn default() -> Self {
        Self::new()
    }

    pub fn restart_service(&mut self, id: ServiceID) -> Result<(), InitError> {
        for svc_option in &mut self.services {
            if let Some(ref mut svc) = *svc_option {
                if svc.id() == id {
                    return svc.restart();
                }
            }
        }
        Err(InitError::ServiceNotFound)
        None
    }

    /// Resolves concurrent staging launch groups where each group consists of services that can start in parallel
    pub fn resolve_parallel_groups(&self, service_ids: &[ServiceID]) -> Vec<Vec<ServiceID>> {
        let mut groups = Vec::new();
        let mut remaining = Vec::new();
        for &id in service_ids {
            remaining.push(id);
        }

        let mut satisfied = Vec::new();

        while !remaining.is_empty() {
            let mut current_stage = Vec::new();
            let mut i = 0;
            while i < remaining.len() {
                let id = remaining[i];
                let mut deps_satisfied = true;
                if let Some(svc) = self.get_service(id) {
                    let deps = svc.dependencies();
                    for j in 0..deps.len() {
                        let dep = deps[j];
                        if service_ids.contains(&dep) && !satisfied.contains(&dep) {
                            deps_satisfied = false;
                            break;
                        }
                    }
                } else {
                    deps_satisfied = false;
                }

                if deps_satisfied {
                    current_stage.push(id);
                    remaining.remove(i);
                } else {
                    i += 1;
                }
            }

            if current_stage.is_empty() {
                let mut fallback = Vec::new();
                while let Some(id) = remaining.pop() {
                    fallback.push(id);
                }
                groups.push(fallback);
                break;
            }

            for j in 0..current_stage.len() {
                satisfied.push(current_stage[j]);
            }
            groups.push(current_stage);
        }

        groups
    }

    /// systemd-analyze / OpenRC timing timeline blame metrics
    pub fn get_boot_timeline(&self) -> Vec<(ServiceID, u64)> {
        let mut timeline = Vec::new();
        for i in 0..self.services.len() {
            if let Some(ref svc) = self.services[i] {
                let duration = svc.end_time_ms().saturating_sub(svc.start_time_ms());
                if duration > 0 {
                    timeline.push((svc.id(), duration));
                }
            }
        }

        // Sort descending by duration
        for i in 0..timeline.len() {
            for j in (i + 1)..timeline.len() {
                if timeline[j].1 > timeline[i].1 {
                    let tmp = timeline[i];
                    timeline[i] = timeline[j];
                    timeline[j] = tmp;
                }
            }
        }
        timeline
    }
}

impl Default for SigmaInit {
    fn default() -> Self {
        Self::new()
    }

    pub fn restart_service(&mut self, id: ServiceID) -> Result<(), InitError> {
        self.stop_service(id)?;
        self.start_service(id)?;
        Ok(())
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
||||||| 52d783ca0
        // Fetch dependencies first to avoid double borrowing
        let mut deps = Vec::new();
        let mut req_level = Runlevel::Level2_MultiUser;
        for svc_option in &self.services {
            if let Some(ref svc) = *svc_option {
||||||| 43be3a7e8
        for svc_option in &mut self.services {
            if let Some(ref mut svc) = *svc_option {
        // Fetch dependencies first to avoid double borrowing
||||||| 0ddf2eac7
        // Fetch dependencies first to avoid double borrowing
        // Fetch dependencies first
        let mut deps = Vec::new();
        let mut req_level = Runlevel::Level2_MultiUser;
        for svc_option in &self.services {
            if let Some(ref svc) = *svc_option {
||||||| 52d783ca0
        for svc_option in &self.services {
            if let Some(ref svc) = *svc_option {
        for i in 0..self.services.len() {
            if let Some(ref svc) = self.services[i] {
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
||||||| 52d783ca0
        for dep_id in deps {
        for i in 0..deps.len() {
            let dep_id = deps[i];
            self.start_service(dep_id)?;
        }

        for i in 0..self.services.len() {
            if let Some(ref mut svc) = self.services[i] {
                if svc.id() == id {
                    deps = svc.dependencies();
                    req_level = svc.required_runlevel();
                    break;
||||||| 43be3a7e8
                    let deps = svc.dependencies();
                    for dep_id in deps {
                        self.start_service(dep_id)?;
                    }
                    return svc.start();
                    return svc.start();
                }
            }
||||||| 0ddf2eac7
        for svc_option in &mut self.services {
            if let Some(ref mut svc) = *svc_option {
                if svc.id() == id {
                    return svc.start();
                }
            }
        if let Some(svc) = self.get_service_mut(id) {
            return svc.start();
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
||||||| 52d783ca0
        for svc_option in &mut self.services {
            if let Some(ref mut svc) = *svc_option {
                if svc.id() == id {
                    return svc.stop();
                }
            }
        for i in 0..self.services.len() {
            if let Some(ref mut svc) = self.services[i] {
                if svc.id() == id {
                    return svc.stop();
                }
            }
        }
        Err(InitError::ServiceNotFound)
    }

    fn get_service(&self, id: ServiceID) -> Option<&dyn Service> {
        for i in 0..self.services.len() {
            if let Some(ref svc) = self.services[i] {
                if svc.id() == id {
                    return Some(svc.as_ref());
                }
            }
        }
        None
    }

    fn get_all_services(&self) -> Vec<ServiceID> {
        let mut ids = Vec::new();
        for i in 0..self.services.len() {
            if let Some(ref svc) = self.services[i] {
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

        for i in 0..services.len() {
            let id = services[i];
            if !visited.contains(&id) {
                self.visit(id, &mut order, &mut visited)?;
            }
        }

        Ok(order)
    }

    fn detect_cycles(&self, services: &[ServiceID]) -> bool {
        let mut visited = Vec::new();
        let mut rec_stack = Vec::new();

        for i in 0..services.len() {
            let id = services[i];
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
            let deps = svc.dependencies();
            for i in 0..deps.len() {
                let dep_id = deps[i];
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
            let deps = svc.dependencies();
            for i in 0..deps.len() {
                let dep_id = deps[i];
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
    pub auto_restart_enabled: bool,
}

impl SimpleServiceMonitor {
    pub fn new(init: SigmaInit) -> Self {
        SimpleServiceMonitor {
            init,
            monitored: Vec::new(),
            auto_restart_enabled: true,
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
        if !self.auto_restart_enabled {
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
||||||| 43be3a7e8
||||||| 984d1301f
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContainerDaemonType {
    SystemDaemon, // PID 1 System Docker equivalent managing core OS containers
    UserDaemon,   // User Docker equivalent managing user workloads
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContainerState {
    Created,
    Running,
    Exited,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SovereignSystemContainer {
    pub container_id: u32,
    pub name: [u8; 32],
    pub image_name: [u8; 32],
    pub state: ContainerState,
}

/// RancherOS-style Dual Container Daemon Init System
pub struct RancherContainerInit {
    pub system_daemon_active: bool,
    pub user_daemon_active: bool,
    pub system_containers: Vec<SovereignSystemContainer>,
    pub user_containers: Vec<SovereignSystemContainer>,
}

impl RancherContainerInit {
    pub fn new() -> Self {
        Self {
            system_daemon_active: false,
            user_daemon_active: false,
            system_containers: Vec::new(),
            user_containers: Vec::new(),
        }
    }

    /// Initializes PID 1 System Daemon managing system containers (syslog, udev, etc.)
    pub fn start_system_daemon(&mut self) {
        self.system_daemon_active = true;
        // Seed default RancherOS system-level containers
        let mut sys_log = SovereignSystemContainer {
            container_id: 1,
            name: [0; 32],
            image_name: [0; 32],
            state: ContainerState::Running,
        };
        sys_log.name[..6].copy_from_slice(b"syslog");
        sys_log.image_name[..13].copy_from_slice(b"system-syslog");

        let mut sys_udev = SovereignSystemContainer {
            container_id: 2,
            name: [0; 32],
            image_name: [0; 32],
            state: ContainerState::Running,
        };
        sys_udev.name[..4].copy_from_slice(b"udev");
        sys_udev.image_name[..11].copy_from_slice(b"system-udev");

        self.system_containers.push(sys_log);
        self.system_containers.push(sys_udev);
    }

    /// System Docker starts the secondary User Docker daemon to host user applications
    pub fn start_user_daemon(&mut self) -> Result<(), &'static str> {
        if !self.system_daemon_active {
            return Err("Cannot start User Daemon: System Daemon (PID 1) must be active first");
        }
        self.user_daemon_active = true;
        Ok(())
    }

    /// Spawn a new container managed by either the System or User daemon
    pub fn launch_container(
        &mut self,
        name: &str,
        image: &str,
        daemon: ContainerDaemonType,
    ) -> Result<u32, &'static str> {
        let mut name_arr = [0u8; 32];
        let mut img_arr = [0u8; 32];

        let n_len = name.len().min(31);
        let i_len = image.len().min(31);
        name_arr[..n_len].copy_from_slice(&name.as_bytes()[..n_len]);
        img_arr[..i_len].copy_from_slice(&image.as_bytes()[..i_len]);

        match daemon {
            ContainerDaemonType::SystemDaemon => {
                if !self.system_daemon_active {
                    return Err("System Daemon inactive");
                }
                let id = (self.system_containers.len() + 1) as u32;
                self.system_containers.push(SovereignSystemContainer {
                    container_id: id,
                    name: name_arr,
                    image_name: img_arr,
                    state: ContainerState::Running,
                });
                Ok(id)
            }
            ContainerDaemonType::UserDaemon => {
                if !self.user_daemon_active {
                    return Err("User Daemon inactive");
                }
                let id = (self.user_containers.len() + 1) as u32;
                self.user_containers.push(SovereignSystemContainer {
                    container_id: id,
                    name: name_arr,
                    image_name: img_arr,
                    state: ContainerState::Running,
                });
                Ok(id)
            }
        }
    }
}

impl Default for RancherContainerInit {
    fn default() -> Self {
        Self::new()
    }
}

struct Vec<T> { data: *mut T, len: usize, capacity: usize }
/// Advanced OOP-driven Firmware Port Class Hierarchy
||||||| 0ddf2eac7
/// Advanced OOP-driven Firmware Port Class Hierarchy
// ============================================================================
// Firmware Ports & Security Ports
// ============================================================================

pub trait FirmwarePort {
    fn boot_type(&self) -> &'static str;
    fn handoff(&self) -> Result<(), &'static str>;
}

pub trait FirmwarePort {
    fn boot_type(&self) -> &'static str;
    fn handoff(&self) -> Result<(), &'static str>;
}

pub struct BIOSPort;
impl FirmwarePort for BIOSPort {
    fn boot_type(&self) -> &'static str {
        "Legacy BIOS (MBR)"
||||||| 43be3a7e8
impl<T> Vec<T> {
    fn new() -> Self { Vec { data: core::ptr::null_mut(), len: 0, capacity: 0 } }
    fn push(&mut self, item: T) {
        unsafe {
            if self.len >= self.capacity { self.grow(); }
            if self.capacity > self.len {
                core::ptr::write(self.data.add(self.len), item);
                self.len += 1;
            }
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
        let network = SimpleService::new(2, "network")
            .with_deps(vec![1])
            .with_runlevel(Runlevel::Level3_MultiUserNetwork);
        let gdm = SimpleService::new(3, "zenith-gdm")
            .with_deps(vec![2])
            .with_runlevel(Runlevel::Level5_Graphical);

        init.register_service(Box::new(udev)).unwrap();
        init.register_service(Box::new(network)).unwrap();
        init.register_service(Box::new(gdm)).unwrap();

        // 1. Enter CLI Mode (Runlevel 3)
        init.switch_runlevel(Runlevel::Level3_MultiUserNetwork)
            .unwrap();
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
        assert_eq!(init.get_service(3).unwrap().state(), ServiceState::Stopped);
        // gdm stopped
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
||||||| 43be3a7e8
extern "C" { fn alloc(size: usize) -> *mut u8; fn free(ptr: *mut u8); }
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

/// Advanced OOP-driven Security Port Class Hierarchy
||||||| 0ddf2eac7
/// Advanced OOP-driven Security Port Class Hierarchy
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

    #[test]
    fn test_sigma_init_parallel_groups_and_timeline() {
        let mut init = SigmaInit::new();

        let mut s1 = SimpleService::new(10, b"s1");
        let mut s2 = SimpleService::new(20, b"s2");
        s2.deps.push(10); // s2 depends on s1

        let mut s3 = SimpleService::new(30, b"s3"); // s3 independent

        init.register_service(Box::new(s1)).unwrap();
        init.register_service(Box::new(s2)).unwrap();
        init.register_service(Box::new(s3)).unwrap();

        // Staging check
        let list = [10, 20, 30];
        let groups = init.resolve_parallel_groups(&list);
        assert_eq!(groups.len(), 2);
        // Stage 0: 10 (s1) and 30 (s3) start first
        assert!(groups[0].contains(&10));
        assert!(groups[0].contains(&30));
        // Stage 1: 20 (s2) starts next
        assert_eq!(groups[1].len(), 1);
        assert_eq!(groups[1][0], 20);

        // Start services to populate timeline
        init.start_service(20).unwrap();
        init.start_service(30).unwrap();

        let timeline = init.get_boot_timeline();
        assert_eq!(timeline.len(), 3);
        assert_eq!(timeline[0].1, 130); // 180 - 50 = 130ms duration
    }
}
||||||| 984d1301f

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rancher_container_init() {
        let mut r_init = RancherContainerInit::new();
        assert!(!r_init.system_daemon_active);
        assert!(!r_init.user_daemon_active);

        // Try launching a container before starting System daemon -> should fail
        assert!(r_init.launch_container("test", "img", ContainerDaemonType::SystemDaemon).is_err());

        // Start system daemon (PID 1)
        r_init.start_system_daemon();
        assert!(r_init.system_daemon_active);
        assert_eq!(r_init.system_containers.len(), 2); // syslog and udev seeded

        // Launch system-level container (e.g. ntp daemon)
        let ntp_id = r_init.launch_container("ntpd", "system-ntpd", ContainerDaemonType::SystemDaemon).unwrap();
        assert_eq!(ntp_id, 3);
        assert_eq!(r_init.system_containers.len(), 3);

        // Try starting user daemon before starting system daemon -> should succeed now
        assert!(r_init.start_user_daemon().is_ok());
        assert!(r_init.user_daemon_active);

        // Launch user-level workload container
        let web_id = r_init.launch_container("nginx", "user-nginx", ContainerDaemonType::UserDaemon).unwrap();
        assert_eq!(web_id, 1);
        assert_eq!(r_init.user_containers.len(), 1);
    }
}
