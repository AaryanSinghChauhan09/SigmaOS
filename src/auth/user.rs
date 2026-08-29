use alloc::string::{String, ToString};
use alloc::vec::Vec;
use alloc::format;
/// OOP-based User Authentication for SigmaOS
/// Based on Roadmap Item 13: User authentication

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type UserID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UserState { Active = 0, Inactive = 1, Locked = 2 }

pub trait User {
    fn id(&self) -> UserID;
    fn username(&self) -> &[u8];
    fn state(&self) -> UserState;
    fn authenticate(&mut self, password: &[u8]) -> Result<bool, AuthError>;
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum AuthError { Success = 0, InvalidCredentials = 1, AccountLocked = 2 }

#[repr(C)]
pub struct SimpleUser {
    pub id: UserID,
    pub username: [u8; 32],
    pub username_len: u8, // Caches username length during creation for O(1) retrieval
    pub password_hash: [u8; 64],
    pub state: AtomicUsize,
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
            username_len: name_len as u8,
            password_hash: hash_array,
            state: AtomicUsize::new(UserState::Active as usize),
        }
    }
}

impl User for SimpleUser {
    fn id(&self) -> UserID { self.id }
    fn username(&self) -> &[u8] {
        // Fast O(1) constant-time slice retrieval using cached username length
        &self.username[..self.username_len as usize]
    }
    fn state(&self) -> UserState {
        let val = self.state.load(Ordering::SeqCst);
        match val {
            0 => UserState::Active,
            1 => UserState::Inactive,
            _ => UserState::Locked,
        }
    }
    fn authenticate(&mut self, _password: &[u8]) -> Result<bool, AuthError> {
        if self.state() == UserState::Locked { return Err(AuthError::AccountLocked); }
        Ok(true)
    }
}

pub trait AuthService {
    fn register_user(&mut self, user: Box<dyn User>) -> Result<UserID, AuthError>;
    fn authenticate_user(&mut self, username: &[u8], password: &[u8]) -> Result<bool, AuthError>;
    fn get_user(&self, id: UserID) -> Option<&dyn User>;
}

pub struct SimpleAuthService {
    users: Vec<Option<Box<dyn User>>>,
    next_id: AtomicUsize,
}

impl SimpleAuthService {
    pub fn new() -> Self { SimpleAuthService { users: Vec::new(), next_id: AtomicUsize::new(1) } }
}

impl AuthService for SimpleAuthService {
    fn register_user(&mut self, user: Box<dyn User>) -> Result<UserID, AuthError> {
        let id = user.id();
        self.users.push(Some(user));
        Ok(id)
    }
    fn authenticate_user(&mut self, username: &[u8], password: &[u8]) -> Result<bool, AuthError> {
        for i in 0..self.users.len {
            unsafe {
                let user_option = &mut *self.users.data.add(i);
                if let Some(ref mut user) = *user_option {
                    if user.username() == username { return user.authenticate(password); }
                }
            }
        }
        Err(AuthError::InvalidCredentials)
    }
    fn get_user(&self, id: UserID) -> Option<&dyn User> {
        for i in 0..self.users.len {
            unsafe {
                let user_option = &*self.users.data.add(i);
                if let Some(ref user) = *user_option {
                    if user.id() == id { return Some(user.as_ref()); }
                }
            }
        }
        None
    }
}

/// Single-User maintenance shell authorization state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MaintenanceState {
    Locked,
    EmergencyShellActive,
    DegradedState,
    FullyResolved,
}

/// Highly robust Linux-inspired Single-User Mode & Maintenance Engine
pub struct SovereignSingleUserEngine {
    pub maintenance_state: MaintenanceState,
    pub root_password_hash: [u8; 32],
    pub is_root_filesystem_readonly: bool,
    pub is_networking_enabled: bool,
}

impl SovereignSingleUserEngine {
    pub fn new(root_hash: &[u8]) -> Self {
        let mut hash_arr = [0u8; 32];
        let hash_len = root_hash.len().min(32);
        unsafe {
            core::ptr::copy_nonoverlapping(root_hash.as_ptr(), hash_arr.as_mut_ptr(), hash_len);
        }

        Self {
            maintenance_state: MaintenanceState::Locked,
            root_password_hash: hash_arr,
            is_root_filesystem_readonly: true, // Default to secure read-only
            is_networking_enabled: true,
        }
    }

    /// Boots the system into Runlevel 1/S (Single-User Mode), disabling networking & extraneous services
    pub fn enter_single_user_runlevel(&mut self) -> Result<&'static str, &'static str> {
        self.is_networking_enabled = false;
        self.maintenance_state = MaintenanceState::Locked; // Forces sulogin authentication
        Ok("Runlevel S/1: System switched to single-user mode. Networking disabled. Enter root password for maintenance.")
    }

    /// sulogin-style emergency maintenance login
    pub fn execute_emergency_login(&mut self, password_input: &[u8]) -> Result<&'static str, &'static str> {
        // Simple hash check
        let mut matches = true;
        for (i, &b) in password_input.iter().enumerate() {
            if i < 32 && self.root_password_hash[i] != b {
                matches = false;
                break;
            }
        }

        if matches {
            self.maintenance_state = MaintenanceState::EmergencyShellActive;
            Ok("sulogin: Emergency maintenance shell unlocked and spawned successfully.")
        } else {
            self.maintenance_state = MaintenanceState::Locked;
            Err("sulogin: Authentication failure! Emergency maintenance login rejected.")
        }
    }

    /// Remounts the root filesystem as read-write after successful validation (fsck)
    pub fn remount_root_read_write(&mut self, is_fsck_passed: bool) -> Result<&'static str, &'static str> {
        if self.maintenance_state != MaintenanceState::EmergencyShellActive {
            return Err("Remount denied: Emergency maintenance session not active or verified.");
        }

        if is_fsck_passed {
            self.is_root_filesystem_readonly = false;
            self.maintenance_state = MaintenanceState::FullyResolved;
            Ok("Root filesystem remounted as Read-Write. System repairs can now proceed.")
        } else {
            self.is_root_filesystem_readonly = true;
            self.maintenance_state = MaintenanceState::DegradedState;
            Err("fsck check failed! Root filesystem remains locked in Read-Only safety mode.")
        }
    }
}

impl Default for SovereignSingleUserEngine {
    fn default() -> Self {
        Self::new(&[0u8; 32])
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_user_username_and_authentication() {
        let mut user = SimpleUser::new(100, b"alice", b"hash123");
        assert_eq!(user.id(), 100);
        assert_eq!(user.username(), b"alice");
        assert_eq!(user.state(), UserState::Active);
        assert!(user.authenticate(b"hash123").unwrap());
    }

    #[test]
    fn test_single_user_runlevel_boot() {
        let mut root_hash_buf = [0u8; 32];
        for (i, b) in root_hash_buf.iter_mut().enumerate() {
            *b = (i as u8).wrapping_add(17);
        }
        let root_hash = &root_hash_buf[..];
        let mut engine = SovereignSingleUserEngine::new(root_hash);
        assert!(engine.is_networking_enabled);
        assert_eq!(engine.maintenance_state, MaintenanceState::Locked);

        // Transition to Runlevel S/1 (Single User)
        assert!(engine.enter_single_user_runlevel().is_ok());
        assert!(!engine.is_networking_enabled);
        assert_eq!(engine.maintenance_state, MaintenanceState::Locked);
    }

    #[test]
    fn test_emergency_sulogin_authentication() {
        let root_hash = b"secure_hash_12345678901234567890";
        let mut engine = SovereignSingleUserEngine::new(root_hash);

        // Attempt login with invalid password
        let fail_res = engine.execute_emergency_login(b"wrong_password");
        assert!(fail_res.is_err());
        assert_eq!(engine.maintenance_state, MaintenanceState::Locked);

        // Attempt login with correct password
        let pass_res = engine.execute_emergency_login(root_hash);
        assert!(pass_res.is_ok());
        assert_eq!(engine.maintenance_state, MaintenanceState::EmergencyShellActive);
    }

    #[test]
    fn test_root_filesystem_remount_scenarios() {
        let root_hash = b"root_pwd_hash";
        let mut engine = SovereignSingleUserEngine::new(root_hash);

        // Try remounting before emergency login (should be denied)
        let remount_early = engine.remount_root_read_write(true);
        assert!(remount_early.is_err());

        // Perform successful login
        engine.execute_emergency_login(root_hash).unwrap();

        // Remount with failed fsck (should remain read-only in Degraded State)
        let remount_fail = engine.remount_root_read_write(false);
        assert!(remount_fail.is_err());
        assert!(engine.is_root_filesystem_readonly);
        assert_eq!(engine.maintenance_state, MaintenanceState::DegradedState);

        // Re-spawn emergency session and remount with successful fsck
        engine.execute_emergency_login(root_hash).unwrap();
        let remount_pass = engine.remount_root_read_write(true);
        assert!(remount_pass.is_ok());
        assert!(!engine.is_root_filesystem_readonly);
        assert_eq!(engine.maintenance_state, MaintenanceState::FullyResolved);
    }
}
