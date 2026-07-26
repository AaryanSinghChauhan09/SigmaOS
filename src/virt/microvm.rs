use core::mem;
/// OOP-based MicroVM Sandboxing Foundation for SigmaOS
/// Implements microVM sandboxing using OOP principles with traits and structs
/// No dependency on external virtualization frameworks
/// Based on Roadmap Item 19: MicroVM sandboxing foundation
use core::sync::atomic::{AtomicUsize, Ordering};

/// MicroVM ID
pub type MicroVMID = usize;

/// MicroVM state
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MicroVMState {
    Stopped = 0,
    Starting = 1,
    Running = 2,
    Stopping = 3,
    Paused = 4,
}

/// Sandbox policy
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

impl Default for MicroVMCapability {
    fn default() -> Self {
        Self::new()
    }
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
    pub fn new(
        id: MicroVMID,
        name: &[u8],
        sandbox_policy: SandboxPolicy,
        capability: MicroVMCapability,
    ) -> Self {
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
        let state_val = self.state.load(Ordering::SeqCst);
        match state_val {
            0 => MicroVMState::Stopped,
            1 => MicroVMState::Starting,
            2 => MicroVMState::Running,
            3 => MicroVMState::Stopping,
            _ => MicroVMState::Paused,
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
    fn create_microvm(
        &mut self,
        name: &[u8],
        sandbox_policy: SandboxPolicy,
    ) -> Result<MicroVMID, MicroVMError>;
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
#[derive(Debug, Clone, Copy)]
pub struct SandboxStats {
    pub total_microvms: usize,
    pub running_microvms: usize,
    pub paused_microvms: usize,
    pub by_policy: [usize; 3],
}

impl Default for SandboxStats {
    fn default() -> Self {
        Self::new()
    }
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

impl Default for ManagerCapability {
    fn default() -> Self {
        Self::new()
    }
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
    fn create_microvm(
        &mut self,
        name: &[u8],
        sandbox_policy: SandboxPolicy,
    ) -> Result<MicroVMID, MicroVMError> {
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
impl<T> Default for Vec<T> {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Vec<T> {
    pub data: *mut T,
    pub len: usize,
    pub capacity: usize,
}

impl<T> Vec<T> {
    pub fn new() -> Self {
        Vec {
            data: core::ptr::null_mut(),
            len: 0,
            capacity: 0,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    fn push(&mut self, item: T) {
        unsafe {
            if self.len >= self.capacity {
                self.grow();
            }

        // 1. Create a Strict sandbox microVM (e.g. secure, zero network/shared filesystem)
        let microvm_strict_id = manager
            .create_microvm(b"strict-secure-vbox", SandboxPolicy::Strict)
            .unwrap();

        // 2. Create a Permissive sandbox microVM (e.g. development mode)
        let _microvm_permissive_id = manager
            .create_microvm(b"permissive-dev-box", SandboxPolicy::Permissive)
            .unwrap();

    pub fn iter(&self) -> VecIter<'_, T> {
        VecIter {
            vec: self,
            index: 0,
        }
    }

    pub fn iter_mut(&mut self) -> VecIterMut<'_, T> {
        VecIterMut {
            data: self.data,
            len: self.len,
            index: 0,
            _marker: core::marker::PhantomData,
        }
    }

    unsafe fn grow(&mut self) {
        let new_capacity = if self.capacity == 0 {
            4
        } else {
            self.capacity * 2
        };
        let new_data = alloc(new_capacity * mem::size_of::<T>()) as *mut T;

        // Retrieve and start strict sandbox microVM
        assert!(manager.start_microvm(microvm_strict_id).is_ok());

        let microvm_strict = manager.get_microvm(microvm_strict_id).unwrap();
        assert_eq!(microvm_strict.state(), MicroVMState::Running);
        assert_eq!(microvm_strict.sandbox_policy(), SandboxPolicy::Strict);
    }
}

impl<T> core::ops::Index<usize> for Vec<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        if index >= self.len {
            panic!("index out of bounds");
        }
        unsafe { &*self.data.add(index) }
    }
}

impl<T> core::ops::IndexMut<usize> for Vec<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index >= self.len {
            panic!("index out of bounds");
        }
        unsafe { &mut *self.data.add(index) }
    }
}

impl<'a, T> IntoIterator for &'a Vec<T> {
    type Item = &'a T;
    type IntoIter = VecIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut Vec<T> {
    type Item = &'a mut T;
    type IntoIter = VecIterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

pub struct VecIter<'a, T> {
    vec: &'a Vec<T>,
    index: usize,
}

impl<'a, T> Iterator for VecIter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.vec.len() {
            let item = unsafe { &*self.vec.data.add(self.index) };
            self.index += 1;
            Some(item)
        } else {
            None
        }
    }
}

pub struct VecIterMut<'a, T> {
    data: *mut T,
    len: usize,
    index: usize,
    _marker: core::marker::PhantomData<&'a mut T>,
}

impl<'a, T> Iterator for VecIterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.len {
            let item = unsafe { &mut *self.data.add(self.index) };
            self.index += 1;
            Some(item)
        } else {
            None
        }
    }
}

// Allocator shim: uses std allocator on hosted targets (test/dev) and extern C on bare-metal
#[cfg(not(target_os = "none"))]
unsafe fn alloc(size: usize) -> *mut u8 {
    use std::alloc::{alloc as std_alloc, Layout};
    let layout = Layout::from_size_align(size, 8).unwrap();
    std_alloc(layout)
}

#[cfg(not(target_os = "none"))]
unsafe fn free(ptr: *mut u8) {
    let _ = ptr;
}

#[cfg(target_os = "none")]
extern "C" {
    fn alloc(size: usize) -> *mut u8;
    fn free(ptr: *mut u8);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_microvm_sandbox_policy_oop() {
        let manager_cap = ManagerCapability::full();
        let mut manager = SimpleSandboxManager::new(manager_cap);

        // 1. Create a Strict sandbox microVM (e.g. secure, zero network/shared filesystem)
        let microvm_strict_id = manager
            .create_microvm(b"strict-secure-vbox", SandboxPolicy::Strict)
            .unwrap();

        // 2. Create a Permissive sandbox microVM (e.g. development mode)
        let _microvm_permissive_id = manager
            .create_microvm(b"permissive-dev-box", SandboxPolicy::Permissive)
            .unwrap();

        // Verify statistics
        let stats = manager.stats();
        assert_eq!(stats.total_microvms, 2);
        assert_eq!(stats.by_policy[SandboxPolicy::Strict as usize], 1);
        assert_eq!(stats.by_policy[SandboxPolicy::Permissive as usize], 1);

        // Retrieve and start strict sandbox microVM
        assert!(manager.start_microvm(microvm_strict_id).is_ok());

        let microvm_strict = manager.get_microvm(microvm_strict_id).unwrap();
        assert_eq!(microvm_strict.state(), MicroVMState::Running);
        assert_eq!(microvm_strict.sandbox_policy(), SandboxPolicy::Strict);
    }
}
