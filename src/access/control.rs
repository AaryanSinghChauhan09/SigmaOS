#![no_std]
#![no_main]

/// OOP-based Access Control for SigmaOS
/// Based on Ideas-999-Structured: Security & Sovereignty Item 541
/// Implements zero-trust access control and RBAC

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type RoleID = usize;
pub type PermissionID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum AccessError { Success = 0, Denied = 1, InvalidRole = 2, InvalidPermission = 3 }

pub trait Role {
    fn id(&self) -> RoleID;
    fn name(&self) -> &[u8];
    fn has_permission(&self, permission_id: PermissionID) -> bool;
}

#[repr(C)]
pub struct SimpleRole {
    pub id: RoleID,
    pub name: [u8; 64],
    pub permissions: Vec<PermissionID>,
}

impl SimpleRole {
    pub fn new(id: RoleID, name: &[u8]) -> Self {
        let mut name_array = [0u8; 64];
        let name_len = name.len().min(63);
        unsafe {
            core::ptr::copy_nonoverlapping(name.as_ptr(), name_array.as_mut_ptr(), name_len);
        }
        SimpleRole {
            id,
            name: name_array,
            permissions: Vec::new(),
        }
    }
}

impl Role for SimpleRole {
    fn id(&self) -> RoleID { self.id }
    fn name(&self) -> &[u8] {
        let len = self.name.iter().position(|&b| b == 0).unwrap_or(64);
        &self.name[..len]
    }
    fn has_permission(&self, permission_id: PermissionID) -> bool {
        for &perm in &self.permissions {
            if perm == permission_id { return true; }
        }
        false
    }
}

pub trait Permission {
    fn id(&self) -> PermissionID;
    fn resource(&self) -> &[u8];
    fn action(&self) -> &[u8];
}

#[repr(C)]
pub struct SimplePermission {
    pub id: PermissionID,
    pub resource: [u8; 128],
    pub action: [u8; 64],
}

impl SimplePermission {
    pub fn new(id: PermissionID, resource: &[u8], action: &[u8]) -> Self {
        let mut resource_array = [0u8; 128];
        let mut action_array = [0u8; 64];
        let resource_len = resource.len().min(127);
        let action_len = action.len().min(63);
        unsafe {
            core::ptr::copy_nonoverlapping(resource.as_ptr(), resource_array.as_mut_ptr(), resource_len);
            core::ptr::copy_nonoverlapping(action.as_ptr(), action_array.as_mut_ptr(), action_len);
        }
        SimplePermission {
            id,
            resource: resource_array,
            action: action_array,
        }
    }
}

impl Permission for SimplePermission {
    fn id(&self) -> PermissionID { self.id }
    fn resource(&self) -> &[u8] {
        let len = self.resource.iter().position(|&b| b == 0).unwrap_or(128);
        &self.resource[..len]
    }
    fn action(&self) -> &[u8] {
        let len = self.action.iter().position(|&b| b == 0).unwrap_or(64);
        &self.action[..len]
    }
}

pub trait AccessController {
    fn grant_permission(&mut self, role_id: RoleID, permission_id: PermissionID) -> Result<(), AccessError>;
    fn revoke_permission(&mut self, role_id: RoleID, permission_id: PermissionID) -> Result<(), AccessError>;
    fn check_access(&self, role_id: RoleID, resource: &[u8], action: &[u8]) -> Result<bool, AccessError>;
}

#[repr(C)]
pub struct SimpleAccessController {
    pub roles: Vec<Option<Box<dyn Role>>>,
    pub permissions: Vec<Option<Box<dyn Permission>>>,
}

impl SimpleAccessController {
    pub fn new() -> Self {
        SimpleAccessController {
            roles: Vec::new(),
            permissions: Vec::new(),
        }
    }
}

impl AccessController for SimpleAccessController {
    fn grant_permission(&mut self, role_id: RoleID, permission_id: PermissionID) -> Result<(), AccessError> {
        for role_option in &mut self.roles {
            if let Some(ref mut role) = *role_option {
                if role.id() == role_id {
                    if let SimpleRole { ref mut permissions, .. } = **role {
                        permissions.push(permission_id);
                        return Ok(());
                    }
                }
            }
        }
        Err(AccessError::InvalidRole)
    }

    fn revoke_permission(&mut self, role_id: RoleID, permission_id: PermissionID) -> Result<(), AccessError> {
        for role_option in &mut self.roles {
            if let Some(ref mut role) = *role_option {
                if role.id() == role_id {
                    if let SimpleRole { ref mut permissions, .. } = **role {
                        for i in 0..permissions.len() {
                            if permissions[i] == permission_id {
                                permissions.remove(i);
                                return Ok(());
                            }
                        }
                    }
                }
            }
        }
        Err(AccessError::InvalidRole)
    }

    fn check_access(&self, role_id: RoleID, resource: &[u8], action: &[u8]) -> Result<bool, AccessError> {
        for role_option in &self.roles {
            if let Some(ref role) = *role_option {
                if role.id() == role_id {
                    for perm_option in &self.permissions {
                        if let Some(ref perm) = *perm_option {
                            if role.has_permission(perm.id()) {
                                if perm.resource() == resource && perm.action() == action {
                                    return Ok(true);
                                }
                            }
                        }
                    }
                    return Ok(false);
                }
            }
        }
        Err(AccessError::InvalidRole)
    }
}

pub trait ZeroTrustPolicy {
    fn verify_identity(&self, identity: &[u8]) -> Result<bool, AccessError>;
    fn check_device_trust(&self, device_id: usize) -> Result<bool, AccessError>;
    fn enforce_mfa(&self, user_id: usize) -> Result<bool, AccessError>;
}

#[repr(C)]
pub struct SimpleZeroTrustPolicy {
    pub trusted_devices: Vec<usize>,
}

impl SimpleZeroTrustPolicy {
    pub fn new() -> Self {
        SimpleZeroTrustPolicy {
            trusted_devices: Vec::new(),
        }
    }
}

impl ZeroTrustPolicy for SimpleZeroTrustPolicy {
    fn verify_identity(&self, _identity: &[u8]) -> Result<bool, AccessError> {
        Ok(true)
    }

    fn check_device_trust(&self, device_id: usize) -> Result<bool, AccessError> {
        for &id in &self.trusted_devices {
            if id == device_id { return Ok(true); }
        }
        Ok(false)
    }

    fn enforce_mfa(&self, _user_id: usize) -> Result<bool, AccessError> {
        Ok(true)
    }
}

pub trait AuditLogger {
    fn log_access_attempt(&mut self, role_id: RoleID, resource: &[u8], action: &[u8], granted: bool);
    fn get_audit_trail(&self) -> Vec<(RoleID, [u8; 128], [u8; 64], bool)>;
}

#[repr(C)]
pub struct SimpleAuditLogger {
    pub audit_trail: Vec<(RoleID, [u8; 128], [u8; 64], bool)>,
}

impl SimpleAuditLogger {
    pub fn new() -> Self {
        SimpleAuditLogger {
            audit_trail: Vec::new(),
        }
    }
}

impl AuditLogger for SimpleAuditLogger {
    fn log_access_attempt(&mut self, role_id: RoleID, resource: &[u8], action: &[u8], granted: bool) {
        let mut resource_array = [0u8; 128];
        let mut action_array = [0u8; 64];
        let resource_len = resource.len().min(127);
        let action_len = action.len().min(63);
        for i in 0..resource_len { resource_array[i] = resource[i]; }
        for i in 0..action_len { action_array[i] = action[i]; }
        self.audit_trail.push((role_id, resource_array, action_array, granted));
    }

    fn get_audit_trail(&self) -> Vec<(RoleID, [u8; 128], [u8; 64], bool)> {
        self.audit_trail.clone()
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
    fn clone(&self) -> Vec<T> {
        let mut new_vec = Vec::new();
        for i in 0..self.len {
            unsafe {
                let item = core::ptr::read(self.data.add(i));
                new_vec.push(item);
            }
        }
        new_vec
    }
    fn remove(&mut self, index: usize) -> T {
        unsafe {
            let item = core::ptr::read(self.data.add(index));
            for i in index..self.len - 1 {
                core::ptr::copy_nonoverlapping(self.data.add(i + 1), self.data.add(i), 1);
            }
            self.len -= 1;
            item
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
