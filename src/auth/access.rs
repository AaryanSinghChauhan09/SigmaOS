#![no_std]
#![no_main]

/// OOP-based Access Control System for SigmaOS
/// Based on Roadmap Item 14: Access control system

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type PermissionID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum PermissionType { Read = 0, Write = 1, Execute = 2, Admin = 3 }

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum AccessResult { Granted = 0, Denied = 1 }

pub trait Permission {
    fn id(&self) -> PermissionID;
    fn permission_type(&self) -> PermissionType;
    fn resource(&self) -> &[u8];
}

#[repr(C)]
pub struct SimplePermission {
    pub id: PermissionID,
    pub permission_type: PermissionType,
    pub resource: [u8; 64],
}

impl SimplePermission {
    pub fn new(id: PermissionID, permission_type: PermissionType, resource: &[u8]) -> Self {
        let mut resource_array = [0u8; 64];
        let resource_len = resource.len().min(63);
        unsafe {
            core::ptr::copy_nonoverlapping(resource.as_ptr(), resource_array.as_mut_ptr(), resource_len);
        }
        SimplePermission { id, permission_type, resource: resource_array }
    }
}

impl Permission for SimplePermission {
    fn id(&self) -> PermissionID { self.id }
    fn permission_type(&self) -> PermissionType { self.permission_type }
    fn resource(&self) -> &[u8] {
        let len = self.resource.iter().position(|&b| b == 0).unwrap_or(64);
        &self.resource[..len]
    }
}

pub trait AccessControl {
    fn grant_permission(&mut self, user_id: usize, permission: Box<dyn Permission>) -> Result<(), AccessError>;
    fn revoke_permission(&mut self, user_id: usize, permission_id: PermissionID) -> Result<(), AccessError>;
    fn check_access(&self, user_id: usize, resource: &[u8], permission_type: PermissionType) -> AccessResult;
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum AccessError { Success = 0, PermissionNotFound = 1, AccessDenied = 2 }

pub struct SimpleAccessControl {
    user_permissions: Vec<Vec<(PermissionID, PermissionType, [u8; 64])>>,
    next_id: AtomicUsize,
}

impl SimpleAccessControl {
    pub fn new() -> Self { SimpleAccessControl { user_permissions: Vec::new(), next_id: AtomicUsize::new(1) } }
}

impl AccessControl for SimpleAccessControl {
    fn grant_permission(&mut self, user_id: usize, permission: Box<dyn Permission>) -> Result<(), AccessError> {
        let id = permission.id();
        let perm_type = permission.permission_type();
        let mut resource_array = [0u8; 64];
        let resource = permission.resource();
        let resource_len = resource.len().min(63);
        unsafe {
            core::ptr::copy_nonoverlapping(resource.as_ptr(), resource_array.as_mut_ptr(), resource_len);
        }
        
        while user_id >= self.user_permissions.len() {
            self.user_permissions.push(Vec::new());
        }
        self.user_permissions[user_id].push((id, perm_type, resource_array));
        Ok(())
    }
    fn revoke_permission(&mut self, user_id: usize, permission_id: PermissionID) -> Result<(), AccessError> {
        if user_id >= self.user_permissions.len() { return Err(AccessError::PermissionNotFound); }
        let permissions = &mut self.user_permissions[user_id];
        for i in 0..permissions.len() {
            if permissions[i].0 == permission_id {
                permissions.remove(i);
                return Ok(());
            }
        }
        Err(AccessError::PermissionNotFound)
    }
    fn check_access(&self, user_id: usize, resource: &[u8], permission_type: PermissionType) -> AccessResult {
        if user_id >= self.user_permissions.len() { return AccessResult::Denied; }
        for (id, perm_type, res) in &self.user_permissions[user_id] {
            if *perm_type == permission_type {
                let res_len = res.iter().position(|&b| b == 0).unwrap_or(64);
                if &res[..res_len] == resource { return AccessResult::Granted; }
            }
        }
        AccessResult::Denied
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
    fn len(&self) -> usize { self.len }
    fn remove(&mut self, index: usize) {
        unsafe {
            for i in index..self.len-1 {
                core::ptr::copy(self.data.add(i+1), self.data.add(i), 1);
            }
            self.len -= 1;
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
