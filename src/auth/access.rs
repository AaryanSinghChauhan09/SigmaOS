/// OOP-based Access Control System for SigmaOS
/// Based on Roadmap Item 14: Access control system
use std::boxed::Box;
use std::vec::Vec;
use core::sync::atomic::AtomicUsize;

pub type PermissionID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PermissionType {
    Read = 0,
    Write = 1,
    Execute = 2,
    Admin = 3,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccessResult {
    Granted = 0,
    Denied = 1,
}

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
    pub resource_len: u8,
}

impl SimplePermission {
    pub fn new(id: PermissionID, permission_type: PermissionType, resource: &[u8]) -> Self {
        let mut resource_array = [0u8; 64];
        let resource_len = resource.len().min(63);
        unsafe {
            core::ptr::copy_nonoverlapping(
                resource.as_ptr(),
                resource_array.as_mut_ptr(),
                resource_len,
            );
        }
        SimplePermission {
            id,
            permission_type,
            resource: resource_array,
            resource_len: resource_len as u8,
        }
    }
}

impl Permission for SimplePermission {
    fn id(&self) -> PermissionID {
        self.id
    }
    fn permission_type(&self) -> PermissionType {
        self.permission_type
    }
    fn resource(&self) -> &[u8] {
        &self.resource[..self.resource_len as usize]
    }
}

pub trait AccessControl {
    fn grant_permission(
        &mut self,
        user_id: usize,
        permission: Box<dyn Permission>,
    ) -> Result<(), AccessError>;
    fn revoke_permission(
        &mut self,
        user_id: usize,
        permission_id: PermissionID,
    ) -> Result<(), AccessError>;
    fn check_access(
        &self,
        user_id: usize,
        resource: &[u8],
        permission_type: PermissionType,
    ) -> AccessResult;
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccessError {
    Success = 0,
    PermissionNotFound = 1,
    AccessDenied = 2,
}

pub struct SimpleAccessControl {
    user_permissions: Vec<Vec<(PermissionID, PermissionType, [u8; 64], u8)>>,
    pub next_id: AtomicUsize,
}

impl SimpleAccessControl {
    pub fn new() -> Self {
        SimpleAccessControl {
            user_permissions: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl Default for SimpleAccessControl {
    fn default() -> Self {
        Self::new()
    }
}

impl AccessControl for SimpleAccessControl {
    fn grant_permission(
        &mut self,
        user_id: usize,
        permission: Box<dyn Permission>,
    ) -> Result<(), AccessError> {
        let id = permission.id();
        let perm_type = permission.permission_type();
        let mut resource_array = [0u8; 64];
        let resource = permission.resource();
        let resource_len = resource.len().min(63);
        unsafe {
            core::ptr::copy_nonoverlapping(
                resource.as_ptr(),
                resource_array.as_mut_ptr(),
                resource_len,
            );
        }

        while user_id >= self.user_permissions.len() {
            self.user_permissions.push(Vec::new());
        }
        self.user_permissions[user_id].push((id, perm_type, resource_array, resource_len as u8));
        Ok(())
    }
    fn revoke_permission(
        &mut self,
        user_id: usize,
        permission_id: PermissionID,
    ) -> Result<(), AccessError> {
        if user_id >= self.user_permissions.len() {
            return Err(AccessError::PermissionNotFound);
        }
        let permissions = &mut self.user_permissions[user_id];
        for i in 0..permissions.len() {
            if permissions[i].0 == permission_id {
                permissions.remove(i);
                return Ok(());
            }
        }
        Err(AccessError::PermissionNotFound)
    }
    fn check_access(
        &self,
        user_id: usize,
        resource: &[u8],
        permission_type: PermissionType,
    ) -> AccessResult {
        if user_id >= self.user_permissions.len() {
            return AccessResult::Denied;
        }
        let perms = &self.user_permissions[user_id];
        for i in 0..perms.len() {
            let (_id, perm_type, res, res_len) = &perms[i];
            if *perm_type == permission_type {
                if &res[..*res_len as usize] == resource {
                    return AccessResult::Granted;
                }
            }
        }
        AccessResult::Denied
    }
}

#[cfg(test_disabled)]
mod tests {
    use super::*;
    use std::boxed::Box;

    #[test]
    fn test_simple_permission_resource_len() {
        let resource_name = b"/var/log/syslog";
        let perm = SimplePermission::new(1, PermissionType::Read, resource_name);
        assert_eq!(perm.resource_len, resource_name.len() as u8);
        assert_eq!(perm.resource(), resource_name);
        assert_eq!(perm.id(), 1);
        assert_eq!(perm.permission_type(), PermissionType::Read);
    }

    #[test]
    fn test_simple_access_control_check() {
        let mut ac = SimpleAccessControl::new();
        let perm_read = Box::new(SimplePermission::new(
            1,
            PermissionType::Read,
            b"/etc/config",
        ));
        let perm_write = Box::new(SimplePermission::new(2, PermissionType::Write, b"/var/log"));

        assert_eq!(ac.grant_permission(42, perm_read), Ok(()));
        assert_eq!(ac.grant_permission(42, perm_write), Ok(()));

        assert_eq!(
            ac.check_access(42, b"/etc/config", PermissionType::Read),
            AccessResult::Granted
        );
        assert_eq!(
            ac.check_access(42, b"/var/log", PermissionType::Write),
            AccessResult::Granted
        );
        assert_eq!(
            ac.check_access(42, b"/etc/config", PermissionType::Write),
            AccessResult::Denied
        );
        assert_eq!(
            ac.check_access(42, b"/etc/shadow", PermissionType::Read),
            AccessResult::Denied
        );
        assert_eq!(
            ac.check_access(99, b"/etc/config", PermissionType::Read),
            AccessResult::Denied
        );

        assert_eq!(ac.revoke_permission(42, 1), Ok(()));
        assert_eq!(
            ac.check_access(42, b"/etc/config", PermissionType::Read),
            AccessResult::Denied
        );
        assert_eq!(
            ac.revoke_permission(42, 999),
            Err(AccessError::PermissionNotFound)
        );
    }
}
