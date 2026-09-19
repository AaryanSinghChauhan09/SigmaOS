// SPDX-License-Identifier: MIT
// SigmaOS Native DRM/KMS Framebuffer & Atomic Modesetting Engine
// Direct Rendering Manager (DRM) and Kernel Mode Setting (KMS) engine for display hardware

#![allow(dead_code)]

use std::vec::Vec;

/// Display Connector Types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DrmConnectorType {
    Unknown = 0,
    Vga = 1,
    DviI = 2,
    DviD = 3,
    DviA = 4,
    Composite = 5,
    SVideo = 6,
    LVDS = 7,
    Component = 8,
    NinePinDIN = 9,
    DisplayPort = 10,
    HdmiA = 11,
    HdmiB = 12,
    TV = 13,
    eDP = 14,
    Virtual = 15,
    DSI = 16,
}

/// DRM Display Mode Timings
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DrmDisplayMode {
    pub name: &'static str,
    pub clock_khz: u32,
    pub hdisplay: u16,
    pub hsync_start: u16,
    pub hsync_end: u16,
    pub htotal: u16,
    pub vdisplay: u16,
    pub vsync_start: u16,
    pub vsync_end: u16,
    pub vtotal: u16,
    pub vrefresh: u32,
}

impl DrmDisplayMode {
    pub const MODE_1080P60: Self = Self {
        name: "1920x1080@60Hz",
        clock_khz: 148500,
        hdisplay: 1920,
        hsync_start: 2008,
        hsync_end: 2052,
        htotal: 2200,
        vdisplay: 1080,
        vsync_start: 1084,
        vsync_end: 1089,
        vtotal: 1125,
        vrefresh: 60,
    };

    pub const MODE_4K60: Self = Self {
        name: "3840x2160@60Hz",
        clock_khz: 594000,
        hdisplay: 3840,
        hsync_start: 4016,
        hsync_end: 4104,
        htotal: 4400,
        vdisplay: 2160,
        vsync_start: 2168,
        vsync_end: 2178,
        vtotal: 2250,
        vrefresh: 60,
    };
}

/// DRM Framebuffer Descriptor
#[derive(Debug, Clone)]
pub struct DrmFramebuffer {
    pub fb_id: u32,
    pub width: u32,
    pub height: u32,
    pub pitch: u32,
    pub bpp: u8,
    pub depth: u8,
    pub handle: u32,
    pub physical_address: u64,
}

/// CRTC (Cathode Ray Tube Controller) State
#[derive(Debug, Clone)]
pub struct DrmCrtc {
    pub crtc_id: u32,
    pub active: bool,
    pub mode: DrmDisplayMode,
    pub current_fb_id: Option<u32>,
    pub gamma_lut_size: u32,
}

/// Plane (Primary, Overlay, Cursor) Descriptor
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DrmPlaneType {
    Primary = 0,
    Cursor = 1,
    Overlay = 2,
}

#[derive(Debug, Clone)]
pub struct DrmPlane {
    pub plane_id: u32,
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
}

/// DRM Connector
#[derive(Debug, Clone)]
pub struct DrmConnector {
    pub connector_id: u32,
    pub connector_type: DrmConnectorType,
    pub connected: bool,
    pub encoder_id: Option<u32>,
    pub supported_modes: Vec<DrmDisplayMode>,
}

/// DRM Atomic State Commitment Request
#[derive(Debug, Clone)]
pub struct DrmAtomicCommitReq {
    pub crtc_updates: Vec<(u32, DrmDisplayMode, u32)>, // (crtc_id, mode, fb_id)
    pub plane_updates: Vec<(u32, u32, i32, i32, u32, u32)>, // (plane_id, fb_id, x, y, w, h)
}

/// Sovereign Native DRM/KMS Driver Subsystem
#[derive(Debug)]
pub struct SovereignDrmKmsEngine {
    pub framebuffers: Vec<DrmFramebuffer>,
    pub crtcs: Vec<DrmCrtc>,
    pub planes: Vec<DrmPlane>,
    pub connectors: Vec<DrmConnector>,
    pub vblank_counter: u64,
}

impl SovereignDrmKmsEngine {
    pub fn new() -> Self {
        Self {
            framebuffers: Vec::new(),
            crtcs: Vec::new(),
            planes: Vec::new(),
            connectors: Vec::new(),
            vblank_counter: 0,
        }
    }

    pub fn initialize_gop_fallback(&mut self, base_addr: u64, width: u32, height: u32, pitch: u32) -> u32 {
        let fb_id = (self.framebuffers.len() as u32) + 1;
        let fb = DrmFramebuffer {
            fb_id,
            width,
            height,
            pitch,
            bpp: 32,
            depth: 24,
            handle: fb_id,
            physical_address: base_addr,
        };
        self.framebuffers.push(fb);

        let crtc = DrmCrtc {
            crtc_id: 1,
            active: true,
            mode: DrmDisplayMode::MODE_1080P60,
            current_fb_id: Some(fb_id),
            gamma_lut_size: 256,
        };
        self.crtcs.push(crtc);

        let plane = DrmPlane {
            plane_id: 1,
            plane_type: DrmPlaneType::Primary,
            crtc_id: Some(1),
            fb_id: Some(fb_id),
            src_x: 0,
            src_y: 0,
            src_w: width,
            src_h: height,
            crtc_x: 0,
            crtc_y: 0,
            crtc_w: width,
            crtc_h: height,
        };
        self.planes.push(plane);

        let connector = DrmConnector {
            connector_id: 1,
            connector_type: DrmConnectorType::eDP,
            connected: true,
            encoder_id: Some(1),
            supported_modes: vec![DrmDisplayMode::MODE_1080P60, DrmDisplayMode::MODE_4K60],
        };
        self.connectors.push(connector);

        fb_id
    }

    pub fn atomic_commit(&mut self, req: DrmAtomicCommitReq) -> Result<(), &'static str> {
        for (crtc_id, mode, fb_id) in req.crtc_updates {
            if let Some(crtc) = self.crtcs.iter_mut().find(|c| c.crtc_id == crtc_id) {
                crtc.mode = mode;
                crtc.current_fb_id = Some(fb_id);
            }
        }

        for (plane_id, fb_id, x, y, w, h) in req.plane_updates {
            if let Some(plane) = self.planes.iter_mut().find(|p| p.plane_id == plane_id) {
                plane.fb_id = Some(fb_id);
                plane.crtc_x = x;
                plane.crtc_y = y;
                plane.crtc_w = w;
                plane.crtc_h = h;
            }
        }

        self.vblank_counter += 1;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_drm_kms_gop_fallback_and_atomic_commit() {
        let mut engine = SovereignDrmKmsEngine::new();
        let fb_id = engine.initialize_gop_fallback(0xFD000000, 1920, 1080, 1920 * 4);
        assert_eq!(fb_id, 1);

        let req = DrmAtomicCommitReq {
            crtc_updates: vec![(1, DrmDisplayMode::MODE_1080P60, fb_id)],
            plane_updates: vec![(1, fb_id, 0, 0, 1920, 1080)],
        };
        assert!(engine.atomic_commit(req).is_ok());
        assert_eq!(engine.vblank_counter, 1);
    }
}
