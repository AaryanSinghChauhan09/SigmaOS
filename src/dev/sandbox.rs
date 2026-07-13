#![no_std]
#![no_main]

/// OOP-based Dev Sandbox Manager for SigmaOS
/// Implements sandbox management using OOP principles with traits and structs
/// No dependency on external sandbox frameworks
/// Based on Roadmap Item 88: Dev sandbox manager

use core::ptr::{self, NonNull};
use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

/// Sandbox ID
pub type SandboxID = usize;

/// Sandbox state
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum SandboxState {
    Creating = 0,
    Running = 1,
    Paused = 2,
    Stopped = 3,
    Failed = 4,
}

/// Sandbox trait (OOP interface)
pub trait Sandbox {
    /// Get sandbox ID
    fn id(&self) -> SandboxID;
    /// Get sandbox name
    fn name(&self) -> &[u8];
    /// Start sandbox
    fn start(&mut self) -> Result<(), SandboxError>;
    /// Stop sandbox
    fn stop(&mut self) -> Result<(), SandboxError>;
    /// Pause sandbox
    fn pause(&mut self) -> Result<(), SandboxError>;
    /// Resume sandbox
    fn resume(&mut self) -> Result<(), SandboxError>;
    /// Get sandbox state
    fn state(&self) -> SandboxState;
    /// Get sandbox info
    fn info(&self) -> SandboxInfo;
}

/// Sandbox error types
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum SandboxError {
    Success = 0,
    AlreadyRunning = 1,
    AlreadyStopped = 2,
    StartFailed = 3,
    StopFailed = 4,
    PermissionDenied = 5,
}

/// Sandbox info
#[repr(C)]
pub struct SandboxInfo {
    pub id: SandboxID,
    pub name: [u8; 64],
    pub state: SandboxState,
    pub memory_limit: u64,
    pub cpu_limit: u32,
    pub capability: SandboxCapability,
}

impl SandboxInfo {
    pub fn new(id: SandboxID) -> Self {
        SandboxInfo {
            id,
            name: [0; 64],
            state: SandboxState::Creating,
            memory_limit: 0,
            cpu_limit: 0,
            capability: SandboxCapability::new(),
        }
    }
}

/// Sandbox capability
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SandboxCapability {
    pub can_start: bool,
    pub can_stop: bool,
    pub can_modify: bool,
}

impl SandboxCapability {
    pub fn new() -> Self {
        SandboxCapability {
            can_start: false,
            can_stop: false,
            can_modify: false,
        }
    }

    pub fn full() -> Self {
        SandboxCapability {
            can_start: true,
            can_stop: true,
            can_modify: true,
        }
    }
}

/// Simple sandbox (OOP: Concrete sandbox class)
#[repr(C)]
pub struct SimpleSandbox {
    pub id: SandboxID,
    pub name: [u8; 64],
    pub state: AtomicUsize, // SandboxState as usize
    pub memory_limit: u64,
    pub cpu_limit: u32,
    pub capability: SandboxCapability,
    pub environment: [u8; 512],
}

impl SimpleSandbox {
    pub fn new(id: SandboxID, name: &[u8], capability: SandboxCapability) -> Self {
        let mut name_array = [0u8; 64];
        let name_len = name.len().min(63);

        unsafe {
            core::ptr::copy_nonoverlapping(name.as_ptr(), name_array.as_mut_ptr(), name_len);
        }

        SimpleSandbox {
            id,
            name: name_array,
            state: AtomicUsize::new(SandboxState::Creating as usize),
            memory_limit: 0,
            cpu_limit: 0,
            capability,
            environment: [0; 512],
        }
    }

    pub fn set_environment(&mut self, env: &[u8]) {
        let len = env.len().min(511);
        unsafe {
            core::ptr::copy_nonoverlapping(env.as_ptr(), self.environment.as_mut_ptr(), len);
        }
    }

    pub fn set_limits(&mut self, memory_limit: u64, cpu_limit: u32) {
        self.memory_limit = memory_limit;
        self.cpu_limit = cpu_limit;
    }

    pub fn get_state(&self) -> SandboxState {
        unsafe {
            core::mem::transmute(self.state.load(Ordering::SeqCst))
        }
    }

    pub fn set_state(&self, state: SandboxState) {
        self.state.store(state as usize, Ordering::SeqCst);
    }
}

impl Sandbox for SimpleSandbox {
    fn id(&self) -> SandboxID {
        self.id
    }

    fn name(&self) -> &[u8] {
        let len = self.name.iter().position(|&b| b == 0).unwrap_or(64);
        &self.name[..len]
    }

    fn start(&mut self) -> Result<(), SandboxError> {
        if !self.capability.can_start {
            return Err(SandboxError::PermissionDenied);
        }

        let current_state = self.get_state();
        if current_state == SandboxState::Running {
            return Err(SandboxError::AlreadyRunning);
        }

        self.set_state(SandboxState::Running);
        Ok(())
    }

    fn stop(&mut self) -> Result<(), SandboxError> {
        if !self.capability.can_stop {
            return Err(SandboxError::PermissionDenied);
        }

        let current_state = self.get_state();
        if current_state == SandboxState::Stopped {
            return Err(SandboxError::AlreadyStopped);
        }

        self.set_state(SandboxState::Stopped);
        Ok(())
    }

    fn pause(&mut self) -> Result<(), SandboxError> {
        let current_state = self.get_state();
        if current_state != SandboxState::Running {
            return Err(SandboxError::AlreadyStopped);
        }

        self.set_state(SandboxState::Paused);
        Ok(())
    }

    fn resume(&mut self) -> Result<(), SandboxError> {
        let current_state = self.get_state();
        if current_state != SandboxState::Paused {
            return Err(SandboxError::AlreadyStopped);
        }

        self.set_state(SandboxState::Running);
        Ok(())
    }

    fn state(&self) -> SandboxState {
        self.get_state()
    }

    fn info(&self) -> SandboxInfo {
        SandboxInfo {
            id: self.id,
            name: self.name,
            state: self.get_state(),
            memory_limit: self.memory_limit,
            cpu_limit: self.cpu_limit,
            capability: self.capability,
        }
    }
}

/// Sandbox manager trait (OOP interface)
pub trait SandboxManager {
    /// Create sandbox
    fn create_sandbox(&mut self, name: &[u8], capability: SandboxCapability) -> Result<SandboxID, SandboxError>;
    /// Destroy sandbox
    fn destroy_sandbox(&mut self, id: SandboxID) -> Result<(), SandboxError>;
    /// Start sandbox
    fn start_sandbox(&mut self, id: SandboxID) -> Result<(), SandboxError>;
    /// Stop sandbox
    fn stop_sandbox(&mut self, id: SandboxID) -> Result<(), SandboxError>;
    /// Get sandbox
    fn get_sandbox(&self, id: SandboxID) -> Option<&dyn Sandbox>;
    /// List sandboxes
    fn list_sandboxes(&self) -> Vec<SandboxID>;
    /// Get manager statistics
    fn stats(&self) -> SandboxStats;
}

/// Sandbox statistics
#[repr(C)]
pub struct SandboxStats {
    pub total_sandboxes: usize,
    pub running_sandboxes: usize,
    pub paused_sandboxes: usize,
    pub stopped_sandboxes: usize,
}

impl SandboxStats {
    pub fn new() -> Self {
        SandboxStats {
            total_sandboxes: 0,
            running_sandboxes: 0,
            paused_sandboxes: 0,
            stopped_sandboxes: 0,
        }
    }
}

/// Simple sandbox manager (OOP: Concrete manager class)
pub struct SimpleSandboxManager {
    sandboxes: Vec<Option<Box<dyn Sandbox>>>,
    next_id: AtomicUsize,
    stats: SandboxStats,
    capability: ManagerCapability,
}

/// Manager capability
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ManagerCapability {
    pub can_create: bool,
    pub can_destroy: bool,
    pub can_manage: bool,
}

impl ManagerCapability {
    pub fn new() -> Self {
        ManagerCapability {
            can_create: false,
            can_destroy: false,
            can_manage: false,
        }
    }

    pub fn full() -> Self {
        ManagerCapability {
            can_create: true,
            can_destroy: true,
            can_manage: true,
        }
    }
}

impl SimpleSandboxManager {
    pub fn new(capability: ManagerCapability) -> Self {
        SimpleSandboxManager {
            sandboxes: Vec::new(),
            next_id: AtomicUsize::new(1),
            stats: SandboxStats::new(),
            capability,
        }
    }
}

impl SandboxManager for SimpleSandboxManager {
    fn create_sandbox(&mut self, name: &[u8], capability: SandboxCapability) -> Result<SandboxID, SandboxError> {
        if !self.capability.can_create {
            return Err(SandboxError::PermissionDenied);
        }

        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        let sandbox = SimpleSandbox::new(id, name, capability);
        self.sandboxes.push(Some(Box::new(sandbox)));
        self.stats.total_sandboxes += 1;
        self.stats.stopped_sandboxes += 1;
        Ok(id)
    }

    fn destroy_sandbox(&mut self, id: SandboxID) -> Result<(), SandboxError> {
        if !self.capability.can_destroy {
            return Err(SandboxError::PermissionDenied);
        }

        let mut index = None;
        for (i, sandbox_option) in self.sandboxes.iter().enumerate() {
            if let Some(ref sandbox) = *sandbox_option {
                if sandbox.id() == id {
                    index = Some(i);
                    break;
                }
            }
        }

        if let Some(i) = index {
            self.sandboxes[i] = None;
            self.stats.total_sandboxes -= 1;
            Ok(())
        } else {
            Err(SandboxError::PermissionDenied)
        }
    }

    fn start_sandbox(&mut self, id: SandboxID) -> Result<(), SandboxError> {
        if !self.capability.can_manage {
            return Err(SandboxError::PermissionDenied);
        }

        if let Some(ref mut sandbox) = self.get_sandbox_mut(id) {
            let result = sandbox.start();
            if result.is_ok() {
                let state = sandbox.state();
                if state == SandboxState::Running {
                    self.stats.running_sandboxes += 1;
                    self.stats.stopped_sandboxes -= 1;
                }
            }
            result
        } else {
            Err(SandboxError::PermissionDenied)
        }
    }

    fn stop_sandbox(&mut self, id: SandboxID) -> Result<(), SandboxError> {
        if !self.capability.can_manage {
            return Err(SandboxError::PermissionDenied);
        }

        if let Some(ref mut sandbox) = self.get_sandbox_mut(id) {
            let result = sandbox.stop();
            if result.is_ok() {
                let state = sandbox.state();
                if state == SandboxState::Stopped {
                    self.stats.running_sandboxes -= 1;
                    self.stats.stopped_sandboxes += 1;
                }
            }
            result
        } else {
            Err(SandboxError::PermissionDenied)
        }
    }

    fn get_sandbox(&self, id: SandboxID) -> Option<&dyn Sandbox> {
        for sandbox_option in &self.sandboxes {
            if let Some(ref sandbox) = *sandbox_option {
                if sandbox.id() == id {
                    return Some(sandbox.as_ref());
                }
            }
        }
        None
    }

    fn list_sandboxes(&self) -> Vec<SandboxID> {
        let mut ids = Vec::new();
        for sandbox_option in &self.sandboxes {
            if let Some(ref sandbox) = *sandbox_option {
                ids.push(sandbox.id());
            }
        }
        ids
    }

    fn stats(&self) -> SandboxStats {
        self.stats
    }
}

impl SimpleSandboxManager {
    fn get_sandbox_mut(&mut self, id: SandboxID) -> Option<&mut Box<dyn Sandbox>> {
        for sandbox_option in &mut self.sandboxes {
            if let Some(ref mut sandbox) = *sandbox_option {
                if sandbox.id() == id {
                    return Some(sandbox);
                }
            }
        }
        None
    }
}

/// Simple Vec implementation for no_std
struct Vec<T> {
    data: *mut T,
    len: usize,
    capacity: usize,
}

impl<T> Vec<T> {
    fn new() -> Self {
        Vec {
            data: core::ptr::null_mut(),
            len: 0,
            capacity: 0,
        }
    }

    fn push(&mut self, item: T) {
        unsafe {
            if self.len >= self.capacity {
                self.grow();
            }

            if self.capacity > self.len {
                core::ptr::write(self.data.add(self.len), item);
                self.len += 1;
            }
        }
    }

    fn len(&self) -> usize {
        self.len
    }

    unsafe fn grow(&mut self) {
        let new_capacity = if self.capacity == 0 { 4 } else { self.capacity * 2 };
        let new_data = alloc(new_capacity * mem::size_of::<T>()) as *mut T;

        if !new_data.is_null() {
            for i in 0..self.len {
                core::ptr::copy_nonoverlapping(self.data.add(i), new_data.add(i), 1);
            }

            if self.capacity > 0 {
                free(self.data as *mut u8);
            }

            self.data = new_data;
            self.capacity = new_capacity;
        }
    }
}

// External allocator functions
extern "C" {
    fn alloc(size: usize) -> *mut u8;
    fn free(ptr: *mut u8);
}
