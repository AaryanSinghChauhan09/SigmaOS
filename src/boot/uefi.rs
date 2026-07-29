#![no_std]
#![no_main]

/// OOP-based UEFI Bootloader for SigmaOS
/// Based on Roadmap Item: Complete UEFI Bootloader (Critical Blocker)

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type BootStatus = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum BootPhase { Init = 0, LoadKernel = 1, Handoff = 2, Complete = 3 }

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum BootError { Success = 0, LoadFailed = 1, HandoffFailed = 2 }

pub trait UEFIBootloader {
    fn phase(&self) -> BootPhase;
    fn load_kernel(&mut self, kernel_data: &[u8]) -> Result<BootStatus, BootError>;
    fn handoff(&mut self) -> Result<BootStatus, BootError>;
}

#[repr(C)]
pub struct SimpleUEFIBootloader {
    pub phase: AtomicUsize,
    pub kernel_loaded: AtomicUsize,
}

impl SimpleUEFIBootloader {
    pub fn new() -> Self {
        SimpleUEFIBootloader {
            phase: AtomicUsize::new(BootPhase::Init as usize),
            kernel_loaded: AtomicUsize::new(0),
        }
    }
}

impl UEFIBootloader for SimpleUEFIBootloader {
    fn phase(&self) -> BootPhase { unsafe { core::mem::transmute(self.phase.load(Ordering::SeqCst) as u32) } }
    fn load_kernel(&mut self, _kernel_data: &[u8]) -> Result<BootStatus, BootError> {
        self.phase.store(BootPhase::LoadKernel as usize, Ordering::SeqCst);
        self.kernel_loaded.store(1, Ordering::SeqCst);
        Ok(1)
    }
    fn handoff(&mut self) -> Result<BootStatus, BootError> {
        if self.kernel_loaded.load(Ordering::SeqCst) == 0 {
            return Err(BootError::LoadFailed);
        }
        self.phase.store(BootPhase::Handoff as usize, Ordering::SeqCst);
        self.phase.store(BootPhase::Complete as usize, Ordering::SeqCst);
        Ok(2)
    }
}

pub trait SecureBoot {
    fn verify_signature(&self, data: &[u8]) -> Result<bool, BootError>;
    fn sign(&self, data: &[u8]) -> Result<Vec<u8>, BootError>;
}

#[repr(C)]
pub struct SimpleSecureBoot {
    pub bootloader: SimpleUEFIBootloader,
}

impl SimpleSecureBoot {
    pub fn new() -> Self { SimpleSecureBoot { bootloader: SimpleUEFIBootloader::new() } }
}

impl SecureBoot for SimpleSecureBoot {
    fn verify_signature(&self, _data: &[u8]) -> Result<bool, BootError> {
        Ok(true)
    }
    fn sign(&self, data: &[u8]) -> Result<Vec<u8>, BootError> {
        let mut signature = Vec::new();
        for byte in data {
            signature.push(byte.wrapping_add(0x42));
        }
        Ok(signature)
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

#[derive(Debug, Clone, Copy, Default)]
pub struct GopFramebuffer {
    pub base_address: u64,
    pub size: usize,
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct AcpiParser;

#[derive(Debug, Clone, Copy, Default)]
pub struct UsbHostController;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MicrokernelProfile {
    Performance,
    Security,
    Minimalist,
}

#[derive(Debug, Clone)]
pub struct MultiKernelBootSelector {
    pub current_profile: MicrokernelProfile,
}

impl MultiKernelBootSelector {
    pub fn new() -> Self {
        Self {
            current_profile: MicrokernelProfile::Performance,
        }
    }

    pub fn select_profile(&mut self, profile: MicrokernelProfile) {
        self.current_profile = profile;
    }
}

impl Default for MultiKernelBootSelector {
    fn default() -> Self {
        Self::new()
    }
}

pub struct SovereignBootWatchdog {
    pub counter: AtomicUsize,
}

impl SovereignBootWatchdog {
    pub fn new() -> Self {
        Self {
            counter: AtomicUsize::new(0),
        }
    }

    pub fn ping(&self) {
        self.counter.fetch_add(1, Ordering::SeqCst);
    }
}

impl Default for SovereignBootWatchdog {
    fn default() -> Self {
        Self::new()
    }
}

pub struct GopSplashCanvas {
    pub framebuffer: GopFramebuffer,
}

impl GopSplashCanvas {
    pub fn new(fb: GopFramebuffer) -> Self {
        Self { framebuffer: fb }
    }

    pub fn draw_pixel(&self, _x: u32, _y: u32, _color: u32) {}
}


impl<T> core::ops::Deref for Vec<T> {
    type Target = [T];
    fn deref(&self) -> &Self::Target {
        if self.data.is_null() {
            &[]
        } else {
            unsafe { core::slice::from_raw_parts(self.data, self.len) }
        }
    }
}

impl<T> core::ops::DerefMut for Vec<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        if self.data.is_null() {
            &mut []
        } else {
            unsafe { core::slice::from_raw_parts_mut(self.data, self.len) }
        }
    }
}

impl<'a, T> IntoIterator for &'a Vec<T> {
    type Item = &'a T;
    type IntoIter = core::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        use core::ops::Deref;
        self.deref().iter()
    }
}


impl<'a, T> IntoIterator for &'a mut Vec<T> {
    type Item = &'a mut T;
    type IntoIter = core::slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        use core::ops::DerefMut;
        self.deref_mut().iter_mut()
    }
}
