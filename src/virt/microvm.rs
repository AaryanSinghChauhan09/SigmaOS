#![no_std]
#![no_main]

/// OOP-based MicroVM Sandboxing Foundation for SigmaOS
/// Implements microVM sandboxing using OOP principles with traits and structs
/// No dependency on external virtualization frameworks
/// Based on Roadmap Item 19: MicroVM sandboxing foundation

use core::ptr::{self, NonNull};
use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

/// MicroVM ID
pub type MicroVMID = usize;

/// MicroVM state
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum MicroVMState {
    Stopped = 0,
    Starting = 1,
    Running = 2,
    Stopping = 3,
    Paused = 4,
}

/// Sandbox policy
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum SandboxPolicy {
    Strict = 0,
    Moderate = 1,
    Permissive = 2,
}

/// MicroVM trait (OOP interface)
pub trait MicroVM {
    /// Get microVM ID
    fn id(&self) -> MicroVMID;
    /// Get microVM name
    fn name(&self) -> &[u8];
    /// Get sandbox policy
    fn sandbox_policy(&self) -> SandboxPolicy;
    /// Start microVM
    fn start(&mut self) -> Result<(), MicroVMError>;
    /// Stop microVM
    fn stop(&mut self) -> Result<(), MicroVMError>;
    /// Pause microVM
    fn pause(&mut self) -> Result<(), MicroVMError>;
    /// Resume microVM
    fn resume(&mut self) -> Result<(), MicroVMError>;
    /// Get microVM state
    fn state(&self) -> MicroVMState;
    /// Get microVM info
    fn info(&self) -> MicroVMInfo;
}

/// MicroVM error types
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum MicroVMError {
    Success = 0,
    AlreadyRunning = 1,
    StartFailed = 2,
    StopFailed = 3,
    PermissionDenied = 4,
}

/// MicroVM info
#[repr(C)]
pub struct MicroVMInfo {
    pub id: MicroVMID,
    pub name: [u8; 64],
    pub state: MicroVMState,
    pub sandbox_policy: SandboxPolicy,
    pub memory: u64,
    pub cpus: u32,
    pub capability: MicroVMCapability,
}

impl MicroVMInfo {
    pub fn new(id: MicroVMID) -> Self {
        MicroVMInfo {
            id,
            name: [0; 64],
            state: MicroVMState::Stopped,
            sandbox_policy: SandboxPolicy::Strict,
            memory: 0,
            cpus: 0,
            capability: MicroVMCapability::new(),
        }
    }
}

/// MicroVM capability
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct MicroVMCapability {
    pub can_start: bool,
    pub can_stop: bool,
    pub can_pause: bool,
}

impl MicroVMCapability {
    pub fn new() -> Self {
        MicroVMCapability {
            can_start: false,
            can_stop: false,
            can_pause: false,
        }
    }

    pub fn full() -> Self {
        MicroVMCapability {
            can_start: true,
            can_stop: true,
            can_pause: true,
        }
    }
}

/// Simple microVM (OOP: Concrete microVM class)
#[repr(C)]
pub struct SimpleMicroVM {
    pub id: MicroVMID,
    pub name: [u8; 64],
    pub sandbox_policy: SandboxPolicy,
    pub state: AtomicUsize, // MicroVMState as usize
    pub memory: u64,
    pub cpus: u32,
    pub capability: MicroVMCapability,
}

impl SimpleMicroVM {
    pub fn new(id: MicroVMID, name: &[u8], sandbox_policy: SandboxPolicy, capability: MicroVMCapability) -> Self {
        let mut name_array = [0u8; 64];
        let name_len = name.len().min(63);

        unsafe {
            core::ptr::copy_nonoverlapping(name.as_ptr(), name_array.as_mut_ptr(), name_len);
        }

        SimpleMicroVM {
            id,
            name: name_array,
            sandbox_policy,
            state: AtomicUsize::new(MicroVMState::Stopped as usize),
            memory: 512,
            cpus: 1,
            capability,
        }
    }

    pub fn set_resources(&mut self, memory: u64, cpus: u32) {
        self.memory = memory;
        self.cpus = cpus;
    }

    pub fn get_state(&self) -> MicroVMState {
        unsafe {
            core::mem::transmute(self.state.load(Ordering::SeqCst))
        }
    }

    pub fn set_state(&self, state: MicroVMState) {
        self.state.store(state as usize, Ordering::SeqCst);
    }
}

impl MicroVM for SimpleMicroVM {
    fn id(&self) -> MicroVMID {
        self.id
    }

    fn name(&self) -> &[u8] {
        let len = self.name.iter().position(|&b| b == 0).unwrap_or(64);
        &self.name[..len]
    }

    fn sandbox_policy(&self) -> SandboxPolicy {
        self.sandbox_policy
    }

    fn start(&mut self) -> Result<(), MicroVMError> {
        if !self.capability.can_start {
            return Err(MicroVMError::PermissionDenied);
        }

        let current_state = self.get_state();
        if current_state == MicroVMState::Running {
            return Err(MicroVMError::AlreadyRunning);
        }

        self.set_state(MicroVMState::Starting);
        self.set_state(MicroVMState::Running);
        Ok(())
    }

    fn stop(&mut self) -> Result<(), MicroVMError> {
        if !self.capability.can_stop {
            return Err(MicroVMError::PermissionDenied);
        }

        self.set_state(MicroVMState::Stopping);
        self.set_state(MicroVMState::Stopped);
        Ok(())
    }

    fn pause(&mut self) -> Result<(), MicroVMError> {
        if !self.capability.can_pause {
            return Err(MicroVMError::PermissionDenied);
        }

        let current_state = self.get_state();
        if current_state != MicroVMState::Running {
            return Err(MicroVMError::StartFailed);
        }

        self.set_state(MicroVMState::Paused);
        Ok(())
    }

    fn resume(&mut self) -> Result<(), MicroVMError> {
        let current_state = self.get_state();
        if current_state != MicroVMState::Paused {
            return Err(MicroVMError::StartFailed);
        }

        self.set_state(MicroVMState::Running);
        Ok(())
    }

    fn state(&self) -> MicroVMState {
        self.get_state()
    }

    fn info(&self) -> MicroVMInfo {
        MicroVMInfo {
            id: self.id,
            name: self.name,
            state: self.get_state(),
            sandbox_policy: self.sandbox_policy,
            memory: self.memory,
            cpus: self.cpus,
            capability: self.capability,
        }
    }
}

/// Sandbox manager trait (OOP interface)
pub trait SandboxManager {
    /// Create microVM
    fn create_microvm(&mut self, name: &[u8], sandbox_policy: SandboxPolicy) -> Result<MicroVMID, MicroVMError>;
    /// Destroy microVM
    fn destroy_microvm(&mut self, id: MicroVMID) -> Result<(), MicroVMError>;
    /// Start microVM
    fn start_microvm(&mut self, id: MicroVMID) -> Result<(), MicroVMError>;
    /// Stop microVM
    fn stop_microvm(&mut self, id: MicroVMID) -> Result<(), MicroVMError>;
    /// Get microVM
    fn get_microvm(&self, id: MicroVMID) -> Option<&dyn MicroVM>;
    /// List microVMs by policy
    fn list_microvms(&self, sandbox_policy: SandboxPolicy) -> Vec<MicroVMID>;
    /// Get manager statistics
    fn stats(&self) -> SandboxStats;
}

/// Sandbox statistics
#[repr(C)]
pub struct SandboxStats {
    pub total_microvms: usize,
    pub running_microvms: usize,
    pub paused_microvms: usize,
    pub by_policy: [usize; 3],
}

impl SandboxStats {
    pub fn new() -> Self {
        SandboxStats {
            total_microvms: 0,
            running_microvms: 0,
            paused_microvms: 0,
            by_policy: [0; 3],
        }
    }
}

/// Simple sandbox manager (OOP: Concrete manager class)
pub struct SimpleSandboxManager {
    microvms: Vec<Option<Box<dyn MicroVM>>>,
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
            microvms: Vec::new(),
            next_id: AtomicUsize::new(1),
            stats: SandboxStats::new(),
            capability,
        }
    }
}

impl SandboxManager for SimpleSandboxManager {
    fn create_microvm(&mut self, name: &[u8], sandbox_policy: SandboxPolicy) -> Result<MicroVMID, MicroVMError> {
        if !self.capability.can_create {
            return Err(MicroVMError::PermissionDenied);
        }

        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        let microvm = SimpleMicroVM::new(id, name, sandbox_policy, MicroVMCapability::full());
        self.microvms.push(Some(Box::new(microvm)));
        self.stats.total_microvms += 1;
        self.stats.by_policy[sandbox_policy as usize] += 1;
        Ok(id)
    }

    fn destroy_microvm(&mut self, id: MicroVMID) -> Result<(), MicroVMError> {
        if !self.capability.can_destroy {
            return Err(MicroVMError::PermissionDenied);
        }

        let mut index = None;
        let mut sandbox_policy = SandboxPolicy::Strict;

        for (i, microvm_option) in self.microvms.iter().enumerate() {
            if let Some(ref microvm) = *microvm_option {
                if microvm.id() == id {
                    index = Some(i);
                    sandbox_policy = microvm.sandbox_policy();
                    break;
                }
            }
        }

        if let Some(i) = index {
            self.microvms[i] = None;
            self.stats.total_microvms -= 1;
            self.stats.by_policy[sandbox_policy as usize] -= 1;
            Ok(())
        } else {
            Err(MicroVMError::PermissionDenied)
        }
    }

    fn start_microvm(&mut self, id: MicroVMID) -> Result<(), MicroVMError> {
        if !self.capability.can_manage {
            return Err(MicroVMError::PermissionDenied);
        }

        for microvm_option in &mut self.microvms {
            if let Some(ref mut microvm) = *microvm_option {
                if microvm.id() == id {
                    let result = microvm.start();
                    if result.is_ok() {
                        self.stats.running_microvms += 1;
                    }
                    return result;
                }
            }
        }
        Err(MicroVMError::PermissionDenied)
    }

    fn stop_microvm(&mut self, id: MicroVMID) -> Result<(), MicroVMError> {
        if !self.capability.can_manage {
            return Err(MicroVMError::PermissionDenied);
        }

        for microvm_option in &mut self.microvms {
            if let Some(ref mut microvm) = *microvm_option {
                if microvm.id() == id {
                    let result = microvm.stop();
                    if result.is_ok() {
                        self.stats.running_microvms -= 1;
                    }
                    return result;
                }
            }
        }
        Err(MicroVMError::PermissionDenied)
    }

    fn get_microvm(&self, id: MicroVMID) -> Option<&dyn MicroVM> {
        for microvm_option in &self.microvms {
            if let Some(ref microvm) = *microvm_option {
                if microvm.id() == id {
                    return Some(microvm.as_ref());
                }
            }
        }
        None
    }

    fn list_microvms(&self, sandbox_policy: SandboxPolicy) -> Vec<MicroVMID> {
        let mut ids = Vec::new();

        for microvm_option in &self.microvms {
            if let Some(ref microvm) = *microvm_option {
                if microvm.sandbox_policy() == sandbox_policy {
                    ids.push(microvm.id());
                }
            }
        }

        ids
    }

    fn stats(&self) -> SandboxStats {
        self.stats
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
