#![no_std]
#![no_main]

/// OOP-based Resource Manager for SigmaOS
/// Implements resource management using OOP principles with traits and structs
/// No dependency on external resource frameworks

use core::ptr::{self, NonNull};
use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

/// Resource ID
pub type ResourceID = usize;

/// Resource type
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum ResourceType {
    Memory = 0,
    File = 1,
    Device = 2,
    Network = 3,
    Custom = 4,
}

/// Resource trait (OOP interface)
pub trait Resource {
    /// Get resource ID
    fn id(&self) -> ResourceID;
    /// Get resource type
    fn resource_type(&self) -> ResourceType;
    /// Acquire resource
    fn acquire(&mut self) -> Result<(), ResourceError>;
    /// Release resource
    fn release(&mut self) -> Result<(), ResourceError>;
    /// Get resource info
    fn info(&self) -> ResourceInfo;
}

/// Resource error types
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum ResourceError {
    Success = 0,
    AlreadyAcquired = 1,
    NotAcquired = 2,
    PermissionDenied = 3,
    ResourceBusy = 4,
    NotFound = 5,
}

/// Resource info
#[repr(C)]
pub struct ResourceInfo {
    pub id: ResourceID,
    pub resource_type: ResourceType,
    pub is_acquired: bool,
    pub ref_count: usize,
    pub capability: ResourceCapability,
}

impl ResourceInfo {
    pub fn new(id: ResourceID, resource_type: ResourceType) -> Self {
        ResourceInfo {
            id,
            resource_type,
            is_acquired: false,
            ref_count: 0,
            capability: ResourceCapability::new(),
        }
    }
}

/// Resource capability
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ResourceCapability {
    pub can_acquire: bool,
    pub can_release: bool,
    pub can_share: bool,
}

impl ResourceCapability {
    pub fn new() -> Self {
        ResourceCapability {
            can_acquire: false,
            can_release: false,
            can_share: false,
        }
    }

    pub fn full() -> Self {
        ResourceCapability {
            can_acquire: true,
            can_release: true,
            can_share: true,
        }
    }
}

/// Simple resource (OOP: Concrete resource class)
#[repr(C)]
pub struct SimpleResource {
    pub id: ResourceID,
    pub resource_type: ResourceType,
    pub acquired: AtomicBool,
    pub ref_count: AtomicUsize,
    pub capability: ResourceCapability,
    pub data: Option<NonNull<u8>>,
    pub data_size: usize,
}

impl SimpleResource {
    pub fn new(id: ResourceID, resource_type: ResourceType, capability: ResourceCapability) -> Self {
        SimpleResource {
            id,
            resource_type,
            acquired: AtomicBool::new(false),
            ref_count: AtomicUsize::new(0),
            capability,
            data: None,
            data_size: 0,
        }
    }

    pub fn set_data(&mut self, data: &[u8]) {
        let data_ptr = unsafe {
            let ptr = alloc(data.len()) as *mut u8;
            if ptr.is_null() {
                return;
            }
            core::ptr::copy_nonoverlapping(data.as_ptr(), ptr, data.len());
            NonNull::new_unchecked(ptr)
        };

        if let Some(old_data) = self.data {
            unsafe {
                free(old_data.as_ptr());
            }
        }

        self.data = Some(data_ptr);
        self.data_size = data.len();
    }

    pub unsafe fn get_data(&self) -> Option<&[u8]> {
        self.data.map(|ptr| core::slice::from_raw_parts(ptr.as_ptr(), self.data_size))
    }

    pub fn increment_ref(&self) {
        self.ref_count.fetch_add(1, Ordering::SeqCst);
    }

    pub fn decrement_ref(&self) -> usize {
        self.ref_count.fetch_sub(1, Ordering::SeqCst) - 1
    }
}

impl Resource for SimpleResource {
    fn id(&self) -> ResourceID {
        self.id
    }

    fn resource_type(&self) -> ResourceType {
        self.resource_type
    }

    fn acquire(&mut self) -> Result<(), ResourceError> {
        if !self.capability.can_acquire {
            return Err(ResourceError::PermissionDenied);
        }

        if self.acquired.load(Ordering::SeqCst) && !self.capability.can_share {
            return Err(ResourceError::AlreadyAcquired);
        }

        self.acquired.store(true, Ordering::SeqCst);
        self.increment_ref();
        Ok(())
    }

    fn release(&mut self) -> Result<(), ResourceError> {
        if !self.capability.can_release {
            return Err(ResourceError::PermissionDenied);
        }

        if !self.acquired.load(Ordering::SeqCst) {
            return Err(ResourceError::NotAcquired);
        }

        let new_count = self.decrement_ref();
        if new_count == 0 {
            self.acquired.store(false, Ordering::SeqCst);
        }

        Ok(())
    }

    fn info(&self) -> ResourceInfo {
        ResourceInfo {
            id: self.id,
            resource_type: self.resource_type,
            is_acquired: self.acquired.load(Ordering::SeqCst),
            ref_count: self.ref_count.load(Ordering::SeqCst),
            capability: self.capability,
        }
    }
}

impl Drop for SimpleResource {
    fn drop(&mut self) {
        unsafe {
            if let Some(data) = self.data {
                free(data.as_ptr());
            }
        }
    }
}

/// Resource manager trait (OOP interface)
pub trait ResourceManager {
    /// Register resource
    fn register_resource(&mut self, resource: Box<dyn Resource>) -> Result<ResourceID, ResourceError>;
    /// Unregister resource
    fn unregister_resource(&mut self, id: ResourceID) -> Result<(), ResourceError>;
    /// Acquire resource
    fn acquire_resource(&mut self, id: ResourceID) -> Result<(), ResourceError>;
    /// Release resource
    fn release_resource(&mut self, id: ResourceID) -> Result<(), ResourceError>;
    /// Get resource
    fn get_resource(&self, id: ResourceID) -> Option<&dyn Resource>;
    /// Get resource mutable
    fn get_resource_mut(&mut self, id: ResourceID) -> Option<&mut Box<dyn Resource>>;
    /// Get manager statistics
    fn stats(&self) -> ResourceStats;
}

/// Resource statistics
#[repr(C)]
pub struct ResourceStats {
    pub total_resources: usize,
    pub acquired_resources: usize,
    pub available_resources: usize,
    pub by_type: [usize; 5],
}

impl ResourceStats {
    pub fn new() -> Self {
        ResourceStats {
            total_resources: 0,
            acquired_resources: 0,
            available_resources: 0,
            by_type: [0; 5],
        }
    }
}

/// Simple resource manager (OOP: Concrete manager class)
pub struct SimpleResourceManager {
    resources: Vec<Option<Box<dyn Resource>>>,
    next_id: AtomicUsize,
    stats: ResourceStats,
    capability: ManagerCapability,
}

/// Manager capability
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ManagerCapability {
    pub can_register: bool,
    pub can_unregister: bool,
    pub can_acquire: bool,
    pub can_release: bool,
}

impl ManagerCapability {
    pub fn new() -> Self {
        ManagerCapability {
            can_register: false,
            can_unregister: false,
            can_acquire: false,
            can_release: false,
        }
    }

    pub fn full() -> Self {
        ManagerCapability {
            can_register: true,
            can_unregister: true,
            can_acquire: true,
            can_release: true,
        }
    }
}

impl SimpleResourceManager {
    pub fn new(capability: ManagerCapability) -> Self {
        SimpleResourceManager {
            resources: Vec::new(),
            next_id: AtomicUsize::new(1),
            stats: ResourceStats::new(),
            capability,
        }
    }
}

impl ResourceManager for SimpleResourceManager {
    fn register_resource(&mut self, resource: Box<dyn Resource>) -> Result<ResourceID, ResourceError> {
        if !self.capability.can_register {
            return Err(ResourceError::PermissionDenied);
        }

        let id = resource.id();
        let resource_type = resource.resource_type();
        self.resources.push(Some(resource));
        self.stats.total_resources += 1;
        self.stats.available_resources += 1;
        self.stats.by_type[resource_type as usize] += 1;
        Ok(id)
    }

    fn unregister_resource(&mut self, id: ResourceID) -> Result<(), ResourceError> {
        if !self.capability.can_unregister {
            return Err(ResourceError::PermissionDenied);
        }

        let mut index = None;
        let mut resource_type = ResourceType::Memory;

        for (i, resource_option) in self.resources.iter().enumerate() {
            if let Some(ref resource) = *resource_option {
                if resource.id() == id {
                    index = Some(i);
                    resource_type = resource.resource_type();
                    break;
                }
            }
        }

        if let Some(i) = index {
            self.resources[i] = None;
            self.stats.total_resources -= 1;
            self.stats.by_type[resource_type as usize] -= 1;
            Ok(())
        } else {
            Err(ResourceError::NotFound)
        }
    }

    fn acquire_resource(&mut self, id: ResourceID) -> Result<(), ResourceError> {
        if !self.capability.can_acquire {
            return Err(ResourceError::PermissionDenied);
        }

        if let Some(ref mut resource) = self.get_resource_mut(id) {
            let result = resource.acquire();
            if result.is_ok() {
                let info = resource.info();
                if info.is_acquired && info.ref_count == 1 {
                    self.stats.available_resources -= 1;
                    self.stats.acquired_resources += 1;
                }
            }
            result
        } else {
            Err(ResourceError::NotFound)
        }
    }

    fn release_resource(&mut self, id: ResourceID) -> Result<(), ResourceError> {
        if !self.capability.can_release {
            return Err(ResourceError::PermissionDenied);
        }

        if let Some(ref mut resource) = self.get_resource_mut(id) {
            let result = resource.release();
            if result.is_ok() {
                let info = resource.info();
                if !info.is_acquired {
                    self.stats.acquired_resources -= 1;
                    self.stats.available_resources += 1;
                }
            }
            result
        } else {
            Err(ResourceError::NotFound)
        }
    }

    fn get_resource(&self, id: ResourceID) -> Option<&dyn Resource> {
        for resource_option in &self.resources {
            if let Some(ref resource) = *resource_option {
                if resource.id() == id {
                    return Some(resource.as_ref());
                }
            }
        }
        None
    }

    fn get_resource_mut(&mut self, id: ResourceID) -> Option<&mut Box<dyn Resource>> {
        for resource_option in &mut self.resources {
            if let Some(ref mut resource) = *resource_option {
                if resource.id() == id {
                    return Some(resource);
                }
            }
        }
        None
    }

    fn stats(&self) -> ResourceStats {
        self.stats
    }
}

/// Simple Vec implementation for no_std
struct Vec<T> {
    data: *mut T,
    len: usize,
    capacity: usize,
}

impl<T> Vec<T> {
    fn new() -> Self {
        Vec {
            data: core::ptr::null_mut(),
            len: 0,
            capacity: 0,
        }
    }

    fn push(&mut self, item: T) {
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

    fn len(&self) -> usize {
        self.len
    }

    unsafe fn grow(&mut self) {
        let new_capacity = if self.capacity == 0 { 4 } else { self.capacity * 2 };
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

// External allocator functions
extern "C" {
    fn alloc(size: usize) -> *mut u8;
    fn free(ptr: *mut u8);
}
