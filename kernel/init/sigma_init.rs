//! SigmaOS Init System
//! Service manager inspired by systemd/OpenRC
//! Handles service lifecycle, dependencies, and monitoring

#![no_std]
#![allow(dead_code)]

extern crate alloc;
use alloc::vec::Vec;

type SigmaU32 = u32;
type SigmaI32 = i32;
type SigmaBool = bool;

/// Service state
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ServiceState {
    Stopped,
    Starting,
    Running,
    Stopping,
    Failed,
}

/// Service definition
#[repr(C)]
pub struct Service {
    pub name: [u8; 64],
    pub description: [u8; 256],
    pub exec_start: [u8; 256],
    pub exec_stop: [u8; 256],
    pub dependencies: [SigmaU32; 16], // Service IDs this depends on
    pub dependency_count: SigmaU32,
    pub state: ServiceState,
    pub pid: SigmaU32,
    pub restart_count: SigmaU32,
    pub auto_restart: SigmaBool,
}

/// Service manager state
const MAX_SERVICES: usize = 128;
static mut SERVICES: [Option<Service>; MAX_SERVICES] = [None; MAX_SERVICES];
static mut SERVICE_COUNT: SigmaU32 = 0;

/// Initialize service manager
#[no_mangle]
pub unsafe extern "C" fn sigma_init_init() -> SigmaI32 {
    SERVICE_COUNT = 0;
    for i in 0..MAX_SERVICES {
        SERVICES[i] = None;
    }
    0 // Success
}

/// Register a service
#[no_mangle]
pub unsafe extern "C" fn sigma_init_register_service(
    name: *const u8,
    description: *const u8,
    exec_start: *const u8,
    exec_stop: *const u8,
    auto_restart: SigmaBool,
) -> SigmaI32 {
    if SERVICE_COUNT >= MAX_SERVICES as SigmaU32 {
        return -1; // Too many services
    }
    
    let service_id = SERVICE_COUNT;
    
    let mut service = Service {
        name: [0; 64],
        description: [0; 256],
        exec_start: [0; 256],
        exec_stop: [0; 256],
        dependencies: [0; 16],
        dependency_count: 0,
        state: ServiceState::Stopped,
        pid: 0,
        restart_count: 0,
        auto_restart,
    };
    
    // Copy strings (simplified - in real implementation would use proper string handling)
    if !name.is_null() {
        for i in 0..63 {
            let byte = *name.add(i);
            if byte == 0 { break; }
            service.name[i] = byte;
        }
    }
    
    if !description.is_null() {
        for i in 0..255 {
            let byte = *description.add(i);
            if byte == 0 { break; }
            service.description[i] = byte;
        }
    }
    
    if !exec_start.is_null() {
        for i in 0..255 {
            let byte = *exec_start.add(i);
            if byte == 0 { break; }
            service.exec_start[i] = byte;
        }
    }
    
    if !exec_stop.is_null() {
        for i in 0..255 {
            let byte = *exec_stop.add(i);
            if byte == 0 { break; }
            service.exec_stop[i] = byte;
        }
    }
    
    SERVICES[service_id as usize] = Some(service);
    SERVICE_COUNT += 1;
    
    service_id as SigmaI32
}

/// Add dependency to service
#[no_mangle]
pub unsafe extern "C" fn sigma_init_add_dependency(service_id: SigmaU32, dependency_id: SigmaU32) -> SigmaI32 {
    if service_id >= SERVICE_COUNT || dependency_id >= SERVICE_COUNT {
        return -1;
    }
    
    if let Some(service) = &mut SERVICES[service_id as usize] {
        if service.dependency_count < 16 {
            service.dependencies[service.dependency_count as usize] = dependency_id;
            service.dependency_count += 1;
            return 0;
        }
    }
    
    -1 // Too many dependencies
}

/// Start a service
#[no_mangle]
pub unsafe extern "C" fn sigma_init_start_service(service_id: SigmaU32) -> SigmaI32 {
    if service_id >= SERVICE_COUNT {
        return -1;
    }
    
    if let Some(service) = &mut SERVICES[service_id as usize] {
        // Check dependencies
        for i in 0..service.dependency_count {
            let dep_id = service.dependencies[i as usize];
            if let Some(dep) = &SERVICES[dep_id as usize] {
                if dep.state != ServiceState::Running {
                    return -2; // Dependency not running
                }
            }
        }
        
        // Start service
        service.state = ServiceState::Starting;
        
        // In a real implementation, this would fork/exec the service
        // For now, we'll just mark it as running
        service.pid = 1000 + service_id; // Placeholder PID
        service.state = ServiceState::Running;
        
        return 0;
    }
    
    -1
}

/// Stop a service
#[no_mangle]
pub unsafe extern "C" fn sigma_init_stop_service(service_id: SigmaU32) -> SigmaI32 {
    if service_id >= SERVICE_COUNT {
        return -1;
    }
    
    if let Some(service) = &mut SERVICES[service_id as usize] {
        service.state = ServiceState::Stopping;
        
        // In a real implementation, this would send SIGTERM/SIGKILL
        // For now, we'll just mark it as stopped
        service.state = ServiceState::Stopped;
        service.pid = 0;
        
        return 0;
    }
    
    -1
}

/// Get service state
#[no_mangle]
pub unsafe extern "C" fn sigma_init_get_state(service_id: SigmaU32) -> SigmaI32 {
    if service_id >= SERVICE_COUNT {
        return -1;
    }
    
    if let Some(service) = &SERVICES[service_id as usize] {
        match service.state {
            ServiceState::Stopped => 0,
            ServiceState::Starting => 1,
            ServiceState::Running => 2,
            ServiceState::Stopping => 3,
            ServiceState::Failed => 4,
        }
    } else {
        -1
    }
}

/// Get service count
#[no_mangle]
pub unsafe extern "C" fn sigma_init_get_service_count() -> SigmaU32 {
    SERVICE_COUNT
}

/// Build dependency graph and perform topological sort (BUG-009 Fix)
unsafe fn build_dependency_order() -> Result<Vec<SigmaU32>, SigmaI32> {
    let mut in_degree = [0u32; MAX_SERVICES];
    let mut order = Vec::new();
    let mut queue = Vec::new();
    
    // Calculate in-degrees
    for i in 0..SERVICE_COUNT as usize {
        if let Some(service) = &SERVICES[i] {
            for j in 0..service.dependency_count as usize {
                let dep_id = service.dependencies[j] as usize;
                if dep_id < MAX_SERVICES {
                    in_degree[dep_id] += 1;
                }
            }
        }
    }
    
    // Find all services with no dependencies (in-degree 0)
    for i in 0..SERVICE_COUNT as usize {
        if in_degree[i] == 0 {
            queue.push(i as SigmaU32);
        }
    }
    
    // Process queue (Kahn's algorithm for topological sort)
    while !queue.is_empty() {
        let current = queue.remove(0);
        order.push(current);
        
        if let Some(service) = &SERVICES[current as usize] {
            for j in 0..service.dependency_count as usize {
                let dep_id = service.dependencies[j] as usize;
                if dep_id < MAX_SERVICES {
                    in_degree[dep_id] -= 1;
                    if in_degree[dep_id] == 0 {
                        queue.push(dep_id as SigmaU32);
                    }
                }
            }
        }
    }
    
    // Check for cycles
    if order.len() != SERVICE_COUNT as usize {
        Err(-3) // Circular dependency detected
    } else {
        Ok(order)
    }
}

/// Start all services in dependency order (BUG-009 Fix - proper topological sort)
#[no_mangle]
pub unsafe extern "C" fn sigma_init_start_all() -> SigmaI32 {
    // Build dependency graph and get startup order
    let order = match build_dependency_order() {
        Ok(o) => o,
        Err(e) => return e,
    };
    
    // Start services in dependency order
    for service_id in order {
        if sigma_init_start_service(service_id) != 0 {
            // Failed to start service - mark as failed and continue
            if let Some(service) = &mut SERVICES[service_id as usize] {
                service.state = ServiceState::Failed;
            }
        }
    }
    
    // Verify all services are running or failed
    let mut running_count = 0;
    for i in 0..SERVICE_COUNT as usize {
        if let Some(service) = &SERVICES[i] {
            if service.state == ServiceState::Running {
                running_count += 1;
            }
        }
    }
    
    if running_count == SERVICE_COUNT as usize {
        0 // All services started successfully
    } else {
        1 // Some services failed to start (but not critical error)
    }
}
