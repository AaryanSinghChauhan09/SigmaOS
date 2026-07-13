// GPU Drivers Module
// Implements GPU drivers for Intel i915, AMD amdgpu, and NVIDIA nouveau
// Zero-dependency Rust implementation for SigmaOS

pub mod i915;
pub mod amdgpu;
pub mod nouveau;

pub use i915::I915Driver;
pub use amdgpu::AmdgpuDriver;
pub use nouveau::NouveauDriver;

// Common GPU types
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum GpuVendor {
    Intel,
    AMD,
    NVIDIA,
    Unknown,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GpuInfo {
    pub vendor: GpuVendor,
    pub device_id: u32,
    pub vram_size: u64,
    pub has_acceleration: bool,
}

// Common GPU trait
pub trait GpuDriver {
    fn detect(&self) -> Option<GpuInfo>;
    fn initialize(&mut self) -> Result<(), GpuError>;
    fn set_mode(&mut self, width: u32, height: u32, bpp: u32) -> Result<(), GpuError>;
    fn get_info(&self) -> GpuInfo;
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum GpuError {
    NotFound,
    InitializationFailed,
    InvalidMode,
    Unsupported,
}
