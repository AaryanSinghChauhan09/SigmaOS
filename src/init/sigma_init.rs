#![no_std]
#![allow(warnings)]
#![allow(clippy::all)]

/// OOP-based Lightweight Init System for SigmaOS
/// Based on Ideas-999-Structured: Core System Item 5
/// Implements minimal init system with service management, dependency resolution, parallel startup,
/// and modular FirmwarePort / SecurityPort structures
extern crate alloc;
use alloc::boxed::Box;
use crate::klib::Vec;

use core::sync::atomic::{AtomicUsize, Ordering};

pub type ServiceID = usize;

#[repr(usize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServiceState {
    Stopped = 0,
    Starting = 1,
    Running = 2,
    Stopping = 3,
    Failed = 4,
}

#[repr(usize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InitError {
    Success = 0,
    ServiceNotFound = 1,
    DependencyFailed = 2,
    StartFailed = 3,
    StopFailed = 4,
}

pub trait Service {
    fn id(&self) -> ServiceID;
    fn name(&self) -> &[u8];
    fn state(&self) -> ServiceState;
    fn dependencies(&self) -> Vec<ServiceID>;
    fn start(&mut self) -> Result<(), InitError>;
    fn stop(&mut self) -> Result<(), InitError>;
    fn restart(&mut self) -> Result<(), InitError>;

    // Timing metrics
    fn start_time_ms(&self) -> u64 {
        0
    }
    fn end_time_ms(&self) -> u64 {
        0
    }
}

pub struct SimpleService {
    pub id: ServiceID,
    pub name: [u8; 64],
    pub state: AtomicUsize,
    pub deps: Vec<ServiceID>,
    pub pid: AtomicUsize,
    pub start_time_ms: u64,
    pub end_time_ms: u64,
}

impl SimpleService {
    pub fn new(id: ServiceID, name: &[u8]) -> Self {
        let mut name_array = [0u8; 64];
        let name_len = name.len().min(63);
        unsafe {
            core::ptr::copy_nonoverlapping(name.as_ptr(), name_array.as_mut_ptr(), name_len);
        }
        SimpleService {
            id,
            name: name_array,
            state: AtomicUsize::new(ServiceState::Stopped as usize),
            deps: Vec::new(),
            pid: AtomicUsize::new(0),
            start_time_ms: 0,
            end_time_ms: 0,
        }
    }
}

impl Service for SimpleService {
    fn id(&self) -> ServiceID {
        self.id
    }
    fn name(&self) -> &[u8] {
        let len = self.name.iter().position(|&b| b == 0).unwrap_or(64);
        &self.name[..len]
    }
    fn state(&self) -> ServiceState {
        unsafe { core::mem::transmute(self.state.load(Ordering::SeqCst)) }
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

    fn start_time_ms(&self) -> u64 {
        self.start_time_ms
    }

    fn end_time_ms(&self) -> u64 {
        self.end_time_ms
    }
}

pub trait InitSystem {
    fn register_service(&mut self, service: Box<dyn Service>) -> Result<ServiceID, InitError>;
    fn start_service(&mut self, id: ServiceID) -> Result<(), InitError>;
    fn stop_service(&mut self, id: ServiceID) -> Result<(), InitError>;
    fn get_service(&self, id: ServiceID) -> Option<&dyn Service>;
    fn get_all_services(&self) -> Vec<ServiceID>;
}

pub struct SigmaInit {
    pub services: Vec<Option<Box<dyn Service>>>,
    pub next_id: AtomicUsize,
    pub parallel_startup: AtomicUsize,
}

impl SigmaInit {
    pub fn new() -> Self {
        SigmaInit {
            services: Vec::new(),
            next_id: AtomicUsize::new(1),
            parallel_startup: AtomicUsize::new(1),
        }
    }

    pub fn enable_parallel_startup(&mut self) {
        self.parallel_startup.store(1, Ordering::SeqCst);
    }

    pub fn disable_parallel_startup(&mut self) {
        self.parallel_startup.store(0, Ordering::SeqCst);
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
}

impl InitSystem for SigmaInit {
    fn register_service(&mut self, service: Box<dyn Service>) -> Result<ServiceID, InitError> {
        let id = service.id();
        self.services.push(Some(service));
        Ok(id)
    }

    fn start_service(&mut self, id: ServiceID) -> Result<(), InitError> {
        let mut deps = Vec::new();
        for i in 0..self.services.len() {
            if let Some(ref svc) = self.services[i] {
                if svc.id() == id {
                    deps = svc.dependencies();
                    break;
                }
            }
        }

        for i in 0..deps.len() {
            let dep_id = deps[i];
            self.start_service(dep_id)?;
        }

        for i in 0..self.services.len() {
            if let Some(ref mut svc) = self.services[i] {
                if svc.id() == id {
                    return svc.start();
                }
            }
        }
        Err(InitError::ServiceNotFound)
    }

    fn stop_service(&mut self, id: ServiceID) -> Result<(), InitError> {
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
}

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

/// Advanced OOP-driven Firmware Port Class Hierarchy
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
    } // Absolute strict verification
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_service_dependency_resolution() {
        let mut init = SigmaInit::new();

        let mut svc1 = SimpleService::new(1, b"udev");
        let mut svc2 = SimpleService::new(2, b"display");
        svc2.deps.push(1);

        init.register_service(Box::new(svc1)).unwrap();
        init.register_service(Box::new(svc2)).unwrap();

        let resolver = SimpleDependencyResolver::new(init);
        let order = resolver.resolve_startup_order(&[2]).unwrap();
        assert_eq!(order.len(), 2);
        assert_eq!(order[0], 1); // udev must start first
        assert_eq!(order[1], 2);
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
