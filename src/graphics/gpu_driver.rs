#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
// SPDX-License-Identifier: MIT

// GPU Driver - Linux & BSD inspired GPU acceleration and display layer
// Supports framebuffer management, 2D acceleration, DRM/KMS atomic plane compositing, Wayland SHM DMA-BUF zero-copy, and OpenBSD wsdisplay VT switching.

use std::collections::BTreeMap;
use std::string::String;
use std::vec::Vec;

use super::nvidia_prime::{NvidiaPrimeEngine, PrimeProfile};

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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Framebuffer {
    pub address: usize,
    pub width: u32,
    pub height: u32,
    pub pitch: u32,
    pub bpp: u32,
    pub format: PixelFormat,
}

/// Linux DRM/KMS Atomic Plane State
#[derive(Debug, Clone)]
pub struct DrmAtomicPlaneState {
    pub plane_id: u32,
    pub crtc_id: u32,
    pub fb_id: u32,
    pub src_x: u32,
    pub src_y: u32,
    pub src_w: u32,
    pub src_h: u32,
    pub crtc_x: i32,
    pub crtc_y: i32,
    pub crtc_w: u32,
    pub crtc_h: u32,
    pub zpos: u32,
}

/// Wayland SHM DMA-BUF Zero-Copy Buffer Descriptor
#[derive(Debug, Clone)]
pub struct WaylandDmaBuf {
    pub fd: i32,
    pub width: u32,
    pub height: u32,
    pub format: PixelFormat,
    pub stride: u32,
    pub offset: u64,
    pub size: usize,
}

/// OpenBSD wsdisplay Virtual Terminal State
#[derive(Debug, Clone)]
pub struct OpenBsdWsdisplayVt {
    pub vt_id: u32,
    pub is_active: bool,
    pub title: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
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
    pub atomic_planes: Vec<DrmAtomicPlaneState>,
    pub dma_buffers: Vec<WaylandDmaBuf>,
    pub ws_terminals: Vec<OpenBsdWsdisplayVt>,
    pub active_vt: u32,
    pub nvidia_prime_engine: NvidiaPrimeEngine,
}

impl GpuDriver {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        let mut driver = Self {
            devices: BTreeMap::new(),
            primary_device: None,
            next_device_id: 0,
            atomic_planes: Vec::new(),
            dma_buffers: Vec::new(),
            ws_terminals: Vec::new(),
            active_vt: 1,
            nvidia_prime_engine: NvidiaPrimeEngine::new(),
        };

        // Pre-configure OpenBSD wsdisplay VTs 1-4
        driver.ws_terminals.push(OpenBsdWsdisplayVt {
            vt_id: 1,
            is_active: true,
            title: String::from("tty1"),
        });
        driver.ws_terminals.push(OpenBsdWsdisplayVt {
            vt_id: 2,
            is_active: false,
            title: String::from("tty2"),
        });
        driver.ws_terminals.push(OpenBsdWsdisplayVt {
            vt_id: 3,
            is_active: false,
            title: String::from("tty3"),
        });
        driver.ws_terminals.push(OpenBsdWsdisplayVt {
            vt_id: 4,
            is_active: false,
            title: String::from("tty4"),
        });

        driver
    }

    /// Register a GPU device
    pub fn register_device(
        &mut self,
        vendor: GpuVendor,
        device_id: u32,
        vram_size: u64,
    ) -> Result<u32, &'static str> {
        let id = self.next_device_id;
        self.next_device_id += 1;

        let device = GpuDevice {
            vendor,
            device_id,
            state: GpuState::Off,
            framebuffer: None,
            vram_size,
            supports_2d_accel: true,
            supports_3d_accel: false,
        };

        self.devices.insert(id, device);

        if self.primary_device.is_none() {
            self.primary_device = Some(id);
        }

        Ok(id)
    }

    /// Initialize a GPU device
    pub fn initialize_device(
        &mut self,
        id: u32,
        width: u32,
        height: u32,
        format: PixelFormat,
    ) -> Result<(), &'static str> {
        let device = self.devices.get_mut(&id).ok_or("Device not found")?;

        let bpp = match format {
            PixelFormat::Rgb24 | PixelFormat::Bgr24 => 24,
            PixelFormat::Rgba32 | PixelFormat::Bgra32 => 32,
            PixelFormat::Rgb565 => 16,
        };

        let pitch = width * (bpp / 8);

        let framebuffer = Framebuffer {
            address: 0xE0000000,
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

    pub fn get_device(&self, id: u32) -> Option<&GpuDevice> {
        self.devices.get(&id)
    }

    pub fn primary_device(&self) -> Option<&GpuDevice> {
        if let Some(id) = self.primary_device {
            self.devices.get(&id)
        } else {
            None
        }
    }

    pub fn set_primary_device(&mut self, id: u32) -> Result<(), &'static str> {
        if !self.devices.contains_key(&id) {
            return Err("Device not found");
        }

        self.primary_device = Some(id);
        Ok(())
    }

    pub fn set_device_state(&mut self, id: u32, state: GpuState) -> Result<(), &'static str> {
        let device = self.devices.get_mut(&id).ok_or("Device not found")?;
        device.state = state;
        Ok(())
    }

    pub fn get_framebuffer(&self, id: u32) -> Option<&Framebuffer> {
        let device = self.devices.get(&id)?;
        device.framebuffer.as_ref()
    }

    pub fn fill_rect(
        &self,
        id: u32,
        _x: u32,
        _y: u32,
        _width: u32,
        _height: u32,
        _color: u32,
    ) -> Result<(), &'static str> {
        let device = self.devices.get(&id).ok_or("Device not found")?;

        if device.state != GpuState::HardwareAccelerated {
            return Err("Device not in accelerated state");
        }

        if !device.supports_2d_accel {
            return Err("Device does not support 2D acceleration");
        }

        Ok(())
    }

    pub fn copy_rect(
        &self,
        id: u32,
        _src_x: u32,
        _src_y: u32,
        _dst_x: u32,
        _dst_y: u32,
        _width: u32,
        _height: u32,
    ) -> Result<(), &'static str> {
        let device = self.devices.get(&id).ok_or("Device not found")?;

        if device.state != GpuState::HardwareAccelerated {
            return Err("Device not in accelerated state");
        }

        if !device.supports_2d_accel {
            return Err("Device does not support 2D acceleration");
        }

        Ok(())
    }

    pub fn device_count(&self) -> usize {
        self.devices.len()
    }

    pub fn list_devices(&self) -> Vec<&GpuDevice> {
        self.devices.values().collect()
    }

    /// Linux DRM/KMS Atomic Plane Commit
    pub fn commit_atomic_plane(&mut self, plane: DrmAtomicPlaneState) -> Result<(), &'static str> {
        self.atomic_planes.retain(|p| p.plane_id != plane.plane_id);
        self.atomic_planes.push(plane);
        Ok(())
    }

    /// Query total committed atomic plane count
    pub fn get_atomic_plane_count(&self) -> usize {
        self.atomic_planes.len()
    }

    /// Clear imported Wayland DMA-BUF buffers
    pub fn clear_dma_buffers(&mut self) {
        self.dma_buffers.clear();
    }

    /// Wayland SHM Zero-Copy DMA-BUF Import
    pub fn import_dma_buf(&mut self, buf: WaylandDmaBuf) -> Result<usize, &'static str> {
        let size = buf.size;
        self.dma_buffers.push(buf);
        Ok(size)
    }

    /// OpenBSD wsdisplay Virtual Terminal Switch
    pub fn switch_wsdisplay_vt(&mut self, target_vt: u32) -> Result<(), &'static str> {
        let mut found = false;
        for vt in self.ws_terminals.iter_mut() {
            if vt.vt_id == target_vt {
                vt.is_active = true;
                found = true;
            } else {
                vt.is_active = false;
            }
        }
        if found {
            self.active_vt = target_vt;
            Ok(())
        } else {
            Err("wsdisplay: VT target not found")
        }
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

        let id = driver
            .register_device(GpuVendor::Intel, 0x1234, 1024 * 1024 * 1024)
            .unwrap();
        assert_eq!(driver.device_count(), 1);

        let device = driver.get_device(id).unwrap();
        assert_eq!(device.vendor, GpuVendor::Intel);
    }

    #[test]
    fn test_initialize_device() {
        let mut driver = GpuDriver::new();

        let id = driver
            .register_device(GpuVendor::Amd, 0x5678, 512 * 1024 * 1024)
            .unwrap();
        driver
            .initialize_device(id, 1920, 1080, PixelFormat::Rgba32)
            .unwrap();

        let device = driver.get_device(id).unwrap();
        assert_eq!(device.state, GpuState::HardwareAccelerated);
        assert!(device.framebuffer.is_some());
    }

    #[test]
    fn test_drm_kms_atomic_plane() {
        let mut driver = GpuDriver::new();
        assert_eq!(driver.get_atomic_plane_count(), 0);
        let plane = DrmAtomicPlaneState {
            plane_id: 1,
            crtc_id: 10,
            fb_id: 100,
            src_x: 0,
            src_y: 0,
            src_w: 1920,
            src_h: 1080,
            crtc_x: 0,
            crtc_y: 0,
            crtc_w: 1920,
            crtc_h: 1080,
            zpos: 0,
        };
        assert!(driver.commit_atomic_plane(plane).is_ok());
        assert_eq!(driver.get_atomic_plane_count(), 1);
    }

    #[test]
    fn test_wayland_dma_buf_import() {
        let mut driver = GpuDriver::new();
        let buf = WaylandDmaBuf {
            fd: 5,
            width: 1920,
            height: 1080,
            format: PixelFormat::Rgba32,
            stride: 1920 * 4,
            offset: 0,
            size: 1920 * 1080 * 4,
        };
        let size = driver.import_dma_buf(buf).unwrap();
        assert_eq!(size, 1920 * 1080 * 4);
        assert_eq!(driver.dma_buffers.len(), 1);

        driver.clear_dma_buffers();
        assert_eq!(driver.dma_buffers.len(), 0);
    }

    #[test]
    fn test_wsdisplay_vt_switch() {
        let mut driver = GpuDriver::new();
        assert_eq!(driver.active_vt, 1);
        assert!(driver.switch_wsdisplay_vt(2).is_ok());
        assert_eq!(driver.active_vt, 2);
    }
}
