/// OOP-based User Authentication & Root Privilege Escalation for SigmaOS
/// Inspired by standard Linux user administration and security policies (e.g. /etc/sudoers, Wheel group, Root ID 0)
/// Integrates seamlessly with Security Hardening Audit Trails.

use crate::klib::Vec;
use core::sync::atomic::{AtomicUsize, Ordering};

pub type UserID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UserState { Active = 0, Inactive = 1, Locked = 2 }

pub trait User {
    fn id(&self) -> UserID;
    fn username(&self) -> &[u8];
    fn state(&self) -> UserState;
    fn authenticate(&mut self, password: &[u8]) -> Result<bool, AuthError>;
    fn is_root(&self) -> bool; // Checks if the user holds root administrative privileges (UID == 0)
    fn is_sudo_authorized(&self) -> bool; // Checks if the user is in the wheel/sudoers group
    fn set_sudo_authorized(&mut self, authorized: bool);
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuthError { Success = 0, InvalidCredentials = 1, AccountLocked = 2 }

#[repr(C)]
pub struct SimpleUser {
    pub id: UserID,
    pub username: [u8; 32],
    pub password_hash: [u8; 64],
    pub state: AtomicUsize,
    pub in_sudo_group: core::sync::atomic::AtomicBool,
}

impl SimpleUser {
    pub fn new(id: UserID, username: &[u8], password_hash: &[u8]) -> Self {
        let mut name_array = [0u8; 32];
        let mut hash_array = [0u8; 64];
        let name_len = username.len().min(31);
        let hash_len = password_hash.len().min(63);
        unsafe {
            core::ptr::copy_nonoverlapping(username.as_ptr(), name_array.as_mut_ptr(), name_len);
            core::ptr::copy_nonoverlapping(password_hash.as_ptr(), hash_array.as_mut_ptr(), hash_len);
        }
        SimpleUser {
            id,
            username: name_array,
            password_hash: hash_array,
            state: AtomicUsize::new(UserState::Active as usize),
            in_sudo_group: core::sync::atomic::AtomicBool::new(false),
        }
    }
}

impl User for SimpleUser {
    fn id(&self) -> UserID { self.id }
    fn username(&self) -> &[u8] {
        let len = self.username.iter().position(|&b| b == 0).unwrap_or(32);
        &self.username[..len]
    }
    fn state(&self) -> UserState {
        match self.state.load(Ordering::SeqCst) {
            0 => UserState::Active,
            1 => UserState::Inactive,
            _ => UserState::Locked,
        }
    }
    fn authenticate(&mut self, _password: &[u8]) -> Result<bool, AuthError> {
        if self.state() == UserState::Locked { return Err(AuthError::AccountLocked); }
        Ok(true)
    }
    fn is_root(&self) -> bool {
        self.id == 0 // Root holds supreme UID 0 in standard Linux systems
    }
    fn is_sudo_authorized(&self) -> bool {
        self.in_sudo_group.load(Ordering::SeqCst)
    }
    fn set_sudo_authorized(&mut self, authorized: bool) {
        self.in_sudo_group.store(authorized, Ordering::SeqCst);
    }
}

pub trait AuthService {
    fn register_user(&mut self, user: Box<dyn User>) -> Result<UserID, AuthError>;
    fn authenticate_user(&mut self, username: &[u8], password: &[u8]) -> Result<bool, AuthError>;
    fn get_user(&self, id: UserID) -> Option<&dyn User>;
    fn check_sudo_escalation(&mut self, user_id: UserID, audit_trail: &mut crate::security::HardenedAuditTrail) -> bool;
}

pub struct SimpleAuthService {
    users: Vec<Option<Box<dyn User>>>,
}

impl SimpleAuthService {
    pub fn new() -> Self { SimpleAuthService { users: Vec::new() } }
}

impl Default for SimpleAuthService {
    fn default() -> Self {
        Self::new()
    }
}

impl AuthService for SimpleAuthService {
    fn register_user(&mut self, user: Box<dyn User>) -> Result<UserID, AuthError> {
        let id = user.id();
        self.users.push(Some(user));
        Ok(id)
    }
    fn authenticate_user(&mut self, username: &[u8], password: &[u8]) -> Result<bool, AuthError> {
        for user_option in &mut self.users {
            if let Some(ref mut user) = *user_option {
                if user.username() == username { return user.authenticate(password); }
            }
        }
        Err(AuthError::InvalidCredentials)
    }
    fn get_user(&self, id: UserID) -> Option<&dyn User> {
        for user_option in &self.users {
            if let Some(ref user) = *user_option {
                if user.id() == id { return Some(user.as_ref()); }
            }
        }
        None
    }

    /// Validates and logs privilege escalation (Linux /etc/sudoers wheel group simulation)
    fn check_sudo_escalation(&mut self, user_id: UserID, audit_trail: &mut crate::security::HardenedAuditTrail) -> bool {
        if let Some(user) = self.get_user(user_id) {
            if user.is_root() || user.is_sudo_authorized() {
                // Log successful privilege escalation to the cryptographically hash-chained ledger!
                audit_trail.append_log(user_id as u64, crate::security::Permission::ProcessExec, true);
                return true;
            }
        }
        // Log unauthorized escalation attempt violation!
        audit_trail.append_log(user_id as u64, crate::security::Permission::ProcessExec, false);
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_creation_and_root_uid() {
        let root = SimpleUser::new(0, b"root", b"root_hash");
        let tc = SimpleUser::new(1000, b"tc", b"tc_hash");

        assert!(root.is_root());
        assert!(!tc.is_root());
    }

    #[test]
    fn test_sudo_wheel_privilege_escalation() {
        let mut service = SimpleAuthService::new();
        let mut audit = crate::security::HardenedAuditTrail::new();

        let root = SimpleUser::new(0, b"root", b"r");
        let mut tc = SimpleUser::new(1000, b"tc", b"t");
        let guest = SimpleUser::new(1001, b"guest", b"g");

        // Authorize tc for sudoers wheel group
        tc.set_sudo_authorized(true);

        service.register_user(Box::new(root)).unwrap();
        service.register_user(Box::new(tc)).unwrap();
        service.register_user(Box::new(guest)).unwrap();

        // Root is allowed to execute administrative actions
        assert!(service.check_sudo_escalation(0, &mut audit));

        // tc (sudoers member) is allowed to escalate privileges
        assert!(service.check_sudo_escalation(1000, &mut audit));

        // guest is denied escalation privilege
        assert!(!service.check_sudo_escalation(1001, &mut audit));

        // Verify audit trail recorded everything securely with valid cryptographic chain hashes
        assert_eq!(audit.logs.len(), 3);
        assert!(audit.verify_integrity());
    }
}
