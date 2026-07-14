#![no_std]
#![no_main]

/// OOP-based Syscall Interface for SigmaOS
/// Based on Roadmap Item 11: Syscall interface

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type SyscallID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum SyscallType { Read = 0, Write = 1, Open = 2, Close = 3 }

pub trait Syscall {
    fn id(&self) -> SyscallID;
    fn syscall_type(&self) -> SyscallType;
    fn invoke(&mut self, args: &[usize]) -> Result<usize, SyscallError>;
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum SyscallError { Success = 0, InvalidArgs = 1, PermissionDenied = 2 }

#[repr(C)]
pub struct SimpleSyscall {
    pub id: SyscallID,
    pub syscall_type: SyscallType,
}

impl SimpleSyscall {
    pub fn new(id: SyscallID, syscall_type: SyscallType) -> Self {
        SimpleSyscall { id, syscall_type }
    }
}

impl Syscall for SimpleSyscall {
    fn id(&self) -> SyscallID { self.id }
    fn syscall_type(&self) -> SyscallType { self.syscall_type }
    fn invoke(&mut self, _args: &[usize]) -> Result<usize, SyscallError> { Ok(0) }
}

pub trait SyscallInterface {
    fn register_syscall(&mut self, syscall: Box<dyn Syscall>) -> Result<SyscallID, SyscallError>;
    fn invoke_syscall(&mut self, id: SyscallID, args: &[usize]) -> Result<usize, SyscallError>;
}

pub struct SimpleSyscallInterface {
    syscalls: Vec<Option<Box<dyn Syscall>>>,
    next_id: AtomicUsize,
}

impl SimpleSyscallInterface {
    pub fn new() -> Self { SimpleSyscallInterface { syscalls: Vec::new(), next_id: AtomicUsize::new(1) } }
}

impl SyscallInterface for SimpleSyscallInterface {
    fn register_syscall(&mut self, syscall: Box<dyn Syscall>) -> Result<SyscallID, SyscallError> {
        let id = syscall.id();
        self.syscalls.push(Some(syscall));
        Ok(id)
    }
    fn invoke_syscall(&mut self, id: SyscallID, args: &[usize]) -> Result<usize, SyscallError> {
        for syscall_option in &mut self.syscalls {
            if let Some(ref mut syscall) = *syscall_option {
                if syscall.id() == id { return syscall.invoke(args); }
            }
        }
        Err(SyscallError::InvalidArgs)
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
