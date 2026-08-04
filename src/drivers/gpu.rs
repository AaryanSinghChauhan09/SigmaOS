#![allow(clippy::all)]
#![allow(warnings)]

// SigmaOS GPU Driver
// Hardware abstraction for graphics rendering and advanced DRM/KMS modesetting

use crate::security::CapabilityToken;
extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

/// GPU command type
#[derive(Debug, Clone)]
pub enum GpuCommand {
    ClearScreen {
        r: u8,
        g: u8,
        b: u8,
    },
    DrawRect {
        x: u32,
        y: u32,
        width: u32,
        height: u32,
    },
    DrawText {
        x: u32,
        y: u32,
        text: String,
    },
    Present,
}

/// Linux/BSD-inspired DRM Mode Settings timing parameters
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DrmModeInfo {
    pub name: String,
    pub clock: u32,       // Pixel clock in kHz
    pub hdisplay: u16,    // Horizontal active resolution
    pub hsync_start: u16, // Horizontal sync start timing
    pub hsync_end: u16,   // Horizontal sync end timing
    pub htotal: u16,      // Horizontal total timing
    pub vdisplay: u16,    // Vertical active resolution
    pub vsync_start: u16, // Vertical sync start timing
    pub vsync_end: u16,   // Vertical sync end timing
    pub vtotal: u16,      // Vertical total timing
    pub vrefresh: u32,    // Vertical refresh rate in Hz
    pub flags: u32,       // Synchronization and signal flags
}

impl DrmModeInfo {
    /// Create a standard mode timing info from display resolution and refresh rate
    pub fn new_simple(width: u16, height: u16, refresh: u32) -> Self {
        let hdisplay = width;
        let hsync_start = hdisplay + 40;
        let hsync_end = hsync_start + 80;
        let htotal = hsync_end + 120;

        let vdisplay = height;
        let vsync_start = vdisplay + 3;
        let vsync_end = vsync_start + 6;
        let vtotal = vsync_end + 25;

        let clock = ((htotal as u32 * vtotal as u32 * refresh) as f32 / 1000.0) as u32;

        Self {
            name: alloc::format!("{}x{}@{}", width, height, refresh),
            clock,
            hdisplay,
            hsync_start,
            hsync_end,
            htotal,
            vdisplay,
            vsync_start,
            vsync_end,
            vtotal,
            vrefresh: refresh,
            flags: 0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DrmCrtc {
    pub id: u32,
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub active: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DrmConnector {
    pub id: u32,
    pub connected: bool,
    pub modes: Vec<DrmModeInfo>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DrmPlaneType {
    Primary,
    Overlay,
    Cursor,
}

/// DRM Plane representing hardware-supported visual compositing layers
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DrmPlane {
    pub id: u32,
    pub plane_type: DrmPlaneType,
    pub crtc_id: Option<u32>,
    pub fb_id: Option<u32>,
    pub src_x: u32,
    pub src_y: u32,
    pub src_w: u32,
    pub src_h: u32,
    pub crtc_x: i32,
    pub crtc_y: i32,
    pub crtc_w: u32,
    pub crtc_h: u32,
    pub zpos: i32,
    pub formats: Vec<String>,
}

impl DrmPlane {
    pub fn new(id: u32, plane_type: DrmPlaneType, formats: Vec<String>) -> Self {
        Self {
            id,
            plane_type,
            crtc_id: None,
            fb_id: None,
            src_x: 0,
            src_y: 0,
            src_w: 0,
            src_h: 0,
            crtc_x: 0,
            crtc_y: 0,
            crtc_w: 0,
            crtc_h: 0,
            zpos: match plane_type {
                DrmPlaneType::Primary => 0,
                DrmPlaneType::Overlay => 1,
                DrmPlaneType::Cursor => 2,
            },
            formats,
        }
    }
}

/// Atomic KMS State Commitment supporting transactional check-only and active updates
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DrmAtomicCommit {
    pub allow_modeset: bool,
    pub test_only: bool,
    pub plane_updates: Vec<PlaneUpdate>,
    pub crtc_updates: Vec<CrtcUpdate>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlaneUpdate {
    pub plane_id: u32,
    pub crtc_id: Option<u32>,
    pub fb_id: Option<u32>,
    pub src_x: u32,
    pub src_y: u32,
    pub src_w: u32,
    pub src_h: u32,
    pub crtc_x: i32,
    pub crtc_y: i32,
    pub crtc_w: u32,
    pub crtc_h: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CrtcUpdate {
    pub crtc_id: u32,
    pub active: bool,
    pub mode: Option<DrmModeInfo>,
}

pub struct GpuDriver {
    pub width: u32,
    pub height: u32,
    pub capabilities: CapabilityToken,
    pub frame_buffer: Vec<u32>,
    pub back_buffer: Vec<u32>, // Secondary framebuffer supporting double-buffered page-flip
    pub crtc: Option<DrmCrtc>,
    pub connector: Option<DrmConnector>,
    pub planes: Vec<DrmPlane>,
}

impl GpuDriver {
    pub fn new(width: u32, height: u32) -> Self {
        let size = (width * height) as usize;
        Self {
            width,
            height,
            capabilities: CapabilityToken::new(),
            frame_buffer: vec![0; size],
            back_buffer: vec![0; size],
            crtc: None,
            connector: None,
            planes: Vec::new(),
        }
    }

    pub fn execute_command(&mut self, command: GpuCommand) -> Result<(), GpuError> {
        match command {
            GpuCommand::ClearScreen { r, g, b } => {
                let color = ((r as u32) << 16) | ((g as u32) << 8) | (b as u32);
                self.frame_buffer.fill(color);
            }
            GpuCommand::DrawRect {
                x,
                y,
                width,
                height,
                ..
            } => {
                let color = 0xFFFFFF; // White
                for row in y..(y + height).min(self.height) {
                    for col in x..(x + width).min(self.width) {
                        let idx = (row * self.width + col) as usize;
                        if idx < self.frame_buffer.len() {
                            self.frame_buffer[idx] = color;
                        }
                    }
                }
            }
            GpuCommand::Present => {
                // Swaps buffer on presentation
                let _ = self.page_flip();
            }
            GpuCommand::DrawText { .. } => {
                // Text rendering implementation
            }
        }
        Ok(())
    }

    pub fn set_capabilities(&mut self, capabilities: CapabilityToken) {
        self.capabilities = capabilities;
    }

    pub fn has_capability(&self, capability: u64) -> bool {
        (self.capabilities.bits() & capability) != 0
    }

    /// DRM/KMS mode setting API
    pub fn set_drm_mode(
        &mut self,
        connector_id: u32,
        crtc_id: u32,
        mode: DrmModeInfo,
    ) -> Result<(), GpuError> {
        self.width = mode.hdisplay as u32;
        self.height = mode.vdisplay as u32;
        self.frame_buffer = vec![0; (self.width * self.height) as usize];
        self.back_buffer = vec![0; (self.width * self.height) as usize];

        self.crtc = Some(DrmCrtc {
            id: crtc_id,
            x: 0,
            y: 0,
            width: self.width,
            height: self.height,
            active: true,
        });

        self.connector = Some(DrmConnector {
            id: connector_id,
            connected: true,
            modes: vec![mode],
        });

        Ok(())
    }

    /// Perform a standard double-buffered page flip operation
    pub fn page_flip(&mut self) -> Result<(), GpuError> {
        if self.frame_buffer.len() != self.back_buffer.len() {
            return Err(GpuError::OutOfBounds);
        }
        core::mem::swap(&mut self.frame_buffer, &mut self.back_buffer);
        Ok(())
    }

    /// Execute atomic KMS check and commitment updates
    pub fn atomic_commit(&mut self, commit: &DrmAtomicCommit) -> Result<(), GpuError> {
        // Validation Stage (Check-Only)
        for p_up in &commit.plane_updates {
            let exists = self.planes.iter().any(|p| p.id == p_up.plane_id);
            if !exists {
                return Err(GpuError::InvalidCommand);
            }
        }

        for c_up in &commit.crtc_updates {
            if let Some(ref crtc) = self.crtc {
                if crtc.id != c_up.crtc_id {
                    return Err(GpuError::InvalidCommand);
                }
            } else {
                return Err(GpuError::InvalidCommand);
            }
        }

        if commit.test_only {
            return Ok(()); // Verified valid check
        }

        // Execution Stage (Commit Properties)
        for p_up in &commit.plane_updates {
            if let Some(plane) = self.planes.iter_mut().find(|p| p.id == p_up.plane_id) {
                plane.crtc_id = p_up.crtc_id;
                plane.fb_id = p_up.fb_id;
                plane.src_x = p_up.src_x;
                plane.src_y = p_up.src_y;
                plane.src_w = p_up.src_w;
                plane.src_h = p_up.src_h;
                plane.crtc_x = p_up.crtc_x;
                plane.crtc_y = p_up.crtc_y;
                plane.crtc_w = p_up.crtc_w;
                plane.crtc_h = p_up.crtc_h;
            }
        }

        for c_up in &commit.crtc_updates {
            if let Some(ref mut crtc) = self.crtc {
                crtc.active = c_up.active;
                if let Some(ref mode) = c_up.mode {
                    self.width = mode.hdisplay as u32;
                    self.height = mode.vdisplay as u32;
                    self.frame_buffer = vec![0; (self.width * self.height) as usize];
                    self.back_buffer = vec![0; (self.width * self.height) as usize];
                    crtc.width = self.width;
                    crtc.height = self.height;
                }
            }
        }

        Ok(())
    }
}

/// GPU errors
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GpuError {
    InvalidCommand,
    OutOfBounds,
    PermissionDenied,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gpu_creation() {
        let gpu = GpuDriver::new(1920, 1080);
        assert_eq!(gpu.width, 1920);
        assert_eq!(gpu.height, 1080);
    }

    #[test]
    fn test_clear_screen() {
        let mut gpu = GpuDriver::new(100, 100);
        let command = GpuCommand::ClearScreen { r: 255, g: 0, b: 0 };
        assert!(gpu.execute_command(command).is_ok());
    }

    #[test]
    fn test_draw_rect() {
        let mut gpu = GpuDriver::new(100, 100);
        let command = GpuCommand::DrawRect {
            x: 10,
            y: 10,
            width: 20,
            height: 20,
        };
        assert!(gpu.execute_command(command).is_ok());
    }

    #[test]
    fn test_drm_mode_settings_timings() {
        let mode = DrmModeInfo::new_simple(1920, 1080, 60);
        assert_eq!(mode.hdisplay, 1920);
        assert_eq!(mode.vdisplay, 1080);
        assert_eq!(mode.vrefresh, 60);
        assert!(mode.clock > 0);
    }

    #[test]
    fn test_drm_planes() {
        let plane = DrmPlane::new(1, DrmPlaneType::Primary, vec![String::from("ARGB8888")]);
        assert_eq!(plane.plane_type, DrmPlaneType::Primary);
        assert_eq!(plane.zpos, 0);
    }

    #[test]
    fn test_atomic_modeset_commit() {
        let mut gpu = GpuDriver::new(800, 600);
        gpu.set_drm_mode(1, 42, DrmModeInfo::new_simple(800, 600, 60)).unwrap();

        gpu.planes.push(DrmPlane::new(10, DrmPlaneType::Cursor, vec![String::from("ARGB8888")]));

        let commit = DrmAtomicCommit {
            allow_modeset: true,
            test_only: false,
            plane_updates: vec![PlaneUpdate {
                plane_id: 10,
                crtc_id: Some(42),
                fb_id: Some(101),
                src_x: 0,
                src_y: 0,
                src_w: 64,
                src_h: 64,
                crtc_x: 10,
                crtc_y: 10,
                crtc_w: 64,
                crtc_h: 64,
            }],
            crtc_updates: vec![CrtcUpdate {
                crtc_id: 42,
                active: true,
                mode: Some(DrmModeInfo::new_simple(1024, 768, 60)),
            }],
        };

        let result = gpu.atomic_commit(&commit);
        assert!(result.is_ok());
        assert_eq!(gpu.width, 1024);
        assert_eq!(gpu.height, 768);
        assert_eq!(gpu.planes[0].fb_id, Some(101));
    }
}
