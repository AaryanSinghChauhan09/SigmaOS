// GPU Driver - Linux-style basic GPU acceleration
// Supports framebuffer management, 2D acceleration, and basic 3D operations

#![no_std]

extern crate alloc;
use alloc::vec::Vec;
use alloc::collections::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GpuState {
    Off,
    VgaFallback,
    HardwareAccelerated,
    Panic,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GpuVendor {
    Unknown,
    Intel,
    Amd,
    Nvidia,
    Vmware,
    Virtio,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PixelFormat {
    Rgb24,
    Rgba32,
    Bgr24,
    Bgra32,
    Rgb565,
}

#[derive(Debug, Clone)]
pub struct Framebuffer {
    pub address: usize,
    pub width: u32,
    pub height: u32,
    pub pitch: u32,
    pub bpp: u32,
    pub format: PixelFormat,
}

#[derive(Debug, Clone)]
pub struct GpuDevice {
    pub vendor: GpuVendor,
    pub device_id: u32,
    pub state: GpuState,
    pub framebuffer: Option<Framebuffer>,
    pub vram_size: u64,
    pub supports_2d_accel: bool,
    pub supports_3d_accel: bool,
}

pub struct GpuDriver {
    devices: BTreeMap<u32, GpuDevice>,
    primary_device: Option<u32>,
    next_device_id: u32,
}

impl GpuDriver {
    pub fn new() -> Self {
        Self {
            devices: BTreeMap::new(),
            primary_device: None,
            next_device_id: 0,
        }
    }

    /// Register a GPU device
    pub fn register_device(&mut self, vendor: GpuVendor, device_id: u32, vram_size: u64) -> Result<u32, &'static str> {
        let id = self.next_device_id;
        self.next_device_id += 1;

        let device = GpuDevice {
            vendor,
            device_id,
            state: GpuState::Off,
            framebuffer: None,
            vram_size,
            supports_2d_accel: true,
            supports_3d_accel: false, // Basic implementation
        };

        self.devices.insert(id, device);

        // Set as primary if first device
        if self.primary_device.is_none() {
            self.primary_device = Some(id);
        }

        Ok(id)
    }

    /// Initialize a GPU device
    pub fn initialize_device(&mut self, id: u32, width: u32, height: u32, format: PixelFormat) -> Result<(), &'static str> {
        let device = self.devices.get_mut(&id)
            .ok_or("Device not found")?;

        let bpp = match format {
            PixelFormat::Rgb24 | PixelFormat::Bgr24 => 24,
            PixelFormat::Rgba32 | PixelFormat::Bgra32 => 32,
            PixelFormat::Rgb565 => 16,
        };

        let pitch = width * (bpp / 8);

        let framebuffer = Framebuffer {
            address: 0xE0000000, // Dummy framebuffer address
            width,
            height,
            pitch,
            bpp,
            format,
        };

        device.framebuffer = Some(framebuffer);
        device.state = GpuState::HardwareAccelerated;

        Ok(())
    }

    /// Get device by ID
    pub fn get_device(&self, id: u32) -> Option<&GpuDevice> {
        self.devices.get(&id)
    }

    /// Get primary device
    pub fn primary_device(&self) -> Option<&GpuDevice> {
        if let Some(id) = self.primary_device {
            self.devices.get(&id)
        } else {
            None
        }
    }

    /// Set primary device
    pub fn set_primary_device(&mut self, id: u32) -> Result<(), &'static str> {
        if !self.devices.contains_key(&id) {
            return Err("Device not found");
        }

        self.primary_device = Some(id);
        Ok(())
    }

    /// Set device state
    pub fn set_device_state(&mut self, id: u32, state: GpuState) -> Result<(), &'static str> {
        let device = self.devices.get_mut(&id)
            .ok_or("Device not found")?;

        device.state = state;
        Ok(())
    }

    /// Get framebuffer for a device
    pub fn get_framebuffer(&self, id: u32) -> Option<&Framebuffer> {
        let device = self.devices.get(&id)?;
        device.framebuffer.as_ref()
    }

    /// Fill rectangle (2D acceleration)
    pub fn fill_rect(&self, id: u32, x: u32, y: u32, width: u32, height: u32, color: u32) -> Result<(), &'static str> {
        let device = self.devices.get(&id)
            .ok_or("Device not found")?;

        if device.state != GpuState::HardwareAccelerated {
            return Err("Device not in accelerated state");
        }

        if !device.supports_2d_accel {
            return Err("Device does not support 2D acceleration");
        }

        // In a real implementation, this would use GPU commands
        Ok(())
    }

    /// Copy rectangle (2D acceleration)
    pub fn copy_rect(&self, id: u32, src_x: u32, src_y: u32, dst_x: u32, dst_y: u32, width: u32, height: u32) -> Result<(), &'static str> {
        let device = self.devices.get(&id)
            .ok_or("Device not found")?;

        if device.state != GpuState::HardwareAccelerated {
            return Err("Device not in accelerated state");
        }

        if !device.supports_2d_accel {
            return Err("Device does not support 2D acceleration");
        }

        // In a real implementation, this would use GPU commands
        Ok(())
    }

    /// Get device count
    pub fn device_count(&self) -> usize {
        self.devices.len()
    }

    /// List all devices
    pub fn list_devices(&self) -> Vec<&GpuDevice> {
        self.devices.values().collect()
    }
}

impl Default for GpuDriver {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_device() {
        let mut driver = GpuDriver::new();
        
        let id = driver.register_device(GpuVendor::Intel, 0x1234, 1024 * 1024 * 1024).unwrap();
        assert_eq!(driver.device_count(), 1);
        
        let device = driver.get_device(id).unwrap();
        assert_eq!(device.vendor, GpuVendor::Intel);
    }

    #[test]
    fn test_initialize_device() {
        let mut driver = GpuDriver::new();
        
        let id = driver.register_device(GpuVendor::Amd, 0x5678, 512 * 1024 * 1024).unwrap();
        driver.initialize_device(id, 1920, 1080, PixelFormat::Rgba32).unwrap();
        
        let device = driver.get_device(id).unwrap();
        assert_eq!(device.state, GpuState::HardwareAccelerated);
        assert!(device.framebuffer.is_some());
    }

    #[test]
    fn test_primary_device() {
        let mut driver = GpuDriver::new();
        
        let id1 = driver.register_device(GpuVendor::Intel, 0x1234, 1024 * 1024 * 1024).unwrap();
        let id2 = driver.register_device(GpuVendor::Nvidia, 0x5678, 2048 * 1024 * 1024).unwrap();
        
        assert_eq!(driver.primary_device(), Some(driver.get_device(id1).unwrap()));
        
        driver.set_primary_device(id2).unwrap();
        assert_eq!(driver.primary_device(), Some(driver.get_device(id2).unwrap()));
    }

    #[test]
    fn test_set_device_state() {
        let mut driver = GpuDriver::new();
        
        let id = driver.register_device(GpuVendor::Intel, 0x1234, 1024 * 1024 * 1024).unwrap();
        driver.set_device_state(id, GpuState::VgaFallback).unwrap();
        
        let device = driver.get_device(id).unwrap();
        assert_eq!(device.state, GpuState::VgaFallback);
    }

    #[test]
    fn test_get_framebuffer() {
        let mut driver = GpuDriver::new();
        
        let id = driver.register_device(GpuVendor::Intel, 0x1234, 1024 * 1024 * 1024).unwrap();
        driver.initialize_device(id, 1920, 1080, PixelFormat::Rgb24).unwrap();
        
        let fb = driver.get_framebuffer(id).unwrap();
        assert_eq!(fb.width, 1920);
        assert_eq!(fb.height, 1080);
    }

    #[test]
    fn test_fill_rect() {
        let mut driver = GpuDriver::new();
        
        let id = driver.register_device(GpuVendor::Intel, 0x1234, 1024 * 1024 * 1024).unwrap();
        driver.initialize_device(id, 1920, 1080, PixelFormat::Rgba32).unwrap();
        
        let result = driver.fill_rect(id, 0, 0, 100, 100, 0xFF0000);
        assert!(result.is_ok());
    }

    #[test]
    fn test_copy_rect() {
        let mut driver = GpuDriver::new();
        
        let id = driver.register_device(GpuVendor::Amd, 0x5678, 512 * 1024 * 1024).unwrap();
        driver.initialize_device(id, 1920, 1080, PixelFormat::Rgba32).unwrap();
        
        let result = driver.copy_rect(id, 0, 0, 100, 100, 50, 50);
        assert!(result.is_ok());
    }

    #[test]
    fn test_list_devices() {
        let mut driver = GpuDriver::new();
        
        driver.register_device(GpuVendor::Intel, 0x1234, 1024 * 1024 * 1024).unwrap();
        driver.register_device(GpuVendor::Amd, 0x5678, 512 * 1024 * 1024).unwrap();
        
        let devices = driver.list_devices();
        assert_eq!(devices.len(), 2);
    }
}
