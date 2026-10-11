use std::boxed::Box;
use std::vec::Vec;

use core::sync::atomic::{AtomicUsize, Ordering};

/// Service ID
pub type ServiceID = usize;

/// Service priority
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ServicePriority {
    Critical = 0,
    High = 1,
    Medium = 2,
    Low = 3,
}

/// Boot service trait (OOP interface)
pub trait BootService {
    /// Get service ID
    fn id(&self) -> ServiceID;
    /// Get service name
    fn name(&self) -> &[u8];
    /// Get service priority
    fn priority(&self) -> ServicePriority;
    /// Get estimated startup time (ms)
    fn startup_time(&self) -> u32;
    /// Initialize service
    fn initialize(&mut self) -> Result<(), BootError>;
    /// Get service info
    fn info(&self) -> BootServiceInfo;
}

/// Boot error types
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum BootError {
    Success = 0,
    InitializationFailed = 1,
    DependencyFailed = 2,
    Timeout = 3,
}

/// Boot service info
#[repr(C)]
pub struct BootServiceInfo {
    pub id: ServiceID,
    pub name: [u8; 64],
    pub priority: ServicePriority,
    pub startup_time: u32,
    pub status: ServiceStatus,
    pub capability: ServiceCapability,
}

impl BootServiceInfo {
    pub fn new(id: ServiceID) -> Self {
        BootServiceInfo {
            id,
            name: [0; 64],
            priority: ServicePriority::Medium,
            startup_time: 0,
            status: ServiceStatus::Pending,
            capability: ServiceCapability::new(),
        }
    }
}

/// Service status
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum ServiceStatus {
    Pending = 0,
    Initializing = 1,
    Ready = 2,
    Failed = 3,
}

/// Service capability
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ServiceCapability {
    pub can_initialize: bool,
    pub can_parallelize: bool,
}

impl ServiceCapability {
    pub fn new() -> Self {
        ServiceCapability {
            can_initialize: false,
            can_parallelize: false,
        }
    }

    pub fn full() -> Self {
        ServiceCapability {
            can_initialize: true,
            can_parallelize: true,
        }
    }
}

/// Simple boot service (OOP: Concrete service class)
#[repr(C)]
pub struct SimpleBootService {
    pub id: ServiceID,
    pub name: [u8; 64],
    /// Precomputed byte length of service name to enable O(1) slice lookup
    pub name_len: u8,
    pub priority: ServicePriority,
    pub startup_time: u32,
    pub status: AtomicUsize, // ServiceStatus as usize
    pub capability: ServiceCapability,
    pub dependencies: Vec<ServiceID>,
}

impl SimpleBootService {
    pub fn new(
        id: ServiceID,
        name: &[u8],
        priority: ServicePriority,
        startup_time: u32,
        capability: ServiceCapability,
    ) -> Self {
        let mut name_array = [0u8; 64];
        let len = name.len().min(63);

        unsafe {
            core::ptr::copy_nonoverlapping(name.as_ptr(), name_array.as_mut_ptr(), len);
        }

        SimpleBootService {
            id,
            name: name_array,
            name_len: len as u8,
            priority,
            startup_time,
            status: AtomicUsize::new(ServiceStatus::Pending as usize),
            capability,
            dependencies: Vec::new(),
        }
    }

    pub fn add_dependency(&mut self, dependency: ServiceID) {
        self.dependencies.push(dependency);
    }

    pub fn get_status(&self) -> ServiceStatus {
        let raw = self.status.load(Ordering::SeqCst) as u32;
        match raw {
            1 => ServiceStatus::Initializing,
            2 => ServiceStatus::Ready,
            3 => ServiceStatus::Failed,
            _ => ServiceStatus::Pending,
        }
    }

    pub fn set_status(&self, status: ServiceStatus) {
        self.status.store(status as usize, Ordering::SeqCst);
    }
}

impl BootService for SimpleBootService {
    fn id(&self) -> ServiceID {
        self.id
    }

    fn name(&self) -> &[u8] {
        // O(1) slice lookup using cached name_len, avoiding O(N) zero-byte linear scan (.position(|&b| b == 0))
        let len = (self.name_len as usize).min(63);
        &self.name[..len]
    }

    fn priority(&self) -> ServicePriority {
        self.priority
    }

    fn startup_time(&self) -> u32 {
        self.startup_time
    }

    fn initialize(&mut self) -> Result<(), BootError> {
        if !self.capability.can_initialize {
            return Err(BootError::InitializationFailed);
        }

        self.set_status(ServiceStatus::Initializing);

        // In a real implementation, this would initialize the service
        // For now, simulate initialization
        self.set_status(ServiceStatus::Ready);
        Ok(())
    }

    fn info(&self) -> BootServiceInfo {
        BootServiceInfo {
            id: self.id,
            name: self.name,
            priority: self.priority,
            startup_time: self.startup_time,
            status: self.get_status(),
            capability: self.capability,
        }
    }
}

/// Boot optimizer trait (OOP interface)
pub trait BootOptimizer {
    /// Register service
    fn register_service(&mut self, service: Box<dyn BootService>) -> Result<ServiceID, BootError>;
    /// Unregister service
    fn unregister_service(&mut self, id: ServiceID) -> Result<(), BootError>;
    /// Initialize service
    fn initialize_service(&mut self, id: ServiceID) -> Result<(), BootError>;
    /// Optimize boot order
    fn optimize_boot_order(&mut self) -> Result<Vec<ServiceID>, BootError>;
    /// Initialize all services
    fn initialize_all(&mut self) -> Result<(), BootError>;
    /// Get service
    fn get_service(&self, id: ServiceID) -> Option<&dyn BootService>;
    /// Get optimizer statistics
    fn stats(&self) -> BootStats;
}

/// Boot statistics
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BootStats {
    pub total_services: usize,
    pub ready_services: usize,
    pub failed_services: usize,
    pub total_boot_time: u32,
}

impl BootStats {
    pub fn new() -> Self {
        BootStats {
            total_services: 0,
            ready_services: 0,
            failed_services: 0,
            total_boot_time: 0,
        }
    }
}

/// Simple boot optimizer (OOP: Concrete optimizer class)
pub struct SimpleBootOptimizer {
    services: Vec<Option<Box<dyn BootService>>>,
    next_id: AtomicUsize,
    stats: BootStats,
    capability: OptimizerCapability,
}

/// Optimizer capability
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct OptimizerCapability {
    pub can_register: bool,
    pub can_initialize: bool,
    pub can_optimize: bool,
}

impl OptimizerCapability {
    pub fn new() -> Self {
        OptimizerCapability {
            can_register: false,
            can_initialize: false,
            can_optimize: false,
        }
    }

    pub fn full() -> Self {
        OptimizerCapability {
            can_register: true,
            can_initialize: true,
            can_optimize: true,
        }
    }
}

impl SimpleBootOptimizer {
    pub fn new(capability: OptimizerCapability) -> Self {
        SimpleBootOptimizer {
            services: Vec::new(),
            next_id: AtomicUsize::new(1),
            stats: BootStats::new(),
            capability,
        }
    }
}

impl BootOptimizer for SimpleBootOptimizer {
    fn register_service(&mut self, service: Box<dyn BootService>) -> Result<ServiceID, BootError> {
        if !self.capability.can_register {
            return Err(BootError::InitializationFailed);
        }

        let id = service.id();
        self.services.push(Some(service));
        self.stats.total_services += 1;
        Ok(id)
    }

    fn unregister_service(&mut self, id: ServiceID) -> Result<(), BootError> {
        if !self.capability.can_register {
            return Err(BootError::InitializationFailed);
        }

        let mut index = None;
        for (i, service_option) in self.services.iter().enumerate() {
            if let Some(ref service) = *service_option {
                if service.id() == id {
                    index = Some(i);
                    break;
                }
            }
        }

        if let Some(i) = index {
            self.services[i] = None;
            self.stats.total_services -= 1;
            Ok(())
        } else {
            Err(BootError::InitializationFailed)
        }
    }

    fn initialize_service(&mut self, id: ServiceID) -> Result<(), BootError> {
        if !self.capability.can_initialize {
            return Err(BootError::InitializationFailed);
        }

        for service_option in &mut self.services {
            if let Some(ref mut service) = *service_option {
                if service.id() == id {
                    let result = service.initialize();
                    if result.is_ok() {
                        self.stats.ready_services += 1;
                        self.stats.total_boot_time += service.startup_time();
                    } else {
                        self.stats.failed_services += 1;
                    }
                    return result;
                }
            }
        }
        Err(BootError::InitializationFailed)
    }

    fn optimize_boot_order(&mut self) -> Result<Vec<ServiceID>, BootError> {
        if !self.capability.can_optimize {
            return Err(BootError::InitializationFailed);
        }

        let mut ordered_ids = Vec::new();

        // Collect all service IDs with their priorities
        let mut services_with_priority: Vec<(ServiceID, ServicePriority)> = Vec::new();

        for service_option in &self.services {
            if let Some(ref service) = *service_option {
                services_with_priority.push((service.id(), service.priority()));
            }
        }

        // O(N log N) sort by priority (lower priority enum variant = higher boot importance)
        services_with_priority.sort_by_key(|&(_, priority)| priority);

        for (id, _) in &services_with_priority {
            ordered_ids.push(*id);
        }

        Ok(ordered_ids)
    }

    fn initialize_all(&mut self) -> Result<(), BootError> {
        if !self.capability.can_initialize {
            return Err(BootError::InitializationFailed);
        }

        let optimized_order = self.optimize_boot_order()?;

        for id in &optimized_order {
            let _ = self.initialize_service(*id);
        }

        Ok(())
    }

    fn get_service(&self, id: ServiceID) -> Option<&dyn BootService> {
        for service_option in &self.services {
            if let Some(ref service) = *service_option {
                if service.id() == id {
                    return Some(service.as_ref());
                }
            }
        }
        None
    }

    fn stats(&self) -> BootStats {
        self.stats
    }
}

#[cfg(any(test, feature = "standalone_test"))]
mod tests {
    use super::*;

    #[test]
    fn test_simple_boot_service_name_length_caching() {
        let cap = ServiceCapability::full();
        let service =
            SimpleBootService::new(101, b"network-manager", ServicePriority::High, 150, cap);
        assert_eq!(service.name_len, 15);
        assert_eq!(service.name(), b"network-manager");
    }

    #[test]
    fn test_simple_boot_service_dependencies_and_status() {
        let cap = ServiceCapability::full();
        let mut service =
            SimpleBootService::new(102, b"display-daemon", ServicePriority::Medium, 200, cap);
        assert_eq!(service.get_status() as u32, ServiceStatus::Pending as u32);

        service.add_dependency(101);
        assert_eq!(service.dependencies.len(), 1);
        assert_eq!(service.dependencies[0], 101);

        assert!(service.initialize().is_ok());
        assert_eq!(service.get_status() as u32, ServiceStatus::Ready as u32);
    }

    #[test]
    fn test_boot_optimizer_ordering_and_initialization() {
        let opt_cap = OptimizerCapability::full();
        let mut optimizer = SimpleBootOptimizer::new(opt_cap);

        let service_low = Box::new(SimpleBootService::new(
            1,
            b"cron-daemon",
            ServicePriority::Low,
            50,
            ServiceCapability::full(),
        ));
        let service_crit = Box::new(SimpleBootService::new(
            2,
            b"vfs-root",
            ServicePriority::Critical,
            10,
            ServiceCapability::full(),
        ));
        let service_high = Box::new(SimpleBootService::new(
            3,
            b"udev-device-mgr",
            ServicePriority::High,
            30,
            ServiceCapability::full(),
        ));

        assert!(optimizer.register_service(service_low).is_ok());
        assert!(optimizer.register_service(service_crit).is_ok());
        assert!(optimizer.register_service(service_high).is_ok());

        let order = optimizer.optimize_boot_order().unwrap();
        assert_eq!(order, vec![2, 3, 1]);

        assert!(optimizer.initialize_all().is_ok());
        let stats = optimizer.stats();
        assert_eq!(stats.total_services, 3);
        assert_eq!(stats.ready_services, 3);
        assert_eq!(stats.failed_services, 0);
        assert_eq!(stats.total_boot_time, 90);
    }
}
