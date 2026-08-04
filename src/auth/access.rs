#![allow(unused_variables)]
/// OOP-based Access Control System for SigmaOS
/// Based on Roadmap Item 14: Access control system

use crate::klib::Vec;
use core::sync::atomic::AtomicUsize;

pub type PermissionID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PermissionType { Read = 0, Write = 1, Execute = 2, Admin = 3 }

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccessError { Success = 0, PermissionNotFound = 1, AccessDenied = 2 }

pub struct SimpleAccessControl {
    user_permissions: Vec<Vec<(PermissionID, PermissionType, [u8; 64])>>,
    pub next_id: AtomicUsize,
}

impl SimpleAccessControl {
    pub fn new() -> Self { SimpleAccessControl { user_permissions: Vec::new(), next_id: AtomicUsize::new(1) } }
}

impl Default for SimpleAccessControl {
    fn default() -> Self {
        Self::new()
    }
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
                if &res[..res_len] == resource {
                    let _id = id;
                    return AccessResult::Granted;
                }
            }
        }
        AccessResult::Denied
    }
}
