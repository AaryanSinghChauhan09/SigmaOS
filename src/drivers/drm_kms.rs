//! DRM (Direct Rendering Manager) and KMS (Kernel Mode Setting)
//! Modern graphics driver infrastructure inspired by Linux DRM subsystem
//! Reference: Linux drivers/gpu/drm/

#![no_std]

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicU32, Ordering};

/// DRM device types
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DrmDeviceType {
    Primary = 0,
    Control = 1,
    Render = 2,
}

/// Display connector types (from DRM)
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DrmConnectorType {
    Unknown = 0,
    Vga = 1,
    DviI = 2,
    DviD = 3,
    DviA = 4,
    Composite = 5,
    SVideo = 6,
    Lvds = 7,
    Component = 8,
    Dp = 10,    // DisplayPort
    Hdmi = 11,  // HDMI Type A
    HdmiB = 12, // HDMI Type B
    Tv = 13,
    Edp = 14, // Embedded DisplayPort
    Virtual = 15,
    Dsi = 16, // DSI (Mobile)
    Dpi = 17, // DPI (Mobile)
    Writeback = 18,
    Spi = 19,
    UsbC = 20,
}

/// Connector status
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DrmConnectorStatus {
    Connected = 1,
    Disconnected = 2,
    Unknown = 3,
}

/// Display mode flags
#[derive(Debug, Clone, Copy)]
pub struct DrmModeFlags {
    pub interlace: bool,
    pub doublescan: bool,
    pub csync: bool,
    pub pvsync: bool, // Positive vsync
    pub nvsync: bool, // Negative vsync
    pub phsync: bool, // Positive hsync
    pub nhsync: bool, // Negative hsync
}

/// Display mode timing information
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct DrmModeInfo {
    pub clock: u32,       // Pixel clock in kHz
    pub hdisplay: u16,    // Horizontal display size
    pub hsync_start: u16, // Horizontal sync start
    pub hsync_end: u16,   // Horizontal sync end
    pub htotal: u16,      // Horizontal total size
    pub vdisplay: u16,    // Vertical display size
    pub vsync_start: u16, // Vertical sync start
    pub vsync_end: u16,   // Vertical sync end
    pub vtotal: u16,      // Vertical total size
    pub vrefresh: u16,    // Refresh rate in Hz
    pub flags: u32,       // Mode flags
    pub name: [u8; 32],   // Mode name
}

impl DrmModeInfo {
    /// Create standard 1920x1080@60Hz mode
    pub fn mode_1080p_60() -> Self {
        Self {
            clock: 148500,
            hdisplay: 1920,
            hsync_start: 2008,
            hsync_end: 2052,
            htotal: 2200,
            vdisplay: 1080,
            vsync_start: 1084,
            vsync_end: 1089,
            vtotal: 1125,
            vrefresh: 60,
            flags: 0x5, // +hsync +vsync
            name: *b"1920x1080\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        }
    }

    /// Create standard 1280x720@60Hz mode
    pub fn mode_720p_60() -> Self {
        Self {
            clock: 74250,
            hdisplay: 1280,
            hsync_start: 1390,
            hsync_end: 1430,
            htotal: 1650,
            vdisplay: 720,
            vsync_start: 725,
            vsync_end: 730,
            vtotal: 750,
            vrefresh: 60,
            flags: 0x5,
            name: *b"1280x720\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        }
    }
}

/// Framebuffer pixel format (FourCC codes)
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DrmPixelFormat {
    Rgb565 = 0x36314752,   // 'RG16'
    Rgb888 = 0x34324752,   // 'RG24'
    Xrgb8888 = 0x34325258, // 'XR24'
    Argb8888 = 0x34325241, // 'AR24'
    Bgr565 = 0x36314742,   // 'BG16'
    Xbgr8888 = 0x34324258, // 'XB24'
    Abgr8888 = 0x34324241, // 'AB24'
}

impl DrmPixelFormat {
    pub fn bytes_per_pixel(&self) -> u32 {
        match self {
            DrmPixelFormat::Rgb565 | DrmPixelFormat::Bgr565 => 2,
            DrmPixelFormat::Rgb888 => 3,
            DrmPixelFormat::Xrgb8888
            | DrmPixelFormat::Argb8888
            | DrmPixelFormat::Xbgr8888
            | DrmPixelFormat::Abgr8888 => 4,
        }
    }
}

/// DRM Framebuffer object
#[derive(Debug, Clone)]
pub struct DrmFramebuffer {
    pub fb_id: u32,
    pub width: u32,
    pub height: u32,
    pub pitch: u32, // Bytes per scanline
    pub format: DrmPixelFormat,
    pub modifier: u64,   // Format modifier (tiling, compression)
    pub gem_handle: u32, // GEM buffer object handle
}

/// DRM CRTC (Cathode Ray Tube Controller - display controller)
pub struct DrmCrtc {
    pub crtc_id: u32,
    pub pipe: u32, // Hardware pipe index
    pub enabled: bool,
    pub mode: Option<DrmModeInfo>,
    pub fb_id: Option<u32>, // Current framebuffer
    pub x: i32,             // Panning offset X
    pub y: i32,             // Panning offset Y
}

impl DrmCrtc {
    pub fn new(crtc_id: u32, pipe: u32) -> Self {
        Self {
            crtc_id,
            pipe,
            enabled: false,
            mode: None,
            fb_id: None,
            x: 0,
            y: 0,
        }
    }

    /// Set display mode
    pub fn set_mode(&mut self, mode: DrmModeInfo, fb_id: u32) -> Result<(), DrmError> {
        self.mode = Some(mode);
        self.fb_id = Some(fb_id);
        self.enabled = true;
        Ok(())
    }

    /// Disable CRTC
    pub fn disable(&mut self) {
        self.enabled = false;
        self.mode = None;
        self.fb_id = None;
    }
}

/// DRM Encoder (converts CRTC output to connector signal)
pub struct DrmEncoder {
    pub encoder_id: u32,
    pub encoder_type: u32,
    pub possible_crtcs: u32,  // Bitmask of compatible CRTCs
    pub possible_clones: u32, // Bitmask of clone-able encoders
}

/// DRM Connector (physical display output)
pub struct DrmConnector {
    pub connector_id: u32,
    pub connector_type: DrmConnectorType,
    pub connector_type_id: u32,
    pub status: DrmConnectorStatus,
    pub modes: Vec<DrmModeInfo>,
    pub encoder_id: Option<u32>,
    pub dpms: DrmDpmsMode,
}

impl DrmConnector {
    pub fn new(connector_id: u32, connector_type: DrmConnectorType) -> Self {
        Self {
            connector_id,
            connector_type,
            connector_type_id: 0,
            status: DrmConnectorStatus::Unknown,
            modes: Vec::new(),
            encoder_id: None,
            dpms: DrmDpmsMode::Off,
        }
    }

    /// Detect connector and read EDID
    pub fn detect(&mut self) -> Result<DrmConnectorStatus, DrmError> {
        // In real implementation: probe hardware and read EDID via I2C/DDC
        // For now, simulate detection
        self.status = DrmConnectorStatus::Connected;

        // Add standard modes (in real implementation, these come from EDID)
        self.modes.push(DrmModeInfo::mode_1080p_60());
        self.modes.push(DrmModeInfo::mode_720p_60());

        // Add 2560x1440@60Hz mode
        self.modes.push(DrmModeInfo {
            clock: 241500,
            hdisplay: 2560,
            hsync_start: 2608,
            hsync_end: 2648,
            htotal: 2720,
            vdisplay: 1440,
            vsync_start: 1443,
            vsync_end: 1448,
            vtotal: 1481,
            vrefresh: 60,
            flags: 0x5,
            name: *b"2560x1440\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        });

        Ok(self.status)
    }
}

/// DPMS (Display Power Management Signaling) modes
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DrmDpmsMode {
    On = 0,
    Standby = 1,
    Suspend = 2,
    Off = 3,
}

/// GEM (Graphics Execution Manager) buffer object
pub struct DrmGemObject {
    pub handle: u32,
    pub size: u64,
    pub phys_addr: u64, // Physical address (or GPU address)
    pub flags: u32,
    pub refcount: AtomicU32,
}

impl DrmGemObject {
    pub fn new(handle: u32, size: u64) -> Self {
        Self {
            handle,
            size,
            phys_addr: 0,
            flags: 0,
            refcount: AtomicU32::new(1),
        }
    }
}

/// DRM atomic commit (for mode setting)
pub struct DrmAtomicCommit {
    pub flags: u32,
    pub crtc_updates: Vec<(u32, Option<DrmModeInfo>)>,
    pub fb_updates: Vec<(u32, u32)>,
    pub connector_updates: Vec<(u32, u32)>,
}

/// Main DRM device structure
pub struct DrmDevice {
    pub dev_type: DrmDeviceType,
    pub vendor_id: u16,
    pub device_id: u16,
    pub crtcs: Vec<DrmCrtc>,
    pub encoders: Vec<DrmEncoder>,
    pub connectors: Vec<DrmConnector>,
    pub framebuffers: BTreeMap<u32, DrmFramebuffer>,
    pub gem_objects: BTreeMap<u32, DrmGemObject>,
    next_id: AtomicU32,
}

impl DrmDevice {
    pub fn new(dev_type: DrmDeviceType, vendor_id: u16, device_id: u16) -> Self {
        Self {
            dev_type,
            vendor_id,
            device_id,
            crtcs: Vec::new(),
            encoders: Vec::new(),
            connectors: Vec::new(),
            framebuffers: BTreeMap::new(),
            gem_objects: BTreeMap::new(),
            next_id: AtomicU32::new(1),
        }
    }

    /// Allocate new object ID
    fn alloc_id(&self) -> u32 {
        self.next_id.fetch_add(1, Ordering::SeqCst)
    }

    /// Initialize device and enumerate outputs
    pub fn init(&mut self) -> Result<(), DrmError> {
        // Create CRTCs (typically 2-4 per GPU)
        for pipe in 0..2 {
            let crtc = DrmCrtc::new(self.alloc_id(), pipe);
            self.crtcs.push(crtc);
        }

        // Create connectors
        let connector = DrmConnector::new(self.alloc_id(), DrmConnectorType::Hdmi);
        self.connectors.push(connector);

        // Detect connected displays
        for connector in &mut self.connectors {
            connector.detect()?;
        }

        Ok(())
    }

    /// Create GEM buffer object
    pub fn gem_create(&mut self, size: u64) -> Result<u32, DrmError> {
        let handle = self.alloc_id();
        let gem = DrmGemObject::new(handle, size);
        self.gem_objects.insert(handle, gem);
        Ok(handle)
    }

    /// Create framebuffer
    pub fn add_framebuffer(
        &mut self,
        width: u32,
        height: u32,
        format: DrmPixelFormat,
        gem_handle: u32,
    ) -> Result<u32, DrmError> {
        if !self.gem_objects.contains_key(&gem_handle) {
            return Err(DrmError::InvalidGemHandle);
        }

        let fb_id = self.alloc_id();
        let pitch = width * format.bytes_per_pixel();

        let fb = DrmFramebuffer {
            fb_id,
            width,
            height,
            pitch,
            format,
            modifier: 0,
            gem_handle,
        };

        self.framebuffers.insert(fb_id, fb);
        Ok(fb_id)
    }

    /// Set CRTC mode (mode setting)
    pub fn set_crtc(
        &mut self,
        crtc_id: u32,
        fb_id: u32,
        mode: DrmModeInfo,
    ) -> Result<(), DrmError> {
        let crtc = self
            .crtcs
            .iter_mut()
            .find(|c| c.crtc_id == crtc_id)
            .ok_or(DrmError::InvalidCrtc)?;

        if !self.framebuffers.contains_key(&fb_id) {
            return Err(DrmError::InvalidFramebuffer);
        }

        crtc.set_mode(mode, fb_id)?;

        // In real implementation: program hardware registers
        Ok(())
    }

    /// Page flip (atomic buffer swap)
    pub fn page_flip(&mut self, crtc_id: u32, fb_id: u32) -> Result<(), DrmError> {
        let crtc = self
            .crtcs
            .iter_mut()
            .find(|c| c.crtc_id == crtc_id)
            .ok_or(DrmError::InvalidCrtc)?;

        if !self.framebuffers.contains_key(&fb_id) {
            return Err(DrmError::InvalidFramebuffer);
        }

        crtc.fb_id = Some(fb_id);

        // In real implementation: wait for vblank and flip
        Ok(())
    }

    /// Get connector by ID
    pub fn get_connector(&self, connector_id: u32) -> Option<&DrmConnector> {
        self.connectors
            .iter()
            .find(|c| c.connector_id == connector_id)
    }
}

/// DRM error types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DrmError {
    InvalidCrtc,
    InvalidEncoder,
    InvalidConnector,
    InvalidFramebuffer,
    InvalidGemHandle,
    InvalidMode,
    NoMemory,
    Busy,
    NotSupported,
}

#[cfg(test)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_drm_device_init() {
        let mut dev = DrmDevice::new(DrmDeviceType::Primary, 0x8086, 0x1916);
        assert!(dev.init().is_ok());
        assert!(dev.crtcs.len() > 0);
    }

    #[test]
    fn test_gem_create() {
        let mut dev = DrmDevice::new(DrmDeviceType::Primary, 0x8086, 0x1916);
        let handle = dev.gem_create(4096).unwrap();
        assert!(dev.gem_objects.contains_key(&handle));
    }

    #[test]
    fn test_mode_timing() {
        let mode = DrmModeInfo::mode_1080p_60();
        assert_eq!(mode.hdisplay, 1920);
        assert_eq!(mode.vdisplay, 1080);
        assert_eq!(mode.vrefresh, 60);
    }

    #[test]
    fn test_pixel_format_bpp() {
        assert_eq!(DrmPixelFormat::Rgb565.bytes_per_pixel(), 2);
        assert_eq!(DrmPixelFormat::Xrgb8888.bytes_per_pixel(), 4);
    }
}
