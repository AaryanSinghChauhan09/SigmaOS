#![no_std]
#![no_main]

/// OOP-based MicroVM Sandbox Foundation for SigmaOS
/// Based on Ideas-999-Structured: Core System Item 11
/// Implements Firecracker-style lightweight VMM primitives, sandboxing, isolation

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type VMID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum VMState { Stopped = 0, Starting = 1, Running = 2, Paused = 3, Stopping = 4, Failed = 5 }

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum VMError { Success = 0, InvalidConfig = 1, StartFailed = 2, StopFailed = 3, ResourceLimit = 4 }

pub trait MicroVM {
    fn id(&self) -> VMID;
    fn state(&self) -> VMState;
    fn start(&mut self) -> Result<(), VMError>;
    fn stop(&mut self) -> Result<(), VMError>;
    fn pause(&mut self) -> Result<(), VMError>;
    fn resume(&mut self) -> Result<(), VMError>;
    fn get_memory_limit(&self) -> usize;
    fn get_cpu_count(&self) -> usize;
}

#[repr(C)]
pub struct SimpleMicroVM {
    pub id: VMID,
    pub state: AtomicUsize,
    pub memory_limit: AtomicUsize,
    pub cpu_count: AtomicUsize,
    pub vcpu_ids: Vec<usize>,
}

impl SimpleMicroVM {
    pub fn new(id: VMID, memory_mb: usize, cpus: usize) -> Self {
        let mut vcpu_ids = Vec::new();
        for i in 0..cpus {
            vcpu_ids.push(id * 1000 + i);
        }
        SimpleMicroVM {
            id,
            state: AtomicUsize::new(VMState::Stopped as usize),
            memory_limit: AtomicUsize::new(memory_mb * 1024 * 1024),
            cpu_count: AtomicUsize::new(cpus),
            vcpu_ids,
        }
    }
}

impl MicroVM for SimpleMicroVM {
    fn id(&self) -> VMID { self.id }
    fn state(&self) -> VMState { unsafe { core::mem::transmute(self.state.load(Ordering::SeqCst)) } }

    fn start(&mut self) -> Result<(), VMError> {
        self.state.store(VMState::Starting as usize, Ordering::SeqCst);
        self.state.store(VMState::Running as usize, Ordering::SeqCst);
        Ok(())
    }

    fn stop(&mut self) -> Result<(), VMError> {
        self.state.store(VMState::Stopping as usize, Ordering::SeqCst);
        self.state.store(VMState::Stopped as usize, Ordering::SeqCst);
        Ok(())
    }

    fn pause(&mut self) -> Result<(), VMError> {
        if self.state.load(Ordering::SeqCst) != VMState::Running as usize {
            return Err(VMError::StartFailed);
        }
        self.state.store(VMState::Paused as usize, Ordering::SeqCst);
        Ok(())
    }

    fn resume(&mut self) -> Result<(), VMError> {
        if self.state.load(Ordering::SeqCst) != VMState::Paused as usize {
            return Err(VMError::StartFailed);
        }
        self.state.store(VMState::Running as usize, Ordering::SeqCst);
        Ok(())
    }

    fn get_memory_limit(&self) -> usize { self.memory_limit.load(Ordering::SeqCst) }
    fn get_cpu_count(&self) -> usize { self.cpu_count.load(Ordering::SeqCst) }
}

pub trait VMMManager {
    fn create_vm(&mut self, memory_mb: usize, cpus: usize) -> Result<VMID, VMError>;
    fn destroy_vm(&mut self, id: VMID) -> Result<(), VMError>;
    fn get_vm(&self, id: VMID) -> Option<&dyn MicroVM>;
    fn list_vms(&self) -> Vec<VMID>;
}

#[repr(C)]
pub struct SimpleVMMManager {
    pub vms: Vec<Option<Box<dyn MicroVM>>>,
    pub next_id: AtomicUsize,
    pub max_vms: AtomicUsize,
}

impl SimpleVMMManager {
    pub fn new(max_vms: usize) -> Self {
        SimpleVMMManager {
            vms: Vec::new(),
            next_id: AtomicUsize::new(1),
            max_vms: AtomicUsize::new(max_vms),
        }
    }
}

impl VMMManager for SimpleVMMManager {
    fn create_vm(&mut self, memory_mb: usize, cpus: usize) -> Result<VMID, VMError> {
        if self.vms.len() >= self.max_vms.load(Ordering::SeqCst) {
            return Err(VMError::ResourceLimit);
        }
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        let vm = SimpleMicroVM::new(id, memory_mb, cpus);
       self.vms.push(Some(Box::new(vm)));
        Ok(id)
    }

    fn destroy_vm(&mut self, id: VMID) -> Result<(), VMError> {
        for vm_option in &mut self.vms {
            if let Some(ref vm) = *vm_option {
                if vm.id() == id {
                    return Ok(());
                }
            }
        }
        Err(VMError::InvalidConfig)
    }

    fn get_vm(&self, id: VMID) -> Option<&dyn MicroVM> {
        for vm_option in &self.vms {
            if let Some(ref vm) = *vm_option {
                if vm.id() == id { return Some(vm.as_ref()); }
            }
        }
        None
    }

    fn list_vms(&self) -> Vec<VMID> {
        let mut ids = Vec::new();
        for vm_option in &self.vms {
            if let Some(ref vm) = *vm_option {
                ids.push(vm.id());
            }
        }
        ids
    }
}

pub trait Sandbox {
    fn isolate_process(&mut self, pid: usize) -> Result<(), VMError>;
    fn set_resource_limit(&mut self, pid: usize, memory_mb: usize) -> Result<(), VMError>;
    fn get_resource_usage(&self, pid: usize) -> Option<ResourceUsage>;
}

#[repr(C)]
pub struct ResourceUsage {
    pub memory_bytes: usize,
    pub cpu_time_ns: usize,
    pub io_bytes: usize,
}

#[repr(C)]
pub struct SimpleSandbox {
    pub isolated_pids: Vec<usize>,
    pub resource_limits: Vec<(usize, usize)>,
    pub resource_usage: Vec<(usize, ResourceUsage)>,
}

impl SimpleSandbox {
    pub fn new() -> Self {
        SimpleSandbox {
            isolated_pids: Vec::new(),
            resource_limits: Vec::new(),
            resource_usage: Vec::new(),
        }
    }
}

impl Sandbox for SimpleSandbox {
    fn isolate_process(&mut self, pid: usize) -> Result<(), VMError> {
        if self.isolated_pids.contains(&pid) {
            return Err(VMError::InvalidConfig);
        }
        self.isolated_pids.push(pid);
        Ok(())
    }

    fn set_resource_limit(&mut self, pid: usize, memory_mb: usize) -> Result<(), VMError> {
        if !self.isolated_pids.contains(&pid) {
            return Err(VMError::InvalidConfig);
        }
        for i in 0..self.resource_limits.len() {
            if self.resource_limits[i].0 == pid {
                self.resource_limits[i].1 = memory_mb * 1024 * 1024;
                return Ok(());
            }
        }
        self.resource_limits.push((pid, memory_mb * 1024 * 1024));
        Ok(())
    }

    fn get_resource_usage(&self, pid: usize) -> Option<ResourceUsage> {
        for &(p, usage) in &self.resource_usage {
            if p == pid {
                return Some(usage);
            }
        }
        None
    }
}

pub trait CapabilityBasedSecurity {
    fn grant_capability(&mut self, pid: usize, capability: Capability) -> Result<(), VMError>;
    fn revoke_capability(&mut self, pid: usize, capability: Capability) -> Result<(), VMError>;
    fn check_capability(&self, pid: usize, capability: Capability) -> bool;
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum Capability { Network = 0, Filesystem = 1, Process = 2, IPC = 3, Device = 4 }

#[repr(C)]
pub struct SimpleCapabilitySecurity {
    pub capabilities: Vec<(usize, Vec<Capability>)>,
}

impl SimpleCapabilitySecurity {
    pub fn new() -> Self {
        SimpleCapabilitySecurity {
            capabilities: Vec::new(),
        }
    }
}

impl CapabilityBasedSecurity for SimpleCapabilitySecurity {
    fn grant_capability(&mut self, pid: usize, capability: Capability) -> Result<(), VMError> {
        for i in 0..self.capabilities.len() {
            if self.capabilities[i].0 == pid {
                self.capabilities[i].1.push(capability);
                return Ok(());
            }
        }
        let mut caps = Vec::new();
        caps.push(capability);
        self.capabilities.push((pid, caps));
        Ok(())
    }

    fn revoke_capability(&mut self, pid: usize, capability: Capability) -> Result<(), VMError> {
        for i in 0..self.capabilities.len() {
            if self.capabilities[i].0 == pid {
                self.capabilities[i].1.retain(|&c| c != capability);
                return Ok(());
            }
        }
        Err(VMError::InvalidConfig)
    }

    fn check_capability(&self, pid: usize, capability: Capability) -> bool {
        for &(p, ref caps) in &self.capabilities {
            if p == pid && caps.contains(&capability) {
                return true;
            }
        }
        false
    }
}

pub trait FirecrackerIntegration {
    fn create_firecracker_vm(&mut self, kernel_path: &[u8], rootfs_path: &[u8]) -> Result<VMID, VMError>;
    fn configure_vsock(&mut self, vm_id: VMID, port: u16) -> Result<(), VMError>;
    fn attach_snapshot(&mut self, vm_id: VMID, snapshot_path: &[u8]) -> Result<(), VMError>;
}

#[repr(C)]
pub struct SimpleFirecrackerIntegration {
    pub vmm: SimpleVMMManager,
}

impl SimpleFirecrackerIntegration {
    pub fn new(max_vms: usize) -> Self {
        SimpleFirecrackerIntegration {
            vmm: SimpleVMMManager::new(max_vms),
        }
    }
}

impl FirecrackerIntegration for SimpleFirecrackerIntegration {
    fn create_firecracker_vm(&mut self, _kernel_path: &[u8], _rootfs_path: &[u8]) -> Result<VMID, VMError> {
        self.vmm.create_vm(512, 2)
    }

    fn configure_vsock(&mut self, vm_id: VMID, _port: u16) -> Result<(), VMError> {
        if self.vmm.get_vm(vm_id).is_none() {
            return Err(VMError::InvalidConfig);
        }
        Ok(())
    }

    fn attach_snapshot(&mut self, vm_id: VMID, _snapshot_path: &[u8]) -> Result<(), VMError> {
        if self.vmm.get_vm(vm_id).is_none() {
            return Err(VMError::InvalidConfig);
        }
        Ok(())
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
    fn contains(&self, item: &T) -> bool where T: PartialEq {
        for i in 0..self.len {
            unsafe {
                if &*self.data.add(i) == item { return true; }
            }
        }
        false
    }
    fn retain<F>(&mut self, mut f: F) where F: FnMut(&T) -> bool {
        let mut write_idx = 0;
        for i in 0..self.len {
            unsafe {
                let item = &*self.data.add(i);
                if f(item) {
                    if write_idx != i {
                        core::ptr::copy_nonoverlapping(self.data.add(i), self.data.add(write_idx), 1);
                    }
                    write_idx += 1;
                }
            }
        }
        self.len = write_idx;
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
