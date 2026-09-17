use std::boxed::Box;
use std::string::{String, ToString};
use std::vec::Vec;
use std::format;

/// OOP-based GPU Driver for SigmaOS
/// Based on Ideas-999-Structured: Kernel & Hardware Item 71
/// Implements GPU device management and rendering

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type GPUDeviceID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum GPUVendor { Intel = 0, AMD = 1, NVIDIA = 2, Other = 3 }

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum GPUError { Success = 0, NotFound = 1, InitFailed = 2, RenderFailed = 3 }

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
    fn id(&self) -> GPUDeviceID { self.id }
    fn vendor(&self) -> GPUVendor {
        match self.vendor.load(Ordering::SeqCst) {
            0 => GPUVendor::Intel,
            1 => GPUVendor::AMD,
            2 => GPUVendor::NVIDIA,
            _ => GPUVendor::Other,
        }
    }
    fn model(&self) -> &[u8] {
        let len = self.model.iter().position(|&b| b == 0).unwrap_or(64);
        &self.model[..len]
    }
    fn vram_size(&self) -> usize { self.vram_size.load(Ordering::SeqCst) }

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
            if let Some(ref gpu) = self.gpus.data()[0] {
                return Some(gpu.as_ref());
            }
        }
        None
    }

    fn list_gpus(&self) -> Vec<GPUDeviceID> {
        let mut ids = Vec::new();
        for i in 0..self.gpus.len() {
            if let Some(ref gpu) = self.gpus.data()[i] {
                ids.push(gpu.id());
            }
        }
        ids
    }
}

pub trait Framebuffer {
    fn create_framebuffer(&mut self, width: usize, height: usize, format: u32) -> Result<usize, GPUError>;
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
    pub fn new() -> Self {
        SimpleFramebuffer {
            framebuffers: Vec::new(),
            current: AtomicUsize::new(0),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl Framebuffer for SimpleFramebuffer {
    fn create_framebuffer(&mut self, width: usize, height: usize, format: u32) -> Result<usize, GPUError> {
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        self.framebuffers.push((id, width, height, format));
        Ok(id)
    }

    fn bind_framebuffer(&mut self, fb_id: usize) -> Result<(), GPUError> {
        for i in 0..self.framebuffers.len() {
            let &(id, _, _, _) = &self.framebuffers.data()[i];
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
    fn create_pipeline(&mut self, vertex_shader: &[u8], fragment_shader: &[u8]) -> Result<usize, GPUError>;
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
    pub fn new() -> Self {
        SimpleRenderPipeline {
            pipelines: Vec::new(),
            current: AtomicUsize::new(0),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl RenderPipeline for SimpleRenderPipeline {
    fn create_pipeline(&mut self, vertex_shader: &[u8], fragment_shader: &[u8]) -> Result<usize, GPUError> {
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        let mut vs_array = [0u8; 256];
        let mut fs_array = [0u8; 256];
        let vs_len = vertex_shader.len().min(255);
        let fs_len = fragment_shader.len().min(255);
        for i in 0..vs_len { vs_array[i] = vertex_shader[i]; }
        for i in 0..fs_len { fs_array[i] = fragment_shader[i]; }
        self.pipelines.push((id, vs_array, fs_array));
        Ok(id)
    }

    fn bind_pipeline(&mut self, pipeline_id: usize) -> Result<(), GPUError> {
        for i in 0..self.pipelines.len() {
            let &(id, _, _) = &self.pipelines.data()[i];
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

