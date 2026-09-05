use std::boxed::Box;
use core::mem;
/// OOP-based Hypervisor for SigmaOS
/// Based on Ideas-999-Structured: Kernel & Hardware Item 181
/// Implements virtualization and guest management
use core::sync::atomic::{AtomicUsize, Ordering};

pub type GuestID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GuestState {
    Stopped = 0,
    Running = 1,
    Paused = 2,
    Crashed = 3,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum HypervisorError {
    Success = 0,
    NotFound = 1,
    StartFailed = 2,
    InvalidConfig = 3,
}

/// Represents the virtualization generation model (Legacy software vs Modern hardware-assisted)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VirtualizationGeneration {
    LegacySoftware, // Binary translation, shadow page tables (QEMU/VirtualBox style)
    ModernHardwareAssisted, // VT-x/AMD-V, Extended Page Tables (EPT), Nested Virtualization
}

pub trait Guest {
    fn id(&self) -> GuestID;
    fn name(&self) -> &[u8];
    fn state(&self) -> GuestState;
    fn set_state(&mut self, state: GuestState);
    fn vcpus(&self) -> u32;
    fn memory_mb(&self) -> u32;
    fn generation(&self) -> VirtualizationGeneration;
    fn configure_nested_virtualization(&mut self, enabled: bool) -> Result<(), HypervisorError>;
    fn hardware_exit_count(&self) -> usize;
    fn increment_exit_count(&mut self);
}

#[repr(C)]
pub struct SimpleGuest {
    pub id: GuestID,
    pub name: [u8; 64],
    pub state: AtomicUsize,
    pub vcpus: AtomicUsize,
    pub memory_mb: AtomicUsize,
    pub gen: VirtualizationGeneration,
    pub nested_virt: bool,
    pub exit_count: AtomicUsize,
}

impl SimpleGuest {
    pub fn new(
        id: GuestID,
        name: &[u8],
        vcpus: u32,
        memory_mb: u32,
        gen: VirtualizationGeneration,
    ) -> Self {
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
            gen,
            nested_virt: false,
            exit_count: AtomicUsize::new(0),
        }
    }
}

impl Guest for SimpleGuest {
    fn id(&self) -> GuestID {
        self.id
    }
    fn name(&self) -> &[u8] {
        let len = self.name.iter().position(|&b| b == 0).unwrap_or(64);
        &self.name[..len]
    }
    fn state(&self) -> GuestState {
        let state_val = self.state.load(Ordering::SeqCst);
        match state_val {
            0 => GuestState::Stopped,
            1 => GuestState::Running,
            2 => GuestState::Paused,
            _ => GuestState::Crashed,
        }
    }
    fn set_state(&mut self, state: GuestState) {
        self.state.store(state as usize, Ordering::SeqCst);
    }
    fn vcpus(&self) -> u32 {
        self.vcpus.load(Ordering::SeqCst) as u32
    }
    fn memory_mb(&self) -> u32 {
        self.memory_mb.load(Ordering::SeqCst) as u32
    }
    fn generation(&self) -> VirtualizationGeneration {
        self.gen
    }
    fn configure_nested_virtualization(&mut self, enabled: bool) -> Result<(), HypervisorError> {
        if self.gen == VirtualizationGeneration::LegacySoftware {
            return Err(HypervisorError::InvalidConfig); // Legacy cannot run nested VMs
        }
        self.nested_virt = enabled;
        Ok(())
    }
    fn hardware_exit_count(&self) -> usize {
        self.exit_count.load(Ordering::SeqCst)
    }
    fn increment_exit_count(&mut self) {
        self.exit_count.fetch_add(1, Ordering::SeqCst);
    }
}

pub trait Hypervisor {
    fn create_guest(
        &mut self,
        name: &[u8],
        vcpus: u32,
        memory_mb: u32,
        gen: VirtualizationGeneration,
    ) -> Result<GuestID, HypervisorError>;
    fn destroy_guest(&mut self, id: GuestID) -> Result<(), HypervisorError>;
    fn start_guest(&mut self, id: GuestID) -> Result<(), HypervisorError>;
    fn stop_guest(&mut self, id: GuestID) -> Result<(), HypervisorError>;
    fn pause_guest(&mut self, id: GuestID) -> Result<(), HypervisorError>;
    fn resume_guest(&mut self, id: GuestID) -> Result<(), HypervisorError>;
    fn inject_hardware_exit(&mut self, id: GuestID) -> Result<(), HypervisorError>;
}

#[repr(C)]
pub struct SimpleHypervisor {
    pub guests: Vec<Option<Box<dyn Guest>>>,
    pub next_id: AtomicUsize,
}

impl Default for SimpleHypervisor {
    fn default() -> Self {
        Self::new()
    }
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
    fn create_guest(
        &mut self,
        name: &[u8],
        vcpus: u32,
        memory_mb: u32,
        gen: VirtualizationGeneration,
    ) -> Result<GuestID, HypervisorError> {
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        let guest = SimpleGuest::new(id, name, vcpus, memory_mb, gen);
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
                    guest.set_state(GuestState::Running);
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
                    guest.set_state(GuestState::Stopped);
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
                    guest.set_state(GuestState::Paused);
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
                    guest.set_state(GuestState::Running);
                    return Ok(());
                }
            }
        }
        Err(HypervisorError::NotFound)
    }

    fn inject_hardware_exit(&mut self, id: GuestID) -> Result<(), HypervisorError> {
        for guest_option in &mut self.guests {
            if let Some(ref mut guest) = *guest_option {
                if guest.id() == id {
                    guest.increment_exit_count();
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
#[allow(clippy::type_complexity)]
pub struct SimpleVMExitHandler {
    pub handlers: Vec<(u64, fn(GuestID, u64))>,
}

impl Default for SimpleVMExitHandler {
    fn default() -> Self {
        Self::new()
    }
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

impl<T> Default for Vec<T> {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Vec<T> {
    data: *mut T,
    len: usize,
    capacity: usize,
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
            if self.capacity > self.len {
                core::ptr::write(self.data.add(self.len), item);
                self.len += 1;
            }
        }
    }
    fn len(&self) -> usize {
        self.len
    }
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
    use std::alloc::{alloc, Layout};
    let layout = Layout::from_size_align(size, 8).unwrap();
    std::alloc::alloc(layout)
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
    fn test_hypervisor_and_guest_generations_oop() {
        let mut hypervisor = SimpleHypervisor::new();

        // 1. Create a legacy software guest VM (e.g. simulating QEMU translation)
        let guest_legacy_id = hypervisor
            .create_guest(
                b"qemu-legacy-winxp",
                1,
                512,
                VirtualizationGeneration::LegacySoftware,
            )
            .unwrap();

        // 2. Create a modern hardware-assisted guest VM (e.g. simulating VT-x / EPT nested guest)
        let guest_modern_id = hypervisor
            .create_guest(
                b"kvm-modern-linux",
                4,
                4096,
                VirtualizationGeneration::ModernHardwareAssisted,
            )
            .unwrap();

        // Verify ID assignment and retrieve guests
        assert_eq!(guest_legacy_id, 1);
        assert_eq!(guest_modern_id, 2);

        // Mutably configure guest parameters via OOP traits
        for guest_opt in &mut hypervisor.guests {
            if let Some(ref mut guest) = *guest_opt {
                if guest.id() == guest_legacy_id {
                    assert_eq!(guest.generation(), VirtualizationGeneration::LegacySoftware);
                    assert_eq!(guest.state(), GuestState::Stopped);
                    // Legacy software guests should fail on nested virtualization configuration
                    assert!(guest.configure_nested_virtualization(true).is_err());
                } else if guest.id() == guest_modern_id {
                    assert_eq!(
                        guest.generation(),
                        VirtualizationGeneration::ModernHardwareAssisted
                    );
                    assert_eq!(guest.state(), GuestState::Stopped);
                    // Modern nested virtualization configuration should succeed
                    assert!(guest.configure_nested_virtualization(true).is_ok());
                }
            }
        }

        // Test start/stop guest lifecycles
        assert!(hypervisor.start_guest(guest_modern_id).is_ok());
        for guest_opt in &hypervisor.guests {
            if let Some(ref guest) = *guest_opt {
                if guest.id() == guest_modern_id {
                    assert_eq!(guest.state(), GuestState::Running);
                }
            }
        }

        // Test Hardware-assisted VMExit handler statistics
        assert!(hypervisor.inject_hardware_exit(guest_modern_id).is_ok());
        for guest_opt in &hypervisor.guests {
            if let Some(ref guest) = *guest_opt {
                if guest.id() == guest_modern_id {
                    assert_eq!(guest.hardware_exit_count(), 1);
                }
            }
        }
    }

    #[test]
    fn test_vmexit_handler() {
        let mut exit_handler = SimpleVMExitHandler::new();

        // Register sample handler for exit reason 0x80 (e.g. CPUID vmexit instruction)
        fn handle_cpuid(_guest_id: GuestID, _reason: u64) {
            println!("VMEXIT handle CPUID");
        }
        exit_handler.register_handler(0x80, handle_cpuid);

        assert!(exit_handler.handle_exit(1, 0x80).is_ok());
        assert!(exit_handler.handle_exit(1, 0x99).is_err()); // Unregistered exit reason
    }
}
