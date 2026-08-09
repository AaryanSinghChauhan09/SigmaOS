#![no_std]
#![no_main]

/// OOP-based Boot Performance Optimization for SigmaOS
/// Implements boot optimization using OOP principles with traits and structs
/// No dependency on external optimization frameworks
/// Based on Roadmap Item 20: Boot performance optimization
/// Incorporates Sysinternals-grade Autoruns and Soluto-grade startup accelerators.

use core::ptr::{self, NonNull};
use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BootError {
    Success = 0,
    InitializationFailed = 1,
    DependencyFailed = 2,
    Timeout = 3,
    SecurityViolation = 4,
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServiceStatus {
    Pending = 0,
    Initializing = 1,
    Ready = 2,
    Failed = 3,
}

/// Service capability
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    pub priority: ServicePriority,
    pub startup_time: u32,
    pub status: AtomicUsize, // ServiceStatus as usize
    pub capability: ServiceCapability,
    pub dependencies: Vec<ServiceID>,
}

impl SimpleBootService {
    pub fn new(id: ServiceID, name: &[u8], priority: ServicePriority, startup_time: u32, capability: ServiceCapability) -> Self {
        let mut name_array = [0u8; 64];
        let name_len = name.len().min(63);

        unsafe {
            core::ptr::copy_nonoverlapping(name.as_ptr(), name_array.as_mut_ptr(), name_len);
        }

        SimpleBootService {
            id,
            name: name_array,
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
        match self.status.load(Ordering::SeqCst) {
            0 => ServiceStatus::Pending,
            1 => ServiceStatus::Initializing,
            2 => ServiceStatus::Ready,
            _ => ServiceStatus::Failed,
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
        let len = self.name.iter().position(|&b| b == 0).unwrap_or(64);
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

/// Sysinternals-grade Autoruns Category representing boot execution entry points
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AutorunsCategory {
    Logon,          // Logon auto-start (registry Run/RunOnce, Startup folder)
    ScheduledTask,  // Background scheduled cron-like automation
    BootExecute,    // Early boot execution (native execution before session manager)
    Driver,         // Kernel driver registration
    ImageHijack,    // Image File Execution Options hijacking
    AppInitDll,     // Early injection AppInit DLL libraries
}

/// Soluto-grade Startup Acceleration Classification
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SolutoClassification {
    NoWay,          // Mandatory critical service that cannot be delayed (e.g. HAL, paging)
    Delay,          // Non-critical background task that can be delayed past idle state
    Remove,         // Safe-to-remove/disable application that shouldn't auto-start
}

/// Sysinternals/Soluto-inspired Auto-Start Configuration Entry
pub struct AutorunsEntry {
    pub entry_id: usize,
    pub name: [u8; 32],
    pub category: AutorunsCategory,
    pub soluto_class: SolutoClassification,
    pub delay_offset_ms: u32,       // Soluto-style delay offset before starting
    pub expected_hash: u32,         // Simple CRC-like binary integrity hash
    pub active: bool,
}

impl AutorunsEntry {
    pub fn new(id: usize, name: &[u8], category: AutorunsCategory, soluto_class: SolutoClassification, expected_hash: u32) -> Self {
        let mut name_arr = [0u8; 32];
        let len = name.len().min(31);
        name_arr[..len].copy_from_slice(&name[..len]);

        AutorunsEntry {
            entry_id: id,
            name: name_arr,
            category,
            soluto_class,
            delay_offset_ms: 0,
            expected_hash,
            active: true,
        }
    }
}

/// Simple boot optimizer (OOP: Concrete optimizer class)
pub struct SimpleBootOptimizer {
    services: Vec<Option<Box<dyn BootService>>>,
    next_id: AtomicUsize,
    stats: BootStats,
    capability: OptimizerCapability,
    // Expanded Autoruns / Soluto containers
    pub autoruns_entries: Vec<AutorunsEntry>,
}

/// Optimizer capability
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OptimizerCapability {
    None = 0,
    Full = 1,
}

impl OptimizerCapability {
    pub fn new() -> Self {
        OptimizerCapability::None
    }

    pub fn full() -> Self {
        OptimizerCapability::Full
    }
}

impl SimpleBootOptimizer {
    pub fn new(capability: OptimizerCapability) -> Self {
        SimpleBootOptimizer {
            services: Vec::new(),
            next_id: AtomicUsize::new(1),
            stats: BootStats::new(),
            capability,
            autoruns_entries: Vec::new(),
        }
    }

    /// Add an Autoruns entry to the Sysinternals registry scan list
    pub fn add_autoruns_entry(&mut self, entry: AutorunsEntry) {
        self.autoruns_entries.push(entry);
    }

    /// Soluto Boot Optimization algorithm: Calculates non-overlapping delayed startup offsets
    /// Shifts non-critical services (Delay) past the initial boot phase to ensure CPU idle responsiveness.
    pub fn calculate_soluto_boot_timeline(&mut self) -> u32 {
        let mut total_delay = 0;
        for i in 0..self.autoruns_entries.len() {
            let entry = &mut self.autoruns_entries[i];
            if entry.active && entry.soluto_class == SolutoClassification::Delay {
                total_delay += 1500; // Incrementally shift each delayed entry by 1.5 seconds
                entry.delay_offset_ms = total_delay;
            }
        }
        total_delay
    }

    /// Sysinternals Autorun Integrity Scanner: Audits binary hashes to detect Image Hijacking or malicious malware alterations
    pub fn audit_binary_integrity(&self, entries_actual_hashes: &[(usize, u32)]) -> usize {
        let mut suspicious_entries_count = 0;
        for i in 0..self.autoruns_entries.len() {
            let entry = &self.autoruns_entries[i];
            if entry.active {
                // Find matching actual hash
                let mut actual_hash = None;
                for j in 0..entries_actual_hashes.len() {
                    let &(id, hash) = &entries_actual_hashes[j];
                    if id == entry.entry_id {
                        actual_hash = Some(hash);
                        break;
                    }
                }

                if let Some(hash) = actual_hash {
                    if hash != entry.expected_hash {
                        // Integrity mismatch: possible malware hijacker detected!
                        suspicious_entries_count += 1;
                    }
                }
            }
        }
        suspicious_entries_count
    }

    /// Toggle Autorun Entry Active state (Soluto Remove / Sysinternals Disable action)
    pub fn toggle_autorun_entry(&mut self, entry_id: usize, enabled: bool) -> bool {
        for i in 0..self.autoruns_entries.len() {
            if self.autoruns_entries[i].entry_id == entry_id {
                self.autoruns_entries[i].active = enabled;
                return true;
            }
        }
        false
    }
}

impl BootOptimizer for SimpleBootOptimizer {
    fn register_service(&mut self, service: Box<dyn BootService>) -> Result<ServiceID, BootError> {
        if self.capability != OptimizerCapability::Full {
            return Err(BootError::InitializationFailed);
        }

        let id = service.id();
        self.services.push(Some(service));
        self.stats.total_services += 1;
        Ok(id)
    }

    fn unregister_service(&mut self, id: ServiceID) -> Result<(), BootError> {
        if self.capability != OptimizerCapability::Full {
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
        if self.capability != OptimizerCapability::Full {
            return Err(BootError::InitializationFailed);
        }

        for i in 0..self.services.len() {
            if let Some(ref mut service) = self.services[i] {
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
        if self.capability != OptimizerCapability::Full {
            return Err(BootError::InitializationFailed);
        }

        let mut ordered_ids = Vec::new();

        // Collect all service IDs with their priorities
        let mut services_with_priority: Vec<(ServiceID, ServicePriority)> = Vec::new();

        for i in 0..self.services.len() {
            if let Some(ref service) = self.services[i] {
                services_with_priority.push((service.id(), service.priority()));
            }
        }

        // Sort by priority (lower priority = higher importance)
        for i in 0..services_with_priority.len() {
            for j in (i + 1)..services_with_priority.len() {
                if services_with_priority[j].1 < services_with_priority[i].1 {
                    let temp = services_with_priority[i];
                    services_with_priority[i] = services_with_priority[j];
                    services_with_priority[j] = temp;
                }
            }
        }

        for i in 0..services_with_priority.len() {
            ordered_ids.push(services_with_priority[i].0);
        }

        Ok(ordered_ids)
    }

    fn initialize_all(&mut self) -> Result<(), BootError> {
        if self.capability != OptimizerCapability::Full {
            return Err(BootError::InitializationFailed);
        }

        let optimized_order = self.optimize_boot_order()?;

        for i in 0..optimized_order.len() {
            let _ = self.initialize_service(optimized_order[i]);
        }

        Ok(())
    }

    fn get_service(&self, id: ServiceID) -> Option<&dyn BootService> {
        for i in 0..self.services.len() {
            if let Some(ref service) = self.services[i] {
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

/// Custom Box implementation to avoid alloc-crate dependencies on standalone boot environments
#[repr(transparent)]
pub struct Box<T: ?Sized> {
    ptr: NonNull<T>,
}

impl<T> Box<T> {
    pub fn new(val: T) -> Self {
        unsafe {
            let ptr = alloc(mem::size_of::<T>()) as *mut T;
            if ptr.is_null() {
                panic!("Allocation failed");
            }
            ptr::write(ptr, val);
            Box {
                ptr: NonNull::new_unchecked(ptr),
            }
        }
    }
}

impl<T: ?Sized> AsRef<T> for Box<T> {
    fn as_ref(&self) -> &T {
        unsafe { self.ptr.as_ref() }
    }
}

impl<T: ?Sized> AsMut<T> for Box<T> {
    fn as_mut(&mut self) -> &mut T {
        unsafe { self.ptr.as_mut() }
    }
}

impl<T: ?Sized> core::ops::Deref for Box<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { self.ptr.as_ref() }
    }
}

impl<T: ?Sized> core::ops::DerefMut for Box<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.ptr.as_mut() }
    }
}

impl<T: ?Sized> Drop for Box<T> {
    fn drop(&mut self) {
        unsafe {
            ptr::drop_in_place(self.ptr.as_ptr());
            free(self.ptr.as_ptr() as *mut u8);
        }
    }
}

/// Simple Vec implementation for no_std
pub struct Vec<T> {
    data: *mut T,
    len: usize,
    capacity: usize,
}

impl<T> Default for Vec<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Vec<T> {
    pub fn new() -> Self {
        Vec {
            data: core::ptr::null_mut(),
            len: 0,
            capacity: 0,
        }
    }

    pub fn push(&mut self, item: T) {
        unsafe {
            if self.len >= self.capacity {
                self.grow();
            }

            if self.capacity > self.len {
                core::ptr::write(self.data.add(self.len), item);
                self.len += 1;
            }
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn iter(&self) -> VecIter<'_, T> {
        VecIter {
            vec: self,
            index: 0,
        }
    }

    pub fn iter_mut(&mut self) -> VecIterMut<'_, T> {
        VecIterMut {
            data: self.data,
            len: self.len,
            index: 0,
            _marker: core::marker::PhantomData,
        }
    }

    pub fn as_slice(&self) -> &[T] {
        if self.len == 0 {
            &[]
        } else {
            unsafe { core::slice::from_raw_parts(self.data, self.len) }
        }
    }

    pub fn as_mut_slice(&mut self) -> &mut [T] {
        if self.len == 0 {
            &mut []
        } else {
            unsafe { core::slice::from_raw_parts_mut(self.data, self.len) }
        }
    }

    unsafe fn grow(&mut self) {
        let new_capacity = if self.capacity == 0 {
            4
        } else {
            self.capacity * 2
        };
        let new_data = alloc(new_capacity * mem::size_of::<T>()) as *mut T;

        if !new_data.is_null() {
            for i in 0..self.len {
                core::ptr::copy_nonoverlapping(self.data.add(i), new_data.add(i), 1);
            }

            if self.capacity > 0 {
                free(self.data as *mut u8);
            }

            self.data = new_data;
            self.capacity = new_capacity;
        }
    }
}

impl<T> core::ops::Index<usize> for Vec<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        if index >= self.len {
            panic!("index out of bounds");
        }
        unsafe { &*self.data.add(index) }
    }
}

impl<T> core::ops::IndexMut<usize> for Vec<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index >= self.len {
            panic!("index out of bounds");
        }
        unsafe { &mut *self.data.add(index) }
    }
}

impl<'a, T> IntoIterator for &'a Vec<T> {
    type Item = &'a T;
    type IntoIter = VecIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut Vec<T> {
    type Item = &'a mut T;
    type IntoIter = VecIterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

pub struct VecIter<'a, T> {
    vec: &'a Vec<T>,
    index: usize,
}

impl<'a, T> Iterator for VecIter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.vec.len() {
            let item = unsafe { &*self.vec.data.add(self.index) };
            self.index += 1;
            Some(item)
        } else {
            None
        }
    }
}

pub struct VecIterMut<'a, T> {
    data: *mut T,
    len: usize,
    index: usize,
    _marker: core::marker::PhantomData<&'a mut T>,
}

impl<'a, T> Iterator for VecIterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.len {
            let item = unsafe { &mut *self.data.add(self.index) };
            self.index += 1;
            Some(item)
        } else {
            None
        }
    }
}

impl<T> Drop for Vec<T> {
    fn drop(&mut self) {
        if self.capacity > 0 && !self.data.is_null() {
            for i in 0..self.len {
                unsafe {
                    core::ptr::drop_in_place(self.data.add(i));
                }
            }
            unsafe {
                free(self.data as *mut u8);
            }
        }
    }
}

// Allocator shim: uses std allocator on hosted targets (test/dev) and extern C on bare-metal
#[cfg(not(target_os = "none"))]
unsafe fn alloc(size: usize) -> *mut u8 {
    use std::alloc::{alloc as std_alloc, Layout};
    let layout = Layout::from_size_align(size, 8).unwrap();
    std_alloc(layout)
}

#[cfg(not(target_os = "none"))]
unsafe fn free(ptr: *mut u8) {
    // Hosted target release stub (mock layout)
    let _ = ptr;
}

#[cfg(target_os = "none")]
extern "C" {
    fn alloc(size: usize) -> *mut u8;
    fn free(ptr: *mut u8);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_boot_service_creation() {
        let capability = ServiceCapability { can_initialize: true, can_parallelize: true };
        let service = SimpleBootService::new(1, b"test_service", ServicePriority::High, 150, capability);

        assert_eq!(service.id(), 1);
        assert_eq!(service.name(), b"test_service");
        assert_eq!(service.priority(), ServicePriority::High);
        assert_eq!(service.startup_time(), 150);
        assert_eq!(service.get_status(), ServiceStatus::Pending);
    }

    #[test]
    fn test_boot_optimizer() {
        let opt_capability = OptimizerCapability::full();
        let mut optimizer = SimpleBootOptimizer::new(opt_capability);

        let cap1 = ServiceCapability { can_initialize: true, can_parallelize: true };
        let s1 = SimpleBootService::new(1, b"s1", ServicePriority::Low, 100, cap1);
        let s2 = SimpleBootService::new(2, b"s2", ServicePriority::Critical, 50, cap1);

        assert!(optimizer.register_service(Box::new(s1)).is_ok());
        assert!(optimizer.register_service(Box::new(s2)).is_ok());

        let stats = optimizer.stats();
        assert_eq!(stats.total_services, 2);

        let ordered = optimizer.optimize_boot_order().unwrap();
        assert_eq!(ordered.len(), 2);
        assert_eq!(ordered[0], 2); // s2 has critical priority, should be first
        assert_eq!(ordered[1], 1); // s1 has low priority, should be second

        assert!(optimizer.initialize_all().is_ok());
        let final_stats = optimizer.stats();
        assert_eq!(final_stats.ready_services, 2);
        assert_eq!(final_stats.total_boot_time, 150);
    }

    #[test]
    fn test_sysinternals_autoruns_and_soluto_delay() {
        let opt_cap = OptimizerCapability::full();
        let mut optimizer = SimpleBootOptimizer::new(opt_cap);

        let entry1 = AutorunsEntry::new(101, b"MaliciousHijack", AutorunsCategory::ImageHijack, SolutoClassification::NoWay, 0xABCDE);
        let entry2 = AutorunsEntry::new(102, b"TelemetryUpdate", AutorunsCategory::Logon, SolutoClassification::Delay, 0x55555);
        let entry3 = AutorunsEntry::new(103, b"SystemHal", AutorunsCategory::BootExecute, SolutoClassification::NoWay, 0x11111);

        optimizer.add_autoruns_entry(entry1);
        optimizer.add_autoruns_entry(entry2);
        optimizer.add_autoruns_entry(entry3);

        assert_eq!(optimizer.autoruns_entries.len(), 3);

        // Test Soluto delayed boot calculation
        let max_delay = optimizer.calculate_soluto_boot_timeline();
        assert_eq!(max_delay, 1500); // Only entry2 is classified as Delay (1 * 1500)
        assert_eq!(optimizer.autoruns_entries[1].delay_offset_ms, 1500);

        // Test Sysinternals Autorun Integrity Scanner with fake audit hashes
        // entry1 has expected 0xABCDE, actual is 0xBADFF -> mismatch (suspicious!)
        // entry2 has expected 0x55555, actual is 0x55555 -> valid
        // entry3 has expected 0x11111, actual is 0x11111 -> valid
        let actual_hashes = [
            (101, 0xBADFF),
            (102, 0x55555),
            (103, 0x11111),
        ];

        let suspicious_count = optimizer.audit_binary_integrity(&actual_hashes);
        assert_eq!(suspicious_count, 1);

        // Test Soluto Remove (Toggle Entry Active)
        assert!(optimizer.toggle_autorun_entry(102, false));
        assert!(!optimizer.autoruns_entries[1].active);
    }
}
