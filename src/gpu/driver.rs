#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]

// (no_std only applicable at crate root - removed)
// #![no_main]  // crate-root only

use core::mem;
/// OOP-based GPU Driver for SigmaOS
/// Based on Ideas-999-Structured: Kernel & Hardware Item 71
/// Implements GPU device management and rendering
use core::sync::atomic::{AtomicUsize, Ordering};

pub type GPUDeviceID = usize;

#[repr(usize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GPUVendor {
    Intel = 0,
    AMD = 1,
    NVIDIA = 2,
    Other = 3,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum GPUError {
    Success = 0,
    NotFound = 1,
    InitFailed = 2,
    RenderFailed = 3,
}

pub trait GPUDevice {
    fn id(&self) -> GPUDeviceID;
    fn vendor(&self) -> GPUVendor;
    fn model(&self) -> &[u8];
    fn vram_size(&self) -> usize;
    fn initialize(&mut self) -> Result<(), GPUError>;
}

#[repr(C)]
pub struct SimpleGPUDevice {
    pub id: GPUDeviceID,
    pub vendor: AtomicUsize,
    pub model: [u8; 64],
    pub vram_size: AtomicUsize,
}

impl SimpleGPUDevice {
    pub fn new(id: GPUDeviceID, vendor: GPUVendor, model: &[u8], vram_size: usize) -> Self {
        let mut model_array = [0u8; 64];
        let model_len = model.len().min(63);
        unsafe {
            core::ptr::copy_nonoverlapping(model.as_ptr(), model_array.as_mut_ptr(), model_len);
        }
        SimpleGPUDevice {
            id,
            vendor: AtomicUsize::new(vendor as usize),
            model: model_array,
            vram_size: AtomicUsize::new(vram_size),
        }
    }
}

impl GPUDevice for SimpleGPUDevice {
    fn id(&self) -> GPUDeviceID {
        self.id
    }
    fn vendor(&self) -> GPUVendor {
        unsafe { core::mem::transmute(self.vendor.load(Ordering::SeqCst)) }
    }
    fn model(&self) -> &[u8] {
        let len = self.model.iter().position(|&b| b == 0).unwrap_or(64);
        &self.model[..len]
    }
    fn vram_size(&self) -> usize {
        self.vram_size.load(Ordering::SeqCst)
    }

    fn initialize(&mut self) -> Result<(), GPUError> {
        Ok(())
    }
}

pub trait GPUManager {
    fn register_gpu(&mut self, gpu: Box<dyn GPUDevice>) -> Result<GPUDeviceID, GPUError>;
    fn get_primary_gpu(&self) -> Option<&dyn GPUDevice>;
    fn list_gpus(&self) -> Vec<GPUDeviceID>;
}

#[repr(C)]
pub struct SimpleGPUManager {
    pub gpus: Vec<Option<Box<dyn GPUDevice>>>,
    pub next_id: AtomicUsize,
}

impl SimpleGPUManager {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        SimpleGPUManager {
            gpus: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl GPUManager for SimpleGPUManager {
    fn register_gpu(&mut self, gpu: Box<dyn GPUDevice>) -> Result<GPUDeviceID, GPUError> {
        let id = gpu.id();
        self.gpus.push(Some(gpu));
        Ok(id)
    }

    fn get_primary_gpu(&self) -> Option<&dyn GPUDevice> {
        if !self.gpus.is_empty() {
            if let Some(ref gpu) = self.gpus[0] {
                let g: &dyn GPUDevice = &**gpu;
                return Some(g);
            }
        }
        None
    }

    fn list_gpus(&self) -> Vec<GPUDeviceID> {
        let mut ids = Vec::new();
        for gpu_option in &self.gpus {
            if let Some(ref gpu) = *gpu_option {
                ids.push(gpu.id());
            }
        }
        ids
    }
}

pub trait Framebuffer {
    fn create_framebuffer(
        &mut self,
        width: usize,
        height: usize,
        format: u32,
    ) -> Result<usize, GPUError>;
    fn bind_framebuffer(&mut self, fb_id: usize) -> Result<(), GPUError>;
    fn clear(&mut self, color: u32) -> Result<(), GPUError>;
    fn swap_buffers(&mut self) -> Result<(), GPUError>;
}

#[repr(C)]
pub struct SimpleFramebuffer {
    pub framebuffers: Vec<(usize, usize, usize, u32)>,
    pub current: AtomicUsize,
    pub next_id: AtomicUsize,
}

impl SimpleFramebuffer {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        SimpleFramebuffer {
            framebuffers: Vec::new(),
            current: AtomicUsize::new(0),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl Framebuffer for SimpleFramebuffer {
    fn create_framebuffer(
        &mut self,
        width: usize,
        height: usize,
        format: u32,
    ) -> Result<usize, GPUError> {
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        self.framebuffers.push((id, width, height, format));
        Ok(id)
    }

    fn bind_framebuffer(&mut self, fb_id: usize) -> Result<(), GPUError> {
        for &(id, _, _, _) in &self.framebuffers {
            if id == fb_id {
                self.current.store(fb_id, Ordering::SeqCst);
                return Ok(());
            }
        }
        Err(GPUError::NotFound)
    }

    fn clear(&mut self, _color: u32) -> Result<(), GPUError> {
        Ok(())
    }

    fn swap_buffers(&mut self) -> Result<(), GPUError> {
        Ok(())
    }
}

pub trait RenderPipeline {
    fn create_pipeline(
        &mut self,
        vertex_shader: &[u8],
        fragment_shader: &[u8],
    ) -> Result<usize, GPUError>;
    fn bind_pipeline(&mut self, pipeline_id: usize) -> Result<(), GPUError>;
    fn draw(&mut self, vertex_count: usize) -> Result<(), GPUError>;
}

#[repr(C)]
pub struct SimpleRenderPipeline {
    pub pipelines: Vec<(usize, [u8; 256], [u8; 256])>,
    pub current: AtomicUsize,
    pub next_id: AtomicUsize,
}

impl SimpleRenderPipeline {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        SimpleRenderPipeline {
            pipelines: Vec::new(),
            current: AtomicUsize::new(0),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl RenderPipeline for SimpleRenderPipeline {
    fn create_pipeline(
        &mut self,
        vertex_shader: &[u8],
        fragment_shader: &[u8],
    ) -> Result<usize, GPUError> {
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        let mut vs_array = [0u8; 256];
        let mut fs_array = [0u8; 256];
        let vs_len = vertex_shader.len().min(255);
        let fs_len = fragment_shader.len().min(255);
        for i in 0..vs_len {
            vs_array[i] = vertex_shader[i];
        }
        for i in 0..fs_len {
            fs_array[i] = fragment_shader[i];
        }
        self.pipelines.push((id, vs_array, fs_array));
        Ok(id)
    }

    fn bind_pipeline(&mut self, pipeline_id: usize) -> Result<(), GPUError> {
        for &(id, _, _) in &self.pipelines {
            if id == pipeline_id {
                self.current.store(pipeline_id, Ordering::SeqCst);
                return Ok(());
            }
        }
        Err(GPUError::NotFound)
    }

    fn draw(&mut self, _vertex_count: usize) -> Result<(), GPUError> {
        Ok(())
    }
}

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
    fn is_empty(&self) -> bool {
        self.len == 0
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

extern "C" {
    fn alloc(size: usize) -> *mut u8;
    fn free(ptr: *mut u8);
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
