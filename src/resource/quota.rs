#![no_std]
#![no_main]

/// OOP-based Resource Quota for SigmaOS
/// Based on Ideas-999-Structured: Kernel & Hardware Item 221
/// Implements resource quota management and enforcement

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type QuotaID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum ResourceType { CPU = 0, Memory = 1, Disk = 2, Network = 3 }

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum QuotaError { Success = 0, Exceeded = 1, NotFound = 2 }

pub trait Quota {
    fn id(&self) -> QuotaID;
    fn resource_type(&self) -> ResourceType;
    fn limit(&self) -> u64;
    fn usage(&self) -> u64;
    fn set_limit(&mut self, limit: u64);
    fn add_usage(&mut self, amount: u64) -> Result<(), QuotaError>;
}

#[repr(C)]
pub struct SimpleQuota {
    pub id: QuotaID,
    pub resource_type: AtomicUsize,
    pub limit: AtomicUsize,
    pub usage: AtomicUsize,
}

impl SimpleQuota {
    pub fn new(id: QuotaID, resource_type: ResourceType, limit: u64) -> Self {
        SimpleQuota {
            id,
            resource_type: AtomicUsize::new(resource_type as usize),
            limit: AtomicUsize::new(limit as usize),
            usage: AtomicUsize::new(0),
        }
    }
}

impl Quota for SimpleQuota {
    fn id(&self) -> QuotaID { self.id }
    fn resource_type(&self) -> ResourceType { unsafe { core::mem::transmute(self.resource_type.load(Ordering::SeqCst)) } }
    fn limit(&self) -> u64 { self.limit.load(Ordering::SeqCst) as u64 }
    fn usage(&self) -> u64 { self.usage.load(Ordering::SeqCst) as u64 }
    
    fn set_limit(&mut self, limit: u64) {
        self.limit.store(limit as usize, Ordering::SeqCst);
    }
    
    fn add_usage(&mut self, amount: u64) -> Result<(), QuotaError> {
        let current = self.usage.load(Ordering::SeqCst);
        let limit = self.limit.load(Ordering::SeqCst);
        
        if current + amount as usize > limit {
            Err(QuotaError::Exceeded)
        } else {
            self.usage.fetch_add(amount as usize, Ordering::SeqCst);
            Ok(())
        }
    }
}

pub trait QuotaManager {
    fn create_quota(&mut self, resource_type: ResourceType, limit: u64) -> Result<QuotaID, QuotaError>;
    fn delete_quota(&mut self, id: QuotaID) -> Result<(), QuotaError>;
    fn get_quota(&self, id: QuotaID) -> Option<&dyn Quota>;
    fn check_quota(&self, id: QuotaID, amount: u64) -> Result<(), QuotaError>;
    def reset_usage(&mut self, id: QuotaID) -> Result<(), QuotaError>;
}

#[repr(C)]
pub struct SimpleQuotaManager {
    pub quotas: Vec<Option<Box<dyn Quota>>>,
    pub next_id: AtomicUsize,
}

impl SimpleQuotaManager {
    pub fn new() -> Self {
        SimpleQuotaManager {
            quotas: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl QuotaManager for SimpleQuotaManager {
    fn create_quota(&mut self, resource_type: ResourceType, limit: u64) -> Result<QuotaID, QuotaError> {
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        let quota = SimpleQuota::new(id, resource_type, limit);
        self.quotas.push(Some(Box::new(quota)));
        Ok(id)
    }
    
    fn delete_quota(&mut self, id: QuotaID) -> Result<(), QuotaError> {
        for quota_option in &mut self.quotas {
            if let Some(ref quota) = *quota_option {
                if quota.id() == id {
                    return Ok(());
                }
            }
        }
        Err(QuotaError::NotFound)
    }
    
    fn get_quota(&self, id: QuotaID) -> Option<&dyn Quota> {
        for quota_option in &self.quotas {
            if let Some(ref quota) = *quota_option {
                if quota.id() == id { return Some(quota.as_ref()); }
            }
        }
        None
    }
    
    fn check_quota(&self, id: QuotaID, amount: u64) -> Result<(), QuotaError> {
        if let Some(quota) = self.get_quota(id) {
            let current = quota.usage();
            let limit = quota.limit();
            
            if current + amount > limit {
                Err(QuotaError::Exceeded)
            } else {
                Ok(())
            }
        } else {
            Err(QuotaError::NotFound)
        }
    }
    
    fn reset_usage(&mut self, id: QuotaID) -> Result<(), QuotaError> {
        for quota_option in &mut self.quotas {
            if let Some(ref mut quota) = *quota_option {
                if quota.id() == id {
                    quota.usage.store(0, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(QuotaError::NotFound)
    }
}

pub trait ResourceEnforcer {
    fn enforce(&mut self, resource_type: ResourceType, amount: u64) -> Result<(), QuotaError>;
    fn get_usage(&self, resource_type: ResourceType) -> u64;
}

#[repr(C)]
pub struct SimpleResourceEnforcer {
    pub manager: SimpleQuotaManager,
}

impl SimpleResourceEnforcer {
    pub fn new(manager: SimpleQuotaManager) -> Self {
        SimpleResourceEnforcer { manager }
    }
}

impl ResourceEnforcer for SimpleResourceEnforcer {
    fn enforce(&mut self, resource_type: ResourceType, amount: u64) -> Result<(), QuotaError> {
        for quota_option in &mut self.manager.quotas {
            if let Some(ref mut quota) = *quota_option {
                if quota.resource_type() == resource_type {
                    return quota.add_usage(amount);
                }
            }
        }
        Err(QuotaError::NotFound)
    }
    
    fn get_usage(&self, resource_type: ResourceType) -> u64 {
        for quota_option in &self.manager.quotas {
            if let Some(ref quota) = *quota_option {
                if quota.resource_type() == resource_type {
                    return quota.usage();
                }
            }
        }
        0
    }
}

struct Vec<T> { data: *mut T, len: usize, capacity: usize }

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
    }
    unsafe fn grow(&mut self) {
        let new_capacity = if self.capacity == 0 { 4 } else { self.capacity * 2 };
        let new_data = alloc(new_capacity * mem::size_of::<T>()) as *mut T;
        if !new_data.is_null() {
            for i in 0..self.len { core::ptr::copy_nonoverlapping(self.data.add(i), new_data.add(i), 1); }
            if self.capacity > 0 { free(self.data as *mut u8); }
            self.data = new_data;
            self.capacity = new_capacity;
        }
    }
}

extern "C" { fn alloc(size: usize) -> *mut u8; fn free(ptr: *mut u8); }
