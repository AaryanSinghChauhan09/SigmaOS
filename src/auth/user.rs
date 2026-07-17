#![no_std]
#![no_main]

/// OOP-based User Authentication for SigmaOS
/// Based on Roadmap Item 13: User authentication

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type UserID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
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
            password_hash: hash_array,
            state: AtomicUsize::new(UserState::Active as usize),
        }
    }
}

impl User for SimpleUser {
    fn id(&self) -> UserID { self.id }
    fn username(&self) -> &[u8] {
        let len = self.username.iter().position(|&b| b == 0).unwrap_or(32);
        &self.username[..len]
    }
    fn state(&self) -> UserState { unsafe { core::mem::transmute(self.state.load(Ordering::SeqCst)) } }
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
