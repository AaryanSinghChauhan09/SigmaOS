#![no_std]
#![no_main]

/// OOP-based Hypervisor for SigmaOS
/// Based on Ideas-999-Structured: Kernel & Hardware Item 181
/// Implements virtualization and guest management

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type GuestID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum GuestState { Stopped = 0, Running = 1, Paused = 2, Crashed = 3 }

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum HypervisorError { Success = 0, NotFound = 1, StartFailed = 2, InvalidConfig = 3 }

pub trait Guest {
    fn id(&self) -> GuestID;
    fn name(&self) -> &[u8];
    fn state(&self) -> GuestState;
    fn vcpus(&self) -> u32;
    fn memory_mb(&self) -> u32;
}

#[repr(C)]
pub struct SimpleGuest {
    pub id: GuestID,
    pub name: [u8; 64],
    pub state: AtomicUsize,
    pub vcpus: AtomicUsize,
    pub memory_mb: AtomicUsize,
}

impl SimpleGuest {
    pub fn new(id: GuestID, name: &[u8], vcpus: u32, memory_mb: u32) -> Self {
        let mut name_array = [0u8; 64];
        let name_len = name.len().min(63);
        unsafe {
            core::ptr::copy_nonoverlapping(name.as_ptr(), name_array.as_mut_ptr(), name_len);
        }
        SimpleGuest {
            id,
            name: name_array,
            state: AtomicUsize::new(GuestState::Stopped as usize),
            vcpus: AtomicUsize::new(vcpus as usize),
            memory_mb: AtomicUsize::new(memory_mb as usize),
        }
    }
}

impl Guest for SimpleGuest {
    fn id(&self) -> GuestID { self.id }
    fn name(&self) -> &[u8] {
        let len = self.name.iter().position(|&b| b == 0).unwrap_or(64);
        &self.name[..len]
    }
    fn state(&self) -> GuestState { unsafe { core::mem::transmute(self.state.load(Ordering::SeqCst)) } }
    fn vcpus(&self) -> u32 { self.vcpus.load(Ordering::SeqCst) as u32 }
    fn memory_mb(&self) -> u32 { self.memory_mb.load(Ordering::SeqCst) as u32 }
}

pub trait Hypervisor {
    fn create_guest(&mut self, name: &[u8], vcpus: u32, memory_mb: u32) -> Result<GuestID, HypervisorError>;
    fn destroy_guest(&mut self, id: GuestID) -> Result<(), HypervisorError>;
    fn start_guest(&mut self, id: GuestID) -> Result<(), HypervisorError>;
    fn stop_guest(&mut self, id: GuestID) -> Result<(), HypervisorError>;
    fn pause_guest(&mut self, id: GuestID) -> Result<(), HypervisorError>;
    fn resume_guest(&mut self, id: GuestID) -> Result<(), HypervisorError>;
}

#[repr(C)]
pub struct SimpleHypervisor {
    pub guests: Vec<Option<Box<dyn Guest>>>,
    pub next_id: AtomicUsize,
}

impl SimpleHypervisor {
    pub fn new() -> Self {
        SimpleHypervisor {
            guests: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl Hypervisor for SimpleHypervisor {
    fn create_guest(&mut self, name: &[u8], vcpus: u32, memory_mb: u32) -> Result<GuestID, HypervisorError> {
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        let guest = SimpleGuest::new(id, name, vcpus, memory_mb);
        self.guests.push(Some(Box::new(guest)));
        Ok(id)
    }
    
    fn destroy_guest(&mut self, id: GuestID) -> Result<(), HypervisorError> {
        for guest_option in &mut self.guests {
            if let Some(ref guest) = *guest_option {
                if guest.id() == id {
                    return Ok(());
                }
            }
        }
        Err(HypervisorError::NotFound)
    }
    
    fn start_guest(&mut self, id: GuestID) -> Result<(), HypervisorError> {
        for guest_option in &mut self.guests {
            if let Some(ref mut guest) = *guest_option {
                if guest.id() == id {
                    guest.state.store(GuestState::Running as usize, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(HypervisorError::NotFound)
    }
    
    fn stop_guest(&mut self, id: GuestID) -> Result<(), HypervisorError> {
        for guest_option in &mut self.guests {
            if let Some(ref mut guest) = *guest_option {
                if guest.id() == id {
                    guest.state.store(GuestState::Stopped as usize, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(HypervisorError::NotFound)
    }
    
    fn pause_guest(&mut self, id: GuestID) -> Result<(), HypervisorError> {
        for guest_option in &mut self.guests {
            if let Some(ref mut guest) = *guest_option {
                if guest.id() == id {
                    guest.state.store(GuestState::Paused as usize, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(HypervisorError::NotFound)
    }
    
    fn resume_guest(&mut self, id: GuestID) -> Result<(), HypervisorError> {
        for guest_option in &mut self.guests {
            if let Some(ref mut guest) = *guest_option {
                if guest.id() == id {
                    guest.state.store(GuestState::Running as usize, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(HypervisorError::NotFound)
    }
}

pub trait VMExitHandler {
    fn handle_exit(&mut self, guest_id: GuestID, exit_reason: u64) -> Result<(), HypervisorError>;
    fn register_handler(&mut self, exit_reason: u64, handler: fn(GuestID, u64));
}

#[repr(C)]
pub struct SimpleVMExitHandler {
    pub handlers: Vec<(u64, fn(GuestID, u64))>,
}

impl SimpleVMExitHandler {
    pub fn new() -> Self {
        SimpleVMExitHandler {
            handlers: Vec::new(),
        }
    }
}

impl VMExitHandler for SimpleVMExitHandler {
    fn handle_exit(&mut self, guest_id: GuestID, exit_reason: u64) -> Result<(), HypervisorError> {
        for &(reason, handler) in &self.handlers {
            if reason == exit_reason {
                handler(guest_id, exit_reason);
                return Ok(());
            }
        }
        Err(HypervisorError::InvalidConfig)
    }
    
    fn register_handler(&mut self, exit_reason: u64, handler: fn(GuestID, u64)) {
        self.handlers.push((exit_reason, handler));
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
