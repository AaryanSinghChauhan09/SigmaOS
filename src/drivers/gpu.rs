// SigmaOS GPU Driver
// Hardware abstraction for graphics rendering

extern crate alloc;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;

use crate::security::CapabilityToken;

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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DrmModeInfo {
    pub hdisplay: u16,
    pub vdisplay: u16,
    pub refresh: u16,
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct DrmCrtc {
    pub id: u32,
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub active: bool,
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct DrmPlane {
    pub id: u32,
    pub plane_type: DrmPlaneType,
    pub possible_crtcs: u32, // bitmask of supported CRTC IDs
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

#[derive(Debug, Clone)]
pub struct DrmEncoder {
    pub id: u32,
    pub encoder_type: u32, // HDMI, DP, TMDS, etc.
    pub possible_crtcs: u32,
    pub crtc_id: Option<u32>,
}

#[derive(Debug, Clone)]
pub struct DrmFramebuffer {
    pub id: u32,
    pub width: u32,
    pub height: u32,
    pub pitch: u32,
    pub bpp: u32,
    pub handle: u32, // GEM handle representation
    pub buffer: Vec<u32>,
}

pub struct GpuDriver {
    pub width: u32,
    pub height: u32,
    pub capabilities: CapabilityToken,
    pub frame_buffer: Vec<u32>,
    pub crtc: Option<DrmCrtc>,
    pub connector: Option<DrmConnector>,

    // Dynamic DRM/KMS resource tables
    pub planes: Vec<DrmPlane>,
    pub encoders: Vec<DrmEncoder>,
    pub connectors: Vec<DrmConnector>,
    pub framebuffers: Vec<DrmFramebuffer>,
    pub next_object_id: u32,
    pub vblank_count: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DrmError {
    Success,
    InvalidObject,
    InvalidMode,
    InvalidProperty,
    Busy,
    NoMemory,
    AtomicTestFailed,
}

#[derive(Debug, Clone)]
pub struct DrmPropertyUpdate {
    pub object_id: u32,
    pub name: String,
    pub value: u64,
}

#[derive(Debug, Clone)]
pub struct AtomicCommit {
    pub updates: Vec<DrmPropertyUpdate>,
    pub test_only: bool,
    pub nonblock: bool,
}

impl AtomicCommit {
    pub fn new() -> Self {
        Self {
            updates: Vec::new(),
            test_only: false,
            nonblock: false,
        }
    }

    pub fn add_property(&mut self, object_id: u32, name: &str, value: u64) {
        self.updates.push(DrmPropertyUpdate {
            object_id,
            name: name.to_string(),
            value,
        });
    }

    pub fn set_test_only(&mut self, test_only: bool) {
        self.test_only = test_only;
    }

    pub fn set_nonblock(&mut self, nonblock: bool) {
        self.nonblock = nonblock;
    }
}

impl Default for AtomicCommit {
    fn default() -> Self {
        Self::new()
    }
}

impl GpuDriver {
    pub fn new(width: u32, height: u32) -> Self {
        let size = (width * height) as usize;
        Self {
            width,
            height,
            capabilities: CapabilityToken::new(),
            frame_buffer: vec![0; size],
            crtc: None,
            connector: None,
            planes: Vec::new(),
            encoders: Vec::new(),
            connectors: Vec::new(),
            framebuffers: Vec::new(),
            next_object_id: 1,
            vblank_count: 0,
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
                // Buffer swapping simulation
            }
            GpuCommand::DrawText { .. } => {
                // Text rendering simulation
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

    /// Backwards-compatible legacy DRM/KMS mode setting API
    pub fn set_drm_mode(
        &mut self,
        connector_id: u32,
        crtc_id: u32,
        mode: DrmModeInfo,
    ) -> Result<(), GpuError> {
        self.width = mode.hdisplay as u32;
        self.height = mode.vdisplay as u32;
        self.frame_buffer = vec![0; (self.width * self.height) as usize];

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

    // --- Advanced Linux/BSD-inspired DRM/KMS APIs ---

    /// Helper to identify object types in our resource tables
    pub fn find_object_type(&self, id: u32) -> Option<&'static str> {
        if let Some(ref c) = self.crtc {
            if c.id == id { return Some("crtc"); }
        }
        if let Some(ref conn) = self.connector {
            if conn.id == id { return Some("connector"); }
        }
        for p in &self.planes {
            if p.id == id { return Some("plane"); }
        }
        for e in &self.encoders {
            if e.id == id { return Some("encoder"); }
        }
        for conn in &self.connectors {
            if conn.id == id { return Some("connector"); }
        }
        for fb in &self.framebuffers {
            if fb.id == id { return Some("framebuffer"); }
        }
        None
    }

    /// Allocate a GEM dumb buffer backed framebuffer
    pub fn create_dumb_buffer(&mut self, width: u32, height: u32, bpp: u32) -> Result<u32, DrmError> {
        if width == 0 || height == 0 || (bpp != 32 && bpp != 24 && bpp != 16) {
            return Err(DrmError::InvalidProperty);
        }
        let size = (width * height) as usize;
        let id = self.next_object_id;
        self.next_object_id += 1;

        let framebuffer = DrmFramebuffer {
            id,
            width,
            height,
            pitch: width * (bpp / 8),
            bpp,
            handle: id + 100, // mock GEM handle
            buffer: vec![0; size],
        };

        self.framebuffers.push(framebuffer);
        Ok(id)
    }

    /// Apply a single property change in-place
    pub fn apply_property_update(&mut self, id: u32, prop: &str, val: u64) -> Result<(), DrmError> {
        let obj_type = self.find_object_type(id).ok_or(DrmError::InvalidObject)?;

        match obj_type {
            "crtc" => {
                if let Some(ref mut crtc) = self.crtc {
                    if prop == "active" {
                        crtc.active = val != 0;
                        return Ok(());
                    }
                }
            }
            "plane" => {
                for plane in &mut self.planes {
                    if plane.id == id {
                        match prop {
                            "crtc_id" => {
                                plane.crtc_id = if val == 0 { None } else { Some(val as u32) };
                                return Ok(());
                            }
                            "fb_id" => {
                                plane.fb_id = if val == 0 { None } else { Some(val as u32) };
                                return Ok(());
                            }
                            "src_x" => { plane.src_x = val as u32; return Ok(()); }
                            "src_y" => { plane.src_y = val as u32; return Ok(()); }
                            "src_w" => { plane.src_w = val as u32; return Ok(()); }
                            "src_h" => { plane.src_h = val as u32; return Ok(()); }
                            _ => return Err(DrmError::InvalidProperty),
                        }
                    }
                }
            }
            "connector" => {
                if prop == "DPMS" {
                    // Normalizing DPMS values: 0 = On, 3 = Off
                    if val <= 3 {
                        return Ok(());
                    }
                    return Err(DrmError::InvalidProperty);
                }
            }
            _ => return Err(DrmError::InvalidProperty),
        }

        Err(DrmError::InvalidProperty)
    }

    /// Linux/BSD styled Atomic Commit transaction processing
    pub fn commit_atomic(&mut self, commit: &AtomicCommit) -> Result<(), DrmError> {
        // 1. Dry-run validation phase
        for update in &commit.updates {
            let obj_type = self.find_object_type(update.object_id).ok_or(DrmError::InvalidObject)?;
            match obj_type {
                "crtc" => {
                    if update.name != "active" {
                        return Err(DrmError::AtomicTestFailed);
                    }
                }
                "plane" => {
                    let valid_prop = matches!(
                        update.name.as_str(),
                        "crtc_id" | "fb_id" | "src_x" | "src_y" | "src_w" | "src_h"
                    );
                    if !valid_prop {
                        return Err(DrmError::AtomicTestFailed);
                    }
                }
                "connector" => {
                    if update.name != "DPMS" {
                        return Err(DrmError::AtomicTestFailed);
                    }
                }
                _ => return Err(DrmError::AtomicTestFailed),
            }
        }

        if commit.test_only {
            return Ok(()); // Atomic validation check dry-run successful
        }

        // 2. Commit apply phase
        for update in &commit.updates {
            self.apply_property_update(update.object_id, &update.name, update.value)?;
        }

        Ok(())
    }

    /// Read and parse EDID of a monitor connected to a connector
    pub fn parse_connector_edid(&mut self, connector_id: u32, edid_bytes: &[u8]) -> Result<(), DrmError> {
        let modes = parse_edid(edid_bytes).map_err(|_| DrmError::InvalidMode)?;

        let mut found = false;
        if let Some(ref mut connector) = self.connector {
            if connector.id == connector_id {
                connector.modes = modes.clone();
                connector.connected = true;
                found = true;
            }
        }

        for conn in &mut self.connectors {
            if conn.id == connector_id {
                conn.modes = modes.clone();
                conn.connected = true;
                found = true;
            }
        }

        if !found {
            return Err(DrmError::InvalidObject);
        }

        Ok(())
    }

    /// Double-buffered Page Flip synced to vertical blanking intervals
    pub fn execute_page_flip(&mut self, crtc_id: u32, fb_id: u32) -> Result<(), DrmError> {
        let crtc_match = if let Some(ref crtc) = self.crtc {
            crtc.id == crtc_id && crtc.active
        } else {
            false
        };

        if !crtc_match {
            return Err(DrmError::InvalidObject);
        }

        // Find framebuffer
        let mut fb_idx = None;
        for (i, fb) in self.framebuffers.iter().enumerate() {
            if fb.id == fb_id {
                fb_idx = Some(i);
                break;
            }
        }

        let idx = fb_idx.ok_or(DrmError::InvalidObject)?;
        let fb = &self.framebuffers[idx];

        // Perform atomic pointer/buffer flip
        self.width = fb.width;
        self.height = fb.height;
        self.frame_buffer = fb.buffer.clone();

        // Increment hardware vertical blank count
        self.vblank_count += 1;

        Ok(())
    }
}

/// Standard 128-byte EDID V1.x Structure Parser (established and detailed timings decoders)
pub fn parse_edid(edid: &[u8]) -> Result<Vec<DrmModeInfo>, &'static str> {
    if edid.len() < 128 {
        return Err("EDID must be at least 128 bytes");
    }

    // Verify standard EDID magic header: 00 FF FF FF FF FF FF 00
    let magic = [0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00];
    if edid[0..8] != magic {
        return Err("Invalid EDID magic header");
    }

    // Sum validation of the entire block (must be 0 modulo 256)
    let mut checksum: u8 = 0;
    for &byte in edid {
        checksum = checksum.wrapping_add(byte);
    }
    if checksum != 0 {
        return Err("EDID block checksum invalid");
    }

    let mut modes = Vec::new();

    // Established Timings I & II (Bytes 38..40)
    // Byte 38 bit 7: 800x600 @ 60Hz
    // Byte 38 bit 0: 640x480 @ 60Hz
    // Byte 39 bit 3: 1024x768 @ 60Hz
    if (edid[38] & 0x80) != 0 {
        modes.push(DrmModeInfo {
            hdisplay: 800,
            vdisplay: 600,
            refresh: 60,
            name: "800x600".to_string(),
        });
    }
    if (edid[38] & 0x01) != 0 {
        modes.push(DrmModeInfo {
            hdisplay: 640,
            vdisplay: 480,
            refresh: 60,
            name: "640x480".to_string(),
        });
    }
    if (edid[39] & 0x08) != 0 {
        modes.push(DrmModeInfo {
            hdisplay: 1024,
            vdisplay: 768,
            refresh: 60,
            name: "1024x768".to_string(),
        });
    }

    // Detailed Timing Descriptors (4 blocks of 18 bytes starting from byte 54)
    for block_offset in (54..126).step_by(18) {
        let pixel_clock = ((edid[block_offset + 1] as u16) << 8) | edid[block_offset] as u16;
        if pixel_clock != 0 {
            // Read timing parameters
            let h_active = edid[block_offset + 2] as u16 | (((edid[block_offset + 4] as u16) & 0xF0) << 4);
            let v_active = edid[block_offset + 5] as u16 | (((edid[block_offset + 7] as u16) & 0xF0) << 4);
            if h_active > 0 && v_active > 0 {
                let name = format!("{}x{}", h_active, v_active);
                modes.push(DrmModeInfo {
                    hdisplay: h_active,
                    vdisplay: v_active,
                    refresh: 60,
                    name,
                });
            }
        }
    }

    if modes.is_empty() {
        modes.push(DrmModeInfo {
            hdisplay: 1920,
            vdisplay: 1080,
            refresh: 60,
            name: "1920x1080".to_string(),
        });
    }

    Ok(modes)
}

impl Default for GpuDriver {
    fn default() -> Self {
        Self::new(1920, 1080)
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
    fn test_edid_block_parsing() {
        // Construct a valid mock 128-byte EDID block
        let mut edid = [0u8; 128];
        // 1. Write EDID header: 00 FF FF FF FF FF FF 00
        edid[0..8].copy_from_slice(&[0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00]);

        // 2. Set Established Timings
        edid[38] = 0x81; // Bit 7 (800x600) + Bit 0 (640x480)
        edid[39] = 0x08; // Bit 3 (1024x768)

        // 3. Set a detailed timing block starting at offset 54
        // Pixel clock non-zero
        edid[54] = 0x01;
        edid[55] = 0x01;
        // H active = 1280 (0x500): lower 8 bits (0x00) into byte 56, higher 4 bits (0x05) into byte 58 high nibble
        edid[56] = 0x00;
        edid[58] = 0x50; // high 4 bits for H active and H blanking
        // V active = 720 (0x2D0): lower 8 bits (0xD0) into byte 59, higher 4 bits (0x02) into byte 61 high nibble
        edid[59] = 0xD0;
        edid[61] = 0x20;

        // 4. Calculate proper checksum for byte 127
        let mut sum: u8 = 0;
        for i in 0..127 {
            sum = sum.wrapping_add(edid[i]);
        }
        edid[127] = (256u16 - sum as u16) as u8;

        let modes = parse_edid(&edid).unwrap();
        // Should parse 800x600, 640x480, 1024x768 established modes, and 1280x720 detailed timing mode
        assert!(modes.iter().any(|m| m.hdisplay == 800 && m.vdisplay == 600));
        assert!(modes.iter().any(|m| m.hdisplay == 640 && m.vdisplay == 480));
        assert!(modes.iter().any(|m| m.hdisplay == 1024 && m.vdisplay == 768));
        assert!(modes.iter().any(|m| m.hdisplay == 1280 && m.vdisplay == 720));
    }

    #[test]
    fn test_atomic_commit_dry_run_and_apply() {
        let mut gpu = GpuDriver::new(800, 600);

        // Seed with active components
        gpu.crtc = Some(DrmCrtc {
            id: 10,
            x: 0,
            y: 0,
            width: 800,
            height: 600,
            active: false,
        });

        gpu.planes.push(DrmPlane {
            id: 20,
            plane_type: DrmPlaneType::Primary,
            possible_crtcs: 1,
            crtc_id: None,
            fb_id: None,
            src_x: 0,
            src_y: 0,
            src_w: 800,
            src_h: 600,
            crtc_x: 0,
            crtc_y: 0,
            crtc_w: 800,
            crtc_h: 600,
        });

        // Create transactional property commit
        let mut commit = AtomicCommit::new();
        commit.add_property(10, "active", 1);
        commit.add_property(20, "crtc_id", 10);
        commit.add_property(20, "fb_id", 300);

        // 1. Dry run validation (test-only)
        commit.set_test_only(true);
        let res_validation = gpu.commit_atomic(&commit);
        assert_eq!(res_validation, Ok(()));
        // Dry-run must not mutate values in-place yet
        assert_eq!(gpu.crtc.as_ref().unwrap().active, false);
        assert_eq!(gpu.planes[0].crtc_id, None);

        // 2. Real transactional commit
        commit.set_test_only(false);
        let res_commit = gpu.commit_atomic(&commit);
        assert_eq!(res_commit, Ok(()));
        // Mutated successfully
        assert_eq!(gpu.crtc.as_ref().unwrap().active, true);
        assert_eq!(gpu.planes[0].crtc_id, Some(10));
        assert_eq!(gpu.planes[0].fb_id, Some(300));
    }

    #[test]
    fn test_connector_edid_configuration() {
        let mut gpu = GpuDriver::new(1024, 768);
        gpu.connector = Some(DrmConnector {
            id: 100,
            connected: false,
            modes: Vec::new(),
        });

        let mut edid = [0u8; 128];
        edid[0..8].copy_from_slice(&[0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00]);
        edid[38] = 0x80; // 800x600 @ 60Hz
        let mut sum: u8 = 0;
        for i in 0..127 { sum = sum.wrapping_add(edid[i]); }
        edid[127] = (256u16 - sum as u16) as u8;

        let res = gpu.parse_connector_edid(100, &edid);
        assert_eq!(res, Ok(()));

        let conn = gpu.connector.as_ref().unwrap();
        assert!(conn.connected);
        assert!(conn.modes.iter().any(|m| m.hdisplay == 800 && m.vdisplay == 600));
    }

    #[test]
    fn test_double_buffered_vblank_page_flip() {
        let mut gpu = GpuDriver::new(800, 600);
        gpu.crtc = Some(DrmCrtc {
            id: 1,
            x: 0,
            y: 0,
            width: 800,
            height: 600,
            active: true,
        });

        let fb1_id = gpu.create_dumb_buffer(1024, 768, 32).unwrap();
        let fb2_id = gpu.create_dumb_buffer(1280, 720, 32).unwrap();

        assert_eq!(gpu.vblank_count, 0);

        // Page Flip 1
        let res1 = gpu.execute_page_flip(1, fb1_id);
        assert_eq!(res1, Ok(()));
        assert_eq!(gpu.vblank_count, 1);
        assert_eq!(gpu.width, 1024);
        assert_eq!(gpu.height, 768);

        // Page Flip 2
        let res2 = gpu.execute_page_flip(1, fb2_id);
        assert_eq!(res2, Ok(()));
        assert_eq!(gpu.vblank_count, 2);
        assert_eq!(gpu.width, 1280);
        assert_eq!(gpu.height, 720);
    }
}
